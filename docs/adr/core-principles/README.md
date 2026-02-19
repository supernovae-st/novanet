# Core Principles

Foundational philosophy and methodology for NovaNet.

## ADRs in this Domain

| ADR | Name | Status | Summary |
|-----|------|--------|---------|
| [001](adr-001-arc-terminology.md) | Arc Terminology | stable | Use "Arc" (not Edge/Relation) for directed links |
| [003](adr-003-yaml-first.md) | YAML-First Architecture | stable | YAML = source of truth, all code is generated |
| [007](adr-007-generation-not-translation.md) | Generation, Not Translation | stable | Content is generated natively, NOT translated |
| [010](adr-010-skill-first-dx.md) | Skill-First DX | stable | Update DX tools BEFORE implementing code |
| [021](adr-021-query-first.md) | Query-First Architecture | active | Cypher query = source of truth for visualization |

## Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  CORE PRINCIPLES AT A GLANCE                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ADR-007: Generate natively, don't translate                    │
│  ADR-003: YAML is the single source of truth                    │
│  ADR-021: Cypher query determines what you see                  │
│  ADR-001: "Arc" not "Edge" or "Relation"                        │
│  ADR-010: Update skills/docs BEFORE coding                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## When to Consult

- **Starting a new feature**: Check ADR-010 (Skill-First DX)
- **Adding schema elements**: Check ADR-003 (YAML-First)
- **Working with localization**: Check ADR-007 (Generation, Not Translation)
- **Graph visualization**: Check ADR-021 (Query-First)
- **Naming relationships**: Check ADR-001 (Arc Terminology)

## Key Insight

> "If you remember one thing: **Generate natively, don't translate.** This is NovaNet's core philosophy."
