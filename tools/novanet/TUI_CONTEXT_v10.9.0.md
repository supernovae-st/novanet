# TUI Context: v10.9.0 Naming Convention Refactor

**Date**: 2026-02-08
**For**: TUI extraction agents (info.rs, tree.rs)
**From**: Schema refactor agent

---

## IMPORTANT: Schema Changes Affecting TUI

The following v10.9.0 changes have been committed and may affect your TUI extraction work:

### Node Renames (update any references)

| Old Name | New Name | Layer | Trait |
|----------|----------|-------|-------|
| `EntityL10n` | `EntityContent` | semantic | localized |
| `BlockL10n` | `BlockGenerated` | output | **derived** |
| `PageL10n` | `PageGenerated` | output | **derived** |

### Arc Renames (update Cypher queries)

| Old Arc | New Arc | Family |
|---------|---------|--------|
| `HAS_L10N` | `HAS_CONTENT` | localization |
| `HAS_OUTPUT` | `HAS_GENERATED` | generation |
| `L10N_OF` | `CONTENT_OF` | localization |
| `OUTPUT_OF` | `GENERATED_FOR` | generation |

### New Arc: HAS_CHILD

```yaml
arc:
  name: HAS_CHILD
  family: ownership
  source: Entity
  target: Entity
  cardinality: many_to_many
  properties:
    position: int
    visibility: enum [public, draft, internal]
    featured: boolean
  cycle_protection: true
  max_depth: 3
```

---

## TUI-Specific Updates

### 1. Generated icons.rs is updated

The file `tools/novanet/src/tui/icons.rs` has been regenerated with v10.9.0 changes.

### 2. Data types to check

If your extraction references these types in `data.rs`, verify they still exist:
- `RealmNode`
- `LayerNode`
- `KindNode`
- `InstanceNode`
- `ArcFamilyNode`
- `ArcKindNode`

The current `data.rs` uses:
- `RealmInfo`, `LayerInfo`, `KindInfo` (for meta)
- `InstanceData` (for instances)
- `ArcFamilyInfo`, `ArcKindInfo` (for arcs)

### 3. Cypher queries to update

Any Cypher in TUI code should use new arc names:
```cypher
// OLD
MATCH (e:Entity)-[:HAS_L10N]->(el:EntityL10n)

// NEW
MATCH (e:Entity)-[:HAS_CONTENT]->(ec:EntityContent)
```

---

## Current Git State

```
94b1972 feat(schema): v10.9.0 naming convention refactor (54 files)
7bef414 fix(schema): complete v10.9.0 arc naming migration (17 files)
```

Your extraction work (info.rs, tree.rs) was temporarily removed to allow schema generation.
You may need to:
1. Pull latest changes
2. Re-extract with correct type names
3. Update any hardcoded node/arc names

---

## Contact

If you need the full design spec, read:
`docs/plans/2026-02-08-naming-convention-design.md`
