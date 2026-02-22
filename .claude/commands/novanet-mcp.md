---
description: NovaNet MCP Server operations - build, test, debug, or check status
argument-hint: [status|test|build|debug]
---

# NovaNet MCP Server

Based on `$ARGUMENTS`, execute the appropriate action:

## `status` (default)

Check server health and configuration:

```bash
cd tools/novanet-mcp && cargo check && cargo test --no-run 2>&1 | tail -5
```

Show key stats:
- Compilation status
- Test count
- Dependencies

## `test`

Run the test suite:

```bash
cd tools/novanet-mcp && cargo test -- --nocapture
```

Expected: 15 tests passing

## `build`

Build release binary:

```bash
cd tools/novanet-mcp && cargo build --release
```

Output: `tools/novanet-mcp/target/release/novanet-mcp`

## `debug`

Start with debug logging (requires Neo4j running):

```bash
cd tools/novanet-mcp && RUST_LOG=novanet_mcp=debug cargo run
```

Note: This starts the MCP server with stdio transport. Use Ctrl+C to stop.

## `clippy`

Run linter:

```bash
cd tools/novanet-mcp && cargo clippy -- -D warnings
```

## Quick Reference

| Tool | Description |
|------|-------------|
| `novanet_query` | Execute read-only Cypher queries |
| `novanet_describe` | Bootstrap agent with schema knowledge |
| `novanet_search` | Fulltext + property search with hybrid mode |
| `novanet_traverse` | Graph traversal with depth/direction filters |
| `novanet_assemble` | Assemble token-aware context for LLM |
| `novanet_atoms` | Retrieve knowledge atoms for locale |
| `novanet_generate` | Full RLM-on-KG context assembly |
| `novanet_introspect` | Schema introspection for agents |

| Env Variable | Default |
|--------------|---------|
| `NOVANET_MCP_NEO4J_URI` | bolt://localhost:7687 |
| `NOVANET_MCP_NEO4J_PASSWORD` | (required) |

See `tools/novanet-mcp/CLAUDE.md` for full documentation.
