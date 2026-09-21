# Stellaris roadmap

## Completed foundation

- [x] Establish Stellaris project identity and document its direction.
- [x] Add a nightly upstream-sync workflow that creates a reviewable PR instead of updating the default branch directly.
- [x] Set the initial application identity to `com.blutech.stellaris` and version to `1.0.0`.
- [x] Add unsigned macOS Apple Silicon and Linux x86_64 packaging.
- [x] Add a manual GitHub-release workflow and GitHub Releases updater contract.

## Release readiness

- [ ] Run the first `1.0.0` release workflow from a reviewed `main` commit and verify both published assets install correctly.
- [ ] Replace inherited Zed app icons with original Stellaris artwork before making a public release.
- [ ] Add macOS code signing and notarization once Apple Developer credentials are available.
- [ ] Add a release-notes process for Stellaris-specific changes and upstream-sync releases.
- [ ] Document the supported installation and update paths for macOS and Linux.
- [ ] Decide whether to add macOS Intel and Windows artifacts after the first release is stable.
- [ ] Keep `crates/zed/Cargo.toml` and `Cargo.lock` versions aligned before each release; the release workflow intentionally rejects a mismatched version input.
- [ ] Evaluate adopting Conventional Commits and semantic-release for Stellaris-owned changes. Define `fix` as a patch release, `feat` as a minor release, and explicit breaking-change markers as a major release.
- [ ] Define the semantic-release source of truth: conventional commit messages, validated pull-request labels, or both. Do not infer releases solely from an AI-generated summary.
- [ ] Define an upstream-sync policy before enabling semantic releases: Zed merge commits and imported upstream history must not independently trigger Stellaris version bumps or releases.
- [ ] Keep a human approval gate before publication until upstream synchronization and fork-specific CI are proven stable; semantic-release can calculate the next version and changelog without immediately publishing it.

## Upstream maintenance

- [ ] Enable write-scoped `GITHUB_TOKEN` permissions in repository Actions settings so the sync and release workflows can create PRs and releases.
- [ ] Run the upstream-sync workflow manually once to verify it can create `automation/sync-zed-main` in the GitHub repository.
- [ ] Define a conflict-resolution procedure for upstream-sync PRs and validate Stellaris-specific behavior after every upstream merge.
- [ ] Add targeted CI coverage for fork-owned functionality before enabling unattended releases.

## Git workflow improvements from Lathe

- [ ] Compare Lathe's thematic commits with current Stellaris and identify the minimal dependency set for each feature.
- [ ] Port interactive rebase first, adapting it to current `git_ui` APIs rather than importing a broad historical merge.
- [ ] Reproduce and fix the reported interactive-rebase base-commit regression: selecting a commit, configuring later commits, and starting the rebase must retain the selected base.
- [ ] Add tests for interactive-rebase actions, cancellation, failure recovery, and repository refresh after completion.
- [ ] Port improved merge-conflict resolution UX after interactive rebase is stable.
- [ ] Evaluate and selectively port Git explorer, branch tree, undo, and Git Flow features.

## GitHub, GitLab, and Bitbucket reviews

- [ ] Inventory which provider capabilities already exist in `git_hosting_providers` and `git_ui` before adding duplicate plumbing.
- [ ] Add authenticated provider clients using secure credential storage.
- [ ] Implement in-editor pull-request/merge-request review in provider-specific layers.
- [ ] Support GitHub, GitLab, and Bitbucket independently; do not assume GitHub API semantics for all providers.
- [ ] Add integration tests using provider API fixtures and clear user-visible authentication errors.

## Jira integration

- [ ] Design a dedicated Jira integration crate/module rather than coupling Jira API logic to `git_ui`.
- [ ] Start with Jira Cloud and secure account/token configuration; define a follow-up path for self-hosted Jira.
- [ ] Add project selection, assigned/open issue list, and issue detail UI.
- [ ] Implement a configurable "Start work" flow: create or check out a feature branch and transition the Jira issue to an in-progress status.
- [ ] Add issue creation, comments, work logs, sprint/board filters, and pull-request linking only after the first vertical slice is stable.

## Knowledge management and agent history

- [ ] Decide the Suzuri import scope: its existing fork focuses on Markdown live preview, wikilinks, backlinks, PDF viewing, and Typst/LaTeX, not AI-history storage.
- [ ] If importing Suzuri capabilities, port fork-owned crates selectively and keep them independent from agent-thread persistence.
- [ ] Design an opt-in local export or mirror of agent threads to an Obsidian-compatible vault.
- [ ] Define transcript privacy controls, redaction behavior, attachment handling, and a stable Markdown naming/layout convention before writing exported data.

## Panda Note evaluation

Panda Note is a Zed-native Markdown notes concept with a separate desktop fork, a self-hosted content-addressed sync/MCP server, and a mobile client. Its top-level repository is currently documentation-oriented; the server is private, so Stellaris cannot directly reuse its implementation without separate access and license review.

- [ ] Assess Panda Note's public desktop fork, its divergence from upstream Zed, its license, and its reusable GPUI/editor components.
- [ ] Compare Panda Note's Markdown-first note workflow with Suzuri's live preview, wikilinks, backlinks, PDF, and typesetting scope to avoid importing overlapping note-editor surfaces.
- [ ] Validate whether Stellaris should offer an IDE-first project chooser with distinct entry points for a code project and a local note vault, while retaining one workspace/editor architecture.
- [ ] Prototype the smallest useful interface: create or open a code project, or create or open a Markdown note vault, without adding a second application mode, webview, or sync backend.
- [ ] Determine whether note-vault behavior can remain local-file based and optional; do not make a hosted account, self-hosted server, or content-addressed synchronization system a prerequisite for Stellaris notes.
- [ ] Evaluate whether an agent-facing local/MCP interface would complement agent-history export and vault workflows, including authorization and privacy boundaries.
- [ ] Make a go/no-go decision after the prototype. Continue only if the feature is useful alongside a first-class IDE, remains modular, and does not substantially increase upstream-sync or release-maintenance cost.
