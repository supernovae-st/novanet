// Auto-wire IN_SUBCATEGORY relationships (no APOC version)
// ═══════════════════════════════════════════════════════════════════════════════
// This script runs AFTER all other seeds to connect instance nodes to their
// Subcategory nodes via IN_SUBCATEGORY relationships.
//
// Idempotent: Uses WHERE NOT EXISTS and MERGE to allow repeated execution
// Static: No dynamic Cypher or APOC required
//
// Based on: packages/db/seed/00.5-organizing-principles.cypher
// Total: 35 node types -> 9 subcategories
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────
// GLOBAL SCOPE
// ─────────────────────────────────────────────────────────────────────────────

// Global > Config (1 type: Locale)
MATCH (n:Locale)
MATCH (sub:Subcategory {key: 'config'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Global > Knowledge (14 types)
MATCH (n:Constraint)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:Expression)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleCulture)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleCultureReferences)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleIdentity)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleLexicon)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleMarket)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleRulesAdaptation)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleRulesFormatting)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleRulesSlug)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:LocaleVoice)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:Metaphor)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:Pattern)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:Reference)
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// ─────────────────────────────────────────────────────────────────────────────
// PROJECT SCOPE
// ─────────────────────────────────────────────────────────────────────────────

// Project > Foundation (3 types)
MATCH (n:BrandIdentity)
MATCH (sub:Subcategory {key: 'foundation'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:Project)
MATCH (sub:Subcategory {key: 'foundation'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:ProjectL10n)
MATCH (sub:Subcategory {key: 'foundation'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Structure (2 types)
MATCH (n:Block)
MATCH (sub:Subcategory {key: 'structure'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:Page)
MATCH (sub:Subcategory {key: 'structure'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Semantic (2 types)
MATCH (n:Concept)
MATCH (sub:Subcategory {key: 'semantic'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:ConceptL10n)
MATCH (sub:Subcategory {key: 'semantic'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Instruction (5 types)
MATCH (n:BlockPrompt)
MATCH (sub:Subcategory {key: 'instruction'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:BlockRules)
MATCH (sub:Subcategory {key: 'instruction'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:BlockType)
MATCH (sub:Subcategory {key: 'instruction'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:PagePrompt)
MATCH (sub:Subcategory {key: 'instruction'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:PageType)
MATCH (sub:Subcategory {key: 'instruction'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Output (2 types)
MATCH (n:BlockL10n)
MATCH (sub:Subcategory {key: 'output'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:PageL10n)
MATCH (sub:Subcategory {key: 'output'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// ─────────────────────────────────────────────────────────────────────────────
// SHARED SCOPE
// ─────────────────────────────────────────────────────────────────────────────

// Shared > SEO (3 types)
MATCH (n:SEOKeywordL10n)
MATCH (sub:Subcategory {key: 'seo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:SEOKeywordMetrics)
MATCH (sub:Subcategory {key: 'seo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:SEOMiningRun)
MATCH (sub:Subcategory {key: 'seo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Shared > GEO (3 types)
MATCH (n:GEOMiningRun)
MATCH (sub:Subcategory {key: 'geo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:GEOSeedL10n)
MATCH (sub:Subcategory {key: 'geo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

MATCH (n:GEOSeedMetrics)
MATCH (sub:Subcategory {key: 'geo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// ═══════════════════════════════════════════════════════════════════════════════
// SUMMARY QUERY
// ═══════════════════════════════════════════════════════════════════════════════
// Run this to verify the wiring was successful:

MATCH (scope:Scope)-[:HAS_SUBCATEGORY]->(sub:Subcategory)
OPTIONAL MATCH (sub)<-[r:IN_SUBCATEGORY]-(n)
WITH scope.key AS scope, sub.key AS subcategory, count(r) AS instanceCount
ORDER BY scope, instanceCount DESC
RETURN scope, subcategory, instanceCount;
