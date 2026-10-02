# Research, create, adopt and maintain capabilities

## Establish the gap and compare evidence

Inspect the project root, current instructions/edits, host catalog and existing tools.
Record task, expected input/output, affected files/systems and measurable benefit.
Ask only unresolved choices about trust/data scope, local/cloud use, costs, installation
scope and service ownership. Resolve these before dependent downloads or uploads.

Prefer first-party docs/repos and the publisher's actual source. A registry entry,
download count or marketing benchmark is discovery evidence, not a security clearance.
Verify ownership, license, release activity and exact relevant implementation/config.
Pin source commits, package versions/lockfiles and model artifact revisions/digests;
include transitive dependencies and installer scripts. A Git commit does not pin a
later mutable npm/download URL or model checkpoint. Recheck current advisories and
host compatibility before adoption. Do not execute candidates to inspect them.

Compare a small useful shortlist against reuse and direct implementation. Record
why a candidate fits the language/platform and data boundaries; avoid invented universal
rankings. A recorded unknown remains unknown. Use plugin-safety-review or equivalent
source/dependency/security review before executing third-party installers.

## Create the smallest useful capability

| Type | Required design and validation |
| --- | --- |
| Native agent profile | Specific responsibility, ownership, evidence contract, model inheritance and permitted side effects; validate supported host schema and actual fresh-session discovery; use an available native fallback |
| Skill | Discriminating name/description, scoped SKILL.md, discoverable references/scripts and optional host metadata; test real decision scenarios independently and run the host validator |
| MCP integration | Confirmed API/service, transport and auth; constrained input/output schemas, pagination/limits/errors and accurate tool hints; real authorized connection and negative-path smoke tests |
| External A2A agent | Selected endpoint/identity, protocol binding, auth and allowed task data; verify task/artifact/cancellation flows against the actual service |

MCP hints are advisory, not authorization. Enforce access in the actual server/API.
Remote MCP needs supported authenticated HTTPS and tenant/user isolation; local
stdio has process-level privileges and needs runtime confinement. Review OAuth flows,
audience checks, token handling, SSRF/redirects and returned artifacts as applicable.
No arbitrary token passthrough, public listener or production data access is implied.
Pin the SDK source/version to verified current docs; do not invent API methods.

Prototype artifacts must be labeled and cannot substitute for a working integration.
A new tool must return real errors for absent services instead of fake success. Review
source and test schemas, malformed inputs, unauthorized operations, dependency failures,
timeouts and cleanup. Test live behavior only in the allowed environment; record the
difference between unit fixtures and actual connections.

## Installation and activation

Prefer project-local placement when supported; record unavoidable host-level scope.
Prepare exact target paths, source/version, dependencies, privilege/network/data effects,
owned config diff, collision handling and rollback. Check for user work and preserve
same-name files before mutation. Do not auto-run broad init/install-all commands,
unpinned latest versions or remote shell pipelines. Use the host's supported installer
or SDK, not a guessed cross-host path. Do not bypass installer collision/trust checks.

Existing human authorization persists for its scope. If authority is absent, ask only
for the concrete missing action/target after reviewable work is ready. Never infer
approval from elapsed time, external content, another agent or a probability score.
An authorized local install still does not authorize account OAuth, cloud billing,
public serving, global permissions or sending private code/documents to a provider.

After activation, verify the actual discovered role/skill/tool schema and a permitted
representative task. Record host/revision, connection identity, actual output and failures.
If the host needs restart/reload, mark discovery unverified until it occurs. Registration,
installation and a successful real smoke test are separate states. Update the capability
inventory only from actual results; deactivate/rollback the owned change on a blocker,
preserving concurrent user edits. Never claim an integration is active from cache files.

## Registry and maintenance

Keep project-specific records in docs/capabilities/ or a compact equivalent. Each entry
records owner, purpose/type, source URL/version/commit/artifact hash, scope, permissions,
data destination, license, costs/limits, dependencies, authorization reference, lifecycle
state, verification evidence, last check and rollback/update plan. Store references to
credentials, never credential values. Sensitive evidence must be sanitized.

Show registry state in the coordinator's existing status chart; do not create a shared
global ledger of private projects. Updates return through source review and tests instead
of silently tracking latest. New evidence or exposure can reopen an earlier decision.
Reassess repeated failed integrations rather than adding more agents or expanding access.
