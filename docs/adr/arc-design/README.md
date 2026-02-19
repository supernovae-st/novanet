# Arc Design

Arc families, inverses, and relationship patterns.

## ADRs in this Domain

| ADR | Name | Status | Summary |
|-----|------|--------|---------|
| [015](adr-015-unidirectional-ownership.md) | Unidirectional Ownership Arcs | stable | Only subset needs explicit inverses |
| [016](adr-016-type-constrained-containers.md) | Type-Constrained Container Arcs | stable | CONTAINS_TERM, CONTAINS_EXPRESSION... |
| [026](adr-026-inverse-arc-policy.md) | Inverse Arc Policy | active | TIER 1/2/3 classification |
| [027](adr-027-generation-family.md) | Generation Family Arc Semantics | active | Pipeline docs, llm_context pattern |

## Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  ARC FAMILIES (5)                                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ownership     Parent→Child    HAS_PAGE, HAS_ENTITY, HAS_BLOCK  │
│  localization  Locale links    FOR_LOCALE, HAS_VOICE            │
│  semantic      Meaning links   USES_ENTITY, REFERENCES          │
│  generation    LLM pipeline    GENERATED, COMPILED_FROM         │
│  mining        SEO/GEO intel   TARGETS_KEYWORD, MONITORS_GEO    │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  INVERSE ARC TIERS (ADR-026)                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  TIER 1  Required    HAS_ENTITY↔ENTITY_OF, HAS_PAGE↔PAGE_OF     │
│  TIER 2  Recommended HAS_TERMS↔TERMS_OF, USES_ENTITY↔USED_BY    │
│  TIER 3  Optional    CONTAINS_*, BELONGS_TO_ORG                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## When to Consult

- **Creating a new arc**: Check ADR-026 (inverse policy) + arc-design-guide.md
- **Container relationships**: Check ADR-016 (type-constrained)
- **Generation pipeline arcs**: Check ADR-027 (semantics)
- **Inverse needed?**: Check ADR-015 + ADR-026 (tier classification)

## Key Insight

> "Not every arc needs an inverse. Check the tier before creating one."

## Related

- See also: `arc-design-guide.md` in rules folder for practical patterns
