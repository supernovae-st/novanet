# NovaNet Data Quality Fix - Master Plan

## Overview

This migration set fixes all data quality issues identified in the comprehensive audit.
Organized into 3 plans for safe, logical execution order.

## Execution Order

```
PLAN A (Foundation) → PLAN B (Structure) → PLAN C (Data) → Verify
```

**CRITICAL**: Execute in order. Later plans depend on earlier ones.

## PLAN A: Foundation Layer (084-087)

| Migration | Description | CSR Impact |
|-----------|-------------|------------|
| 084 | Create OrgConfig node | +1 node, enables org hierarchy |
| 085 | Fix supernovae-studio Project | Complete project config |
| 086 | Create ProjectGEOScope/SEOScope | Project-level scopes |
| 087 | Add updated_at to Schema nodes | 227 nodes get timestamps |

**Verification after PLAN A:**
```cypher
MATCH (oc:OrgConfig) RETURN count(oc);  // Should be 1
MATCH (p:Project)-[:HAS_SEO_SCOPE]->() RETURN count(p);  // Should be 2
```

## PLAN B: Structural Gaps (088-094)

| Migration | Description | CSR Impact |
|-----------|-------------|------------|
| 088 | Create EntityCategory nodes | +6 category nodes |
| 089 | Link Entities to Categories | 9 BELONGS_TO_CATEGORY arcs |
| 090 | Create BlockType nodes | +7 block type definitions |
| 091 | Link Blocks to Types | OF_TYPE arcs for all blocks |
| 092 | Add BCP47 to Locales | 203 locales get bcp47 property |
| 093 | Create LanguageBranch nodes | +13 language family nodes |
| 094 | Add Entity descriptions | 9 entities get descriptions |

**Verification after PLAN B:**
```cypher
MATCH (c:EntityCategory) RETURN count(c);  // Should be 6
MATCH (bt:BlockType) RETURN count(bt);  // Should be 7
MATCH (l:Locale) WHERE l.bcp47 IS NOT NULL RETURN count(l);  // Should be 203
```

## PLAN C: Data Quality (095-099)

| Migration | Description | CSR Impact |
|-----------|-------------|------------|
| 095 | Fix FOR_LOCALE null arcs | Fix 60 broken locale links |
| 096 | Fix SEOKeyword data | Add locale_key to 59 keywords |
| 097 | Add locale to Expressions | 17,036 expressions get locale |
| 098 | Populate fr-FR Patterns | 8 patterns get templates |
| 099 | Comprehensive verification | Final CSR check |

**Verification after PLAN C:**
```cypher
// Run migration 099 directly for full report
// Or use MCP:
novanet_audit(target: "all")
```

## Execution Commands

```bash
# From novanet root
cd packages/db

# Execute all migrations
./seed.sh migrations/084-create-orgconfig-node.cypher
./seed.sh migrations/085-fix-supernovae-studio-project.cypher
./seed.sh migrations/086-create-project-scopes.cypher
./seed.sh migrations/087-add-schema-timestamps.cypher
./seed.sh migrations/088-create-entity-categories.cypher
./seed.sh migrations/089-link-entities-to-categories.cypher
./seed.sh migrations/090-create-block-types.cypher
./seed.sh migrations/091-link-blocks-to-types.cypher
./seed.sh migrations/092-add-bcp47-to-locales.cypher
./seed.sh migrations/093-create-language-branches.cypher
./seed.sh migrations/094-add-entity-descriptions.cypher
./seed.sh migrations/095-fix-for-locale-arcs.cypher
./seed.sh migrations/096-fix-seo-keyword-data.cypher
./seed.sh migrations/097-add-locale-to-expressions.cypher
./seed.sh migrations/098-populate-fr-fr-patterns.cypher
./seed.sh migrations/099-comprehensive-data-verification.cypher
```

## CSR Targets

| Metric | Before | Target |
|--------|--------|--------|
| Overall CSR | ~75% | ≥95% |
| FOR_LOCALE integrity | 58% | 100% |
| Expression locale | 0% | 100% |
| Schema timestamps | ~50% | 100% |
| Entity categorization | 0% | 100% |

## Ralph Wiggum Loop

After running migrations:

1. Run `novanet_audit(target: "all")`
2. If CSR < 95%, identify remaining issues
3. Create targeted fix migrations
4. Re-run audit
5. Repeat until CSR ≥ 95%

## Rollback

Migrations are additive (MERGE patterns). To rollback:

```cypher
// Delete OrgConfig (PLAN A)
MATCH (oc:OrgConfig {key: 'supernovae'}) DETACH DELETE oc;

// Delete EntityCategories (PLAN B)
MATCH (c:EntityCategory) DETACH DELETE c;

// Remove bcp47 property (PLAN B)
MATCH (l:Locale) REMOVE l.bcp47;

// Reset SEOKeyword volume (PLAN C)
MATCH (kw:SEOKeyword) SET kw.volume = null, kw.volume_source = null;
```

## Created

- **Date**: 2026-03-10
- **Version**: v0.17.2
- **Author**: Claude + Thibaut
