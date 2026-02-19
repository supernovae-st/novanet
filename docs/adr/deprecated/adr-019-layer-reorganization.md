---
id: "019"
title: "Layer Reorganization"
version: "v11.3"
status: "historical"
domain: "schema-architecture"
note: "Superseded by ADR-020 (Schema Refinement)"
---

# ADR-019: Layer Reorganization

**Status**: Approved (v11.3)

**Problem**: The `locale-knowledge` layer in v11.2 conflated three distinct concerns:
1. BCP-47 locale configuration (Locale, LocaleVoice, LocaleGrammar)
2. Geographic data (Region, Country, GeoFeature)
3. Semantic knowledge atoms (Terms, Expressions, Patterns)

This made queries ambiguous ("knowledge in locale-knowledge" is confusing) and prevented clean layer-based filtering in Studio.

**Decision**: Reorganize the layer structure for better semantic clarity.

## Changes

| Area | Before (v11.2) | After (v11.3) |
|------|----------------|---------------|
| **Shared layers** | 2 (config, locale-knowledge) | 3 (locale, geography, knowledge) |
| **Org layers** | 7 | 8 (+geo) |
| **Total layers** | 9 | 11 |
| **Org config** | Organization + Tenant (2 nodes) | OrgConfig (1 node) |
| **Total nodes** | 62 | 61 |

## Layer Split: locale-knowledge -> 3 layers

```
BEFORE (v11.2):
|-- shared/
|   |-- config/              # 14 nodes
|   |-- locale-knowledge/    # 18 nodes (mixed concerns)

AFTER (v11.3):
|-- shared/
|   |-- locale/              # 7 nodes (Locale, LocaleVoice, LocaleGrammar, etc.)
|   |-- geography/           # 6 nodes (Region, Country, GeoFeature, etc.)
|   |-- knowledge/           # 19 nodes (TermSet, Term, CultureSet, etc.)
```

**Rationale:**
- `locale-knowledge` mixed locale configuration with geographic data and semantic knowledge
- Split into 3 focused layers with clearer purposes:
  - `locale`: BCP-47 locale configuration (voice, grammar, formatting)
  - `geography`: Geographic entities (regions, countries, features)
  - `knowledge`: Semantic atoms (terms, expressions, patterns, culture)

## New Layer: geo (org realm)

```
BEFORE (v11.2):
|-- org/
    |-- seo/                 # 8 nodes (SEO + GEO mixed)

AFTER (v11.3):
|-- org/
    |-- seo/                 # 5 nodes (SEO only)
    |-- geo/                 # 3 nodes (GEOAnswer, GEOMetrics, GEOQuery)
```

**Rationale:**
- GEO (AI search optimization) and SEO (search engine optimization) are distinct disciplines
- Separate layers enable clearer queries and filtering
- GEO nodes have trait `aggregated`, SEO nodes have mixed traits

## Node Merge: Organization + Tenant -> OrgConfig

```
BEFORE (v11.2):
|-- org/
    |-- config/              # Organization, Tenant (2 nodes)

AFTER (v11.3):
|-- org/
    |-- config/              # OrgConfig (1 node)
```

**Rationale:**
- Organization and Tenant were redundant in 2-realm architecture
- Single OrgConfig node holds all org-level configuration
- Simplifies config layer to single entry point

## Architecture Summary (v11.3)

```
REALMS (61 nodes total):
|-- shared/              # Universal locale knowledge (READ-ONLY) - 32 nodes
|   |-- locale/          # 7 nodes (Locale, LocaleVoice, LocaleGrammar, LocaleFormats, etc.)
|   |-- geography/       # 6 nodes (Region, Country, GeoFeature, GeoZone, etc.)
|   |-- knowledge/       # 19 nodes (CategorySet, EntityCategory, TermSet, Term, etc.)
|
|-- org/                 # Organization-specific content - 29 nodes
    |-- config/          # 1 node (OrgConfig)
    |-- foundation/      # 3 nodes (Project, ProjectNative, Brand)
    |-- structure/       # 3 nodes (Page, PageType, Block, BlockType)
    |-- semantic/        # 4 nodes (Entity, EntityNative, Thing, Category)
    |-- instruction/     # 7 nodes (PagePrompt, BlockPrompt, SEOPrompt, etc.)
    |-- seo/             # 5 nodes (SEOKeyword, SEOKeywordMetrics, SEOCluster, etc.)
    |-- geo/             # 3 nodes (GEOQuery, GEOAnswer, GEOMetrics)
    |-- output/          # 3 nodes (PageNative, BlockNative, OutputArtifact)
```

## Migration

1. **Directory restructure**:
   ```bash
   mv shared/locale-knowledge/ -> split into locale/, geography/, knowledge/
   ```

2. **YAML layer field updates**: 32 files in shared realm

3. **New geo layer**: Move GEO* nodes from seo/ to geo/

4. **Node merge**:
   - Delete `organization.yaml` and `tenant.yaml`
   - Create `org-config.yaml`
   - Update arcs referencing Organization/Tenant

5. **Regenerate artifacts**: `cargo run -- schema generate`
