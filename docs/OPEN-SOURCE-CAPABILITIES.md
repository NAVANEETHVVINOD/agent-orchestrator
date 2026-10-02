# Open-source capability shortlist

Researched 3 October 2026. These are scoped references or optional candidates, not
installed providers or a universal safety endorsement. Reuse each user's available
agents/skills/MCP tools first. Select exact artifacts, review their code/license/data
scope, and validate registration and real project behavior before adoption.

| Candidate and source pin | Suitable use | Selection here |
| --- | --- | --- |
| [OpenAI skills](https://github.com/openai/skills/tree/49f948faa9258a0c61caceaf225e179651397431) | Official skills and creation/installation patterns | Reference; use relevant exposed skills and inspect selected skill instructions/license |
| [Agent Skills specification](https://github.com/agentskills/agentskills/tree/69ef37e9424c0a7ea9dd2293b559e43ec8176379) | Portable skill format and validation; repo metadata Apache-2.0 | Format reference; does not implement permissions or agent runtime |
| [Anthropic skills](https://github.com/anthropics/skills/tree/8a1541c4a3ffa5a20a5a91de0dcf3f0bab1d1ef4) | Additional task-specific methodology | Optional selected-skill review; root license metadata is absent, so verify per-skill license before reuse |
| [Official Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk/tree/ae2f9c9b45a2c98d24ee345406e79f507c9f9282) | Typed Rust application adapters and supported transports | Preferred candidate for a future selected Rust adapter; not bundled in core |
| [CLI-Anything](https://github.com/HKUDS/CLI-Anything/tree/34f519533bc175d2fe287ab8316b0dd99bb9cc43) | Per-application CLI harness methodology and specific existing harnesses | Method reference now; optional harness only after separate review and real app tests |
| [A2A specification](https://github.com/a2aproject/A2A/tree/65dadbd9b900fe2d970e05416eb432a91347cd2a) | Explicit external agent service interoperability | Reference only; select endpoint/identity/auth/protocol/data scope separately |

BMAD, Spec Kit, official MCP examples, pgvector, Qdrant, LlamaIndex and Jev/Laya are
covered in [RESEARCH.md](RESEARCH.md). Native orchestration is sufficient for this
core; importing a second runtime would add installation, execution and compatibility
boundaries without establishing a benefit. Knowledge/model integrations stay optional.

## CLI-Anything findings and feasible reuse

The likely project requested as “CLI anywhere” is HKUDS/CLI-Anything. It generates
application-specific harnesses using available source/backend interfaces and JSON
output, with analysis, implementation, tests and real backend workflows. Its documented
generated harness format is Python/Click; it is not a Rust runtime replacement and
does not make an inaccessible application controllable automatically. Upstream broad
marketing claims and test totals were not independently reproduced here.

Read-only review covered the pinned Codex skill, installer, harness methodology, hub
registry/installer/analytics and relevant guides. No harness or installer was executed.
Root and plugin license files use Apache-2.0, while hub package metadata claims MIT;
verify this conflict for any selected hub artifact. Any selected app/harness/dependency still
needs its own license and provenance review. No source from this project is bundled.

Relevant source boundaries:

- cli-hub/cli_hub/registry.py fetches a mutable registry without artifact signatures/hash pins.
  cli-hub/cli_hub/installer.py executes registry install text, including shell=True paths,
  unpinned npm/global installs, latest updates and Git pip sources without commit pins.
- cli-hub/cli_hub/analytics.py:85 makes telemetry opt-out via CLI_HUB_NO_ANALYTICS; events
  include a persistent identifier and discovery query prefix (:194–206, :342–347).
- Preview metadata fingerprints use path/size/mtime rather than source bytes; session
  locking guidance can proceed unlocked on unsupported Windows/import paths.
- Registry OpenAPI and static agent-card metadata do not prove external A2A task/message
  interoperability, authentication or a connected agent service.

Adapted method: inspect the actual app, reuse supported interfaces, define typed
operations/JSON/errors, preserve action identity/state, and run real target E2E with
independent review. Our integration-builder prefers Rust adapters where the chosen
SDK/backend supports them. It does not invoke the upstream hub or shell installer.
If a user chooses a reviewed existing Python harness, disclose that dependency rather
than claiming it was converted to Rust or is part of this core.

