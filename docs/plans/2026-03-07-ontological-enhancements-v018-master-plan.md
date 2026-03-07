# NovaNet v0.18.0 Ontological Enhancements — Master Plan

> **Status:** VERIFIED & COMPLETE
> **Date:** 2026-03-07
> **CSR:** 99.99% (8 warnings = entities without EntityNatives)

---

## Executive Summary

```
+===============================================================================+
|  NOVANET v0.18.0 ONTOLOGICAL ENHANCEMENTS                                     |
+===============================================================================+
|                                                                               |
|  5 IMPROVEMENTS IMPLEMENTED:                                                  |
|                                                                               |
|  [x] 1. Arc Importance Weights    140/140 arcs   importance: float (0.0-1.0) |
|  [x] 2. SEMANTICALLY_EQUIVALENT   ADR-038        Cross-locale EntityNative   |
|  [x] 3. Atom Provenance           ADR-039        5 atom types enhanced       |
|  [x] 4. Extended Denomination     ADR-040        +4 forms (plural,possessive)|
|  [x] 5. Structured Constraints    ADR-041        140/140 arcs have block     |
|                                                                               |
|  POST-AUDIT FIXES:                                                            |
|  [x] 52 inverse arcs with bidirectional declarations                         |
|  [x] 2 files with duplicate constraints blocks consolidated                   |
|  [x] 106 arcs with proper inverse: field                                     |
|                                                                               |
+===============================================================================+
```

---

## 1. Arc Importance Weights

**Status:** COMPLETE (140/140 arcs)

**ADR:** Pre-existing (part of arc schema)

**Purpose:** Enable `novanet_generate` to prioritize arcs by importance for token budget allocation.

**Implementation:**
```yaml
# Example: has-native.yaml
arc:
  name: HAS_NATIVE
  importance: 1.0  # Critical arc for content loading
```

**Importance Scale:**
| Range | Usage |
|-------|-------|
| 1.0 | Critical (HAS_NATIVE, HAS_BLOCK, FOR_LOCALE) |
| 0.8-0.9 | High (HAS_ENTITY, HAS_PAGE, inverse arcs) |
| 0.5-0.7 | Medium (semantic links, references) |
| 0.3-0.4 | Low (mining, schema arcs) |

**Verification:**
```bash
grep -r "importance:" schema/models/arc-classes/ | wc -l
# Result: 140 files
```

---

## 2. SEMANTICALLY_EQUIVALENT Arc (ADR-038)

**Status:** COMPLETE

**ADR:** `/dx/adr/novanet/schema-architecture/adr-038-cross-locale-semantic-links.md`

**Purpose:** Link EntityNatives of the same Entity across different locales for O(1) cross-locale lookup.

**File:** `arc-classes/semantic/semantically-equivalent.yaml`

**Schema:**
```yaml
arc:
  name: SEMANTICALLY_EQUIVALENT
  family: semantic
  scope: intra_realm
  source: EntityNative
  target: EntityNative
  cardinality: many_to_many
  is_self_referential: true
  importance: 0.6
  properties:
    - name: confidence
      type: float
      description: "Semantic equivalence confidence (0.0-1.0)"
    - name: auto_created
      type: boolean
      description: "True if automatically created"
```

**Use Case:**
```cypher
// Find all locale variants of an entity
MATCH (en:EntityNative {entity_key: 'qr-code'})-[:SEMANTICALLY_EQUIVALENT]-(other:EntityNative)
RETURN other.locale_key, other.display_name
```

---

## 3. Atom Provenance (ADR-039)

**Status:** COMPLETE (5/5 atom types)

**ADR:** `/dx/adr/novanet/schema-architecture/adr-039-knowledge-atom-provenance.md`

**Purpose:** Track source, quality, and verification date for knowledge atoms.

**Atom Types Updated:**
1. `Expression` — Idiomatic phrases
2. `Pattern` — Content templates
3. `CultureRef` — Cultural references
4. `Taboo` — Cultural taboos
5. `AudienceTrait` — Audience characteristics

**Properties Added:**
```yaml
# v0.18.0: ADR-039 Provenance & Confidence
provenance:
  type: string
  required: false
  description: "Source of this knowledge atom (e.g., 'rae.es', 'larousse.fr')"

confidence:
  type: float
  required: false
  description: "Quality confidence score (0.0-1.0)"

last_verified:
  type: string
  required: false
  description: "ISO 8601 date of last verification"
```

---

## 4. Extended Denomination Forms (ADR-040)

**Status:** COMPLETE

**ADR:** `/dx/adr/novanet/schema-architecture/adr-040-extended-denomination-forms.md`

**Purpose:** Add additional denomination forms for comprehensive entity naming.

**File:** `node-classes/org/semantic/entity-native.yaml`

**Forms (10 total):**
| Form | Priority | Usage |
|------|----------|-------|
| `text` | 1 | Prose, body content |
| `title` | 1 | H1, H2, meta_title |
| `abbrev` | 1 | After first mention |
| `mixed` | 2 | Native script + tech hybrid (zh-CN: "QR码") |
| `base` | 3 | International reference (zh-CN: "QR Code") |
| `url` | 1 | URL-safe slug (POST-SEO pipeline) |
| `plural` | 5 | Plural form ("QR Codes") |
| `possessive` | 6 | Possessive form ("QR Code's") |
| `phonetic` | 7 | Pronunciation guide |
| `hashtag` | 8 | Social media hashtag (#QRCode) |

**Schema:**
```yaml
denomination_forms:
  type: array
  items:
    type: object
    properties:
      type:
        type: string
        enum: [text, title, abbrev, mixed, base, url, plural, possessive, phonetic, hashtag]
      value:
        type: string
      priority:
        type: integer
        default: 1
```

---

## 5. Structured Arc Constraints (ADR-041)

**Status:** COMPLETE (140/140 arcs)

**ADR:** `/dx/adr/novanet/arc-design/adr-041-structured-arc-constraints.md`

**Purpose:** Machine-readable constraints block for runtime validation.

**Constraints Block:**
```yaml
constraints:
  max_instances: null        # Max arcs from single source (null = unlimited)
  requires_locale: false     # Source/target must share locale
  bidirectional: false       # Auto-create inverse arc
  priority_in_context: 1.0   # Token budget priority (0.0-1.0)
  required_target_properties: []  # Target must have these props
  target_must_exist: false   # Target node must exist before arc
  unique_source: false       # Only one source can point to target
  exclude_in_modes: []       # Hide in specific UI modes
```

**Verification:**
```bash
grep -r "constraints:" schema/models/arc-classes/ | wc -l
# Result: 140 files
```

---

## Verification Results

### Schema Validation
```bash
cd tools/novanet && cargo run -- schema validate
# Result: 0 error(s), 0 warning(s)
```

### CSR Audit
```json
{
  "csr": {
    "rate": 0.9999380637169512,
    "satisfied_count": 129157,
    "violated_count": 8
  },
  "summary": {
    "total_issues": 8,
    "critical_count": 0,
    "warning_count": 8
  }
}
```

### 8 Warnings (Content Gaps)
Entities without EntityNatives:
1. `custom-qr-code`
2. `qr-code-art`
3. `dynamic-qr-code`
4. `static-qr-code`
5. `smart-link`
6. `landing-page`
7. `barcode`
8. `qr-code-generator`

**Recommendation:** Run `00-entity-native-bootstrap` Nika workflow for these entities.

---

## File Inventory

### ADRs Created
| ADR | Title | Path |
|-----|-------|------|
| 038 | Cross-Locale Semantic Links | `dx/adr/novanet/schema-architecture/adr-038-cross-locale-semantic-links.md` |
| 039 | Knowledge Atom Provenance | `dx/adr/novanet/schema-architecture/adr-039-knowledge-atom-provenance.md` |
| 040 | Extended Denomination Forms | `dx/adr/novanet/schema-architecture/adr-040-extended-denomination-forms.md` |
| 041 | Structured Arc Constraints | `dx/adr/novanet/arc-design/adr-041-structured-arc-constraints.md` |

### Arc Files Modified (140)
All files in `schema/models/arc-classes/` now have:
- `importance: float`
- `constraints:` block

### Atom Files Modified (5)
- `schema/models/node-classes/shared/knowledge/atoms/expression.yaml`
- `schema/models/node-classes/shared/knowledge/atoms/pattern.yaml`
- `schema/models/node-classes/shared/knowledge/atoms/culture-ref.yaml`
- `schema/models/node-classes/shared/knowledge/atoms/taboo.yaml`
- `schema/models/node-classes/shared/knowledge/atoms/audience-trait.yaml`

### New Arc Created (1)
- `schema/models/arc-classes/semantic/semantically-equivalent.yaml`

### Inverse Arc Declarations (106)
All arcs with inverse relationships now have bidirectional `inverse:` field.

---

## Architecture Diagram

```
+===============================================================================+
|  ENTITY NATIVE ARCHITECTURE (v0.18.0)                                         |
+===============================================================================+
|                                                                               |
|  Entity (defined)                                                             |
|    │                                                                          |
|    ├──[:HAS_NATIVE]──► EntityNative (generated)                              |
|    │   importance: 1.0       │                                                |
|    │                         ├──[:FOR_LOCALE]──► Locale                      |
|    │                         │                                                |
|    │                         ├──[:SEMANTICALLY_EQUIVALENT]──► EntityNative   |
|    │                         │   (ADR-038)        (other locale)             |
|    │                         │                                                |
|    │                         ├── denomination_forms: (ADR-040)               |
|    │                         │   text, title, abbrev, url,                   |
|    │                         │   plural, possessive, phonetic, hashtag       |
|    │                         │                                                |
|    │                         └──[:USES_TERM]──► Term                         |
|    │                                            │                             |
|    │                                            └── provenance (ADR-039)     |
|    │                                                confidence               |
|    │                                                last_verified            |
|    │                                                                          |
|    └──[:BELONGS_TO]──► EntityCategory                                        |
|        constraints:                                                           |
|          max_instances: null                                                  |
|          bidirectional: true                                                  |
|          priority_in_context: 0.6                                            |
|                                                                               |
+===============================================================================+
```

---

## Next Steps

1. **Generate Missing EntityNatives**
   ```bash
   # Run Nika workflow for 8 missing entities
   nika run 00-entity-native-bootstrap --entities=custom-qr-code,qr-code-art,...
   ```

2. **Create SEMANTICALLY_EQUIVALENT arcs**
   ```cypher
   // Auto-create links between EntityNatives of same Entity
   MATCH (e:Entity)-[:HAS_NATIVE]->(en1:EntityNative)
   MATCH (e)-[:HAS_NATIVE]->(en2:EntityNative)
   WHERE en1.locale_key <> en2.locale_key
   MERGE (en1)-[:SEMANTICALLY_EQUIVALENT {auto_created: true, confidence: 1.0}]->(en2)
   ```

3. **Populate atom provenance**
   ```cypher
   // Add provenance to existing atoms
   MATCH (expr:Expression)
   WHERE expr.provenance IS NULL
   SET expr.provenance = 'manual', expr.confidence = 0.8
   ```

---

## Conclusion

NovaNet v0.18.0 ontological enhancements are **COMPLETE** and **VERIFIED**:

- Schema validation: 0 errors, 0 warnings
- CSR: 99.99% (8 content gap warnings)
- All 5 improvements implemented
- 4 new ADRs documented
- 140 arc files enhanced
- 106 inverse declarations complete

The ontology is ready for production use.
