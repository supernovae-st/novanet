// Migration 104: Fix Enrichment Schema nodes with null name property
// Issue: ExpressionEnrichment, TabooEnrichment, CultureRefEnrichment have name: null
// Fix: Set name = display_name for these Schema/Class nodes

// Fix ExpressionEnrichment
MATCH (c:Schema:Class {key: 'expression-enrichment'})
WHERE c.name IS NULL
SET c.name = c.display_name
RETURN 'Fixed ExpressionEnrichment.name' AS result, c.name AS name;

// Fix TabooEnrichment
MATCH (c:Schema:Class {key: 'taboo-enrichment'})
WHERE c.name IS NULL
SET c.name = c.display_name
RETURN 'Fixed TabooEnrichment.name' AS result, c.name AS name;

// Fix CultureRefEnrichment
MATCH (c:Schema:Class {key: 'culture-ref-enrichment'})
WHERE c.name IS NULL
SET c.name = c.display_name
RETURN 'Fixed CultureRefEnrichment.name' AS result, c.name AS name;

// Verify no remaining null names
MATCH (c:Schema:Class)
WHERE c.name IS NULL
RETURN count(c) AS remaining_null_names;
