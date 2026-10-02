# Validation evidence

Current local validation: Windows, Rust 1.94.1, Git, 3 October 2026.

| Check | Observed result | Scope / limit |
| --- | --- | --- |
| cargo fmt --check | Passed | Current Rust source |
| cargo clippy --locked --all-targets -- -D warnings | Passed | Current source and tests |
| cargo test --locked --lib --tests with isolated target directory | 21 passed, zero failed/ignored | 9 unit + 5 CLI consumer + 7 real-Git integration cases |
| cargo test --locked --release --lib --tests | 21 passed, zero failed/ignored | Same journeys in optimized build |
| cargo build --locked --release | Passed | Actual Rust executable |
| Release CLI validate-package --root . | Passed: 14 skills, 14 profiles | Structural/path/privacy checks; not comprehensive secret or official schema validation |
| Official Agent Plugins 1.0.0 manifest schema | Passed via separate build-time jsonschema validator | Downloaded official schema; no new runtime dependency |
| OSV dependency query | 72 pinned registry packages checked, zero known advisories returned | Current Cargo.lock hash recorded in quality/dependency-scan.json; database coverage is limited |
| Independent source/security review | Passed after five corrections | Exact decimals, reserved JSON tag, gitlinks, CRLF and fixture hooks/signing; see quality/source-review.md |
| Hosted CI | Pending initial authorized push | Must inspect actual current revision; static workflow review is not a remote pass |
| Plugin directory / live adapters / RAG / models / A2A | Unverified | No provider, host install, account or remote service is simulated |

CLI E2E exercises real compiled executable invocation, extracted/copied package validation,
CRLF portability, missing targets/private config rejection, typed bounded routes and
invalid inputs, real Git snapshot/report validation, stale source rejection and missing
merge CI rejection. Gate report contents are synthetic fixtures explicitly labeled as
such; they are not claims that a business application passed tests. Two Unix-only cases
for symlink/execute-bit behavior await Linux hosted CI.

The original Windows debug target later hit LNK1104 output-file access failures. A
clean isolated target rebuilt and passed the same complete debug suite, and the release
suite also passed. No failed run is counted as success; no checks were disabled.

Package helpers were migrated from Python to Rust. Research-only external Python
references remain valid. No CO_OP desktop application was rewritten or tested here;
its source analysis and limitations are documented in CO-OP-ADAPTATION.md.

Archives exclude target/, .git/, caches, bytecode, credentials and personal configuration.
BUILD-INFO.json lists source hashes, while the adjacent checksum covers the archive.
Hashes establish consistency, not authorship, trustworthy evidence or complete safety.
Required local review and real consumer checks precede a code push; hosted verification
and branch rules are assessed separately. OpenAI directory approval is separate.
