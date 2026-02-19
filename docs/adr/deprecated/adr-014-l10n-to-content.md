---
id: "014"
title: "Naming Convention Refactor (L10n to Content/Generated)"
version: "v10.9.0"
status: "historical"
domain: "node-classification"
note: "Foundation for v0.13.0 *Native pattern (ADR-029)"
---

# ADR-014: Naming Convention Refactor (L10n to Content/Generated)

**Status**: Approved (v10.9.0)

**Decision**: Rename `*L10n` nodes and arcs to semantically clearer names that distinguish content storage from generation output.

**Renames**:

| Old Name | New Name | Reason |
|----------|----------|--------|
| `EntityL10n` | `EntityNative` | Stores semantic content, not localization metadata |
| `PageL10n` | `PageNative` | Clarifies this is LLM generation output |
| `BlockL10n` | `BlockNative` | Parallel naming with PageNative |
| `ProjectL10n` | `ProjectNative` | v11.0: Consistent with EntityNative pattern |
| `HAS_L10N` | `HAS_NATIVE` | Content relationship, not localization |
| `HAS_OUTPUT` | `HAS_NATIVE` | Moved to generation family, clarifies purpose |
| `BELONGS_TO_PROJECT_L10N` | `BELONGS_TO_PROJECT_CONTENT` | v11.0: Follows ProjectNative rename |
| `GEOSeedL10n` | `GEOQuery` | v10.7: New GEO schema |
| `GEOSeedMetrics` | `GEOMetrics` | v10.7: New GEO schema |

**Composite Key Pattern**:

Content and generated nodes use composite keys to ensure uniqueness across locales:

```
EntityNative key:  entity:{entity_key}@{locale_key}
PageNative key:  page:{page_key}@{locale_key}
BlockNative key: block:{block_key}@{locale_key}

Examples:
  entity:qr-code-generator@fr-FR
  page:homepage@de-DE
  block:hero-section@ja-JP
```

**Key format**: `{kind}:{invariant_key}@{locale_key}`
- `{kind}`: lowercase node kind prefix (entity, page, block)
- `{invariant_key}`: key of the parent invariant node
- `@`: separator (not valid in keys, unambiguous parsing)
- `{locale_key}`: BCP-47 locale code

**Rationale**:

1. **Semantic clarity**: `L10n` (localization) implies translation, but NovaNet generates natively. `Content` and `Generated` describe what the node actually contains.

2. **Layer distinction**:
   - `EntityNative` lives in **semantic** layer (meaning, knowledge)
   - `PageNative`/`BlockNative` live in **output** layer (rendered artifacts)

3. **Arc family alignment**:
   - `HAS_NATIVE` stays in **semantic** family (content relationship)
   - `HAS_NATIVE` moves to **generation** family (output relationship)

4. **Composite key benefits**:
   - Unique across all locales without additional index
   - Parseable: extract invariant key or locale with simple split
   - Query-friendly: `STARTS WITH 'entity:qr-code-generator@'` finds all locales
   - Self-documenting: key reveals parent and locale at a glance

**Migration**:

```cypher
// Rename node labels
MATCH (n:EntityL10n) SET n:EntityNative REMOVE n:EntityL10n;
MATCH (n:PageL10n) SET n:PageNative REMOVE n:PageL10n;
MATCH (n:BlockL10n) SET n:BlockNative REMOVE n:BlockL10n;

// Update relationship types (requires recreation in Neo4j)
// HAS_L10N -> HAS_NATIVE
// HAS_OUTPUT -> HAS_NATIVE
```

**Code impact**:
- YAML: Update `node-classes/` and `arc-classes/` files
- Generators: Names propagate automatically via YAML-first architecture
- Queries: Search-replace in Cypher files and Rust code
