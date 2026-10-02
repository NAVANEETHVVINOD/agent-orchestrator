# Host capabilities and limits

| Surface/capability | What this package can guide | Required capability |
| --- | --- | --- |
| ChatGPT plugin/skill conversation | Requirements, design plans, review of supplied artifacts and next-step decisions | Plugin skills available in that account/surface |
| Work or another agent-enabled host | Independent analysis/review and coordinated execution where exposed | Eligible host-native agent/runtime tools and authorized environment |
| Local Codex | Project files/commands, scoped native subagents and QA/delivery workflow | Local tooling, host permission and actual project access |
| Connected plugins/MCP | Select task-relevant exposed tools and inspect their outputs | Separate installation/connection/auth and appropriate tool authorization |
| External A2A | Plan and use an explicitly configured external-agent integration | Actual service endpoint, compatible transport/version, auth and data scope |
| Capability research/creation | Compare public-source agents/skills/MCP and implement missing capabilities | Current docs/source access, owned development tools and actual destination/action authority |
| Optional knowledge/RAG | Design or evaluate authorized retrieval and grounding | Selected corpus/provider, actual data permissions and a working retrieval environment |
| Optional decision models | Propose bounded routes with measured scores and uncertainty fallback | Reviewed provider/model revision, authorized input data and task calibration |

If an independent agent/runtime is absent, use sequential planning or artifact analysis
and state the limitation. Do not simulate independent reviewers or claim unrun tests.
Required runtime gates stay blocked when execution is unavailable.

This package supplies no hosted MCP server or A2A endpoint. It does not automatically
install third-party skills/plugins or expose private machines. Native peer communication
stays in the active task tree; separate chats need human-authorized destinations.
Permission overrides and connector-specific policies remain authoritative. Skill
instructions and optional role profiles are not security sandboxes.

Capability-manager can guide creation/install work using the host's supported tools.
It does not download or register providers automatically. Created, installed, loaded,
connected, authenticated and runtime-tested are separate states. Model scores cannot
substitute for permission checks or actual QA/security/E2E evidence.

The task chart updates during active work; no autonomous monitor or scheduler is
installed. Project state belongs in that project's docs/, never a shared global project
ledger. Concurrency respects the host rather than claiming every specialist runs at once.
