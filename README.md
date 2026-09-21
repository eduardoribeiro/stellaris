# Stellaris

Stellaris is a high-performance IDE built on [Zed](https://github.com/zed-industries/zed).

It preserves Zed's Rust-native performance, responsive interface, and upstream improvements while extending the editor with workflows that make it a more capable day-to-day IDE. Stellaris tracks Zed closely and selectively adds integrations and tooling that improve how developers plan, build, review, and ship software.

## Direction

Stellaris is focused on practical IDE capabilities, including:

- Deeper Git workflows, including interactive rebase, conflict resolution, and code review.
- First-class GitHub, GitLab, and Bitbucket integrations.
- Jira integration for viewing work, creating branches from issues, and transitioning tasks.
- Local knowledge-management and writing workflows inspired by Obsidian.
- Better agent workflows and durable, user-controlled history export.

Upstream Zed remains the foundation of Stellaris. Changes are designed to remain maintainable across regular upstream updates rather than replacing Zed's core architecture.

## Installation

Stellaris release builds are not available yet. Until then, build from source using Zed's platform-specific development instructions:

- [Building on macOS](./docs/src/development/macos.md)
- [Building on Linux](./docs/src/development/linux.md)
- [Building on Windows](./docs/src/development/windows.md)

## Development

Stellaris is a fork of Zed and follows its Rust workspace structure and development practices. See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidance.

## Upstream synchronization

Stellaris will track `zed-industries/zed` closely. Upstream updates are synchronized through reviewed pull requests so that Stellaris-specific features are tested before a release is published.

## Licensing

Stellaris inherits Zed's licensing model. Source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked.

License information for third-party dependencies must be correctly provided for CI to pass. Stellaris uses [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to validate dependency-license compliance.
