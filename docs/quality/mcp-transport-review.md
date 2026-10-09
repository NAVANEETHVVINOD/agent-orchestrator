# 0.4.0 local stdio MCP transport review

An independent read-only security reviewer inspected the follow-up transport and subprocess
test diff, committed as `5086812` and merged as `e2ac715`. It found no security or reliability
issue in the local stdio scope. Request IDs remain tracked until response writes complete, the
server closes on a fifth still-outstanding request, and notification timestamps expire from a
rolling 60-second window after 64 accepted events. Unit tests cover overload, notification flood
and expiry. The reviewer identified an E2E assertion gap during review; the compiled-server test
was strengthened to require successful structured results and all three planning-only flags before
the final review cleared the gap.

Windows local verification passed with Rust 1.94.1: 59 debug tests, Clippy, release build, 8 release
CLI E2E tests, 9 release MCP E2E tests and debug/release package validation. PR-head CI run
[37910731823](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37910731823) and
post-merge run [37912634478](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37912634478)
passed all Linux/Windows validation, test, E2E and final-gate jobs.

The review was static and limited to the local stdio MCP service and its tests. Hosted HTTP
authentication, ChatGPT web E2E, live Fusion integration, external agent dispatch and the truth
of caller-supplied evidence remain outside its scope. The dependency advisory/license inventory
is documented separately in `docs/quality/dependency-scan.json`; neither review certifies the
product free of all vulnerabilities or supplies legal advice.
