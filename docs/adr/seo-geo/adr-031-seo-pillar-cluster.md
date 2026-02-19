---
id: ADR-031
title: "SEO Pillar/Cluster Architecture"
version: v0.12.5
status: active
domain: seo-geo
---

# ADR-031: SEO Pillar/Cluster Architecture

**Status**: Approved (v0.12.5)

**Problem**: NovaNet lacked explicit SEO structure for pillar/cluster content strategy:
1. No way to mark pages as "pillars" (main topic hubs)
2. No arc to express "this cluster page belongs to this pillar"
3. No PageRank flow tracking for internal linking
4. SUBTOPIC_OF used for both URL hierarchy and SEO clustering

**Decision**: Introduce explicit pillar/cluster architecture with three distinct hierarchies.

## Three Hierarchies

```
1. Entity.SUBTOPIC_OF  = SEMANTIC hierarchy (topic clusters)
2. Page.SUBTOPIC_OF    = URL hierarchy (routing, navigation)
3. Page.SEO_CLUSTER_OF = SEO hierarchy (pillar/cluster strategy)

Often identical, but CAN differ!
```

**Example where they differ**:
```
Entity:faq-qr-instagram
    │ [:SUBTOPIC_OF] → Entity:qr-instagram (semantically about QR Instagram)

Page:faq-qr-instagram
    │ [:SUBTOPIC_OF] → Page:faq (URL: /faq/qr-instagram)
    │ [:SEO_CLUSTER_OF] → Page:qr-generator (SEO: cluster of QR pillar)
```

## New Properties

**Page (org/structure, defined)**:
```yaml
is_pillar: boolean              # True if this is a pillar page
pillar_strategy: string         # Description of pillar strategy (optional)
```

**Entity (org/semantic, defined)**:
```yaml
is_pillar: boolean              # True if this is a pillar entity
```

**Constraint**: `Page.is_pillar === Entity.is_pillar` (synchronized)

## New Arcs

**SEO_CLUSTER_OF**:
```yaml
arc:
  name: SEO_CLUSTER_OF
  family: semantic
  scope: intra_realm
  source: Page
  target: Page
  cardinality: many_to_one      # Many clusters → one pillar
  properties:
    cluster_role: enum          # supporting, comparison, tutorial, case_study, faq
    link_priority: integer      # Priority for internal linking (1=highest)
  llm_context: |
    USE: when determining SEO pillar/cluster relationships.
    TRIGGERS: pillar, cluster, topic hub, content strategy.
    NOT: URL hierarchy (use SUBTOPIC_OF), semantic meaning (use Entity.SUBTOPIC_OF), population clusters (use CLUSTER_OF).
    RELATES: Page (cluster), Page (pillar).
```

**LINKS_TO** (with PageRank properties):
```yaml
arc:
  name: LINKS_TO
  family: semantic
  scope: intra_realm
  source: Page
  target: Page
  cardinality: many_to_many
  properties:
    via_blocks: [string]        # Which blocks contain this link
    link_type: enum             # contextual | navigation | footer | pillar_backlink
    anchor_source: string       # @entity:X.title | keyword:X | custom
    pr_weight: float            # PageRank weight (0.0-1.0)
    is_reciprocal: boolean      # Does inverse link exist?
    nofollow: boolean           # For external links
  llm_context: |
    USE: when tracking internal links and PageRank flow.
    TRIGGERS: internal links, maillage, link juice, PageRank.
    NOT: URL hierarchy (use SUBTOPIC_OF), SEO clustering (use SEO_CLUSTER_OF).
    RELATES: Page (source), Page (target).
```

## PageRank Flow Rules

```
┌─────────────────────────────────────────────────────────────────────┐
│  PAGERANK DISTRIBUTION                                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│                        Home (PR: 100)                               │
│                            │                                        │
│            ┌───────────────┼───────────────┐                        │
│            ▼               ▼               ▼                        │
│       QR Generator    Templates       Pricing                       │
│       (PR: 35)        (PR: 30)        (PR: 25)                      │
│      [PILLAR]        [PILLAR]        [PAGE]                         │
│            │                                                        │
│   ┌────────┼────────┬────────┐                                      │
│   ▼        ▼        ▼        ▼                                      │
│ QR-Wifi QR-Insta QR-Menu  QR-PDF                                    │
│ (PR: 8) (PR: 12) (PR: 7)  (PR: 6)                                   │
│ [CLUSTER][CLUSTER][CLUSTER][CLUSTER]                                │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│  MAILLAGE RULES                                                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  R1: Pillar → Clusters (OBLIGATOIRE)                                │
│      └── Each pillar MUST link to its clusters                      │
│                                                                     │
│  R2: Cluster → Pillar (OBLIGATOIRE)                                 │
│      └── Each cluster MUST link back to its pillar                  │
│                                                                     │
│  R3: Cluster ↔ Cluster (RECOMMANDE)                                 │
│      └── Siblings CAN link to each other                            │
│                                                                     │
│  R4: Cross-Pillar (MODERE)                                          │
│      └── Only if semantically relevant                              │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Query Examples

```cypher
// Get all clusters for a pillar
MATCH (pillar:Page {key: $pillar_key, is_pillar: true})
MATCH (cluster:Page)-[:SEO_CLUSTER_OF]->(pillar)
RETURN cluster.key, cluster.slug
ORDER BY cluster.key

// Calculate PageRank flow
MATCH (source:Page)-[l:LINKS_TO]->(target:Page)
WHERE l.pr_weight > 0
RETURN source.key, target.key, l.pr_weight, l.link_type
ORDER BY l.pr_weight DESC

// Audit: clusters without pillar backlink
MATCH (cluster:Page)-[:SEO_CLUSTER_OF]->(pillar:Page)
WHERE NOT (cluster)-[:LINKS_TO {link_type: "pillar_backlink"}]->(pillar)
RETURN cluster.key AS missing_backlink
```

**Reference**: `docs/plans/2026-02-14-v0125-architecture-visual.md` (Session 5-6)
