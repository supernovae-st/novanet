# Node Classification

Node naming conventions, traits, and classification axes.

## ADRs in this Domain

| ADR | Name | Status | Summary |
|-----|------|--------|---------|
| [002](adr-002-symmetric-taxonomy.md) | Symmetric Taxonomy | stable | Prefixed types, short properties |
| [023](adr-023-class-instance.md) | Class/Instance Terminology | active | Kind→Class, Meta→Schema |
| [024](adr-024-trait-data-origin.md) | Trait = Data Origin | active | WHERE does data come from? |
| [025](adr-025-instruction-layer.md) | Instruction Layer Renaming | active | PageType→PageStructure, *Prompt→*Instruction |

## Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  TRAIT = DATA ORIGIN (ADR-024)                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  defined    Human creates ONCE       Page, Block, Entity        │
│  authored   Human writes PER locale  EntityNative, ProjectNative│
│  imported   External data brought in Term, SEOKeyword, GEOQuery │
│  generated  Our LLM produces         PageNative, BlockNative    │
│  retrieved  Fetched from external    GEOAnswer, SEOKeywordMetrics│
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  NAMING PATTERNS                                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  *Native    Locale-specific content  EntityNative, PageNative   │
│  *Category  Categorical grouping     EntityCategory             │
│  *Set       Container for atoms      TermSet, ExpressionSet     │
│  *Structure JSON defining order      PageStructure              │
│  *Instruction Markdown with @ refs   PageInstruction            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## When to Consult

- **Naming a new node**: Check ADR-002 (taxonomy patterns)
- **Understanding traits**: Check ADR-024 (data origin)
- **Instruction layer nodes**: Check ADR-025 (Structure vs Instruction)
- **Kind vs Class confusion**: Check ADR-023 (terminology update)

## Key Insight

> "Trait answers WHERE does data come from? — not what it IS (that's Layer)."
