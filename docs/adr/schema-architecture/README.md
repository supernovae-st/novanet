# Schema Architecture

Realm, layer, and node organization for the NovaNet knowledge graph.

## ADRs in this Domain

| ADR | Name | Status | Summary |
|-----|------|--------|---------|
| [006](adr-006-realm-scope.md) | Realm Differentiates Scope | stable | Same type can exist in different realms |
| [012](adr-012-two-realm.md) | 2-Realm Architecture | stable | SHARED (read-only) + ORG (business) |
| [017](adr-017-entity-category.md) | EntityCategory Classification | stable | Entity.type enum → EntityCategory nodes |
| [028](adr-028-page-entity.md) | Page-Entity Architecture | active | 1:1 mandatory, @ refs, Brand Architecture |
| [029](adr-029-native-pattern.md) | *Native Pattern | active | Unified suffix: EntityNative, PageNative |
| [030](adr-030-slug-ownership.md) | Slug Ownership | active | Page owns URL, Entity owns semantics |
| [033](adr-033-denomination-forms.md) | Denomination Forms | active | Prescriptive canonical forms for LLM entity references |

## Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  v0.13.0 ARCHITECTURE                                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  SHARED (40 nodes, READ-ONLY)                                   │
│  ├── config      (3)  EntityCategory, Locale, SEOKeywordFormat  │
│  ├── locale      (6)  Culture, Style, Formatting...             │
│  ├── geography   (7)  Continent, Region, Country...             │
│  └── knowledge  (24)  Terms, Expressions, SEO, GEO...           │
│                                                                 │
│  ORG (21 nodes)                                                 │
│  ├── config      (1)  OrgConfig                                 │
│  ├── foundation  (6)  Project, Brand, BrandDesign...            │
│  ├── structure   (3)  Page, Block, ContentSlot                  │
│  ├── semantic    (4)  Entity, EntityNative...                   │
│  ├── instruction (4)  PageStructure, BlockInstruction...        │
│  └── output      (3)  PageNative, BlockNative, OutputArtifact   │
│                                                                 │
│  TOTAL: 61 nodes, 182 arcs, 10 layers, 6 arc families, 5 traits │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## When to Consult

- **Adding a new node type**: Check ADR-012 (which realm/layer?)
- **Locale-specific content**: Check ADR-029 (*Native Pattern)
- **Page/Entity relationships**: Check ADR-028 and ADR-030
- **Categorizing entities**: Check ADR-017 (EntityCategory)

## Key Insight

> "SHARED = universal knowledge (read-only). ORG = your organization's content."
