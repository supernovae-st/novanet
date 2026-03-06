# Ontology Final Cleanup — D6/D7/D8 + Chantiers 1-2

**Date**: 2026-03-06
**Status**: In Progress
**Goal**: Complete ontology cleanup, reseed DB, achieve CSR 100%

---

## Executive Summary

| Task | Description | Est. Files |
|------|-------------|------------|
| D6 | Consolidate semantic arcs (26 → 12 essential) | ~14 arcs to archive |
| D7 | Create inverse arcs for consolidated arcs | ~6 new arcs |
| D8 | Add workflow_id to all generated/imported nodes | ~20 node YAMLs |
| Chantier 1 | DB Cleanup for CSR 100% | Migrations |
| Chantier 2 | EntityNatives for de-DE, ja-JP, etc. | Seed data |

---

## Phase 1: D6 — Consolidate Semantic Arcs

### Current State (26 active)
```
belongs-to          curates-keyword     monitors-query
compares-a          compares-b          fills-slot
includes            inspired-by-region  links-to
mentions            popular-in          referenced-by
references          references-entity   references-page
represented-by      represents          semantic-link
seo-cluster-of      subtopic-of         use-case-for
uses-culture-ref    uses-entity         uses-expression
uses-pattern        uses-term
```

### Target State (12 essential + 6 newly created)

**KEEP (12):**
| Arc | Reason |
|-----|--------|
| `BELONGS_TO` | EntityCategory classification |
| `SUBTOPIC_OF` | Pillar/cluster SEO hierarchy |
| `REPRESENTS` | Page ↔ Entity 1:1 |
| `REPRESENTED_BY` | Inverse of REPRESENTS |
| `POPULAR_IN` | Geographic popularity |
| `SEMANTIC_LINK` | Typed semantic relations (type_of, variant_of, etc.) |
| `USES_TERM` | Knowledge atom usage |
| `USES_EXPRESSION` | Knowledge atom usage |
| `USES_PATTERN` | Knowledge atom usage |
| `USES_CULTURE_REF` | Knowledge atom usage |
| `CURATES_KEYWORD` | SEO (just created) |
| `MONITORS_QUERY` | GEO (just created) |

**ARCHIVE (14):**
| Arc | Reason | Replacement |
|-----|--------|-------------|
| `compares-a` | Specialized, rarely used | SEMANTIC_LINK type=compares |
| `compares-b` | Specialized, rarely used | SEMANTIC_LINK type=compares |
| `fills-slot` | Block-specific, can use props | Block.slot_type property |
| `includes` | Redundant with SEMANTIC_LINK | SEMANTIC_LINK type=includes |
| `inspired-by-region` | Specialized | POPULAR_IN with props |
| `links-to` | Generic, redundant | REFERENCES |
| `mentions` | Consolidate | REFERENCES |
| `referenced-by` | Keep REFERENCES only | REFERENCES inverse via query |
| `references-entity` | Consolidate | REFERENCES |
| `references-page` | Consolidate | REFERENCES |
| `seo-cluster-of` | Use SUBTOPIC_OF | SUBTOPIC_OF with is_seo_cluster |
| `use-case-for` | Specialized | SEMANTIC_LINK type=use_case |
| `uses-entity` | Redundant | REFERENCES |

**NEW (D7 - Inverse arcs):**
| Arc | Inverse Of | Sources → Targets |
|-----|------------|-------------------|
| `TERM_USED_BY` | USES_TERM | Term → EntityNative/BlockNative |
| `EXPRESSION_USED_BY` | USES_EXPRESSION | Expression → EntityNative/BlockNative |
| `PATTERN_USED_BY` | USES_PATTERN | Pattern → EntityNative/BlockNative |
| `CULTURE_REF_USED_BY` | USES_CULTURE_REF | CultureRef → EntityNative/BlockNative |
| `HAS_SUBTOPIC` | SUBTOPIC_OF | Entity → Entity |
| `CATEGORY_OF` | BELONGS_TO | EntityCategory → Entity |

---

## Phase 2: D7 — Create Inverse Arcs

Create 6 new arc YAMLs in `arc-classes/semantic/`:
1. `term-used-by.yaml`
2. `expression-used-by.yaml`
3. `pattern-used-by.yaml`
4. `culture-ref-used-by.yaml`
5. `has-subtopic.yaml`
6. `category-of.yaml`

---

## Phase 3: D8 — Universal workflow_id

### Nodes that need workflow_id added

**Generated trait (MUST have workflow_id):**
- [x] EntityNative ✓
- [ ] PageNative
- [ ] BlockNative

**Imported trait (SHOULD have workflow_id):**
- [ ] Term
- [ ] Expression
- [ ] Pattern
- [ ] CultureRef
- [ ] Taboo
- [ ] AudienceTrait
- [ ] SEOKeyword
- [ ] GEOQuery

**Retrieved trait (SHOULD have workflow_id):**
- [ ] SEOKeywordMetrics
- [ ] GEOAnswer

**Defined trait (OPTIONAL workflow_id - can be created via TUI):**
- [x] Entity ✓
- [ ] Page
- [ ] Block
- [ ] Project
- [ ] Brand

---

## Phase 4: Chantier 1 — DB Cleanup

Based on novanet_audit results, create migrations for:
1. Orphaned EntityNatives → Create parent Entities
2. Orphaned PageNatives → Create parent Pages
3. Orphaned BlockNatives → Create parent Blocks
4. SEOKeywords without FOR_LOCALE → Create arcs

---

## Phase 5: Chantier 2 — SEO Reality Audit Implementation

Create EntityNatives with correct denomination_forms for:
- de-DE: "QR-Code" (with hyphen)
- ja-JP: "QRコード" (katakana)
- it-IT: "codice QR" or "QR code"
- pt-BR: "código QR"
- pt-PT: "código QR"
- es-ES: "código QR"

---

## Execution Order

```
1. D6: Archive 14 semantic arcs
2. D7: Create 6 inverse arcs
3. D8: Add workflow_id to ~15 nodes
4. Schema generate + validate
5. Chantier 1: Create DB cleanup migrations
6. Chantier 2: Create EntityNative seed data
7. Reseed database
8. Verify alignment (schema validate, novanet_audit)
9. Run Ralph Wiggum audit
10. Report back
```

---

## Verification Checklist

- [ ] `cargo run -- schema validate` passes
- [ ] `cargo test` passes (1279+ tests)
- [ ] `novanet_audit target=all` shows CSR ≥ 99.5%
- [ ] No orphaned nodes
- [ ] All generated nodes have workflow_id
- [ ] Schema ↔ Neo4j alignment verified
