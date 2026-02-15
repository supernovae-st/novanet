# NovaNet v0.13 Schema Completion - Execution Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete v0.13 schema implementation with ADR-029 through ADR-032, taxonomy explosion, and TUI enhancements.

**Architecture:** YAML-first schema with Rust generators. All changes flow through YAML → `cargo run -- schema generate` → TypeScript/Cypher/Mermaid artifacts. TDD with `cargo test` verification after each phase.

**Tech Stack:** Rust (CLI/generators), YAML (schema definitions), TypeScript (Studio), Neo4j (graph database)

---

## Current Status Summary

| Item | Status | Notes |
|------|--------|-------|
| ADR-031 (SEO Pillar/Cluster) | ✅ DONE | SEO_CLUSTER_OF, LINKS_TO arcs + is_pillar property |
| ADR-032 (URL Slugification) | ✅ DONE | DERIVED_SLUG_FROM arc created |
| Icon dual format | ✅ DONE | All 61 nodes converted |
| llm_context coverage | ✅ DONE | 61/61 nodes (100%) |
| Arc count | ✅ DONE | 171 arcs, tests pass |
| ADR-029 (*Native Pattern) | ❌ TODO | Node renames + arc merges |
| ADR-030 (Slug Ownership) | ❌ TODO | Property migrations |
| Taxonomy Explosion | ❌ TODO | 26 files to create |
| TUI YAML Panel | ❌ TODO | Contextual display |

---

## Batch 1: ADR-029 *Native Pattern (4 node renames + 3 arc changes)

### Task 1.1: Rename EntityContent → EntityNative

**Files:**
- Rename: `packages/core/models/node-classes/org/semantic/entity-content.yaml` → `entity-native.yaml`
- Modify: All files referencing `EntityContent`

**Step 1: Rename the file**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq
mv packages/core/models/node-classes/org/semantic/entity-content.yaml \
   packages/core/models/node-classes/org/semantic/entity-native.yaml
```

**Step 2: Update node name in YAML**

```yaml
# entity-native.yaml
node:
  name: EntityNative  # was EntityContent
  # ... rest unchanged except:
  description: "Locale-specific native content for an Entity (human-authored)"
```

**Step 3: Update all references**

```bash
# Find and update all EntityContent references
grep -rl "EntityContent" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/EntityContent/EntityNative/g'
```

**Step 4: Verify no orphan references**

```bash
grep -r "EntityContent" packages/core/models/ --include="*.yaml"
# Expected: No output
```

---

### Task 1.2: Rename ProjectContent → ProjectNative

**Files:**
- Rename: `packages/core/models/node-classes/org/foundation/project-content.yaml` → `project-native.yaml`

**Step 1: Rename the file**

```bash
mv packages/core/models/node-classes/org/foundation/project-content.yaml \
   packages/core/models/node-classes/org/foundation/project-native.yaml
```

**Step 2: Update node name in YAML**

```yaml
# project-native.yaml
node:
  name: ProjectNative  # was ProjectContent
  description: "Locale-specific native content for a Project (human-authored)"
```

**Step 3: Update all references**

```bash
grep -rl "ProjectContent" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/ProjectContent/ProjectNative/g'
```

---

### Task 1.3: Rename PageGenerated → PageNative

**Files:**
- Rename: `packages/core/models/node-classes/org/output/page-generated.yaml` → `page-native.yaml`

**Step 1: Rename the file**

```bash
mv packages/core/models/node-classes/org/output/page-generated.yaml \
   packages/core/models/node-classes/org/output/page-native.yaml
```

**Step 2: Update node name in YAML**

```yaml
# page-native.yaml
node:
  name: PageNative  # was PageGenerated
  trait: generated  # keeps trait
  description: "Locale-specific native content for a Page (LLM-generated)"
```

**Step 3: Update all references**

```bash
grep -rl "PageGenerated" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/PageGenerated/PageNative/g'
```

---

### Task 1.4: Rename BlockGenerated → BlockNative

**Files:**
- Rename: `packages/core/models/node-classes/org/output/block-generated.yaml` → `block-native.yaml`

**Step 1: Rename the file**

```bash
mv packages/core/models/node-classes/org/output/block-generated.yaml \
   packages/core/models/node-classes/org/output/block-native.yaml
```

**Step 2: Update node name in YAML**

```yaml
# block-native.yaml
node:
  name: BlockNative  # was BlockGenerated
  trait: generated  # keeps trait
  description: "Locale-specific native content for a Block (LLM-generated)"
```

**Step 3: Update all references**

```bash
grep -rl "BlockGenerated" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/BlockGenerated/BlockNative/g'
```

---

### Task 1.5: Merge HAS_CONTENT + HAS_GENERATED → HAS_NATIVE

**Files:**
- Rename: `packages/core/models/arc-classes/ownership/has-content.yaml` → `has-native.yaml`
- Delete: `packages/core/models/arc-classes/ownership/has-generated.yaml`

**Step 1: Rename has-content.yaml**

```bash
mv packages/core/models/arc-classes/ownership/has-content.yaml \
   packages/core/models/arc-classes/ownership/has-native.yaml
```

**Step 2: Update arc definition**

```yaml
# has-native.yaml
arc:
  name: HAS_NATIVE
  family: ownership
  scope: intra_realm
  source: [Entity, Project, Page, Block]  # All four types
  target: [EntityNative, ProjectNative, PageNative, BlockNative]
  cardinality: one_to_many
  properties:
    - name: locale
      type: string
      required: true
      description: "BCP-47 locale code"
  inverse: NATIVE_OF
  llm_context: |
    USE: when loading locale-specific content for any invariant node.
    TRIGGERS: native content, localized, per-locale, EntityNative, PageNative.
    NOT: for invariant node traversal (use HAS_BLOCK, HAS_PAGE).
    RELATES: Entity/Project/Page/Block (source), *Native (target), NATIVE_OF (inverse).
```

**Step 3: Delete has-generated.yaml**

```bash
rm packages/core/models/arc-classes/ownership/has-generated.yaml
```

**Step 4: Update all HAS_CONTENT → HAS_NATIVE**

```bash
grep -rl "HAS_CONTENT" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/HAS_CONTENT/HAS_NATIVE/g'
```

**Step 5: Update all HAS_GENERATED → HAS_NATIVE**

```bash
grep -rl "HAS_GENERATED" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/HAS_GENERATED/HAS_NATIVE/g'
```

---

### Task 1.6: Merge CONTENT_OF + GENERATED_FOR → NATIVE_OF

**Files:**
- Rename: `packages/core/models/arc-classes/ownership/content-of.yaml` → `native-of.yaml`
- Delete: `packages/core/models/arc-classes/ownership/generated-for.yaml`

**Step 1: Rename content-of.yaml**

```bash
mv packages/core/models/arc-classes/ownership/content-of.yaml \
   packages/core/models/arc-classes/ownership/native-of.yaml
```

**Step 2: Update arc definition**

```yaml
# native-of.yaml
arc:
  name: NATIVE_OF
  family: ownership
  scope: intra_realm
  source: [EntityNative, ProjectNative, PageNative, BlockNative]
  target: [Entity, Project, Page, Block]
  cardinality: many_to_one
  inverse_of: HAS_NATIVE
  llm_context: |
    USE: when finding the invariant parent of locale-specific content.
    TRIGGERS: parent entity, which page, native content owner.
    NOT: for traversing content (use HAS_NATIVE).
    RELATES: *Native (source), Entity/Project/Page/Block (target), HAS_NATIVE (inverse).
```

**Step 3: Delete generated-for.yaml**

```bash
rm packages/core/models/arc-classes/ownership/generated-for.yaml
```

**Step 4: Update references**

```bash
grep -rl "CONTENT_OF" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/CONTENT_OF/NATIVE_OF/g'
grep -rl "GENERATED_FOR" packages/core/models/ --include="*.yaml" | \
  xargs sed -i '' 's/GENERATED_FOR/NATIVE_OF/g'
```

---

### Task 1.7: Validate Batch 1 + Regenerate

**Step 1: Run schema validation**

```bash
cd tools/novanet
cargo run -- schema validate
```

Expected: `0 error(s), 0 warning(s)`

**Step 2: Regenerate artifacts**

```bash
cargo run -- schema generate
```

Expected: `Generated 11 artifact(s)`

**Step 3: Run tests**

```bash
cargo test
```

Expected: All tests pass (may need to update arc count: 171 → 169 after merging 2 arcs)

**Step 4: Commit Batch 1**

```bash
git add -A
git commit -m "feat(schema): ADR-029 *Native pattern - rename nodes and merge arcs

- EntityContent → EntityNative
- ProjectContent → ProjectNative
- PageGenerated → PageNative
- BlockGenerated → BlockNative
- HAS_CONTENT + HAS_GENERATED → HAS_NATIVE
- CONTENT_OF + GENERATED_FOR → NATIVE_OF

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Batch 2: ADR-030 Slug Ownership (Property Migration)

### Task 2.1: Remove slug properties from EntityNative

**Files:**
- Modify: `packages/core/models/node-classes/org/semantic/entity-native.yaml`

**Step 1: Edit entity-native.yaml**

Remove these properties from `properties:` section:
- `slug`
- `full_path`
- `parent_slug`
- `depth`
- `slug_history`

**Step 2: Verify removal**

```bash
grep -E "(slug|full_path|parent_slug|depth|slug_history)" \
  packages/core/models/node-classes/org/semantic/entity-native.yaml
# Expected: No output (or only in llm_context)
```

---

### Task 2.2: Add slug properties to PageNative

**Files:**
- Modify: `packages/core/models/node-classes/org/output/page-native.yaml`

**Step 1: Add properties to page-native.yaml**

```yaml
properties:
  # ... existing properties ...

  slug:
    type: string
    required: true
    description: "Localized URL segment for this page"
    pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$"
    examples:
      - "generateur-qr-code"
      - "pricing"
      - "instagram"

  slug_source:
    type: string
    required: false
    enum: [seo_derived, brand_invariant, manual]
    description: "How the slug was determined"

  slug_rationale:
    type: string
    required: false
    description: "LLM explanation of slug choice (for audit)"

  full_path:
    type: string
    required: true
    indexed: true
    description: "Full localized URL path including locale prefix"
    examples:
      - "/fr/generateur-qr-code"
      - "/en/qr-code-generator/instagram"
```

---

### Task 2.3: Fix Entity.yaml HAS_CHILD comment

**Files:**
- Modify: `packages/core/models/node-classes/org/semantic/entity.yaml`

**Step 1: Find and fix misleading comment**

```bash
grep -n "HAS_CHILD" packages/core/models/node-classes/org/semantic/entity.yaml
```

**Step 2: Update comment**

Change any comment mentioning "URL path = parent.slug" to:
```yaml
# Entity hierarchy is SEMANTIC, not URL-based.
# URL hierarchy is managed by Page.SUBTOPIC_OF.
# See ADR-030 for slug ownership clarity.
```

---

### Task 2.4: Validate Batch 2 + Regenerate

**Step 1: Run schema validation**

```bash
cargo run -- schema validate
```

**Step 2: Regenerate artifacts**

```bash
cargo run -- schema generate
```

**Step 3: Run tests**

```bash
cargo test
```

**Step 4: Commit Batch 2**

```bash
git add -A
git commit -m "feat(schema): ADR-030 slug ownership - move slug from EntityNative to PageNative

- Remove slug properties from EntityNative
- Add slug, slug_source, slug_rationale, full_path to PageNative
- Fix Entity.yaml HAS_CHILD comment

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Batch 3: Taxonomy Explosion (26 files)

### Task 3.1: Create taxonomy directory structure

**Files:**
- Create: `packages/core/models/taxonomy/` directory tree

**Step 1: Create directories**

```bash
mkdir -p packages/core/models/taxonomy/{realms,layers,traits,arc-families}
```

---

### Task 3.2: Create _index.yaml registry

**Files:**
- Create: `packages/core/models/taxonomy/_index.yaml`

**Step 1: Write index file**

```yaml
# packages/core/models/taxonomy/_index.yaml
# NovaNet Taxonomy Registry v0.13
#
# This file imports all taxonomy components.
# Used by Rust generators to build taxonomy.

version: "0.13"

imports:
  realms:
    - realms/shared.yaml
    - realms/org.yaml

  layers:
    - layers/config.yaml
    - layers/locale.yaml
    - layers/geography.yaml
    - layers/knowledge.yaml
    - layers/foundation.yaml
    - layers/structure.yaml
    - layers/semantic.yaml
    - layers/instruction.yaml
    - layers/output.yaml

  traits:
    - traits/defined.yaml
    - traits/authored.yaml
    - traits/imported.yaml
    - traits/generated.yaml
    - traits/retrieved.yaml

  arc_families:
    - arc-families/ownership.yaml
    - arc-families/localization.yaml
    - arc-families/semantic.yaml
    - arc-families/generation.yaml
    - arc-families/mining.yaml
```

---

### Task 3.3: Create realm files (2)

**Files:**
- Create: `packages/core/models/taxonomy/realms/shared.yaml`
- Create: `packages/core/models/taxonomy/realms/org.yaml`

**Step 1: Write shared.yaml**

```yaml
# packages/core/models/taxonomy/realms/shared.yaml
realm:
  key: shared
  display_name: "Shared"
  color: "#2aa198"  # Solarized cyan
  description: "Universal knowledge available to all organizations (READ-ONLY)"
  icon:
    web: "globe"
    terminal: "◉"
  layers:
    - config
    - locale
    - geography
    - knowledge
```

**Step 2: Write org.yaml**

```yaml
# packages/core/models/taxonomy/realms/org.yaml
realm:
  key: org
  display_name: "Organization"
  color: "#0ea5e9"  # Sky blue
  description: "Organization-specific content and configuration"
  icon:
    web: "building-2"
    terminal: "◎"
  layers:
    - config
    - foundation
    - structure
    - semantic
    - instruction
    - output
```

---

### Task 3.4: Create layer files (9)

**Files:**
- Create: `packages/core/models/taxonomy/layers/*.yaml` (9 files)

**Step 1: Write config.yaml**

```yaml
# packages/core/models/taxonomy/layers/config.yaml
layer:
  key: config
  display_name: "Config"
  color: "#6366f1"  # Indigo
  description: "Configuration and system settings"
  icon:
    web: "settings"
    terminal: "⚙"
  realms: [shared, org]
```

**Step 2: Write locale.yaml**

```yaml
# packages/core/models/taxonomy/layers/locale.yaml
layer:
  key: locale
  display_name: "Locale"
  color: "#8b5cf6"  # Violet
  description: "Locale settings and voice configuration"
  icon:
    web: "globe-2"
    terminal: "●"
  realms: [shared]
```

**Step 3: Write geography.yaml**

```yaml
# packages/core/models/taxonomy/layers/geography.yaml
layer:
  key: geography
  display_name: "Geography"
  color: "#ec4899"  # Pink
  description: "Geographic regions and cultural areas"
  icon:
    web: "map"
    terminal: "◆"
  realms: [shared]
```

**Step 4: Write knowledge.yaml**

```yaml
# packages/core/models/taxonomy/layers/knowledge.yaml
layer:
  key: knowledge
  display_name: "Knowledge"
  color: "#f59e0b"  # Amber
  description: "Knowledge atoms and external data"
  icon:
    web: "book-open"
    terminal: "◊"
  realms: [shared]
```

**Step 5: Write foundation.yaml**

```yaml
# packages/core/models/taxonomy/layers/foundation.yaml
layer:
  key: foundation
  display_name: "Foundation"
  color: "#10b981"  # Emerald
  description: "Projects, brands, and core identities"
  icon:
    web: "layers"
    terminal: "▣"
  realms: [org]
```

**Step 6: Write structure.yaml**

```yaml
# packages/core/models/taxonomy/layers/structure.yaml
layer:
  key: structure
  display_name: "Structure"
  color: "#06b6d4"  # Cyan
  description: "Pages, blocks, and content structure"
  icon:
    web: "layout"
    terminal: "□"
  realms: [org]
```

**Step 7: Write semantic.yaml**

```yaml
# packages/core/models/taxonomy/layers/semantic.yaml
layer:
  key: semantic
  display_name: "Semantic"
  color: "#14b8a6"  # Teal
  description: "Entities and semantic concepts"
  icon:
    web: "tag"
    terminal: "◇"
  realms: [org]
```

**Step 8: Write instruction.yaml**

```yaml
# packages/core/models/taxonomy/layers/instruction.yaml
layer:
  key: instruction
  display_name: "Instruction"
  color: "#f97316"  # Orange
  description: "Generation instructions and prompts"
  icon:
    web: "wand-2"
    terminal: "★"
  realms: [org]
```

**Step 9: Write output.yaml**

```yaml
# packages/core/models/taxonomy/layers/output.yaml
layer:
  key: output
  display_name: "Output"
  color: "#84cc16"  # Lime
  description: "Generated content and artifacts"
  icon:
    web: "file-output"
    terminal: "▲"
  realms: [org]
```

---

### Task 3.5: Create trait files (5)

**Files:**
- Create: `packages/core/models/taxonomy/traits/*.yaml` (5 files)

**Step 1: Write defined.yaml**

```yaml
# packages/core/models/taxonomy/traits/defined.yaml
trait:
  key: defined
  display_name: "Defined"
  color: "#3b82f6"  # Blue
  border_style: solid
  description: "Human-created once, structurally fixed"
  icon:
    web: "square"
    terminal: "■"
```

**Step 2: Write authored.yaml**

```yaml
# packages/core/models/taxonomy/traits/authored.yaml
trait:
  key: authored
  display_name: "Authored"
  color: "#a855f7"  # Purple
  border_style: dashed
  description: "Human-written per locale, editorial content"
  icon:
    web: "pen-tool"
    terminal: "□"
```

**Step 3: Write imported.yaml**

```yaml
# packages/core/models/taxonomy/traits/imported.yaml
trait:
  key: imported
  display_name: "Imported"
  color: "#eab308"  # Yellow
  border_style: double
  description: "External data brought in from corpora"
  icon:
    web: "download"
    terminal: "◊"
```

**Step 4: Write generated.yaml**

```yaml
# packages/core/models/taxonomy/traits/generated.yaml
trait:
  key: generated
  display_name: "Generated"
  color: "#22c55e"  # Green
  border_style: dotted
  description: "Produced by NovaNet LLM"
  icon:
    web: "sparkles"
    terminal: "✦"
```

**Step 5: Write retrieved.yaml**

```yaml
# packages/core/models/taxonomy/traits/retrieved.yaml
trait:
  key: retrieved
  display_name: "Retrieved"
  color: "#64748b"  # Slate
  border_style: dotted
  description: "Fetched from external APIs"
  icon:
    web: "cloud-download"
    terminal: "⋆"
```

---

### Task 3.6: Create arc-family files (5)

**Files:**
- Create: `packages/core/models/taxonomy/arc-families/*.yaml` (5 files)

**Step 1: Write ownership.yaml**

```yaml
# packages/core/models/taxonomy/arc-families/ownership.yaml
arc_family:
  key: ownership
  display_name: "Ownership"
  color: "#3b82f6"  # Blue
  stroke_style: solid
  description: "Parent-child ownership relationships"
  icon:
    web: "git-branch"
    terminal: "→"
```

**Step 2: Write localization.yaml**

```yaml
# packages/core/models/taxonomy/arc-families/localization.yaml
arc_family:
  key: localization
  display_name: "Localization"
  color: "#8b5cf6"  # Violet
  stroke_style: solid
  description: "Locale-related relationships"
  icon:
    web: "languages"
    terminal: "⇢"
```

**Step 3: Write semantic.yaml**

```yaml
# packages/core/models/taxonomy/arc-families/semantic.yaml
arc_family:
  key: semantic
  display_name: "Semantic"
  color: "#14b8a6"  # Teal
  stroke_style: dashed
  description: "Meaning and reference relationships"
  icon:
    web: "link"
    terminal: "⇾"
```

**Step 4: Write generation.yaml**

```yaml
# packages/core/models/taxonomy/arc-families/generation.yaml
arc_family:
  key: generation
  display_name: "Generation"
  color: "#f97316"  # Orange
  stroke_style: dotted
  description: "Content generation pipeline"
  icon:
    web: "wand-2"
    terminal: "⇝"
```

**Step 5: Write mining.yaml**

```yaml
# packages/core/models/taxonomy/arc-families/mining.yaml
arc_family:
  key: mining
  display_name: "Mining"
  color: "#84cc16"  # Lime
  stroke_style: dashed
  description: "SEO/GEO intelligence relationships"
  icon:
    web: "search"
    terminal: "⇀"
```

---

### Task 3.7: Update Rust parser to support taxonomy directory

**Files:**
- Modify: `tools/novanet/src/parsers/taxonomy.rs`

**Note:** This task requires updating the Rust taxonomy parser to:
1. Read from `taxonomy/` directory instead of single `taxonomy.yaml`
2. Load `_index.yaml` first
3. Parse each imported file
4. Merge into single TaxonomyDocument

**Implementation details will be determined during execution.**

---

### Task 3.8: Validate Batch 3 + Regenerate

**Step 1: Run schema validation**

```bash
cargo run -- schema validate
```

**Step 2: Regenerate artifacts**

```bash
cargo run -- schema generate
```

**Step 3: Run tests**

```bash
cargo test
```

**Step 4: Commit Batch 3**

```bash
git add -A
git commit -m "feat(schema): taxonomy explosion - split into 26 files

- Create taxonomy/ directory structure
- Create _index.yaml registry
- Create 2 realm files (shared, org)
- Create 9 layer files
- Create 5 trait files
- Create 5 arc-family files
- Update Rust parser for directory-based loading

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Batch 4: Update Documentation + ADRs

### Task 4.1: Update novanet-decisions.md

**Files:**
- Modify: `.claude/rules/novanet-decisions.md`

**Changes:**
- Update ADR-029 status from "Approved" to "Implemented"
- Update ADR-030 status from "Approved" to "Implemented"
- Add migration notes
- Update arc count in Quick Reference

---

### Task 4.2: Update novanet-terminology.md

**Files:**
- Modify: `.claude/rules/novanet-terminology.md`

**Changes:**
- Add *Native pattern terminology
- Update deprecated terms list
- Update node count references

---

### Task 4.3: Update CHANGELOG.md

**Files:**
- Modify: `CHANGELOG.md`

**Changes:**
- Add [Unreleased] section with all v0.13 changes
- Document breaking changes (node renames, arc merges)
- Document migration path

---

### Task 4.4: Commit Batch 4

```bash
git add -A
git commit -m "docs: update documentation for v0.13 schema changes

- Update ADR statuses in novanet-decisions.md
- Add *Native terminology to novanet-terminology.md
- Update CHANGELOG with v0.13 changes

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Batch 5: Update Rust Tests + Final Verification

### Task 5.1: Update arc count expectations

**Files:**
- Modify: `tools/novanet/src/generators/arc_class.rs`

**Changes:**
- Update arc count from 171 to new value (after merging HAS_CONTENT+HAS_GENERATED, CONTENT_OF+GENERATED_FOR)
- Expected new count: 171 - 2 = 169 arcs

---

### Task 5.2: Update node count expectations

**Files:**
- Modify: Any test files with node count assertions

**Changes:**
- Verify 61 nodes (unchanged count, just renamed)

---

### Task 5.3: Full test suite

```bash
cd tools/novanet
cargo test
```

Expected: All tests pass

---

### Task 5.4: TypeScript type-check

```bash
cd /Users/thibaut/supernovae-st/novanet-hq
pnpm type-check
```

Expected: No errors

---

### Task 5.5: Final commit

```bash
git add -A
git commit -m "test: update test expectations for v0.13 schema

- Update arc count: 171 → 169 (merged arcs)
- Verify all 1031+ tests pass

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Summary: Execution Order

| Batch | Tasks | Description | Est. Time |
|-------|-------|-------------|-----------|
| 1 | 1.1-1.7 | ADR-029 *Native Pattern | 30 min |
| 2 | 2.1-2.4 | ADR-030 Slug Ownership | 15 min |
| 3 | 3.1-3.8 | Taxonomy Explosion | 45 min |
| 4 | 4.1-4.4 | Documentation Updates | 20 min |
| 5 | 5.1-5.5 | Tests + Final Verification | 15 min |

**Total estimated time:** ~2 hours

---

## Verification Checklist

- [ ] All node renames complete (4 nodes)
- [ ] All arc merges complete (2 arcs deleted, 2 renamed)
- [ ] Slug properties migrated (EntityNative → PageNative)
- [ ] Taxonomy directory created (26 files)
- [ ] Schema validates: `cargo run -- schema validate` → 0 errors
- [ ] Schema generates: `cargo run -- schema generate` → 11 artifacts
- [ ] Rust tests pass: `cargo test` → 1000+ tests pass
- [ ] TypeScript compiles: `pnpm type-check` → no errors
- [ ] Documentation updated (ADRs, terminology, changelog)

---

## Related ADRs

| ADR | Status | Description |
|-----|--------|-------------|
| ADR-029 | TODO → Implemented | *Native Pattern |
| ADR-030 | TODO → Implemented | Slug Ownership |
| ADR-031 | ✅ Implemented | SEO Pillar/Cluster |
| ADR-032 | ✅ Implemented | URL Slugification |
