use crate::ReleaseAsset;
use anyhow::{Context as _, Result};
use http_client::{HttpClient, HttpClientWithUrl};
use release_channel::ReleaseChannel;
use serde::Deserialize;
use smol::io::AsyncReadExt;
use std::sync::Arc;

pub const PACKAGE_MANAGER_INSTALL_TITLE: &str = "Stellaris was installed via a package manager.";
pub const UP_TO_DATE_TITLE: &str = "Stellaris is up to date";
pub const UP_TO_DATE_MESSAGE: &str = "You're already running the latest version.";

/// The GitHub Releases API URL embedded in release builds.
///
/// A source build has no update source unless its build environment supplies
/// `STELLARIS_UPDATE_URL`; runtime environment variables deliberately do not
/// enable updates.
pub fn update_base_url(channel: ReleaseChannel) -> Option<&'static str> {
    match channel {
        ReleaseChannel::Stable | ReleaseChannel::Preview | ReleaseChannel::Nightly => {
            option_env!("STELLARIS_UPDATE_URL").filter(|url| !url.is_empty())
        }
        ReleaseChannel::Dev => None,
    }
}

pub fn release_notes_url(channel: ReleaseChannel) -> Option<String> {
    let api_url = update_base_url(channel)?;
    api_url
        .strip_prefix("https://api.github.com/repos/")
        .and_then(|rest| rest.split_once("/releases"))
        .map(|(owner_repo, _)| format!("https://github.com/{owner_repo}/releases"))
        .or_else(|| Some(api_url.to_string()))
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize, Clone)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

fn channel_tag_suffix(channel: ReleaseChannel) -> Option<&'static str> {
    match channel {
        ReleaseChannel::Stable => Some(""),
        ReleaseChannel::Preview => Some("-preview"),
        ReleaseChannel::Nightly => Some("-nightly"),
        ReleaseChannel::Dev => None,
    }
}

fn release_matches_channel(release: &GitHubRelease, channel: ReleaseChannel) -> bool {
    if release.draft {
        return false;
    }

    let Some(suffix) = channel_tag_suffix(channel) else {
        return false;
    };

    if suffix.is_empty() {
        !release.tag_name.contains('-')
    } else {
        release.tag_name.ends_with(suffix)
    }
}

pub struct ReleaseNotes {
    pub title: String,
    pub notes: String,
}

pub async fn get_release_notes(
    http_client: Arc<HttpClientWithUrl>,
    api_url: &str,
    channel: ReleaseChannel,
    version: &str,
) -> Result<ReleaseNotes> {
    let releases = get_releases(http_client, api_url).await?;
    let suffix = channel_tag_suffix(channel).unwrap_or_default();
    let version = version.trim_start_matches('v');
    let tag = format!("v{version}{suffix}");

    let release = releases
        .iter()
        .find(|release| !release.draft && release.tag_name == tag)
        .or_else(|| {
            releases
                .iter()
                .find(|release| release_matches_channel(release, channel))
        })
        .with_context(|| format!("no release notes found for {tag} at {api_url}"))?;

    let notes = release
        .body
        .clone()
        .filter(|body| !body.trim().is_empty())
        .with_context(|| format!("release {} has no release notes", release.tag_name))?;

    Ok(ReleaseNotes {
        title: release
            .name
            .clone()
            .filter(|name| !name.trim().is_empty())
            .unwrap_or_else(|| release.tag_name.clone()),
        notes,
    })
}

async fn get_releases(
    http_client: Arc<HttpClientWithUrl>,
    api_url: &str,
) -> Result<Vec<GitHubRelease>> {
    let mut response = http_client.get(api_url, Default::default(), true).await?;
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "failed to fetch release from {api_url}: {:?}",
        String::from_utf8_lossy(&body),
    );

    if let Ok(single) = serde_json::from_slice::<GitHubRelease>(&body) {
        Ok(vec![single])
    } else {
        serde_json::from_slice(&body).with_context(|| {
            format!(
                "error deserializing release(s) {:?}",
                String::from_utf8_lossy(&body),
            )
        })
    }
}

pub async fn get_release_asset(
    http_client: Arc<HttpClientWithUrl>,
    api_url: &str,
    channel: ReleaseChannel,
    os: &str,
    arch: &str,
) -> Result<ReleaseAsset> {
    let releases = get_releases(http_client, api_url).await?;
    let release = releases
        .into_iter()
        .find(|release| release_matches_channel(release, channel))
        .with_context(|| format!("no release matched channel {channel:?} at {api_url}"))?;

    let matches = |asset: &GitHubAsset, extension: &str| {
        asset.name.starts_with("Stellaris-")
            && asset.name.ends_with(extension)
            && asset.name.contains(arch)
            && (os != "linux" || asset.name.contains("linux"))
    };

    let asset = match os {
        "macos" => release
            .assets
            .iter()
            .find(|asset| matches(asset, ".dmg"))
            .or_else(|| release.assets.iter().find(|asset| matches(asset, ".zip")))
            .cloned(),
        "linux" => release
            .assets
            .iter()
            .find(|asset| matches(asset, ".tar.gz"))
            .cloned(),
        "windows" => release
            .assets
            .iter()
            .find(|asset| matches(asset, ".exe"))
            .cloned(),
        other => anyhow::bail!("unsupported os: {other}"),
    }
    .with_context(|| {
        format!(
            "no matching Stellaris asset for os={os} arch={arch} in release {}",
            release.tag_name
        )
    })?;

    Ok(ReleaseAsset {
        version: release.tag_name.trim_start_matches('v').to_string(),
        url: asset.browser_download_url,
    })
}
