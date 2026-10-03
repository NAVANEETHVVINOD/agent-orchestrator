# Public distribution and directory submission

The package uses the portable Agent Plugins 1.0.0 root manifest, publisher NAVANEETH,
display name AGENT ORCHESTRATOR and Apache-2.0 license. No fake public repo, contact,
website, approval or endpoint has been inserted. The public source repository is
[NAVANEETHVVINOD/agent-orchestrator](https://github.com/NAVANEETHVVINOD/agent-orchestrator).

## Open-source distribution
The implementation, LICENSE/NOTICE and docs are available in
[PR #4](https://github.com/NAVANEETHVVINOD/agent-orchestrator/pull/4) and its
codex/initial-plugin branch, with a versioned 0.3.0 source ZIP prepared locally.
Local/repo distribution and universal public-directory publication are different routes.
Windows/Linux validation, tests and optimized CLI E2E passed on implementation head
fdaf15667ff0e8745ee7c2694f0529d5d1728b20 in the
[PR run](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37061332784).
Protection requires final-gate, one approving GitHub review and resolved conversations,
including for administrators. Automatic merge is enabled; an approving GitHub review
remains required. Later revisions must verify their own actual checks before merge.

## Universal ChatGPT/Codex directory
1. Complete individual/business verification and choose the verified developer identity.
2. Open the OpenAI plugin submission dashboard and upload the plugin ZIP.
3. Inspect automated package/metadata/skill findings; fix and reupload until required
   checks complete. Confirm listing identity/icon and any required public metadata.
4. Submit for review with the requested attestations/materials.
5. After approval, choose Publish plugin. Until then it is not publicly listed.

This is a skills-only submission; no MCP review credentials, fictitious endpoint or
empty MCP/app mapping is included. Current documentation says adding an MCP server to
an existing skills-only plugin is not supported. A future bundled hosted MCP/A2A bridge
therefore needs a separately planned submission with reviewed HTTPS service/auth,
data isolation and applicable privacy/support/terms URLs. Existing user-connected MCP
tools may still be used when the host exposes them.

Verify these changing requirements before submitting:
- https://developers.openai.com/plugins/build/plugins
- https://developers.openai.com/plugins/deploy/submission
- https://developers.openai.com/plugins/plugin-guidelines

The public name is the user's selected name; directory review may require metadata
adjustments. Public approval and safe runtime behavior are not guaranteed by a local
schema check. No submission or publication action has occurred in this package build.

Version 0.2.0 adds instructions for project-specific MCP creation and optional knowledge/
decision integrations; it remains a skills-only package with no bundled MCP service.
Creating a user's project integration is separate from adding a hosted MCP app to this
public listing. Version 0.3.0 adds a local Rust planning kernel and stronger cross-platform
CI; it remains skills-only. Plan a managed-service edition before its submission if wanted.
