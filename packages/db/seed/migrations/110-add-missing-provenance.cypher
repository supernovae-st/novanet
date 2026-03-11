// ============================================================================
// Migration 110: Add missing provenance property (v0.19.0 Standard Properties)
// ============================================================================
//
// v0.19.0 introduces the `provenance` property to track data origin and
// lineage for all nodes in the knowledge graph. This enables:
//   - Data quality auditing (novanet_audit)
//   - Source tracing for debugging
//   - Compliance and governance requirements
//   - Migration tracking
//
// Provenance JSON schema:
//   {
//     "source": "seed" | "migration" | "import" | "api" | "mcp",
//     "file": "filename.cypher" (for seed/migration),
//     "migrated_at": "ISO-8601 datetime",
//     "version": "v0.19.0" (optional)
//   }
//
// This migration adds provenance to all nodes missing this property,
// excluding Schema nodes which have their own metadata.
//
// ============================================================================

// -----------------------------------------------------------------------------
// Step 1: Add provenance to SHARED realm nodes (seed data)
// These were likely created by seed files, mark them accordingly
// -----------------------------------------------------------------------------

// Knowledge atoms (from seed files)
MATCH (n:Expression) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:ExpressionSet) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Pattern) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:PatternSet) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:CultureRef) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:CultureSet) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Taboo) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:TabooSet) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:AudienceTrait) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:AudienceSet) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

// Locale layer
MATCH (n:Locale) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Slugification) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Formatting) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Culture) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Adaptation) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Style) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:LanguageFamily) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:LanguageBranch) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:CulturalRealm) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:CulturalSubRealm) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

// Geography layer
MATCH (n:Country) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:Continent) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:GeoRegion) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:GeoSubRegion) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:PopulationCluster) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:PopulationSubCluster) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:EconomicRegion) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:IncomeGroup) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:LendingCategory) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

// -----------------------------------------------------------------------------
// Step 2: Add provenance to ORG realm nodes
// -----------------------------------------------------------------------------

// Semantic layer
MATCH (n:EntityNative) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "authored", "version": "v0.19.0"}';

MATCH (n:EntityCategory) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

// Structure layer
MATCH (n:BlockType) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:ContentSlot) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

// Output layer
MATCH (n:PageNative) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "generated", "version": "v0.19.0"}';

MATCH (n:BlockNative) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "generated", "version": "v0.19.0"}';

// Foundation layer (SEO/GEO)
MATCH (n:ProjectSEOScope) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:ProjectGEOScope) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:SEOKeywordFormat) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:GEOQuerySet) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:GEOQuery) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "imported", "version": "v0.19.0"}';

MATCH (n:GEOAnswer) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "retrieved", "version": "v0.19.0"}';

MATCH (n:Brand) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:BrandPrinciples) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

// Instruction layer
MATCH (n:PromptStyle) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:PromptArtifact) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "seed", "version": "v0.19.0"}';

MATCH (n:OutputArtifact) WHERE n.provenance IS NULL
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "generated", "version": "v0.19.0"}';

// -----------------------------------------------------------------------------
// Step 3: Catch-all for any remaining nodes without provenance
// Excludes Schema nodes which have their own metadata system
// -----------------------------------------------------------------------------

MATCH (n)
WHERE n.provenance IS NULL
  AND NOT n:Schema
  AND NOT (n)-[:INSTANCE_OF]->(:Schema)
SET n.provenance = '{"source": "migration", "file": "110-add-missing-provenance.cypher", "migrated_at": "' + toString(datetime()) + '", "original_source": "unknown", "version": "v0.19.0"}';

// -----------------------------------------------------------------------------
// Verification query (run manually):
// MATCH (n) WHERE n.provenance IS NULL AND NOT n:Schema
// RETURN labels(n)[0] as label, count(*) as cnt
// ORDER BY cnt DESC;
// Expected: 0 results after migration (excluding Schema nodes)
// -----------------------------------------------------------------------------
