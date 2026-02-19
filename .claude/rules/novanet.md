---
paths:
  - "novanet-dev/**"
---

# NovaNet Project Rules

These rules only load when working in `novanet-dev/`.

## Core Philosophy

```
CRITICAL: Generation, NOT Translation

Entity (defined) → Generate natively → EntityNative (authored)
```

## Current Version

**v0.14.0** | 61 nodes | 182 arcs | 10 layers | 6 arc families | 5 traits

## Architecture

```
SHARED (40 nodes, READ-ONLY)        ORG (21 nodes)
├── config      (3)                 ├── config      (1)
├── locale      (6)                 ├── foundation  (6)
├── geography   (7)                 ├── structure   (3)
└── knowledge  (24)                 ├── semantic    (4)
                                    ├── instruction (4)
                                    └── output      (3)
```

## Must-Know ADRs

| ADR | Name | Key Point |
|-----|------|-----------|
| 029 | *Native Pattern | EntityNative, PageNative (unified suffix) |
| 030 | Slug Ownership | Page owns URL, Entity owns semantics |
| 033 | Denomination Forms | Prescriptive canonical forms for LLM refs |
| 024 | Trait = Data Origin | defined/authored/imported/generated/retrieved |

## Traits (ADR-024)

| Trait | Who Creates | Examples |
|-------|-------------|----------|
| defined | Human, ONCE | Page, Block, Entity |
| authored | Human, PER locale | EntityNative, ProjectNative |
| imported | External data | Term, SEOKeyword |
| generated | Our LLM | PageNative, BlockNative |
| retrieved | External APIs | GEOAnswer |

## Key Commands

```bash
# Schema
cargo run -- schema generate     # Regenerate artifacts
cargo run -- schema validate     # Validate YAML

# TUI
cargo run -- tui                 # Interactive exploration

# Infra
pnpm infra:up && pnpm infra:seed # Start Neo4j + seed
pnpm dev                          # Studio at :3000
```

## Schema Workflow

```
/schema:add-node → YAML created → schema generate → schema validate → db seed
```

## MCP Server (8 tools)

`novanet_query`, `novanet_describe`, `novanet_search`, `novanet_traverse`,
`novanet_assemble`, `novanet_atoms`, `novanet_generate`, `novanet_introspect`

## References

- Full ADRs: `../.claude/rules/adr/`
- Schema standard: `.claude/rules/schema-standard.md`
- Terminology: `.claude/rules/novanet-terminology.md`
- Use `/adr <number>` for quick lookup
