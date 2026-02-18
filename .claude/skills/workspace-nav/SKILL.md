---
name: workspace-nav
description: Navigate between NovaNet and Nika projects. Use when switching context between brain (NovaNet) and body (Nika).
user-invocable: true
---

# Workspace Navigation

## Projects

| Project | Role | Path | Key Command |
|---------|------|------|-------------|
| **NovaNet** | Brain (Knowledge Graph) | `novanet-dev/` | `cargo run -- tui` |
| **Nika** | Body (Workflow Engine) | `nika-dev/` | `cargo run -- run` |

## Switch Context

**To NovaNet**: Read `novanet-dev/CLAUDE.md`
- Schema: `packages/core/models/`
- Rust CLI: `tools/novanet/src/`
- ADRs: `.claude/rules/adr/`

**To Nika**: Read `nika-dev/CLAUDE.md`
- Engine: `tools/nika/src/`
- Verbs: `infer:`, `exec:`, `fetch:`, `invoke:`, `agent:`

## Integration Rule

Nika → MCP → NovaNet → Neo4j (no direct Cypher in Nika)
