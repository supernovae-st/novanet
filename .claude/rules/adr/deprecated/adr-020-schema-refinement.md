---
id: "020"
title: "Schema Refinement"
version: "v11.5"
status: "historical"
domain: "schema-architecture"
note: "Establishes current 10-layer architecture"
---

# ADR-020: Schema Refinement

**Status**: Approved (v11.5)

**Problem**: Two architectural issues emerged from v11.3/v11.4 usage:
1. **Locale misplacement**: `Locale` was in `shared/locale` layer with settings nodes (Style, Formatting), but Locale is a *definition* (invariant trait), not a *setting* (knowledge trait). This caused trait inconsistency.
2. **SEO/GEO redundancy**: SEO/GEO nodes in `org` realm duplicated knowledge that should be universal across organizations. An SEO keyword like "QR code generator" is the same regardless of organization.

**Decision**: Refine the schema with Locale moved to shared/config and SEO/GEO layers consolidated.

## Changes

| Area | Before (v11.4) | After (v11.5) |
|------|----------------|---------------|
| **Locale location** | shared/locale | shared/config |
| **SEO/GEO layers** | org/seo, org/geo (separate) | Removed from org |
| **SEO/GEO nodes** | In org realm | Moved to shared/knowledge |
| **Layer count** | 11 (3 shared + 8 org) | 10 (4 shared + 6 org) |
| **Node distribution** | 32 shared + 29 org | **40 shared + 20 org = 61 nodes** |

## Locale to Config Layer

```
BEFORE (v11.4):
|-- shared/
|   |-- config/          # EntityCategory only
|   |-- locale/          # Locale, Style, Formatting, etc.

AFTER (v11.5):
|-- shared/
|   |-- config/          # EntityCategory + Locale (definitions)
|   |-- locale/          # Style, Formatting, etc. (settings)
```

**Rationale:**
- Locale is a DEFINITION (invariant), not a parameter/setting
- Follows EntityCategory pattern: definitions go in config layer
- shared/locale now contains only locale SETTINGS (Culture, Style, etc.)
- Clean separation: definitions vs settings

## SEO/GEO Consolidation

```
BEFORE (v11.4):
|-- org/
    |-- seo/             # 5 nodes (SEOKeyword, etc.)
    |-- geo/             # 3 nodes (GEOQuery, etc.)

AFTER (v11.5):
|-- shared/
|   |-- knowledge/       # Includes SEO + GEO nodes (now 26 nodes)
|-- org/
    # No seo/geo layers
```

**Rationale:**
- SEO/GEO data is universal market intelligence, not org-specific
- Moving to shared realm enables cross-org analytics
- Reduces org layers from 8 to 6
- Knowledge layer becomes the home for all market intelligence

## Architecture Summary (v11.5)

```
REALMS (61 nodes total):
|-- shared/              # Universal knowledge (READ-ONLY) - 40 nodes
|   |-- config/          # 3 nodes (EntityCategory, Locale, SEOKeywordFormat)
|   |-- locale/          # 6 nodes (Culture, Style, Formatting, etc.)
|   |-- geography/       # 6 nodes (Continent, Region, etc.)
|   |-- knowledge/       # 24 nodes (Terms, Expressions, SEO, GEO, etc.)
|
|-- org/                 # Organization-specific - 20 nodes
    |-- config/          # 1 node (OrgConfig)
    |-- foundation/      # 3 nodes (Project, ProjectNative, BrandIdentity)
    |-- structure/       # 3 nodes (Page, Block, ContentSlot)
    |-- semantic/        # 4 nodes (Entity, EntityNative, AudiencePersona, ChannelSurface)
    |-- instruction/     # 6 nodes (PageStructure, PageInstruction, BlockInstruction, BlockType, BlockRules, PromptArtifact)
    |-- output/          # 3 nodes (PageNative, BlockNative, OutputArtifact)
```
