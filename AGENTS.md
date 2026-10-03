# Repository guidance

This repository contains AGENT ORCHESTRATOR's portable plugin and optional Codex
profiles. Keep changes scoped and preserve unrelated edits. Do not alter the user's
global configuration, install dependencies or launch network services as part of
ordinary package maintenance.

Read [docs/README.md](docs/README.md) for the lifecycle, capability boundaries and
publication process. Keep narrative documentation under docs/; retain this file,
README.md and skills/*/SKILL.md at their required entry locations.

The coordinator owns integration. Delegate independent analysis/review when useful,
with explicit ownership and permitted side effects. Inspect actual evidence before
accepting agent output. External A2A requires an explicitly selected, authorized service.

Package checks require Rust 1.94.1+ and Git:

```text
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib --tests
cargo run --locked -- validate-package --root .
```

Tests create and clean isolated Git fixtures under the system temporary directory. These verify the evidence
checker; they do not prove a user's application passed E2E. Review affected skills,
references, metadata and permission boundaries independently. Exclude caches,
credentials and personal configuration from release archives. Preserve Apache-2.0
license and attribution. Push, submission and publication require the intended target
and the user's authorization; verify actual hosted CI after an authorized push.
