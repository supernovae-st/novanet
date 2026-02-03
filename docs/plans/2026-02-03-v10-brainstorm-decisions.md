# V10 Brainstorm Decisions

**Date**: 2026-02-03
**Status**: Approved
**Participants**: Thibaut, Claude

---

## Overview

This document captures all decisions from the V10 ontology brainstorm session. These changes will be implemented incrementally from V9.6 to V10+, building on the V9.5 nomenclature refactor.

---

## Points Reviewed — Verdicts

### Point A — LocaleRules* Fragmentation

**VERDICT**: ✅ KEEP V9.5

Keep separate LocaleRulesAdaptation, LocaleRulesFormatting, LocaleRulesSlug, etc.

**Rationale**: Préserver l'essence de chaque locale > optimisation node count. No runtime inheritance to avoid losing locale-specific nuances.

### Point B — Generation Artifact Layer

**VERDICT**: ✅ ADD V9.6

Add GenerationJob, PromptArtifact, OutputArtifact, EvaluationSignal.

**Rationale**: "Iterate until perfect" workflow needs artifact tracking for generation pipeline.

### Point C — Intent/Audience/Channel

**VERDICT**: ✅ ADD V9.7

Add AudiencePersona, ContentIntent, ChannelSurface.

**Rationale**: Multi-persona/multi-channel content strategy support.

### Point D — SEO/GEO Generalization

**VERDICT**: ✅ KEEP SEPARATE + EXPAND V10+

Keep SEO (human discovery) separate from GEO (AI discovery). They have fundamentally different metrics.

**V10+ Additions**:
- seo layer: ASOKeyword, SocialHashtag, VoiceQuery
- geo layer: GEOEntity for Schema.org/Wikidata bridging

**Rationale**: SEO metrics (volume, difficulty, CTR) vs GEO metrics (citation rate, AI reference frequency) are fundamentally different.

### Point E — LocalizedVariant Pattern

**VERDICT**: ✅ HYBRID

Keep specific *L10n types + add shared `:Localized` double-label.

**Implementation**:
- taxonomy.yaml: Add shared_props to 'localized' trait
- Neo4j: Double-label (:ProjectL10n:Localized)
- TypeScript: LocalizedProps interface + specific extensions

**Rationale**: Type-safety preserved + generic queries via double-label.

### Point F — LocaleProfile Fusion

**VERDICT**: ✅ KEEP V9.5

Keep separate locale knowledge nodes (LocaleVoice, LocaleCulture, LocaleRules*, etc.).

**Rationale**: Granular RAG retrieval, independent versioning. 2800 nodes is nothing for Neo4j.

### Point G — ContentSlot Pattern

**VERDICT**: ✅ ADD V9.7+

Add ContentSlot for A/B testing + scheduling.

**New Nodes**:
- ContentSlot { position, slot_type, slot_key }

**New Arcs**:
- HAS_SLOT (Page → ContentSlot, 1:N)
- FILLED_BY (ContentSlot → Block, 1:N with weight/schedule props)

### Point H — TopicCluster / Entity

**VERDICT**: ✅ UNIFIED "Thing" MODEL

Replace Concept/Entity with unified Thing differentiated by realm.

**Key Decision**: Realm differentiates scope, not type name.

---

## Major Architectural Decisions

### 1. Unified "Thing" Model (Schema.org aligned)

```
OLD:  Concept (project) vs Entity (shared) — different names
NEW:  Thing (both) — differentiated by realm property

Thing (shared)  → Wikidata-linked, universal definition
Thing (project) → Your brand's specific definition
Thing (project) --SPECIALIZES--> Thing (shared)  // optional override
```

### 2. Realm Clarification

| Realm | Description | Override |
|-------|-------------|----------|
| **GLOBAL** | Locale infrastructure | NEVER — fr-FR is fr-FR everywhere |
| **SHARED** | Universal knowledge graph | CAN be specialized by project |
| **PROJECT** | Your brand's universe | Full control |

### 3. Schema.org Property Alignment

| Current | Schema.org | Decision |
|---------|------------|----------|
| display_name | name | **Rename** |
| wikidata_id | sameAs | **Rename to same_as (array)** |
| (new) | schema_type | **Add** |
| (new) | additional_types | **Add** |
| (new) | url | **Add** |
| (new) | image | **Add** |

### 4. Concept-Driven Internal Linking

```
Block --MENTIONS--> Thing (invariant)
Page --COVERS--> Thing (invariant)

Resolution at generation time:
- anchor = ThingL10n.name (or alias)
- url = PageL10n.slug
```

**Principle**: Structure defined 1×, content resolved 200×.

### 5. Invariant Structure Principle

All relationships defined at invariant level. Localized content resolved at generation time.

```
STRUCTURE = invariant (défini 1×)
CONTENU = localisé (résolu 200×)
```

---

## New Node Kinds

### V9.6 — Generation Domain

| NodeKind | Realm/Layer | Trait |
|----------|-------------|-------|
| GenerationJob | project/output | job |
| PromptArtifact | project/instruction | derived |
| OutputArtifact | project/output | derived |
| EvaluationSignal | project/output | derived |

### V9.7 — Intent Layer

| NodeKind | Realm/Layer | Trait |
|----------|-------------|-------|
| AudiencePersona | project/semantic | invariant |
| ContentIntent | project/semantic | invariant |
| ChannelSurface | project/semantic | invariant |

### V9.7 — Content Slot

| NodeKind | Realm/Layer | Trait |
|----------|-------------|-------|
| ContentSlot | project/structure | invariant |

### V9.8 — Topic Cluster

| NodeKind | Realm/Layer | Trait |
|----------|-------------|-------|
| TopicCluster | project/semantic | invariant |
| TopicClusterL10n | project/semantic | localized |

### V9.9 — Unified Thing Model

| NodeKind | Realm/Layer | Trait |
|----------|-------------|-------|
| Thing | shared/geo OR project/semantic | invariant |
| ThingL10n | (matches parent) | localized |

**Note**: Replaces both Concept and Entity.

---

## New Arc Kinds

### V9.6 — Generation Domain

| ArcKind | Source → Target | Family |
|---------|-----------------|--------|
| USES_PROMPT | GenerationJob → PromptArtifact | generation |
| PRODUCES | GenerationJob → OutputArtifact | generation |
| CREATES_CONTENT | OutputArtifact → BlockL10n | generation |
| HAS_EVALUATION | OutputArtifact → EvaluationSignal | generation |
| TRIGGERED_BY | GenerationJob → trigger source | generation |

### V9.7 — Intent Layer

| ArcKind | Source → Target | Family |
|---------|-----------------|--------|
| TARGETS_PERSONA | Page → AudiencePersona | semantic |
| HAS_INTENT | Page → ContentIntent | semantic |
| FOR_CHANNEL | Page → ChannelSurface | semantic |

### V9.7 — Content Slot

| ArcKind | Source → Target | Props |
|---------|-----------------|-------|
| HAS_SLOT | Page → ContentSlot | position |
| FILLED_BY | ContentSlot → Block | weight, valid_from, valid_until |

### V9.8 — Topic Cluster

| ArcKind | Source → Target | Props |
|---------|-----------------|-------|
| BELONGS_TO_CLUSTER | Page → TopicCluster | role (pillar/spoke), link_priority |

### V9.9 — Semantic Linking

| ArcKind | Source → Target | Props |
|---------|-----------------|-------|
| MENTIONS | Block → Thing | position, link_style, anchor_alias |
| COVERS | Page → Thing | is_primary, depth |
| SPECIALIZES | Thing:project → Thing:shared | override_level |
| RELATED_THING | Thing → Thing | relation_type |
| TARGETS_THING | SEOKeyword → Thing | - |

### Renames

| Old | New |
|-----|-----|
| USES_CONCEPT | USES_THING (or deprecate for MENTIONS) |

---

## Roadmap

### V9.5 (current) — Nomenclature Refactor

- [x] Node/NodeData/NodeKind vocabulary
- [x] Arc/ArcData/ArcKind vocabulary
- [x] Symmetric taxonomy
- [x] taxonomy.yaml, node-kinds/, arc-kinds/
- [ ] 5-phase implementation

### V9.6 — Generation Domain

- [ ] GenerationJob, PromptArtifact, OutputArtifact, EvaluationSignal
- [ ] 5 new ArcKinds for generation pipeline

### V9.7 — Intent Layer + ContentSlot

- [ ] AudiencePersona, ContentIntent, ChannelSurface
- [ ] ContentSlot with A/B testing + scheduling
- [ ] Hybrid :Localized double-label pattern

### V9.8 — TopicCluster + Internal Links

- [ ] TopicCluster (pillar/spoke SEO strategy)
- [ ] BELONGS_TO_CLUSTER arc
- [ ] Auto-generation of cluster links

### V9.9 — Unified Thing Model

- [ ] Rename Concept → Thing (project realm)
- [ ] Add Thing (shared realm) with Wikidata linking
- [ ] MENTIONS, COVERS, SPECIALIZES arcs
- [ ] Concept-driven internal linking

### V10+ — Future Expansions

- [ ] ASOKeyword, SocialHashtag, VoiceQuery (seo layer)
- [ ] ContentState (lifecycle tracking)
- [ ] ContentSegment (granular text units for RAG)
- [ ] ConceptMetrics (volatile metrics split)

---

## Atomic Scalability Patterns

1. **SPLIT BY LIFECYCLE** → ContentState node
2. **SPLIT BY GRANULARITY** → ContentSegment node
3. **SPLIT BY RESPONSIBILITY** → Separate concerns
4. **SPLIT BY VOLATILITY** → Metrics separate from core
5. **EXPLICIT OVER IMPLICIT** → ArcScope on arc
6. **COMPOSITION OVER INHERITANCE** → No LocaleRulesBase

---

## Core Principles

```
┌─────────────────────────────────────────────────────────────────────────┐
│  STRUCTURE = invariant (défini 1×)                                      │
│  CONTENU = localisé (résolu 200×)                                       │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│  GENERATION, NOT TRANSLATION                                            │
│  Concept (invariant) → Generate natively → ThingL10n (local)            │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│  REALM DIFFERENTIATES SCOPE                                             │
│  Same type name (Thing), different realm (shared vs project)            │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│  SCHEMA.ORG ALIGNMENT                                                   │
│  Direct mapping to JSON-LD for GEO optimization                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Summary Counts

| Category | Count |
|----------|-------|
| Points reviewed | 8 (A-H) |
| Keep V9.5 as-is | 3 (A, D partial, F) |
| Add to roadmap | 5 (B, C, E, G, H) |
| New NodeKinds | ~12 |
| New ArcKinds | ~15 |
| Renames | 2 |
