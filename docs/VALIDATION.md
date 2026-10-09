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
| Historical OSV dependency query | 72 locked registry packages checked; zero known advisories returned | Superseded by the exact 129-entry 0.4.0 lock scan below; limited database coverage |
| Independent source/security/evidence/docs review | Accepted after two planning corrections | Same-source evidence invalidation and stale/self-review chart labels; quality/source-review.md |
| Hosted CI for 0.3.0 | Passed after PR #4 merge | Merged-main run [37094555223](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37094555223) passed all seven jobs |
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
by Linux CI. Hosted results for 0.3.0 were subsequently inspected and are linked above;
current 0.4.0 results are recorded in the next section.

## 0.4.0 MCP development validation

PR #7 at head c968d8b (merged as 8e2ddc1) passed PR-head runs 37902677375 and 37902672193.
The merge-commit run 37903287143 passed Linux and Windows validate/test/E2E jobs and final-gate;
its E2E jobs exercised the optimized CLI and MCP test suites and package validation. See the
[PR run](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37902677375) and
[merge-commit run](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37903287143).

The merged Cargo.lock has 130 entries: 129 registry packages and the local workspace package.
The current OSV exact-version batch returned 129 results with zero advisory matches. Crates.io
metadata covered all 129 registry rows: no license fields were missing and no versions were yanked.
There are 127 expressions with permissive options and zero copyleft-only expressions. Two
Unlicense/MIT fields use non-SPDX slash notation but upstream docs/manifests identify both licenses;
unicode-ident requires retaining its Unicode-3.0 notice. Full details are in the [scan record](quality/dependency-scan.json)
and [license inventory](quality/dependency-license-inventory.csv). Independent review of the current
local stdio source found no security or reliability issue; hosted HTTP/authentication and external
agent dispatch remain unverified.

The follow-up branch adds regression tests for rolling notification expiry and four-call
pipelining, including a compiled-server subprocess journey. All current-source Windows local gates
passed. PR #8 and its merge-commit run also passed exact-head and post-merge Linux/Windows CI;
PR #7 results above cover the earlier source revision only.

### T18 follow-up local results

Windows with Rust 1.94.1, 9 October 2026. An isolated Cargo target directory avoided a transient
Windows linker lock in the default target. On the current follow-up worktree:

| Check | Result |
| --- | --- |
| `cargo fmt --check` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo test --locked --lib --tests -- --test-threads=1` | 59 passed, 0 failed (19 library, 8 CLI, 7 gate, 9 MCP, 16 planning) |
| `cargo build --locked --release` | Passed |
| `cargo test --locked --release --test cli` | 8 passed, 0 failed |
| `cargo test --locked --release --test mcp -- --test-threads=1` | 9 passed, 0 failed, including the four-call subprocess journey |
| Debug and release `validate-package --root .` | Passed: 14 skills and 14 profiles; official-schema validation is a separate check |
| Independent current-source MCP review | No security or reliability finding in local stdio scope; hosted HTTP/auth and external agents were not assessed; [review record](quality/mcp-transport-review.md) |

The current follow-up merge commit `e2ac7150e875d834c97e7ef4bdaf3beb2e7a24c4` was validated by
post-merge run [37912634478](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37912634478).
All seven jobs passed: Linux/Windows validate, test and E2E plus final-gate. Its PR-head run
[37910731823](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37910731823) also
passed all seven jobs before merge. PR #8 is [merged](https://github.com/NAVANEETHVVINOD/agent-orchestrator/pull/8).

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

PR #4 was merged after its current-source review and required checks. Its older PR CI
run on head 446504bad6a1ac7325366d6f8f26f80e366029a4 is recorded
[here](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37057788525);
those old checks cannot establish readiness for later source.

Live `main` protection verified 9 October 2026 requires `final-gate` and resolved
conversations, applies to administrators, disables force pushes and deletion, and has
zero required GitHub approvals after the user's explicit request. Independent source
review remains a project delivery gate; it is separate from GitHub's approval count.
The same protection setting must not be mistaken for a passed review on a future PR.
