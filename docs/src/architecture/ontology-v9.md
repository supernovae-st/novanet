# Ontology Evolution

NovaNet's ontology has evolved through several major versions. This document traces the history and explains the current v0.12.0 structure.

## Version History

| Version | Date | Key Changes |
|---------|------|-------------|
| v9.0 | Jan 2026 | Initial Rust CLI, YAML-first architecture |
| v10.0 | Jan 2026 | Company project pattern, knowledge atoms |
| v10.6 | Feb 2026 | 2-realm architecture (global/tenant) |
| v11.0 | Feb 2026 | SEO layer in org realm |
| v11.2 | Feb 2026 | Realm renames (shared/org), trait split |
| v11.5 | Feb 2026 | 10 layers, SEO/GEO consolidation |
| v0.12.0 | Feb 2026 | Unified Tree, 2 modes (Graph/Nexus) |

## Ontology v9: Foundation

v9 established the core principles:

### YAML-First Architecture

```
YAML (source of truth) → Rust CLI → Generated artifacts
                                      ├── TypeScript types
                                      ├── Cypher seeds
                                      └── Mermaid docs
```

### Arc Terminology

Decision: Use "Arc" instead of "Edge" or "Relation" for directed links.

**Rationale**: Graph theory uses "arc" for directed edges. Avoids confusion with React Flow's "Edge" and Neo4j's "Relationship".

### Faceted Classification

Nodes and arcs are classified on multiple independent axes:

**Node axes**: Realm (WHERE), Layer (WHAT), Trait (HOW)
**Arc axes**: Scope, Family, Cardinality

## v10: Knowledge Atoms

v10 introduced granular knowledge nodes for selective LLM context loading.

### Before (JSON blobs)

```yaml
Locale:
  properties:
    terms: { type: object }  # 20K terms as JSON
```

### After (Knowledge atoms)

```
Locale ──[:HAS_TERMS]──> TermSet ──[:CONTAINS_TERM]──> Term
```

**Benefits**:
- Load 50 relevant Terms, not 20K JSON blob
- Graph queries: "Terms used by this Block"
- Atoms are locale-native, not translated

## v10.6: 2-Realm Architecture

Consolidated 3 realms into 2:

| Before | After |
|--------|-------|
| global | shared |
| organization | org |
| project | org |

**Rationale**: Organization + Project distinction added unnecessary complexity. Org is the natural isolation boundary.

## v11.2: Classification Refinement

### Realm Renames

```
global → shared (describes WHAT: shared resources)
tenant → org (describes WHO: organization-specific)
```

### Trait Split

```
derived → generated (LLM output: PageGenerated, BlockGenerated)
       → aggregated (computed metrics: GEOMetrics, SEOKeywordMetrics)
```

### Job Concept Removed

Removed 3 job nodes (GenerationJob, SEOMiningRun, EvaluationSignal). Deferred to v12+ when generation pipeline is more mature.

## v11.5: Schema Refinement

### Locale Moved to Config

```
Before: shared/locale/Locale.yaml
After:  shared/config/Locale.yaml
```

**Rationale**: Locale is a DEFINITION (invariant), not a parameter/setting.

### SEO/GEO Consolidation

```
Before: org/seo (5 nodes), org/geo (3 nodes)
After:  shared/knowledge (includes all SEO/GEO nodes)
```

**Rationale**: SEO/GEO data is universal market intelligence, not org-specific.

## v0.12.0: Unified Tree

### Principle

> "If it's a node in Neo4j, it's a node everywhere"

### Changes

| Aspect | Before (v11.6) | After (v0.12.0) |
|--------|----------------|-----------------|
| Nav modes | 5 (Meta/Data/Overlay/Query/Atlas) | 2 (Graph/Nexus) |
| Realm/Layer | Visual groupings | Clickable nodes |
| Instances | Hidden | Under Class, expandable |
| Icons | Mixed emoji | Dual: Lucide + Unicode |

### Unified Tree Structure

```
▼ Nodes (61)
  ▼ ◉ Realm:shared
    ▼ ⚙ Layer:config
      ▼ ◆ Class:Locale [200]    (v0.12.0: Kind→Class)
        ● Locale:fr-FR
```

## Current Architecture (v0.12.0)

### Statistics

| Metric | Count |
|--------|-------|
| Node types (Classes) | 61 |
| Arc types (ArcClasses) | 156 |
| Realms | 2 |
| Layers | 10 |
| Traits | 5 |

> **v0.12.0 ADR-023**: "Kind" → "Class", "ArcKind" → "ArcClass"

### Realm Distribution

| Realm | Layers | Nodes |
|-------|--------|-------|
| **shared** | config, locale, geography, knowledge | 39 |
| **org** | config, foundation, structure, semantic, instruction, output | 21 |

### Layer Breakdown

**Shared (39 nodes)**:
- config: 3 (EntityCategory, Locale, SEOKeywordFormat)
- locale: 6 (Culture, Style, Formatting, etc.)
- geography: 6 (Continent, Region, Country, etc.)
- knowledge: 24 (Terms, Expressions, SEO, GEO, etc.)

**Org (21 nodes)**:
- config: 1 (OrgConfig)
- foundation: 3 (Project, ProjectContent, BrandIdentity)
- structure: 3 (Page, Block, ContentSlot)
- semantic: 4 (Entity, EntityContent, etc.)
- instruction: 7 (PageType, BlockType, prompts)
- output: 3 (PageGenerated, BlockGenerated, OutputArtifact)

## ADR References

Key architecture decisions are documented in ADRs:

| ADR | Topic |
|-----|-------|
| ADR-003 | YAML-First Architecture |
| ADR-007 | Generation, NOT Translation |
| ADR-012 | 2-Realm Architecture |
| ADR-018 | Classification System Refinement |
| ADR-020 | Schema Refinement (v11.5) |
| ADR-022 | Unified Tree Architecture |

See `.claude/rules/novanet-decisions.md` for full ADR documentation.
