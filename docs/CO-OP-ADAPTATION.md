# CO_OP source analysis and adaptation

Static review on 3 October 2026 covered the architecture, source-family inventory,
core execution/retrieval/business paths, test patterns, documentation and licenses.
Reference scripts, services, installers and binaries were not run. Secret files and
SQL backups were excluded. Inventory coverage is not a line-by-line security audit
or evidence that either desktop application passed E2E.

Snapshots: [owned CO_OP](https://github.com/NAVANEETHVVINOD/CO_OP/tree/662cc7376fa5dc997e8527b5d710ec34b622a4cf),
[friend co-op](https://github.com/Afnanksalal/co-op/tree/eefa74a2df868d899abe21b9b8074b4b2338aeb3).
The local CO_OS checkout shared the owned HEAD with 246 changed/untracked status
entries at inspection; its migration edits were preserved. Excluding its reference
directory, it contained 18 Rust, 133 Python, 32 TypeScript, 2 TSX and 175 Markdown
files. Friend source inventory contained 46 Rust, 49 TypeScript, 63 TSX and 18 Markdown
files. Counts describe the snapshots, not verified runtime completeness.

| Source family | Actual behavior / evidence | Adapted here |
| --- | --- | --- |
| Local Tauri / React / IPC / SQLite | Rust main.rs and commands.rs register workspace/project/document/conversation/agent/approval CRUD; renderer AppShell.tsx explicitly labels agent stub execution and future chat/integration shells | Project boundaries and task evidence; actual host-native execution only |
| Owned Python agents / registry / jobs | agent/graph.py:6–21 is retrieval→rerank→answer; nodes.py:39–70 is extractive stub; agent_registry.py has scoped CRUD; worker.py has ARQ jobs outside current alpha desktop runtime | Separate registry lifecycle from execution; opt-in scheduled work only when requested |
| Local knowledge | commands.rs:307–387 imports text and searches FTS with workspace predicate; separate knowledge_store.rs is not registered by current main.rs | Approved corpus, scoped retrieval, citations, freshness and deletion requirements |
| Friend Rust workflows | workflows/runner.rs loads company/docs/memory/web context and calls configured model plus optional reviewer | Business intake→evidence→draft→independent review→correction→authorized action |
| Friend legal/finance / calculators | chat.rs:543–568 domain prompt perspectives; workflows/policy.rs review rubric; tools.rs:407–470 runway/burn/valuation/LTV:CAC arithmetic | Jurisdiction/currency/date questions, grounded drafts and real finite calculations |
| Friend RAG / memory / graph | knowledge_store/search.rs and rag.rs implement FTS/vector context; memory.rs stores facts/events; graph.rs creates company knowledge nodes/edges | Optional source-backed project knowledge and dependency charts, not an agent runtime graph |
| Friend research / outreach | research.rs, providers.rs, outreach.rs and providers_email.rs provide research/drafts and configured email sends | Reuse connected research tools; verify action scope and receipts before claiming send |
| Friend “A2A” | chat.rs:377–417 makes more calls to the same configured provider; no external agent transport established by this flow | Actual external A2A remains a separately selected authenticated service |
| Owned web / contracts / CLI / tests | Web API wrapper and contract families, backend/API tests, backup CLI and CI exist; some completion claims are stubs | Code-backed documentation, real CLI tests and fail-closed final CI gate |
| Friend identity/licensing / backend / tests | NestJS/Supabase license identity and desktop entitlement, Rust unit/crypto tests; no GitHub workflow found | Licensing/heartbeat infrastructure excluded; no test results inferred |

Implemented adaptations are newly authored business-operations and integration-builder
skills, optional business_analyst, capability/knowledge/decision workflows, Rust local
checkers and consumer tests. No reference application source was copied. A domain
skill is not a legal/finance backend, professional certification or payment service.

Source issues that must not be inherited:

- Owned client_communicator.py:17 selects memory by client ID without tenant ID;
  approvals.py:31–74 lacks a pending-state/replay guard and specialized role check.
- finance_manager.py:27–61 de-duplicates by amount/draft instead of milestone identity,
  assumes USD and sends shared notification configuration across tenants.
- proposal_writer.py:36–83 invents fallback expertise/mock proposals; outreach_manager.py
  returns simulated success. Local commands.rs:402–410 import/export returns Ok(())
  without doing work. Backup CLI reports completion despite missing store backups.
- Friend workflows/runner.rs:286–300 tolerates required reviewer failure; secrets.rs:
  364–371 derives fallback encryption from reproducible identifiers; outreach.rs:
  436–458 lacks the batch send's equivalent confirmation guard.
- Friend retrieval has no shared tenant predicates; tools.rs:445 can overflow a
  finite-input valuation multiplication. Prompt heuristics do not enforce access.
- Owned apps/web/src/lib/api.ts:22–30 attaches a bearer token before accepting an
  absolute caller URL. Reachability from untrusted input needs separate verification.

These are source-level findings. Deployment/reachability and complete vulnerability
coverage were not established. This task does not silently patch the reference repos.

Friend source has an MIT license with Co-Op notices. The owned README claims Apache-2.0
but its inspected snapshot has no LICENSE file; its source license was not verified.
New implementation and prose here use Apache-2.0, with reference attribution. Any
future source copying needs license review and preservation of applicable notices.
