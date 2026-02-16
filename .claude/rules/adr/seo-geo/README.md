# SEO/GEO

SEO pillar/cluster architecture and URL slugification.

## ADRs in this Domain

| ADR | Name | Status | Summary |
|-----|------|--------|---------|
| [031](adr-031-seo-pillar-cluster.md) | SEO Pillar/Cluster Architecture | active | is_pillar, SEO_CLUSTER_OF, LINKS_TO |
| [032](adr-032-url-slugification.md) | URL Slugification Architecture | active | Derivation algorithm, no-repetition rule |

## Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  THREE HIERARCHIES (ADR-031)                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. Entity.SUBTOPIC_OF   = SEMANTIC (topic clusters)            │
│  2. Page.SUBTOPIC_OF     = URL (routing, navigation)            │
│  3. Page.SEO_CLUSTER_OF  = SEO (pillar/cluster strategy)        │
│                                                                 │
│  Often identical, but CAN differ!                               │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  NO-REPETITION RULE (ADR-032)                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ❌ /créer-qr-code/qr-code-pour-instagram  (répétition!)        │
│  ✅ /créer-qr-code/instagram               (différenciateur)   │
│                                                                 │
│  Algorithm: new_terms = keyword_terms - parent_terms            │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  MAILLAGE RULES                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  R1: Pillar → Clusters    (OBLIGATOIRE)                         │
│  R2: Cluster → Pillar     (OBLIGATOIRE)                         │
│  R3: Cluster ↔ Cluster    (RECOMMANDÉ)                          │
│  R4: Cross-Pillar         (MODÉRÉ, si sémantiquement pertinent) │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## When to Consult

- **URL structure**: Check ADR-032 (slugification)
- **Pillar/cluster strategy**: Check ADR-031 (SEO_CLUSTER_OF arc)
- **PageRank flow**: Check ADR-031 (LINKS_TO arc)
- **Avoiding slug repetition**: Check ADR-032 (no-repetition rule)

## Key Insight

> "Page owns URL (slug), Entity owns semantics (key). They can differ."
