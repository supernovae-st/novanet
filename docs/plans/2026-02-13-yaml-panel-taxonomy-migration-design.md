# YAML Panel & Taxonomy Migration Design

**Version**: v0.12.5
**Date**: 2026-02-13
**Status**: Draft
**ADR Reference**: Extends ADR-023 (Class/Instance), ADR-024 (Data Origin)

---

## Executive Summary

This design document covers two related refactoring efforts:

1. **TUI YAML Panel Redesign** — Make the YAML panel contextual based on TreeItem selection
2. **Taxonomy Explosion** — Split `taxonomy.yaml` into individual files for consistency

The goal is to achieve 100% uniformity: every entity (Realm, Layer, Trait, ArcFamily, Class, Arc) has its own YAML file.

---

## Part 1: TUI YAML Panel Redesign

### Current State (Problems)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CURRENT YAML PANEL                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. yaml_active_section() ALWAYS returns YamlViewSection::Class                 │
│     → Instance section never shown automatically                                │
│                                                                                 │
│  2. Tabs [Class *] [Instance o] but Instance never activates                    │
│                                                                                 │
│  3. Selecting Realm/Layer shows... last loaded Class YAML (wrong!)              │
│                                                                                 │
│  4. "node metadata" text when peeking (obsolete term)                           │
│                                                                                 │
│  5. Paths reference non-existent directories:                                   │
│     - packages/core/models/meta/realms/{key}.yaml  (doesn't exist)              │
│     - packages/core/models/meta/layers/{key}.yaml  (doesn't exist)              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Target State

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NEW YAML PANEL BEHAVIOR                                                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  TreeItem Selected    │ YAML File Shown              │ Tabs Visible?            │
│  ─────────────────────┼──────────────────────────────┼────────────────────────  │
│  ClassesSection       │ (none or summary)            │ No                       │
│  Realm (shared)       │ realms/shared.yaml           │ No                       │
│  Layer (semantic)     │ layers/semantic.yaml         │ No                       │
│  Class (Page)         │ node-kinds/.../page.yaml     │ Yes [Class ◆]            │
│  Instance (page:home) │ node-kinds/.../page.yaml     │ Yes [Instance ◆]         │
│  EntityCategory       │ (entity-category.yaml)       │ No                       │
│  ArcsSection          │ (none or summary)            │ No                       │
│  ArcFamily (ownership)│ arc-families/ownership.yaml  │ No                       │
│  ArcClass (HAS_PAGE)  │ arc-kinds/.../has-page.yaml  │ No                       │
│                                                                                 │
│  TAB FORMAT: [Class ◆] [Instance ○]  where ◆=active, ○=inactive                 │
│                                                                                 │
│  PEEK TEXT:                                                                     │
│  • When Class active:    "... properties (N lines) [Enter: peek] ..."           │
│  • When Instance active: "... class definition (N lines) [Enter: peek] ..."     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Files to Modify (TUI)

| File | Changes | Lines |
|------|---------|-------|
| `tui/yaml.rs` | Rename `YamlViewSection::Class` → keep, fix comment "Class metadata" | 17 |
| `tui/yaml_panel.rs` | New tab format `[Class ◆]`, fix "node metadata" → "class definition" | 138, 282-288 |
| `tui/app.rs` | `yaml_active_section()` based on TreeItem, fix meta/ paths | 361-366, 412-422 |
| `tui/data.rs` | Fix `node-classes/` → `node-kinds/` | 488, 3221 |

---

## Part 2: Taxonomy Explosion

### Current Structure (Monolithic)

```
packages/core/models/
├── taxonomy.yaml          ← EVERYTHING in one file (realms, layers, traits, families)
├── visual-encoding.yaml   ← Icons and visual channels
├── node-kinds/            ← Individual Class files ✓
│   └── {realm}/{layer}/{class}.yaml
└── arc-kinds/             ← Individual Arc files ✓
    └── {family}/{arc}.yaml
```

### Target Structure (Uniform)

```
packages/core/models/
├── realms/
│   ├── _index.yaml        ← ["shared", "org"]
│   ├── shared.yaml        ← Full realm definition
│   └── org.yaml
│
├── layers/
│   ├── _index.yaml        ← ["config", "locale", "geography", ...]
│   ├── config.yaml
│   ├── locale.yaml
│   ├── geography.yaml
│   ├── knowledge.yaml
│   ├── foundation.yaml
│   ├── structure.yaml
│   ├── semantic.yaml
│   ├── instruction.yaml
│   └── output.yaml
│
├── traits/
│   ├── _index.yaml        ← ["defined", "authored", "imported", "generated", "retrieved"]
│   ├── defined.yaml
│   ├── authored.yaml
│   ├── imported.yaml
│   ├── generated.yaml
│   └── retrieved.yaml
│
├── arc-families/
│   ├── _index.yaml        ← ["ownership", "localization", "semantic", "generation", "mining"]
│   ├── ownership.yaml
│   ├── localization.yaml
│   ├── semantic.yaml
│   ├── generation.yaml
│   └── mining.yaml
│
├── node-kinds/            ← (unchanged)
│   ├── _index.yaml
│   └── {realm}/{layer}/{class}.yaml
│
├── arc-kinds/             ← (unchanged)
│   └── {family}/{arc}.yaml
│
├── visual-encoding.yaml   ← (unchanged, icons source of truth)
│
└── ❌ taxonomy.yaml       ← DELETED
```

### YAML File Examples

#### realms/shared.yaml

```yaml
realm:
  key: shared
  display_name: "Shared"
  description: |
    Universal knowledge (READ-ONLY). Content shared across all organizations.
    Includes locale definitions, geographic data, and knowledge atoms.
  color: "#2aa198"
  icon:
    web: "globe"
    terminal: "◉"

  layers: [config, locale, geography, knowledge]
  node_count: 39  # Computed by generator

  llm_context: |
    USE: when accessing universal locale knowledge.
    TRIGGERS: "shared data", "universal", "read-only", "locale knowledge".
    NOT: for organization-specific content (use org realm).
    RELATES: Locale (config layer), Term (knowledge layer).
```

#### layers/semantic.yaml

```yaml
layer:
  key: semantic
  realm: org
  display_name: "Semantic"
  description: |
    Meaning and knowledge relationships. Entities and their locale-specific content.
    Contains the core business concepts that drive content generation.
  color: "#d33682"
  icon:
    web: "brain"
    terminal: "◆"

  classes: [Entity, EntityContent, AudiencePersona, ChannelSurface]
  class_count: 4  # Computed by generator

  llm_context: |
    USE: when working with semantic entities and their content.
    TRIGGERS: "entity", "meaning", "audience", "channel".
    NOT: for page structure (use structure layer).
    RELATES: Entity (defined), EntityContent (authored).
```

#### traits/defined.yaml

```yaml
trait:
  key: defined
  display_name: "Defined"
  description: |
    Human-created once. Structure and templates that don't vary by locale.
    The skeleton on which localized content is hung.
  color: "#268bd2"
  border_style: solid
  icon:
    web: "square"
    terminal: "■"

  node_count: 31  # Computed by generator

  context_budget:
    min: 500
    max: 2000
    default: 1000

  llm_context: |
    USE: when working with structural definitions.
    TRIGGERS: "template", "structure", "schema", "invariant".
    NOT: for locale-specific content (use authored trait).
    RELATES: Page (structure), Block (structure), Locale (config).
```

#### arc-families/ownership.yaml

```yaml
arc_family:
  key: ownership
  display_name: "Ownership"
  description: |
    Parent→Child hierarchy relationships. The primary structural backbone
    of the knowledge graph.
  color: "#859900"
  stroke_style: solid
  icon:
    web: "git-branch"
    terminal: "→"

  arc_count: 43  # Computed by generator

  inverse_convention: "*_OF"  # HAS_PAGE → PAGE_OF

  llm_context: |
    USE: when traversing ownership hierarchies.
    TRIGGERS: "parent", "child", "contains", "belongs to".
    NOT: for semantic relationships (use semantic family).
    RELATES: HAS_PAGE, HAS_ENTITY, HAS_BLOCK (key arcs).
```

---

## Part 3: Impact Analysis

### Rust Files to Modify

| File | Current | Changes Needed | Effort |
|------|---------|----------------|--------|
| `config.rs` | `taxonomy_path()` | Add `realms_dir()`, `layers_dir()`, `traits_dir()`, `arc_families_dir()` | 1h |
| `parsers/taxonomy.rs` | Monolithic `TaxonomyDoc` | Split into `RealmDoc`, `LayerDoc`, `TraitDoc`, `ArcFamilyDoc` + loaders | 4h |
| `parsers/mod.rs` | Exports taxonomy | Add new parser modules | 0.5h |
| `generators/organizing.rs` | Reads `TaxonomyDoc` | Read from individual files | 2h |
| `generators/colors.rs` | Uses organizing conversion | Update to new loaders | 1h |
| `generators/hierarchy.rs` | Uses organizing conversion | Update to new loaders | 1h |
| `generators/mermaid.rs` | `trait_color()`, `arc_color()` | Load from individual files | 1h |
| `blueprint/sources.rs` | `taxonomy: TaxonomyDoc` | Replace with individual collections | 2h |
| `commands/schema.rs` | Validates against taxonomy | Update validation to new structure | 1h |
| `commands/doctor.rs` | Checks taxonomy.yaml | Check new directories | 0.5h |
| `tui/app.rs` | Broken meta/ paths | Use new realms/, layers/ paths | 1h |
| `tui/data.rs` | `:Meta:` Cypher queries | Update to `:Schema:` | 1h |
| `tui/theme.rs` | Hardcoded colors | Load from YAML | 1h |
| `tui/icons.rs` | `MODES_META` constant | Rename to `MODES_GRAPH` | 0.5h |

**Total Rust effort: ~18 hours**

### TypeScript Files to Modify

| File | Changes Needed | Effort |
|------|----------------|--------|
| `types/nodes.ts` | Generated, will auto-update | 0h |
| `graph/layers.ts` | Generated, will auto-update | 0h |
| `graph/hierarchy.ts` | Generated, will auto-update | 0h |
| `graph/__tests__/design-system-coherence.test.ts` | Update YAML paths in tests | 1h |

**Total TypeScript effort: ~1 hour** (mostly auto-generated)

### YAML Files to Create

| Directory | Files | Content Source |
|-----------|-------|----------------|
| `realms/` | 3 (index + 2 realms) | Extract from taxonomy.yaml |
| `layers/` | 11 (index + 10 layers) | Extract from taxonomy.yaml |
| `traits/` | 6 (index + 5 traits) | Extract from taxonomy.yaml |
| `arc-families/` | 6 (index + 5 families) | Extract from taxonomy.yaml |

**Total: 26 new YAML files**

---

## Part 4: Implementation Plan

### Phase 1: Create New YAML Structure (Day 1)

**Tasks:**

1. Create directory structure
   ```bash
   mkdir -p packages/core/models/{realms,layers,traits,arc-families}
   ```

2. Extract realm definitions from taxonomy.yaml → `realms/*.yaml`
3. Extract layer definitions from taxonomy.yaml → `layers/*.yaml`
4. Extract trait definitions from taxonomy.yaml → `traits/*.yaml`
5. Extract arc family definitions from taxonomy.yaml → `arc-families/*.yaml`
6. Create `_index.yaml` files for each directory
7. **DO NOT DELETE taxonomy.yaml yet** (keep for comparison)

**Validation:**
- All 26 files created
- YAML syntax valid (`yamllint`)
- Content matches taxonomy.yaml data

### Phase 2: Update Rust Parsers (Day 2-3)

**Tasks:**

1. Add new path functions to `config.rs`:
   ```rust
   pub fn realms_dir(root: &Path) -> PathBuf
   pub fn layers_dir(root: &Path) -> PathBuf
   pub fn traits_dir(root: &Path) -> PathBuf
   pub fn arc_families_dir(root: &Path) -> PathBuf
   ```

2. Create new parser structs in `parsers/`:
   ```rust
   // parsers/realm.rs
   pub struct RealmDoc { pub realm: RealmDef }
   pub fn load_realm(root: &Path, key: &str) -> Result<RealmDoc>
   pub fn load_all_realms(root: &Path) -> Result<Vec<RealmDoc>>

   // parsers/layer.rs
   pub struct LayerDoc { pub layer: LayerDef }
   pub fn load_layer(root: &Path, key: &str) -> Result<LayerDoc>
   pub fn load_all_layers(root: &Path) -> Result<Vec<LayerDoc>>

   // parsers/trait_def.rs (trait is reserved keyword)
   pub struct TraitDoc { pub trait_def: TraitDef }
   pub fn load_trait(root: &Path, key: &str) -> Result<TraitDoc>
   pub fn load_all_traits(root: &Path) -> Result<Vec<TraitDoc>>

   // parsers/arc_family.rs
   pub struct ArcFamilyDoc { pub arc_family: ArcFamilyDef }
   pub fn load_arc_family(root: &Path, key: &str) -> Result<ArcFamilyDoc>
   pub fn load_all_arc_families(root: &Path) -> Result<Vec<ArcFamilyDoc>>
   ```

3. Update `parsers/mod.rs` to export new modules

4. Keep `parsers/taxonomy.rs` as compatibility layer (loads from new files, returns `TaxonomyDoc`)

**Validation:**
- `cargo test` passes
- New parsers load correctly
- Backward compatibility maintained

### Phase 3: Update Generators (Day 3-4)

**Tasks:**

1. Update `generators/organizing.rs`:
   - Load realms, layers, traits, arc_families from individual files
   - Generate same Cypher output

2. Update `generators/colors.rs`:
   - Load colors from individual realm/layer/trait/family files

3. Update `generators/hierarchy.rs`:
   - Build hierarchy from individual layer files

4. Update `generators/mermaid.rs`:
   - Load trait colors from `traits/*.yaml`
   - Load arc family colors from `arc-families/*.yaml`

**Validation:**
- `cargo run -- schema generate` produces identical output
- `cargo run -- schema validate` passes
- Snapshot tests pass

### Phase 4: Update TUI (Day 4-5)

**Tasks:**

1. Fix `tui/app.rs`:
   - Update `yaml_active_section()` to check TreeItem type
   - Fix paths: `meta/realms/` → `realms/`
   - Fix paths: `meta/layers/` → `layers/`
   - Fix paths: `meta/arc-families/` → `arc-families/`

2. Fix `tui/data.rs`:
   - Fix `node-classes/` → `node-kinds/`
   - Update Cypher queries: `:Meta:` → `:Schema:`

3. Fix `tui/yaml_panel.rs`:
   - Update tab format: `[Class ◆] [Instance ○]`
   - Fix peek text: "node metadata" → "class definition"

4. Fix `tui/icons.rs`:
   - Rename `MODES_META` → `MODES_GRAPH`

5. Fix `tui/theme.rs`:
   - Rename `META_HEX` → `GRAPH_HEX`

**Validation:**
- TUI launches without errors
- YAML panel shows correct content for each TreeItem
- All terminology updated

### Phase 5: Update Blueprint & Commands (Day 5)

**Tasks:**

1. Update `blueprint/sources.rs`:
   - Replace `taxonomy: TaxonomyDoc` with individual collections
   - Update `realm_count()`, `layer_count()` methods

2. Update `commands/schema.rs`:
   - Validation logic to use new structure

3. Update `commands/doctor.rs`:
   - Check new directories exist

**Validation:**
- `cargo run -- blueprint` works
- `cargo run -- schema validate` works
- `cargo run -- doctor` reports healthy

### Phase 6: Delete taxonomy.yaml & Final Cleanup (Day 6)

**Tasks:**

1. Delete `packages/core/models/taxonomy.yaml`
2. Remove `parsers/taxonomy.rs` compatibility code (if no longer needed)
3. Update all documentation references
4. Update CLAUDE.md
5. Update ADR documentation

**Validation:**
- Full test suite passes: `cargo nextest run`
- No references to taxonomy.yaml remain
- `cargo run -- schema generate` works
- `cargo run -- tui` works

---

## Part 5: Skills & Tools to Use

### Recommended Skills

| Task | Skill | Why |
|------|-------|-----|
| Creating YAML files | Manual or script | Simple extraction |
| Rust parser changes | `spn-rust:rust-core` | Serde patterns, error handling |
| TUI updates | `spn-rust:rust-core` | Ratatui patterns |
| Testing | `test-driven-development` | Write tests before changing |
| Commit messages | `spn-powers:git:commit` | Conventional commits |
| Final review | `spn-powers:requesting-code-review` | Verify implementation |

### Recommended Agents

| Phase | Agent | Purpose |
|-------|-------|---------|
| Phase 2-3 | `rust-pro` | Complex parser refactoring |
| Phase 4 | `rust-pro` | TUI state management |
| Phase 6 | `code-reviewer` | Final review before merge |

### Testing Strategy

```bash
# After each phase
cargo fmt && cargo clippy -- -D warnings && cargo nextest run

# After Phase 3 (generators)
cargo run -- schema generate --dry-run
diff packages/db/seed/00.5-taxonomy.cypher packages/db/seed/00.5-taxonomy.cypher.bak

# After Phase 4 (TUI)
cargo run -- tui  # Manual testing

# After Phase 6 (final)
cargo deny check && cargo audit
pnpm test --filter=@novanet/core
```

---

## Part 6: Rollback Plan

If issues arise:

1. **Phase 1-2**: Delete new directories, keep taxonomy.yaml
2. **Phase 3-4**: Revert Rust changes, keep YAML files for later
3. **Phase 5-6**: Git revert to pre-deletion commit

**Safe point**: After Phase 4, when both structures work in parallel.

---

## Part 7: Success Criteria

- [ ] All 26 new YAML files created and valid
- [ ] `taxonomy.yaml` deleted
- [ ] `cargo nextest run` — 980+ tests pass
- [ ] `cargo run -- schema generate` produces correct output
- [ ] `cargo run -- tui` shows contextual YAML for all TreeItem types
- [ ] YAML panel tabs show `[Class ◆] [Instance ○]`
- [ ] No "meta", "node kind", "node metadata" terminology in TUI
- [ ] Documentation updated

---

## Appendix A: Full File List to Modify

### Rust (tools/novanet/src/)

```
config.rs                      # Add path functions
parsers/mod.rs                 # Export new modules
parsers/taxonomy.rs            # Compatibility layer or delete
parsers/realm.rs               # NEW
parsers/layer.rs               # NEW
parsers/trait_def.rs           # NEW
parsers/arc_family.rs          # NEW
generators/organizing.rs       # Update loaders
generators/colors.rs           # Update loaders
generators/hierarchy.rs        # Update loaders
generators/mermaid.rs          # Update color lookups
blueprint/sources.rs           # Replace TaxonomyDoc
commands/schema.rs             # Update validation
commands/doctor.rs             # Check new dirs
tui/app.rs                     # Fix paths, yaml_active_section
tui/data.rs                    # Fix paths, Cypher labels
tui/yaml.rs                    # Update comments
tui/yaml_panel.rs              # Tab format, peek text
tui/icons.rs                   # Rename MODES_META
tui/theme.rs                   # Rename META_HEX
tui/ui/tree.rs                 # Comments cleanup
tui/ui/info.rs                 # Comments cleanup
tui/ui/overlays.rs             # Comments cleanup
cypher.rs                      # :Meta: → :Schema:
commands/search.rs             # :Meta: → :Schema:
output.rs                      # Test cleanup
```

### YAML (packages/core/models/)

```
realms/_index.yaml             # NEW
realms/shared.yaml             # NEW
realms/org.yaml                # NEW
layers/_index.yaml             # NEW
layers/config.yaml             # NEW
layers/locale.yaml             # NEW
layers/geography.yaml          # NEW
layers/knowledge.yaml          # NEW
layers/foundation.yaml         # NEW
layers/structure.yaml          # NEW
layers/semantic.yaml           # NEW
layers/instruction.yaml        # NEW
layers/output.yaml             # NEW
traits/_index.yaml             # NEW
traits/defined.yaml            # NEW
traits/authored.yaml           # NEW
traits/imported.yaml           # NEW
traits/generated.yaml          # NEW
traits/retrieved.yaml          # NEW
arc-families/_index.yaml       # NEW
arc-families/ownership.yaml    # NEW
arc-families/localization.yaml # NEW
arc-families/semantic.yaml     # NEW
arc-families/generation.yaml   # NEW
arc-families/mining.yaml       # NEW
taxonomy.yaml                  # DELETE
```

### TypeScript (packages/core/src/)

```
graph/__tests__/design-system-coherence.test.ts  # Update paths
```

---

## Appendix B: Terminology Cleanup Reference

| Old Term | New Term | Files Affected |
|----------|----------|----------------|
| `meta` (mode) | `graph` | icons.rs, theme.rs, comments |
| `META_HEX` | `GRAPH_HEX` | theme.rs |
| `MODES_META` | `MODES_GRAPH` | icons.rs |
| `:Meta:` (Neo4j) | `:Schema:` | cypher.rs, data.rs, search.rs |
| `node metadata` | `class definition` | yaml_panel.rs |
| `kind` (variable) | `class` | app.rs, data.rs, tree.rs |
| `node-classes/` | `node-kinds/` | data.rs |
| `meta/realms/` | `realms/` | app.rs |
| `meta/layers/` | `layers/` | app.rs |
| `meta/arc-families/` | `arc-families/` | app.rs |

---

*Document generated by brainstorming session 2026-02-13*
