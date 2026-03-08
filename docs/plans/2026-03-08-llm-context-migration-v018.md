# llm_context Migration Plan — v0.17.3 → v0.18.0

**Date:** 2026-03-08
**Status:** In Progress
**Author:** Claude + Thibaut

---

## Overview

Make `llm_context` a **required standard property** on Entity and EntityNative nodes, enforcing the distinction between `description` (WHAT) and `llm_context` (HOW).

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  THE DISTINCTION                                                              ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  description  = WHAT it IS (factual definition, 1-3 sentences)               ║
║  llm_context  = HOW to USE it (operational guidance, ADR-027 pattern)         ║
║                                                                               ║
║  Example for "QR Code":                                                       ║
║  ───────────────────────────────────────────────────────────────────────────  ║
║  description:  "Two-dimensional barcode that encodes data in a scannable      ║
║                 visual pattern."                                              ║
║                                                                               ║
║  llm_context:  "USE: when discussing 2D barcodes, mobile scanning,            ║
║                 contactless data transfer. TRIGGERS: qr, qr code, scan,       ║
║                 mobile. NOT: for 1D barcodes (use Barcode). RELATES:          ║
║                 Create QR Code (action), QR Code Types (variants)."           ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Decisions Made (Brainstorm Session)

| Topic | Decision | Rationale |
|-------|----------|-----------|
| **llm_context location** | standard_properties (required) | Must be on every instance, not optional |
| **description vs llm_context** | WHAT vs HOW distinction | description = factual, llm_context = operational |
| **EntityNative.definition** | required | Core semantic content |
| **EntityNative.purpose** | required | Why this entity exists |
| **tone_guidance** | NOT NEEDED | Noise - LocaleVoice handles this |
| **target_audience** | NOT NEEDED | Noise - audience_segment on Entity is enough |
| **seo_focus** | NOT NEEDED | Noise - SEOKeyword handles this |
| **"entities fortes"** | is_pillar + graph weight | Human decision + runtime calculation |

---

## Schema Changes

### 1. Entity.yaml

**Before:**
```yaml
# BLOC 2 (class-level llm_context - unchanged)
llm_context: |
  USE: when loading semantic context...

# properties (optional)
properties:
  llm_context:
    type: string
    required: false
    description: "Instance-specific LLM context..."
```

**After:**
```yaml
# BLOC 2 (class-level llm_context - unchanged)
llm_context: |
  USE: when loading semantic context...

# standard_properties (required)
standard_properties:
  llm_context:
    type: string
    required: true
    description: "HOW to use this entity in content generation (ADR-027 USE/TRIGGERS/NOT/RELATES pattern)"
```

**Impact:** Move `llm_context` from properties to standard_properties, make required.

---

### 2. EntityNative.yaml

**Before:**
```yaml
# BLOC 2 - NO llm_context at class level
# properties
properties:
  definition:
    type: string
    required: false   # ← Change to true
  purpose:
    type: string
    required: false   # ← Change to true
```

**After:**
```yaml
# standard_properties (required)
standard_properties:
  llm_context:
    type: string
    required: true
    description: "HOW to use this localized entity in content generation (ADR-027 USE/TRIGGERS/NOT/RELATES pattern)"

# properties
properties:
  definition:
    type: string
    required: true   # ← Now required
  purpose:
    type: string
    required: true   # ← Now required
```

**Impact:** Add `llm_context` to standard_properties (required), make definition/purpose required.

---

## Implementation Tasks

### Task 1: Update entity.yaml

- [ ] Remove `llm_context` from `properties` section
- [ ] Add `llm_context` to `standard_properties` section with required: true
- [ ] Update description to emphasize HOW vs WHAT

### Task 2: Update entity-native.yaml

- [ ] Add `llm_context` to `standard_properties` section with required: true
- [ ] Add class-level `llm_context` in BLOC 2 (for schema consistency)
- [ ] Change `definition` from required: false to required: true
- [ ] Change `purpose` from required: false to required: true

### Task 3: Validate Schema

- [ ] Run `cargo run -- schema validate`
- [ ] Verify no breaking changes to other nodes

### Task 4: Update Documentation

- [ ] Update CHANGELOG-LATEST.md with v0.18.0 entry
- [ ] Update version comments in YAML files

---

## Version Bump

```
v0.17.3 → v0.18.0 (MINOR: new required properties)
```

**Rationale:** Adding required properties is a schema change that affects data validation.

---

## Migration Notes

When adding entities to Neo4j, all Entity and EntityNative nodes MUST have:

| Property | Example |
|----------|---------|
| `description` | "Two-dimensional barcode that encodes data in a scannable visual pattern." |
| `llm_context` | "USE: when discussing 2D barcodes, mobile scanning. TRIGGERS: qr, scan, mobile." |

The Nika workflow `00-entity-native-bootstrap.nika.yaml` will need to generate `llm_context` for each EntityNative.

---

## Validation

After implementation:

```bash
# Validate schema
cargo run -- schema validate

# Regenerate artifacts
cargo run -- schema generate

# Check TypeScript types
pnpm type-check
```
