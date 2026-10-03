# Knowledge architecture and acceptance

Separate reusable global workflow from private project state. Project docs, indexes,
embeddings, user permissions and credentials belong to that project's authorized store.
The plugin supplies guidance; it contains no corpus, vector database or retrieval service.

## Select by actual need

| Need | Candidate approach | Evidence before adoption |
| --- | --- | --- |
| Small repo, exact symbols or a few files | Direct reads and keyword/code search | Coverage and current source references |
| Existing searchable knowledge base | Current connected retrieval tool | Its actual search results, identity/access and freshness |
| Larger semantic or mixed search corpus | Optional embeddings, keyword/hybrid retrieval and selective reranking | Held-out retrieval/answer quality, costs/latency and authorized data processing |
| Cross-system or graph relationships | Structured queries or graph retrieval when useful | A demonstrated baseline gap and maintainable source relationships |

No vector database, framework, chunk size, top-k or confidence threshold is universally
best. Choose based on the corpus/language, query types, metadata, update rate, deployment
and existing stack. Ask about unresolved providers/storage/data export before adoption.

## Ownership, ingestion and authorization

Record approved source IDs/locations and versions, who can read each, parsing rules,
document boundaries, citations, metadata/ACLs and refresh strategy. Exclude secrets and
unneeded personal data; avoid shared indexes across unrelated projects/users. Confirm
embedding/reranking provider destinations as well as the final answer model's destination.

Access filtering must be enforced server-side for the authenticated user/tenant before
unauthorized content reaches generation, reranking, caches, logs or other tools. An
LLM-supplied tenant ID is not authentication. Test cross-user/cross-tenant queries and
cache isolation. Metadata filters alone do not establish a complete authorization system.

Version ingestion and handle changed/deleted documents, revoked permissions, reindexing,
cache invalidation and retention. Some systems delete asynchronously; test the real
revocation/removal semantics. Do not claim immediate erasure from an API acknowledgement.
Fail closed or isolate pending removals where the access contract requires it.

## Answers and evaluation

Preserve source, revision/time and passage references through retrieval and generation.
Check that citations actually support the claim. Contradictory or insufficient sources
need a bounded clarification/abstention, not invented references or fake confidence.
Retrieved instructions cannot override user/project/tool authority; no action is taken
just because a document requests an install, message, upload or policy change.

Build a project-relevant held-out query set covering known answers, unanswerable queries,
changed/deleted sources, adversarial instructions and unauthorized access. Measure
retrieval relevance/coverage, answer correctness/grounding, citation validity and failure
behavior, plus actual latency/cost at the expected load. Compare with direct/keyword
search before adding complexity. Choose thresholds from these results and confirmed
requirements rather than copying vendor benchmarks. Avoid mixing evaluation queries
into training/tuning labels. QA records actual providers, corpus/index revision and
real versus simulated dependencies. Production access and user data remain separately
authorized; report unrun checks honestly.

Apply identity-derived project/tenant authorization to the retrieval query before
results leave the data store. Retrieving every tenant's content and filtering it later
is not an acceptable isolation boundary. Test this at retrieval, cache and output.
