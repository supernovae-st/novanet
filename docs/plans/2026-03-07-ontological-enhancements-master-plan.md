# NovaNet v0.18.0 — Ontological Enhancements Master Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance NovaNet ontology with 5 structural improvements for better LLM context assembly and semantic reasoning.

**Architecture:** Add importance weights to arcs, cross-locale links, provenance on atoms, extended denomination forms, and structured constraints — all backward-compatible additions.

**Tech Stack:** YAML schema definitions, Rust MCP server, Neo4j graph database, TypeScript types

---

## Executive Summary

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  NOVANET v0.18.0 — ONTOLOGICAL ENHANCEMENTS                                  ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  5 PARALLEL WORKSTREAMS                                                       ║
║  ├── 🎯 Plan 1: Arc Importance Weights (HIGH)                                 ║
║  │   └── Add `importance: float` to all 136 ArcClasses                       ║
║  │                                                                            ║
║  ├── 🔗 Plan 2: Cross-Locale Semantic Links (MEDIUM)                         ║
║  │   └── New arc SEMANTICALLY_EQUIVALENT between EntityNatives               ║
║  │                                                                            ║
║  ├── 📊 Plan 3: Provenance & Confidence (HIGH)                               ║
║  │   └── Add provenance/confidence/last_verified to Knowledge Atoms          ║
║  │                                                                            ║
║  ├── 📝 Plan 4: Extended Denomination Forms (MEDIUM)                         ║
║  │   └── Add plural/possessive/phonetic/hashtag forms to ADR-033             ║
║  │                                                                            ║
║  └── 🔒 Plan 5: Structured Arc Constraints (HIGH)                            ║
║      └── Add machine-readable constraints field to ArcClasses                ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Plan 1: Arc Importance Weights

**ADR:** ADR-037 Arc Importance Weights
**Impact:** HIGH — Enables intelligent token budget allocation in novanet_generate
**Effort:** LOW — Single property addition to existing YAML files

### Rationale

Currently `novanet_generate` traverses arcs without semantic priority. A `HAS_NATIVE` arc is as important as a `SIMILAR_TO` arc, wasting token budget on low-value relationships.

### Specification

```yaml
# New property on all ArcClasses
arc:
  name: HAS_NATIVE
  importance: 1.0  # NEW: 0.0-1.0 scale
  # ... existing fields
```

### Importance Scale

| Range | Meaning | Examples |
|-------|---------|----------|
| 1.0 | Always include | HAS_NATIVE, HAS_BLOCK, HAS_PAGE |
| 0.8-0.9 | High priority | REFERENCES, HAS_TERMS, FOR_LOCALE |
| 0.5-0.7 | Medium priority | SEMANTIC_LINK, MONITORS_QUERY |
| 0.3-0.4 | Low priority | SIMILAR_TO, RELATED_TO |
| 0.1-0.2 | Contextual only | Historical arcs, debug arcs |

### Files to Modify

- 136 arc YAML files in `packages/core/models/arc-classes/`
- `tools/novanet-mcp/src/tools/generate.rs` — Use importance in traversal
- `packages/core/models/arc-schema.yaml` — Add importance property definition

---

## Plan 2: Cross-Locale Semantic Links

**ADR:** ADR-038 Cross-Locale Semantic Links
**Impact:** MEDIUM — Enables cross-locale features
**Effort:** MEDIUM — New arc type + migration

### Rationale

EntityNatives of the same Entity have no direct link. To find the French equivalent of an English EntityNative, you must traverse up to Entity then down again.

### Specification

```yaml
# New arc: SEMANTICALLY_EQUIVALENT
arc:
  name: SEMANTICALLY_EQUIVALENT
  family: semantic
  scope: intra_realm
  description: "Links EntityNatives of the same Entity across locales"
  source: EntityNative
  target: EntityNative
  cardinality: many_to_many
  is_self_referential: true
  importance: 0.6
  properties:
    - name: confidence
      type: float
      required: false
      description: "Semantic equivalence confidence (0.0-1.0)"
```

### Auto-Creation Rule

When creating EntityNative, automatically create SEMANTICALLY_EQUIVALENT arcs to all other EntityNatives of the same Entity.

---

## Plan 3: Provenance & Confidence on Knowledge Atoms

**ADR:** ADR-039 Knowledge Atom Provenance
**Impact:** HIGH — Quality-aware context assembly
**Effort:** MEDIUM — Add properties to 6 atom node types

### Rationale

Terms, Expressions, Patterns have no quality metadata. LLM cannot distinguish authoritative sources from user-generated content.

### Specification

```yaml
# New properties on Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait
node:
  name: Term
  properties:
    # ... existing
    - name: provenance
      type: string
      required: false
      description: "Source of this atom (e.g., 'rae.es', 'larousse.fr', 'manual')"
    - name: confidence
      type: float
      required: false
      description: "Quality confidence score (0.0-1.0)"
    - name: last_verified
      type: datetime
      required: false
      description: "Last verification date"
```

---

## Plan 4: Extended Denomination Forms

**ADR:** ADR-040 Extended Denomination Forms (amends ADR-033)
**Impact:** MEDIUM — Better SEO and content variety
**Effort:** LOW — Extend existing ADR-033 types

### Rationale

Current forms (text, title, abbrev, url, base, mixed) don't cover plural, possessive, or hashtag use cases.

### New Form Types

| Type | Usage | Example (en-US) | Example (fr-FR) |
|------|-------|-----------------|-----------------|
| plural | Multiple instances | "QR codes" | "codes QR" |
| possessive | Ownership | "QR code's" | "du code QR" |
| phonetic | Pronunciation | "/ˌkjuː ˈɑːr ˌkoʊd/" | "/ky ɛʁ kɔd/" |
| hashtag | Social media | "#QRCode" | "#CodeQR" |

### Schema Change

```yaml
# In ADR-033, extend valid types
denomination_forms:
  valid_types:
    - text
    - title
    - abbrev
    - url
    - base
    - mixed
    - plural      # NEW
    - possessive  # NEW
    - phonetic    # NEW
    - hashtag     # NEW
```

---

## Plan 5: Structured Arc Constraints

**ADR:** ADR-041 Structured Arc Constraints
**Impact:** HIGH — Foundation for automated ontology reasoning
**Effort:** MEDIUM — Add constraints field to all arcs

### Rationale

`llm_context` is human-readable but not machine-parseable. Automated tools cannot extract traversal rules.

### Specification

```yaml
# New constraints field on all ArcClasses
arc:
  name: HAS_NATIVE
  constraints:
    max_instances: null       # No limit
    requires_locale: true     # Must have locale context
    bidirectional: false      # One-way traversal
    priority_in_context: 1.0  # Same as importance
    required_properties: []   # Properties that must exist on target
    exclude_in_modes: []      # Modes where arc is ignored
```

### Use Cases

1. **novanet_generate**: Use constraints to validate traversal
2. **novanet_check**: Validate writes against constraints
3. **Schema introspection**: Machine-readable ontology rules

---

## Execution Order

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  PARALLEL EXECUTION (5 Opus Agents)                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Phase 1: ADR Creation (parallel)                                              │
│  ├── Agent 1 → ADR-037 Arc Importance Weights                                  │
│  ├── Agent 2 → ADR-038 Cross-Locale Links                                      │
│  ├── Agent 3 → ADR-039 Knowledge Atom Provenance                               │
│  ├── Agent 4 → ADR-040 Extended Denomination Forms                             │
│  └── Agent 5 → ADR-041 Structured Arc Constraints                              │
│                                                                                 │
│  Phase 2: YAML Schema Updates (parallel)                                       │
│  ├── Agent 1 → Add importance to 136 arc files                                 │
│  ├── Agent 2 → Create SEMANTICALLY_EQUIVALENT arc                              │
│  ├── Agent 3 → Add provenance to 6 atom node types                             │
│  ├── Agent 4 → Update ADR-033 with new form types                              │
│  └── Agent 5 → Add constraints to arc files                                    │
│                                                                                 │
│  Phase 3: Validation & Testing                                                 │
│  └── All agents → cargo run -- schema validate                                 │
│                                                                                 │
│  Phase 4: Commit & Release                                                     │
│  └── Single commit → v0.18.0 Ontological Enhancements                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Success Criteria

- [ ] 5 ADRs created and approved
- [ ] 136 arc files have `importance` property
- [ ] SEMANTICALLY_EQUIVALENT arc created
- [ ] 6 atom nodes have provenance/confidence/last_verified
- [ ] ADR-033 extended with 4 new form types
- [ ] All arcs have `constraints` field
- [ ] Schema validation: 0 errors
- [ ] All tests pass

---

## Version Impact

| Component | Before | After |
|-----------|--------|-------|
| NovaNet | v0.17.3 | v0.18.0 |
| ArcClasses | 136 | 137 (+SEMANTICALLY_EQUIVALENT) |
| Node properties | Current | +3 on atoms, +1 on arcs |
| ADRs | 36 | 41 (+5 new) |
