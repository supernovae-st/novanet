// Migration 111: Add missing standard properties to Locale and Country nodes
// Issue: Seeds created nodes without v0.19.0 required standard properties
// Adds: node_class, content, llm_context, provenance where missing
// Executed: 2026-03-12

// Step 1: Add node_class to Locale nodes
MATCH (l:Locale)
WHERE l.node_class IS NULL
SET l.node_class = 'Locale'
RETURN count(l) AS locales_node_class_fixed;

// Step 2: Add content to Locale nodes (derive from display_name)
MATCH (l:Locale)
WHERE l.content IS NULL AND l.display_name IS NOT NULL
SET l.content = l.display_name + ' locale for ' + COALESCE(l.country_code, 'this region')
RETURN count(l) AS locales_content_fixed;

// Step 3: Add llm_context to Locale nodes
MATCH (l:Locale)
WHERE l.llm_context IS NULL
SET l.llm_context = 'USE: for content targeting ' + COALESCE(l.display_name, l.key) + '. TRIGGERS: ' + l.key + ', ' + COALESCE(l.language_code, '') + ', ' + COALESCE(l.name_native, '') + '. RELATES: Country (geographic), LanguageBranch (linguistic).'
RETURN count(l) AS locales_llm_context_fixed;

// Step 4: Add provenance to Locale nodes
MATCH (l:Locale)
WHERE l.provenance IS NULL
SET l.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(l) AS locales_provenance_fixed;

// Step 5: Add node_class to Country nodes
MATCH (c:Country)
WHERE c.node_class IS NULL
SET c.node_class = 'Country'
RETURN count(c) AS countries_node_class_fixed;

// Step 6: Add provenance to Country nodes
MATCH (c:Country)
WHERE c.provenance IS NULL
SET c.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(c) AS countries_provenance_fixed;

// Step 7: Ensure all Locale nodes have created_at/updated_at
MATCH (l:Locale)
WHERE l.created_at IS NULL
SET l.created_at = datetime(), l.updated_at = datetime()
RETURN count(l) AS locales_timestamps_fixed;

// Step 8: Ensure all Country nodes have created_at/updated_at
MATCH (c:Country)
WHERE c.created_at IS NULL
SET c.created_at = datetime(), c.updated_at = datetime()
RETURN count(c) AS countries_timestamps_fixed;

// Verification: Check remaining Locale nodes without required properties
MATCH (l:Locale)
WHERE l.node_class IS NULL OR l.content IS NULL OR l.provenance IS NULL
RETURN count(l) AS remaining_locale_issues;

// Verification: Check remaining Country nodes without required properties
MATCH (c:Country)
WHERE c.node_class IS NULL OR c.provenance IS NULL
RETURN count(c) AS remaining_country_issues;
