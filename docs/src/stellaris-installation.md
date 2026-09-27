---
title: Install Stellaris
description: Install Stellaris release builds on macOS and Linux.
---

# Install Stellaris

When a Stellaris release is available, download the matching asset from the
[Stellaris releases page](https://github.com/eduardoribeiro/stellaris/releases).
Initial release builds support Apple Silicon macOS and x86_64 Linux.

> [!NOTE]
> The macOS release is currently unsigned and not notarized. Windows and Intel
> macOS release artifacts are not available yet.

## macOS {#macos}

Download `Stellaris-<version>-aarch64-macos.zip`, then unzip it and move
`Stellaris.app` to `/Applications` or another location you control.

Because the app is unsigned, macOS may warn that it cannot verify the
developer. Open the app from Finder with Control-click, then choose **Open** to
confirm that you want to run it.

The archive includes the app and its command-line client. To use the CLI from
a terminal, run:

```sh
/Applications/Stellaris.app/Contents/MacOS/cli
```

## Linux {#linux}

Download `Stellaris-<version>-x86_64-linux.tar.gz`, then extract it into
`~/.local`:

```sh
mkdir -p ~/.local
tar -xzf Stellaris-<version>-x86_64-linux.tar.gz -C ~/.local
```

This installs the app at `~/.local/stellaris.app`. Start it with:

```sh
~/.local/stellaris.app/bin/stellaris
```

To make `stellaris` available on your `PATH`, create a stable symlink:

```sh
mkdir -p ~/.local/bin
ln -sfn ../stellaris.app/bin/stellaris ~/.local/bin/stellaris
```

Ensure that `~/.local/bin` is in your `PATH`, then you can run:

```sh
stellaris
```

The archive also includes a desktop entry. To install it for your user after
creating the CLI symlink:

```sh
mkdir -p ~/.local/share/applications
cp ~/.local/stellaris.app/share/applications/com.blutech.stellaris.desktop \
  ~/.local/share/applications/
```

## Updates {#updates}

Published Stellaris releases check GitHub Releases for newer versions and
download updates in the background. The update is applied when the app
restarts.

- On macOS, the downloaded ZIP replaces the running `Stellaris.app` bundle.
- On Linux, the downloaded archive replaces `~/.local/stellaris.app`. The
  `~/.local/bin/stellaris` symlink above continues to point to the updated
  command.

Updates require the `rsync` utility on both supported platforms. On Linux,
updates are supported for the archive installation described above; package
manager installations are not managed by Stellaris.

## Build from source {#build-from-source}

If your platform is not covered by a release artifact, build Stellaris from
source using the existing development instructions:

- [macOS](./development/macos.md)
- [Linux](./development/linux.md)
- [Windows](./development/windows.md)
