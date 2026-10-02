# Control applications without an existing MCP or CLI

Use [integration-builder](../skills/integration-builder/SKILL.md) to create a
project-specific adapter. Start with the exact app/version and desired operations,
then inspect official interfaces and your host's actual capabilities. Examples:

```text
This application has no MCP or CLI. Research its supported API/SDK and the control
operations I need. Design and implement a scoped Rust CLI or MCP adapter, review its
security and test the actual application integration before calling it ready.
```

```text
This is my local application source. Add a supported automation interface for the
specified operations and expose it through a typed Rust adapter. Preserve my edits,
document permissions and verify the complete real application journey.
```

Supported browser/accessibility automation can be a fallback when the host and app
expose it. Arbitrary desktop control is not available in every ChatGPT surface. A
plugin instruction does not override OS permission, create a backend API or bypass
an application's authentication. Unsupported apps receive an evidence-backed
limitation and alternatives, rather than a fake working interface.

The official [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk) is a
candidate for a selected integration; inspect its exact published version and protocol
compatibility before adoption. It is not a dependency of this skills-only core package.
Keep local stdio and hosted remote services distinct; hosting, identity, transport and
data handling require an actual deployment design. See the skill's
[adapter contract](../skills/integration-builder/references/adapter-design.md).
