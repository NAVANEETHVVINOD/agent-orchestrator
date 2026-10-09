# Goal-linked delivery task list

Updated 9 October 2026. Goal: complete the selected AGENT ORCHESTRATOR product for
ChatGPT web and Codex, including a Rust MCP interface, reviewed capabilities,
professional agent workflow, real verification and current documentation.
Publication remains on hold until readiness has been established.

Symbols: blank = pending; `-` = working; `✓` = verified complete;
`✗` = blocked, failed or a confirmed issue requiring correction. A completed
implementation is not marked complete until its applicable review and QA pass.

The native Goals API exposes one objective/status, not per-task checkboxes. This
file is the detailed ledger for that active goal and is updated at actual task
transitions. It can stay open in the Codex file panel. No background monitor or
autonomous scheduling is implied.

| ID | Status | Task / acceptance | Owner | Dependencies | Evidence / QA | Next action |
| --- | --- | --- | --- | --- | --- | --- |
| T01 | ✓ | Recover existing work, preserve unrelated changes, create requested goal | Coordinator | None | Existing main/dirty publication doc inspected; goal created | Maintain goal and ledger |
| T02 | ✓ | Relocate complete active checkout to F:/kannan/projects/agent-orchestrator | Coordinator | T01 | Canonical Git root/origin verified; `CO_OS` confirmed as separate `NAVANEETHVVINOD/CO_OP` checkout with pre-existing uncommitted work preserved; support archive/research separated | Make source edits/builds only in canonical repo |
| T03 | ✓ | Research pinned open-source agent/skill/protocol architecture | architecture_research + coordinator | T01 | Pinned OpenAI Agents JS and A2A reference clones; primary source/license comparison | Reference only; no framework installation or full Python port |
| T04 | ✓ | Audit and install requested UI UX Pro Max skill | ux_skill_review + coordinator | T01 | Pin 50d8a7d; MIT retained; independent payload/path/provenance verification; actual local search returned 3 relevant matches | Available next turn; read-only searches default |
| T05 | ✓ | Document architecture and explicit role/capability/communication mapping | Coordinator + independent reviewer | T03 | AGENT-ARCHITECTURE.md maps 14 roles, native messaging/MCP/A2A, question gates and correction flow; independent static review | Keep docs aligned with actual implementation |
| T06 | ✓ | Build bounded stateless Rust stdio MCP planning interface | coordinator | Existing planning kernel | Six read-only tools; 8 MiB wire cap including newline; merged-main allowed one tracked request and bounded notifications; transport/service failures exit nonzero; clean EOF passes. Windows release build and compiled MCP journey passed at 8e2ddc1. Follow-up bounds are tracked under T18. | Keep Rust transport bounds and docs aligned |
| T07 | ✓ | Verify MCP security, dependencies, negative paths and actual subprocess E2E | Independent reviewer + coordinator | T06 | Current Windows worktree: fmt, Clippy, 59 debug tests, release build, 8 CLI E2E, 9 MCP E2E, debug/release package validation passed. Independent current-source review found no security/reliability issue in local stdio scope. PR #7 and its post-merge Linux/Windows runs passed on their own exact revisions. OSV exact-version batch returned 0 advisories for all 129 locked registry entries; 129/129 license fields present, no yanked versions, no copyleft-only expression. Two upstream `Unlicense/MIT` fields use non-SPDX slash notation; retain the Unicode-3.0 notice for unicode-ident. No known vulnerability found; this is not a complete security certification or legal opinion. | Preserve third-party notices and repeat dependency review when the lock changes; hosted transport/auth is separate |
| T08 | - | Maintain goal-linked status flow and project lifecycle documentation | Coordinator | T01, T05 | This live ledger; native goal API limitation explicit | Update after verified output transitions |
| T09 | - | Choose remote Rust MCP hosting and data/identity boundary | Coordinator + user | Official provider research; user's zero-budget constraint | HOSTING.md compares Cloudflare Rust/Wasm beta (10 ms CPU), Render cold starts and Vercel Hobby commercial restrictions. No account/endpoint provisioned. | User's data/retention choice, protocol compatibility and exact kernel benchmark remain |
| T10 |  | Implement/review remote Rust MCP transport and deployment configuration | Owned worker + reviewer | T07, T09 | No hosted endpoint exists | Actual bounded HTTPS/identity implementation |
| T11 |  | Connect real ChatGPT web/Codex service and perform host E2E | Coordinator + user | T10, actual account access | Unverified | Use real service/account; no mocked host success |
| T12a | ✓ | Research Autodesk's supported Fusion interfaces | Coordinator + co_op_analysis | User-selected Fusion target | Official local desktop and cloud Data MCP are GA; Compute MCP is beta; no third-party code copied | Prefer official desktop MCP for the user's live local Fusion session |
| T12b |  | Connect and complete real disposable CAD task | Coordinator + user | Installed/licensed app, exact task, supported host transport | No Fusion connection, CAD edit or application E2E performed | Need app/version/license and chosen CAD or CAM workflow |
| T13 | ✓ | Merge reviewed 0.3 planning kernel after actual required CI/E2E | Coordinator + previous reviewer | Prior local QA and user merge authorization | PR #4 merged; merged-main run 37094555223 all 7 jobs passed | New feature revisions need their own review/QA/CI |
| T14 | ✓ | Push reviewed current milestone; inspect CI, PR and merge gates | Coordinator + independent reviewer | T05–T07 local acceptance | PR #7 head c968d8b merged to main as 8e2ddc1. PR run 37902677375 and merge-commit run 37903287143 passed validate, test, Linux/Windows E2E and final-gate. Local Windows debug/release and Clippy checks passed; an independent read-only MCP review approved the bounded transport and test-harness correction. GitHub has no review object on the PR; repository approval count was zero. [PR #7](https://github.com/NAVANEETHVVINOD/agent-orchestrator/pull/7) | Continue the separate remote-service and publication tasks under their own gates |
| T15 |  | Finish privacy/terms/support metadata and public readiness assessment | Coordinator + user | T09–T12 selected release scope | Drafts only; identity/account readiness unverified | Resolve real public metadata and service evidence |
| T16 |  | Publish directory listing after complete readiness and user release decision | Coordinator + user | T15 | Explicitly held by user | No upload/submission/publication yet |
| T17 | ✓ | Add declarative agent/skill workflow configuration validated by Rust | Coordinator | T05, T06 | JSON Schema authoring aid plus reusable 14-role example; Rust CLI/MCP validate unique role keys, external profile/skill reference syntax and eight required gate names. Debug 56 tests, release CLI/MCP E2E, Clippy, package/path validation and independent config/security reviews passed. No command/endpoint/credential/permission fields; runtime discovery and dispatch remain outside this milestone. | Maintain in sync as host discovery/dispatch is designed |
| T18 | - | Fix MCP long-session notification accounting and support bounded pipelined tool calls | Coordinator + independent security reviewer | T07 audit findings | Source change allows four outstanding calls and expires notification counts after 60 seconds. Full current-source Windows gates pass: fmt, Clippy, 59 debug tests, optimized build, 8 CLI E2E, 9 MCP E2E and debug/release package validation. Independent review found no local stdio security/reliability issue. Hosted CI, PR review and merge are still pending. | Commit and push a PR, inspect exact-head Linux/Windows checks and final-gate, then merge if they all pass |

Actual native agents remain responsible for authorized execution. MCP connects tools;
A2A connects an explicitly selected external service. Neither protocol supplies a
review pass, user authority, provider account or live connection by itself.
