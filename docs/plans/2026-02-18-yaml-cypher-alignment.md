# YAML-Cypher Alignment Plan

**Date**: 2026-02-18
**Version**: v0.13.1
**Status**: In Progress
**Author**: Claude Opus 4.5

## Problem Statement

Explore agents identified 5 CRITICAL alignment issues between YAML definitions and Cypher seed files:

| Issue | YAML | Cypher | Severity |
|-------|------|--------|----------|
| TARGETS.rank vs priority | `priority` | `rank` | CRITICAL |
| TARGETS.is_slug_source | MISSING | Present | CRITICAL |
| SEOKeyword.locale_key | MISSING | Present | CRITICAL |
| SEOKeyword.slug_form | MISSING | Present | CRITICAL |
| BlockNative.content | `generated` | `content` | CRITICAL |

## Root Cause

ADR-030 (Slug Ownership) and ADR-032 (URL Slugification) introduced new properties that were implemented in Cypher but not reflected back to YAML definitions.

## Implementation Tasks

### Task 1: Fix TARGETS Arc (5 min)

**File**: `packages/core/models/arc-classes/semantic/targets.yaml`

**Changes**:
1. Rename `priority` → `rank` (align with Cypher)
2. Add `is_slug_source` property (boolean)
3. Add `created_at` property (standard arc timestamp)

**Before**:
```yaml
properties:
  - name: priority
    type: string
    required: false
    enum: [primary, secondary, tertiary]
    description: "Targeting priority for this keyword"
  - name: target_position
    type: int
    required: false
    description: "Target ranking position (1-10)"
```

**After**:
```yaml
properties:
  - name: rank
    type: string
    required: true
    enum: [primary, secondary, tertiary]
    description: "Targeting rank for this keyword (ADR-030)"
  - name: is_slug_source
    type: boolean
    required: false
    default: false
    description: "True if this keyword's slug_form was used for the URL slug (ADR-030)"
  - name: target_position
    type: int
    required: false
    description: "Target ranking position (1-10)"
  - name: created_at
    type: datetime
    required: false
    description: "Arc creation timestamp"
```

**Verification**:
```bash
cargo run -- schema validate --strict
grep -r "is_slug_source" packages/core/models/
```

---

### Task 2: Fix SEOKeyword Node (5 min)

**File**: `packages/core/models/node-classes/shared/knowledge/seo-keyword.yaml`

**Changes**:
1. Add `locale_key` to standard_properties (denormalized key)
2. Add `slug_form` to properties (ADR-032 slug derivation)
3. Add `source_date` to properties (data freshness tracking)
4. Update key pattern to composite format `seo:{slug}@{locale}`

**Add to standard_properties** (after `key`):
```yaml
locale_key:
  type: string
  required: true
  pattern: "^[a-z]{2}-[A-Z]{2}$"
  indexed: true
  description: "Locale key (denormalized for fast lookup). BCP 47 format."
  examples:
    - "fr-FR"
    - "en-US"
    - "ja-JP"
```

**Add to properties**:
```yaml
slug_form:
  type: string
  required: true
  description: "URL-safe slug form of the keyword. Used as input for slug derivation (ADR-032)."
  example: "créer-un-qr-code"

source_date:
  type: date
  required: false
  description: "Date when keyword data was fetched from source (e.g., Ahrefs)"
  example: "2026-02-17"
```

**Update key pattern**:
```yaml
key:
  type: string
  required: true
  pattern: "^seo:[a-z0-9-]+@[a-z]{2}-[A-Z]{2}$"
  description: "Composite key: seo:{slug}@{locale_key} (ADR-029)"
  examples:
    - "seo:qr-code-generator@en-US"
    - "seo:creer-un-qr-code@fr-FR"
```

**Verification**:
```bash
cargo run -- schema validate --strict
```

---

### Task 3: Fix BlockNative Node (3 min)

**File**: `packages/core/models/node-classes/org/output/block-native.yaml`

**Decision**: Rename `generated` → `content` in YAML (Cypher is authoritative for head-seo-meta pattern)

**Rationale**: The Cypher uses `content` which is more semantically correct — the property contains the actual content (slug, meta_title, etc.), not the "generated output" which implies LLM generation.

**Add to properties**:
```yaml
content:
  type: json
  required: true
  description: "Block content matching BlockType.structure. For head-seo-meta: {slug, meta_title, meta_description}"

block_type:
  type: string
  required: true
  description: "Reference to BlockType.key (denormalized for fast filtering)"
  example: "head-seo-meta"
```

**Remove**: `generated` property (replaced by `content`)

**Verification**:
```bash
grep -r '"generated":' packages/db/seed/*.cypher  # Should be 0
grep -r '"content":' packages/db/seed/*.cypher   # Should match BlockNative
```

---

### Task 4: Update DERIVED_SLUG_FROM Arc (2 min)

**File**: `packages/core/models/arc-classes/generation/derived-slug-from.yaml`

**Changes**: Add `derivation_timestamp` property (used in Cypher)

```yaml
- name: derivation_timestamp
  type: datetime
  required: false
  description: "When the slug derivation was computed"
```

---

### Task 5: Update Schema Validation Rules (5 min)

**File**: `tools/novanet/src/parsers/schema_rules.rs`

**Add validation rule**: Verify SEOKeyword has composite key pattern `seo:*@*`

```rust
// SEOKeyword must have composite key pattern
fn validate_seokeyword_key_pattern(node: &ParsedNode) -> Option<SchemaIssue> {
    if node.def.name != "SEOKeyword" {
        return None;
    }
    // Check key pattern exists and matches seo:{slug}@{locale}
    // ...
}
```

---

### Task 6: Run Full Test Suite (3 min)

```bash
cd tools/novanet
cargo test --lib
cargo run -- schema validate --strict
cargo run -- db seed  # Verify Cypher still works
```

## ADR References

- **ADR-029**: *Native Pattern (composite keys)
- **ADR-030**: Slug Ownership (is_slug_source on TARGETS)
- **ADR-032**: URL Slugification (slug_form property)

## Rollback Plan

If issues are found:
1. Revert YAML changes via git
2. Keep Cypher unchanged (it's working)
3. Document delta in ADR amendment

## Success Criteria

1. `cargo run -- schema validate --strict` passes
2. `cargo test --lib` passes (1139 tests)
3. All YAML properties match Cypher usage
4. No clippy warnings
