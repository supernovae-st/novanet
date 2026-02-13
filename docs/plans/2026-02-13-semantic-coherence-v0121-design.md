# NovaNet Semantic Coherence Plan v0.12.1

**Date**: 2026-02-13
**Status**: Draft
**Author**: Thibaut + Claude

## Executive Summary

Comprehensive audit and improvement plan for semantic coherence between NovaNet nodes and arcs, followed by propagation to all artifacts (YAML → Neo4j → Cypher → TUI → Studio).

**Current State**: 59 nodes, 115 arcs, 5 families
**Target**: Improved semantic clarity for LLM consumption, consistent patterns, complete documentation

---

## Research Findings

### 1. LLM-Optimized Naming Best Practices

From web research on Graph RAG, RDF/OWL, and Schema.org patterns:

| Principle | Description | NovaNet Alignment |
|-----------|-------------|-------------------|
| **Semantic Transparency** | Names should read as natural language predicates | ✅ Good (HAS_PAGE, USES_ENTITY) |
| **Directionality** | Explicit direction in verb form | ⚠️ Partial (38 HAS_* but only 5 inverses) |
| **Compositional Names** | Suffixes encode meaning (*Content, *Generated) | ✅ Excellent (ADR-014) |
| **Schema.org Alignment** | Match common ontology patterns | ✅ Good (hasPart → HAS_BLOCK) |

**Key Insight**: LLMs process relationships better when names are:
- Active verbs for forward direction (HAS_, USES_, ENABLES_)
- Passive/prepositional for inverse (*_OF, *_BY, BELONGS_TO_*)

### 2. llm_context Field Standard

**Recommended Template** (from Claude docs analysis):

```yaml
llm_context: |
  USE: [1-2 sentences: when to retrieve/use this node]
  TRIGGERS: [keywords, including multilingual: qr, code qr, QR-Code]
  NOT: [disambiguation with alternatives in parentheses]
  RELATES: [2-3 key connected nodes for context]
```

**Current Coverage**:
- 34% follow USE/TRIGGERS/NOT pattern
- 66% are free-form or incomplete
- Knowledge layer nodes especially weak

### 3. Arc Naming Convention Standard

| Pattern | Use Case | Example |
|---------|----------|---------|
| `HAS_*` | Ownership (parent→child) | HAS_PAGE, HAS_ENTITY |
| `*_OF` | Inverse ownership | BLOCK_OF, CONTENT_OF |
| `CONTAINS_*` | Container→Atom (no inverse) | CONTAINS_TERM |
| `USES_*` / `USED_BY` | Reference relationships | USES_ENTITY ↔ USED_BY |
| `*_TO` / `*_FOR` | Directional semantic | FOR_LOCALE, BELONGS_TO |
| `ENABLES` / `ENABLED_BY` | Dependency pairs | Symmetric naming |

---

## Issues Found (4 Agents)

### Critical (P0) - Block Release

| Issue | Location | Impact |
|-------|----------|--------|
| Broken inverse: `HAS_CHILD` refs non-existent `CHILD_OF` | has-child.yaml | Schema validation fails |
| Broken inverse: `HAS_INSTRUCTION` refs non-existent `INSTRUCTION_OF` | has-instruction.yaml | Schema validation fails |
| ProjectContent uses legacy relation format | project-content.yaml | Inconsistent parsing |

### High (P1) - Next Sprint

| Issue | Count | Impact |
|-------|-------|--------|
| Missing inverses for core ownership arcs | 23 arcs | Cannot traverse graph bidirectionally |
| llm_context not following pattern | 88 nodes | Poor LLM retrieval |
| PromptArtifact missing FOR_LOCALE | 1 node | Locale tracking broken |
| GEOAnswer/SEOKeywordMetrics have comment-only incoming arcs | 2 nodes | Generator can't validate |

### Medium (P2) - Polish

| Issue | Count | Impact |
|-------|-------|--------|
| Mixed relation declaration formats | 5 nodes | Developer confusion |
| Generation family arc semantics unclear | 4 arcs | GENERATED vs HAS_GENERATED ambiguous |
| Weak llm_context quality | 31 nodes | Suboptimal LLM context |

---

## Implementation Plan

### Phase 1: Fix Critical Issues (P0)

**1.1 Create Missing Inverse Arc Files**

```bash
# Create child-of.yaml
packages/core/models/arc-kinds/ownership/child-of.yaml

# Create instruction-of.yaml
packages/core/models/arc-kinds/ownership/instruction-of.yaml
```

**1.2 Migrate ProjectContent to Modern Format**

```yaml
# BEFORE (legacy)
relations:
  - relation: FOR_LOCALE
    to: Locale
    direction: outgoing

# AFTER (v11 format)
relations:
  outgoing:
    - type: FOR_LOCALE
      to: Locale
      cardinality: "N:1"
```

**1.3 Validate and Regenerate**

```bash
cargo run -- schema validate --strict
cargo run -- schema generate
```

### Phase 2: Improve Semantic Coherence (P1)

**2.1 Define Inverse Policy (ADR-026)**

Clarify which HAS_* arcs need explicit inverses:

| Tier | Arc | Inverse | Rationale |
|------|-----|---------|-----------|
| **TIER 1** (Required) | HAS_ENTITY | ENTITY_OF | "Which pages use this entity?" |
| **TIER 1** (Required) | HAS_PAGE | PAGE_OF | "Which project owns this page?" |
| **TIER 1** (Required) | HAS_PROJECT | PROJECT_OF | "Which org owns this project?" |
| **TIER 2** (Recommended) | HAS_TERMS | TERMS_OF | Locale ↔ TermSet traversal |
| **TIER 2** (Recommended) | HAS_EXPRESSIONS | EXPRESSIONS_OF | Knowledge atom traversal |
| **TIER 3** (Optional) | HAS_* (config) | - | Low traversal frequency |

**2.2 Standardize llm_context Fields**

Create template and apply to all 88 non-compliant nodes:

```yaml
# Template for Entity-like nodes
llm_context: |
  USE: when generating content about [core concept].
  TRIGGERS: [keyword1], [keyword2], [multilingual: fr, es, de].
  NOT: [similar concept] (use [Alternative]).
  RELATES: [Parent], [Child], [Sibling].

# Template for Container nodes (*Set)
llm_context: |
  USE: when loading [atom type] collection for a Locale.
  TRIGGERS: [domain] terms, [domain] vocabulary.
  NOT: individual [atoms] (traverse via CONTAINS_*).
  RELATES: Locale (owner), [Atom] (contents).

# Template for Generated nodes
llm_context: |
  USE: when retrieving LLM-generated [content type].
  TRIGGERS: generated [type], [locale] output.
  NOT: source content (use [Invariant]).
  RELATES: [Invariant] (source), Locale (target).
```

**2.3 Fix Specific Node Issues**

| Node | Fix |
|------|-----|
| PromptArtifact | Add FOR_LOCALE arc |
| GEOAnswer | Declare incoming HAS_GEO_ANSWERS explicitly |
| SEOKeywordMetrics | Declare incoming HAS_METRICS explicitly |
| OutputArtifact | Add CREATED_FROM inverse provenance |

### Phase 3: Document Generation Family (P2)

**3.1 Clarify GENERATED vs HAS_GENERATED**

```
                    ┌─────────────────────────────────────────────────────┐
                    │  GENERATION FAMILY ARC SEMANTICS                    │
                    ├─────────────────────────────────────────────────────┤
                    │                                                     │
                    │  HAS_GENERATED (ownership)                          │
                    │  Page ──[:HAS_GENERATED]──> PageGenerated           │
                    │  "Page OWNS its generated outputs"                  │
                    │                                                     │
                    │  GENERATED (provenance)                             │
                    │  PageInstruction ──[:GENERATED]──> PageGenerated    │
                    │  "Instruction PRODUCED this output"                 │
                    │                                                     │
                    │  GENERATED_FOR (inverse of HAS_GENERATED)           │
                    │  PageGenerated ──[:GENERATED_FOR]──> Page           │
                    │  "Output belongs to this Page"                      │
                    │                                                     │
                    │  GENERATED_FROM (optimization shortcut)             │
                    │  BlockGenerated ──[:GENERATED_FROM]──> BlockType    │
                    │  "Direct link to schema for validation"             │
                    │                                                     │
                    └─────────────────────────────────────────────────────┘
```

### Phase 4: Propagate to Artifacts

**4.1 Propagation Order**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SOURCE OF TRUTH PROPAGATION                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. YAML (source)                                                           │
│     packages/core/models/node-kinds/*.yaml                                  │
│     packages/core/models/arc-kinds/*.yaml                                   │
│     packages/core/models/taxonomy.yaml                                      │
│                                                                             │
│  2. Schema Generation                                                       │
│     cargo run -- schema generate                                            │
│     ├── packages/core/src/graph/*.generated.ts                              │
│     ├── packages/db/seed/*.cypher                                           │
│     └── tools/novanet/src/tui/icons.rs                                      │
│                                                                             │
│  3. Neo4j Database                                                          │
│     cargo run -- db seed                                                    │
│     ├── :Schema:Class nodes (from NodeClass)                                │
│     ├── :Schema:ArcClass nodes (from ArcClass)                              │
│     └── Constraints & indexes                                               │
│                                                                             │
│  4. TUI Runtime                                                             │
│     cargo run -- tui                                                        │
│     └── Loads from YAML at startup (Theme::with_root)                       │
│                                                                             │
│  5. Studio Frontend                                                         │
│     pnpm dev                                                                │
│     └── Imports from @novanet/core (generated TS)                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**4.2 Validation Commands**

```bash
# 1. Validate YAML coherence
cargo run -- schema validate --strict

# 2. Regenerate all artifacts
cargo run -- schema generate

# 3. Verify Rust compiles
cargo check

# 4. Run Rust tests
cargo test

# 5. Verify TypeScript compiles
pnpm type-check

# 6. Run TypeScript tests
pnpm test

# 7. Seed Neo4j
cargo run -- db seed

# 8. Verify TUI loads
cargo run -- tui

# 9. Verify Studio loads
pnpm dev
```

---

## Detailed Task Checklist

### Phase 1: Critical Fixes (Est: 2h)

- [ ] Create `arc-kinds/ownership/child-of.yaml`
- [ ] Create `arc-kinds/ownership/instruction-of.yaml`
- [ ] Migrate `project-content.yaml` to v11 relation format
- [ ] Run `cargo run -- schema validate`
- [ ] Commit: `fix(schema): create missing inverse arcs`

### Phase 2: Semantic Coherence (Est: 4h)

- [ ] Write ADR-026: Inverse Arc Policy
- [ ] Create 3 TIER 1 inverse arcs (ENTITY_OF, PAGE_OF, PROJECT_OF)
- [ ] Create llm_context template document
- [ ] Update 20 highest-priority llm_context fields
- [ ] Add FOR_LOCALE to PromptArtifact
- [ ] Add explicit incoming arcs to GEOAnswer, SEOKeywordMetrics
- [ ] Commit: `feat(schema): improve semantic coherence per ADR-026`

### Phase 3: Documentation (Est: 2h)

- [ ] Document generation family arc semantics
- [ ] Add inline comments to GEOAnswer/SEOKeywordMetrics explaining trait
- [ ] Create developer guide: "Arc Design Best Practices"
- [ ] Commit: `docs(schema): generation family semantics`

### Phase 4: Propagation (Est: 1h)

- [ ] Run full schema generation
- [ ] Seed Neo4j
- [ ] Verify TUI displays new arcs
- [ ] Verify Studio schema view
- [ ] Commit: `chore(schema): regenerate all artifacts for v0.12.1`

---

## Success Criteria

| Metric | Before | Target |
|--------|--------|--------|
| llm_context following pattern | 34% | 80%+ |
| Ownership arcs with inverses | 5 | 28 (TIER 1+2) |
| Schema validation warnings | Unknown | 0 |
| Broken arc references | 2 | 0 |
| Relation format consistency | 3 formats | 1 format |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Breaking existing queries | Run full test suite before/after |
| Neo4j migration complexity | Use `db reset` for clean slate |
| TUI crash on new arcs | Test with `cargo run -- tui` |
| Studio type errors | Run `pnpm type-check` |

---

## Timeline

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 1 (Critical) | 2h | None |
| Phase 2 (Coherence) | 4h | Phase 1 |
| Phase 3 (Docs) | 2h | Phase 2 |
| Phase 4 (Propagate) | 1h | Phase 3 |
| **Total** | **9h** | |

---

## Appendix A: llm_context Examples

### Good (Entity)
```yaml
llm_context: |
  USE: when generating content about QR codes, scanning, 2D barcodes.
  TRIGGERS: qr, qr code, qrcode, scan, code qr, matrix code.
  NOT: 1D barcode (use Barcode), shortened URL without QR (use ShortLink).
  RELATES: ShortLink (destination), Analytics (tracking), CustomQRCode (styled variant).
```

### Good (Arc: USES_ENTITY)
```yaml
llm_context: |
  Spreading activation link: Page/Block uses Entity for context.
  Traverse when loading generation context (max 2 hops).
  temperature: 0.0-1.0 (relevance to content).
  Cutoff: Skip links with temperature < 0.3.
  Example: Page:pricing --[:USES_ENTITY {temp: 0.9}]--> Entity:qr-code-generator
```

### Weak (needs improvement)
```yaml
# BEFORE
llm_context: "Container for GEO queries."

# AFTER
llm_context: |
  USE: when loading GEO monitoring queries for a Locale.
  TRIGGERS: geo queries, ai search, llm monitoring.
  NOT: individual GEOQuery atoms (traverse via CONTAINS_GEO_QUERY).
  RELATES: Locale (owner), GEOQuery (contents), GEOAnswer (results).
```

## Appendix B: Inverse Arc Naming Convention

| Forward Arc | Inverse Arc | Pattern |
|-------------|-------------|---------|
| HAS_PAGE | PAGE_OF | ownership |
| HAS_ENTITY | ENTITY_OF | ownership |
| HAS_BLOCK | BLOCK_OF | ownership |
| HAS_CONTENT | CONTENT_OF | localization |
| HAS_GENERATED | GENERATED_FOR | generation |
| USES_ENTITY | USED_BY | semantic |
| ENABLES | ENABLED_BY | semantic |
| REQUIRES | REQUIRED_BY | semantic |

## Appendix C: Files to Modify

### New Files (4)
- `arc-kinds/ownership/child-of.yaml`
- `arc-kinds/ownership/instruction-of.yaml`
- `arc-kinds/ownership/entity-of.yaml`
- `arc-kinds/ownership/page-of.yaml`

### Modified Files (Priority)
1. `node-kinds/org/foundation/project-content.yaml` (relation format)
2. `node-kinds/org/instruction/prompt-artifact.yaml` (add FOR_LOCALE)
3. `node-kinds/shared/knowledge/geo-answer.yaml` (explicit incoming)
4. `node-kinds/shared/knowledge/seo-keyword-metrics.yaml` (explicit incoming)
5. `arc-kinds/ownership/has-child.yaml` (fix inverse ref)
6. `arc-kinds/ownership/has-instruction.yaml` (fix inverse ref)

### llm_context Updates (Top 20)
1. GEOQuerySet
2. SEOKeywordSet
3. TermSet
4. ExpressionSet
5. PatternSet
6. CultureSet
7. TabooSet
8. AudienceSet
9. GEOQuery
10. GEOAnswer
11. SEOKeyword
12. SEOKeywordMetrics
13. CulturalRealm
14. CulturalSubRealm
15. LanguageFamily
16. LanguageBranch
17. PopulationCluster
18. PopulationSubCluster
19. PromptArtifact
20. OutputArtifact
