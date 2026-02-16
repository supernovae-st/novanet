---
id: "018"
title: "Classification System Refinement"
version: "v11.2"
status: "historical"
domain: "node-classification"
note: "Superseded by ADR-024 (Trait Redefinition)"
---

# ADR-018: Classification System Refinement

**Status**: Approved (v11.2)

**Decision**: Comprehensive refinement of the NovaNet classification system.

## Changes

| Area | Before | After |
|------|--------|-------|
| **Realm names** | `global`, `tenant` | `shared`, `org` |
| **Trait split** | `derived` (8 nodes) | `generated` (4) + `aggregated` (3) |
| **Job trait** | `job` (3 nodes) | Removed |
| **Container traits** | `knowledge` | `invariant` (6 containers) |
| **Node count** | 65 nodes | 62 nodes |

## Realm Renames

```
BEFORE (confusing):
|-- global/config      <- "global" sounds like "everywhere"
|-- tenant/config      <- "tenant" is SaaS jargon

AFTER (clear):
|-- shared/config      <- Describes WHAT (shared resources)
|-- org/config         <- Describes WHO (organization owns it)
```

**Benefits:**
- `shared` = describes purpose (shared resources), not scope
- `org` = familiar terminology (GitHub/Slack orgs), avoids SaaS jargon
- Short: `org` (3 chars) vs `tenant` (6 chars)
- No collision: `Organization` node exists in `org/config`

## Trait Changes

| Node | v11.1 Trait | v11.2 Trait |
|------|-------------|-------------|
| PageNative | derived | **generated** |
| BlockNative | derived | **generated** |
| OutputArtifact | derived | **generated** |
| PromptArtifact | derived | **generated** |
| GEOAnswer | derived | **aggregated** |
| GEOMetrics | derived | **aggregated** |
| SEOKeywordMetrics | derived | **aggregated** |

**Trait summary (v11.2):**
- `invariant`: 30 nodes (+6 containers)
- `localized`: 2 nodes (EntityNative, ProjectNative)
- `knowledge`: 23 nodes (-6 containers)
- `generated`: 4 nodes (LLM output)
- `aggregated`: 3 nodes (computed metrics)
- ~~`job`~~: REMOVED (0 nodes)

## Removed Nodes (Job Concept)

```
DELETED:
- GenerationJob    (org/output)
- EvaluationSignal (org/output)
- SEOMiningRun     (org/seo)
```

**Rationale**: Job concept deferred until generation pipeline is more mature. Clean architecture now, add workflow nodes in v12+ when needed.

## Container Traits

Containers changed from `knowledge` to `invariant`:

```
TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
```

**Rationale:**
- Containers are universal categories (pricing, legal, technical) -> exist in ALL locales
- Atoms remain `knowledge` -> locale-specific content

## Architecture Summary (v11.2)

```
REALMS (62 nodes total):
|-- shared/              # Universal locale knowledge (READ-ONLY) - 32 nodes
|   |-- config/          # 14 nodes (incl. EntityCategory)
|   |-- locale-knowledge/# 18 nodes (6 containers + 12 atoms)
|
|-- org/                 # Organization-specific content - 30 nodes
    |-- config/          # 2 nodes (Organization, Tenant)
    |-- foundation/      # 3 nodes
    |-- structure/       # 3 nodes
    |-- semantic/        # 4 nodes
    |-- seo/             # 8 nodes (SEOMiningRun removed)
    |-- instruction/     # 7 nodes
    |-- output/          # 3 nodes (job nodes removed)
```

**Migration**:
- Directory renames: `node-classes/global/` -> `shared/`, `tenant/` -> `org/`
- YAML realm field updates: 65 files
- Rust code: 250+ occurrences
- TypeScript code: 80+ occurrences
- Test assertions: 20+ updates
