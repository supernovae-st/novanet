# Models Cleanup & Meta-Nodes Plan

**Date**: 2026-02-04
**Status**: Complete
**Goal**: Every graph element has its own YAML file — meta-nodes included

---

## Problem

Meta-nodes (Realm, Layer, Trait, ArcFamily) are defined inline in `taxonomy.yaml`, not as individual YAML files like node-kinds and arc-kinds. This breaks the "YAML for everything" principle.

## Actions

### Phase 1: Create meta/ Structure

```
models/meta/
├── realms/
│   ├── global.yaml
│   ├── project.yaml
│   └── shared.yaml
├── layers/
│   ├── config.yaml
│   ├── knowledge.yaml
│   ├── foundation.yaml
│   ├── structure.yaml
│   ├── semantic.yaml
│   ├── instruction.yaml
│   ├── output.yaml
│   ├── seo.yaml
│   └── geo.yaml
├── traits/
│   ├── invariant.yaml
│   ├── localized.yaml
│   ├── knowledge.yaml
│   ├── derived.yaml
│   └── job.yaml
└── arc-families/
    ├── ownership.yaml
    ├── localization.yaml
    ├── semantic.yaml
    ├── generation.yaml
    └── mining.yaml
```

### Phase 2: Archive Dead Files

- `schema/nodes.schema.json` → `archive/`
- `docs/NOMENCLATURE-REVIEW.md` → `archive/`
- `docs/LOCALE-INDEX.md` → `archive/`
- `archive/GRAPH.md`, `archive/GRAPH-DETAILED.md` → delete (already in archive)

### Phase 3: Cleanup Duplication

- `relations.yaml` — KEEP for now (parsers still use it)
- Add deprecation notice pointing to `arc-kinds/`
- Future: migrate parser to read arc-kinds/ directly

### Phase 4: Update Versions

- All files referencing v9.0, v9.5, v9.7, v9.8 → v9.9.0

---

## Not in Scope

- Migrating Rust parser from relations.yaml to arc-kinds/ (separate task)
- Refactoring taxonomy.yaml generator (depends on meta/ structure)

## Success Criteria

- [x] models/meta/ exists with 22 YAML files (3+9+5+5)
- [x] Dead files archived (schema/, NOMENCLATURE-REVIEW.md, LOCALE-INDEX.md)
- [x] No version inconsistencies (all v9.9.0)
- [x] relations.yaml has deprecation notice
