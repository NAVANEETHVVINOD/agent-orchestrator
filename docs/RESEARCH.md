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

## Project-capability extension

Research checked 2 October 2026. These are candidates and implementation references;
none was installed, model-inferred, indexed or security-cleared for a user's project.
Choose only after a concrete gap, compatibility/data decision and exact-source review.

| Need | Primary-source candidate/reference | Reviewed identity/snapshot |
| --- | --- | --- |
| Skills and scoped acquisition | OpenAI skills/installer patterns | 49f948faa9258a0c61caceaf225e179651397431 |
| Cross-host skill discovery | Vercel skills | 3694740352eeef5cdd689af694c485f1ff62eec3 |
| MCP integration design | Official reference servers; educational examples, not production-ready | f46d9578190b476b3501923ea8977d899e8db2cb |
| MCP security contract | Official 2026-07-28 specification/docs | 3098fe94caa1b9e0afaaa6d30e040b61d5802471 |
| Vectors in existing PostgreSQL | pgvector 0.8.7 | f37c13f68b57d2c3472b2214fbcff699d6d34876 |
| Dedicated vector service | Qdrant 1.19.1 | 6ab21cac18ebb6f4ae29102c7f8f5cc11affd5de |
| Ingestion/retrieval abstractions | LlamaIndex 0.14.25 | f12d46acab73f5b2243ef49c2f00101617b38ce4 |
| Hosted bounded decisions | TypeSafe Jev | Versioned API model jev-1.13.0; no call performed |
| Local/open-weight bounded decisions | NandhaKishorM/laya, convaiinnovations/laya | Repo 4aa6761be8173de4ce6d92c31b3e40b6eaf59a7c; model 55cf4c4ebb4ebe31b2550e8bdf3bd21b99753851 |

Pinned primary sources:
- https://github.com/openai/skills/tree/49f948faa9258a0c61caceaf225e179651397431/skills/.system
- https://github.com/vercel-labs/skills/blob/3694740352eeef5cdd689af694c485f1ff62eec3/README.md
- https://github.com/modelcontextprotocol/servers/blob/f46d9578190b476b3501923ea8977d899e8db2cb/README.md
- https://github.com/modelcontextprotocol/modelcontextprotocol/tree/3098fe94caa1b9e0afaaa6d30e040b61d5802471
- https://github.com/pgvector/pgvector/blob/f37c13f68b57d2c3472b2214fbcff699d6d34876/README.md
- https://github.com/qdrant/qdrant/tree/6ab21cac18ebb6f4ae29102c7f8f5cc11affd5de
- https://github.com/run-llama/llama_index/tree/f12d46acab73f5b2243ef49c2f00101617b38ce4
- https://github.com/NandhaKishorM/laya/tree/4aa6761be8173de4ce6d92c31b3e40b6eaf59a7c
- https://huggingface.co/convaiinnovations/laya/tree/55cf4c4ebb4ebe31b2550e8bdf3bd21b99753851
- https://github.com/OWASP/CheatSheetSeries/blob/063e9df7b05c0dcf5f74415b974c67ee20a8f359/cheatsheets/RAG_Security_Cheat_Sheet.md

Current official documentation:
- https://docs.typesafe.ai/api
- https://docs.typesafe.ai/models
- https://docs.typesafe.ai/confidence
- https://typesafe.ai/legal/privacy-policy
- https://docs.langchain.com/oss/python/deepagents/retrieval
- https://qdrant.tech/documentation/security/
- https://developers.llamaindex.ai/python/framework/module_guides/evaluating/
- https://developers.openai.com/codex/skills/
- https://developers.openai.com/codex/multi-agent/
- https://developers.openai.com/codex/mcp/

Reuse existing search before indexing. Project/tenant authorization, source freshness,
deletion and actual retrieval/answer evaluation are design requirements, not benefits
automatically supplied by a framework. Likewise, typed model output does not establish
calibration, safety or permission. Laya's current publisher warnings and Jev's documented
confidence semantics require project-specific evaluation. Popularity and API compatibility
cannot substitute for actual evidence or a dependency/security review.

Application adapter and skill source snapshots are recorded in [OPEN-SOURCE-CAPABILITIES.md](OPEN-SOURCE-CAPABILITIES.md).

