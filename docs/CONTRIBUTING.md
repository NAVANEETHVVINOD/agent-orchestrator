# Contribute

Open a scoped issue with observable acceptance criteria, then use a codex/ branch and
pull request. Preserve existing host-specific rules and user capabilities. Read the
root AGENTS.md and [RUST.md](RUST.md); keep narrative documents here under docs/.

Run fmt, clippy, tests, release build and the real consumer validator before a code
push. Inspect actual E2E and independent review evidence. No production placeholder,
invented integration or skipped required check can establish completion. Changes to
the source invalidate affected evidence. Hosted final-gate must succeed on the
current revision; branch protection and reviewer approval remain separate requirements.

For new integrations, include official source/version/license, host compatibility,
data and permission scope, actual registration and target-application E2E. Provide
rollback and sanitized evidence. An app-specific adapter is a separately reviewed
capability; broad installer commands or popularity do not establish trust.

Dependency updates change Cargo.lock and must repeat relevant tests and advisory
checks. Review scanner reachability and business logic rather than assuming a clean
database result proves safety. Use private reporting where a verified publisher
channel is available; do not publish live credentials or exploit account details.
