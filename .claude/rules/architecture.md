# supernovae-agi Architecture Rules

## Rule 1: Mirrored Structure

NovaNet and Nika MUST have identical folder structure for Rust CLIs:

```
tools/<name>/
├── src/
│   ├── core/           # Shared patterns
│   │   ├── config.rs   # Root discovery
│   │   ├── error.rs    # Error handling
│   │   └── output.rs   # Output formatting
│   ├── tui/            # Terminal UI
│   ├── commands/       # CLI commands
│   └── main.rs         # Entry point
├── Cargo.toml
└── CLAUDE.md
```

## Rule 2: MCP Integration

Nika connects to NovaNet via MCP protocol ONLY. No direct Neo4j access from Nika.

```
Nika → MCP Client → NovaNet MCP Server → Neo4j
```

## Rule 3: Independent Repos

novanet-dev/ and nika-dev/ are SEPARATE git repos. They share:
- Folder structure (mirrored)
- Design patterns (core/)
- Documentation style (CLAUDE.md)

They do NOT share:
- Git history
- Cargo workspace
- Direct code imports

## Rule 4: Zero Cypher in Nika

Nika workflows NEVER use raw Cypher. Use semantic MCP tools:

```yaml
# WRONG
- exec: "MATCH (e:Entity) RETURN e"

# RIGHT
- invoke: novanet_traverse
  params:
    start: "entity:qr-code"
    arc: "HAS_NATIVE"
```

## Rule 5: Documentation Placement

- **supernovae-agi/docs/** → Cross-project plans/research (affects both)
- **novanet-dev/docs/** → NovaNet-specific documentation
- **nika-dev/docs/** → Nika-specific documentation

When in doubt: if a plan/research involves BOTH projects, it goes in supernovae-agi/docs/.
