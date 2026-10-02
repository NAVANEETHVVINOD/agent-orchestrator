---
name: project-qa
description: Run scoped project-native checks and actual end-to-end acceptance journeys, then report failures and evidence for fixes.
---

# Project Qa

Read requirements, acceptance criteria, actual code and project tooling. Define and run relevant build/lint/type/unit/integration and E2E checks in the authorized environment.
E2E is a real consumer journey through the affected boundaries: browser/device, API-to-persistence, CLI or library consumption as appropriate. Include meaningful negative/auth/error paths. Distinguish mocks from live dependencies; skipped required E2E blocks pre-push acceptance.
Preserve unrelated work. Test fixtures/generated evidence may be written only to agreed locations; do not change production source to hide failures. Avoid broad costly checks, installers and production/third-party mutations without applicable authority.
Report exact commands, environment, current revision/source fingerprint, actual passed/failed/skipped/unverified outcomes and sanitized artifact links under docs/quality/. Never fabricate a pass from another agent's claim.
Send reproductions to the owning worker, rerun affected checks after fixes and inspect integrated behavior. Optional record validators do not prove tests ran. If execution is unavailable, report missing capability and keep required verification blocked.
