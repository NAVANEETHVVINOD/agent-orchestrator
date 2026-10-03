# Research basis and scope

Checked 2 October 2026. The lifecycle is an original native-host workflow informed by
the following primary sources. Frameworks were not installed or executed, and their
source instructions/automation were not imported as authority. These references do
not certify this plugin or any generated application as secure.

| Source | Immutable snapshot / current baseline | Adapted idea |
| --- | --- | --- |
| BMAD Method | 4f61d4e769e50bc11d0d5d724f48942aac699679 | Scope-sized planning, explicit design/architecture, independent review and correction |
| GitHub Spec Kit | 838f1184d1b2ed254a99e8b818dbc23aa80a7f1f | Requirements -> plan -> tasks -> implementation/convergence; separate bug diagnosis/fix/verification |
| GitHub documentation | 0b8c768bf0d5a13560ec82fd3daa414137e2e436 | Actual revision/check evidence, safe Actions, least privilege and explicit final CI gates |
| OWASP ASVS | Stable 5.0.0, 5cf9b032440be53ce345ab3c130fda46ba1ce7a2 | Relevant application security requirements and negative access-control verification |
| NIST SSDF | Final 1.1, SP 800-218 | Define verification criteria, review/test software, triage findings and improve root causes |

Pinned primary references:
- https://github.com/bmad-code-org/BMAD-METHOD/blob/4f61d4e769e50bc11d0d5d724f48942aac699679/docs/plan/design-ux-and-architecture.md
- https://github.com/bmad-code-org/BMAD-METHOD/blob/4f61d4e769e50bc11d0d5d724f48942aac699679/docs/build/review-a-change.md
- https://github.com/github/spec-kit/blob/838f1184d1b2ed254a99e8b818dbc23aa80a7f1f/docs/reference/agentic-sdd.md
- https://github.com/github/spec-kit/blob/838f1184d1b2ed254a99e8b818dbc23aa80a7f1f/docs/guides/bugfix.md
- https://github.com/github/docs/blob/0b8c768bf0d5a13560ec82fd3daa414137e2e436/content/actions/reference/security/secure-use.md
- https://github.com/github/docs/blob/0b8c768bf0d5a13560ec82fd3daa414137e2e436/content/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks.md
- https://github.com/OWASP/ASVS/blob/5cf9b032440be53ce345ab3c130fda46ba1ce7a2/5.0/en/0x17-V8-Authorization.md
- https://doi.org/10.6028/NIST.SP.800-218

Current official host/plugin references used for packaging:
- https://learn.chatgpt.com/docs/agent-configuration/subagents
- https://learn.chatgpt.com/docs/build-skills
- https://developers.openai.com/plugins/build/plugins
- https://developers.openai.com/plugins/deploy/submission
- https://developers.openai.com/plugins/plugin-guidelines
- https://agent-plugins.org/schemas/1.0.0/plugin.schema.json
- https://a2a-protocol.org/latest/specification/

The schema snapshot is versioned 1.0.0. Documentation and directory requirements can
change; recheck the official pages before publication or a new external integration.
SSDF 1.2 was a draft when researched, not the final baseline. ASVS's automatically
updated bleeding-edge release is not substituted for a stable version.

The Apache-2.0 package contains authored workflow text/scripts and source citations;
it does not redistribute BMAD/Spec Kit code or bundle their installers. Native agent
coordination depends on the host. A2A needs a configured separately hosted service.
