# Validation evidence

Version 0.3.0 local validation: Windows, Rust 1.94.1, Git, 3 October 2026.

| Check | Observed result | Scope / limit |
| --- | --- | --- |
| cargo fmt --check | Passed | Current Rust source |
| cargo clippy --locked --all-targets -- -D warnings, isolated target | Passed | Source and tests |
| cargo test --locked --lib --tests, isolated target | 39 passed, zero failed/ignored | 9 unit, 7 compiled CLI consumer, 7 real-Git and 16 planning cases |
| cargo test --locked --release --lib --tests | 39 passed, zero failed/ignored | Same actual journeys in optimized build |
| cargo build --locked --release | Passed | Actual Rust executable |
| Release CLI validate-package --root . | Passed: 14 skills, 14 profiles | Structural/path/privacy checks; no official-schema claim |
| Official Agent Plugins 1.0.0 manifest schema | Passed via separate jsonschema validator | Downloaded official schema; no runtime dependency added |
| Skill Creator quick checks and UI YAML parsing | Passed: 14 skills and 14 UI records | Authoring validation; host discovery is separate |
| OSV dependency query | 72 locked registry packages checked; zero known advisories returned | Current lock hash in quality/dependency-scan.json; limited database coverage |
| Independent source/security/evidence/docs review | Accepted after two planning corrections | Same-source evidence invalidation and stale/self-review chart labels; quality/source-review.md |
| Hosted CI for this new source | Pending push and exact-head verification | Required Windows/Linux validation, full tests and optimized CLI E2E; final-gate requires all success |
| Directory listing / live adapters / RAG / models / A2A | Unverified | No provider, target application or remote agent service simulated |

CLI E2E invokes the actual compiled executable. It covers package validation,
CRLF portability, missing Markdown/private config rejection, bounded routes and
malformed inputs, real Git snapshot/report/source freshness and missing merge CI,
plus planning validation, bounded wave, safely escaped chart and the full
work/review/fix/review/acceptance journey. Failed/skipped records, stale events,
unknown runtime capabilities, graph errors and oversized JSON fail as expected.
Report and planning evidence contents are explicitly synthetic fixtures used to test
validators; they are not claims of application QA or authenticated reviewer identity.

Planning regressions cover dependencies/cycles, bounded counts, reported live slots,
active and selected ownership conflicts, case semantics, evidence invalidation on
source changes and same-source correction/restart, leaves-first downstream invalidation,
reviewer claims and chart injection. Two additional Unix-only Git cases are exercised
by Linux CI. Actual hosted results must be inspected after this push.

The previous 0.2.0 source passed 21 Windows and 23 hosted Linux tests. Its debug target
once hit LNK1104 output access failures; a clean isolated build and release suite
passed without disabling checks. The 0.3.0 isolated/debug and release runs above passed.
Package helpers use Rust. No CO_OP desktop application was rewritten or executed here;
source-grounded adaptation limits are in [CO-OP-ADAPTATION.md](CO-OP-ADAPTATION.md).

Archives exclude build outputs, Git internals, caches, credentials and personal config.
BUILD-INFO.json indexes authored source hashes; archive checksums and consumer checks
establish consistency, not provenance, truthful evidence or complete safety.
Actual local QA and independent review precede feature-code pushes. Hosted checks,
protected reviews and release authority remain separate gates.

[PR #4](https://github.com/NAVANEETHVVINOD/agent-orchestrator/pull/4) requires final-gate,
one approving GitHub review and resolved conversations; protection applies to admins.
Force pushes/deletion are disabled and private vulnerability reporting is enabled.
The prior head 446504bad6a1ac7325366d6f8f26f80e366029a4 passed its
[PR CI](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37057788525).
Those old checks cannot establish readiness for the new 0.3.0 source.
