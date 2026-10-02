# Install and use

Extract the complete plugin folder. It includes plugin.json, skills/, assets/ and a
local catalog at .agents/plugins/marketplace.json. Use a supported local host to add
that extracted folder as a marketplace root:

```text
codex plugin marketplace add /absolute/path/to/agent-orchestrator
codex plugin list --marketplace agent-orchestrator-local --available --json
codex plugin add agent-orchestrator@agent-orchestrator-local
```

The add commands mutate plugin configuration/cache; inspect the package before use.
Restart or start a fresh chat afterwards. Availability and installation controls vary
by host; the ChatGPT desktop Plugins Directory can also install from a supported local
marketplace. A local source does not publish to the universal directory.

Invoke AGENT ORCHESTRATOR or its global-orchestrator skill in the plugin's namespace
as exposed by your host. Select the skill in the picker if names collide with a
standalone local copy. Examples:

```text
Plan this project first. Ask focused questions about requirements, UX, backend/data,
technology and delivery, then produce a design and evidence-linked implementation plan.
```

```text
Coordinate this feature using available tools and scoped agents. Inspect outputs,
maintain docs and a status chart, and iterate through independent review, security,
QA and fixes. Require actual local E2E before an authorized code push.
```

The fourteen skills are global-orchestrator, plugin-safety-review, project-discovery,
project-design, project-qa, project-security, project-documentation,
project-evidence-review, project-release, capability-manager, project-knowledge and
decision-routing, business-operations and integration-builder. Use only the stage(s) relevant to the task.

Optional Codex TOML profiles are under assets/codex-agent-profiles/. They are reference
files, not automatically installed by this plugin. Review and copy only wanted profiles
into your own Codex agents directory, preserve same-name files, and restart for discovery.
The plugin works with scoped native fallbacks where these profiles are unavailable.
It does not overwrite global AGENTS.md or change sandbox, approval or connector policy.

The optional local helpers use Rust 1.94.1+ and Git. See [RUST.md](RUST.md) for
build, test and CLI commands. The instruction skills require no background service.

For project-specific research/creation, try:

```text
Inspect my existing tools and this project's requirements. Research a small shortlist
for concrete capability gaps, compare against reuse, then create or adopt the selected
reviewed capability in the authorized scope. Verify actual registration and behavior.
Assess whether knowledge retrieval or bounded decision routing adds measured value.
```
