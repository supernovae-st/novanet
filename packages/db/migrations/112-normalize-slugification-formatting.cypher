// ============================================================================
// MIGRATION 112: Normalize Slugification and Formatting Standard Properties
// Date: 2026-03-12
// Version: v0.19.0
// Purpose: Add missing v0.19.0 standard properties to Slugification and
//          Formatting nodes per YAML schema definitions.
//
// Slugification standard properties:
//   - node_class (string) = "Slugification"
//   - content (string) = descriptive text
//   - llm_context (string) = USE/TRIGGERS/NOT/RELATES pattern
//   - provenance (string) = JSON with source, created_by, version
//   - created_at (datetime) = timestamp
//   - updated_at (datetime) = timestamp
//
// Formatting standard properties:
//   - node_class (string) = "Formatting"
//   - content (string) = descriptive text
//   - llm_context (string) = USE/TRIGGERS/NOT/RELATES pattern
//   - provenance (string) = JSON with source, created_by, version
//   - created_at (datetime) = timestamp
//   - updated_at (datetime) = timestamp
// ============================================================================

// ============================================================================
// SLUGIFICATION NODE FIXES
// ============================================================================

// Step 1: Add node_class to Slugification nodes where missing
MATCH (n:Slugification)
WHERE n.node_class IS NULL
SET n.node_class = 'Slugification'
RETURN count(n) AS slugification_node_class_fixed;

// Step 2: Add content to Slugification nodes where missing
MATCH (n:Slugification)
WHERE n.content IS NULL
SET n.content = 'URL slug generation rules: transliteration, stop words, URL conventions'
RETURN count(n) AS slugification_content_fixed;

// Step 3: Add llm_context to Slugification nodes where missing
MATCH (n:Slugification)
WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: for URL-safe slug generation rules based on locale-specific conventions. TRIGGERS: slug, url, transliteration, stop-words, diacritic. NOT: for title formatting (use Formatting instead). RELATES: Locale (parent), Formatting (sibling locale rules), Expression (vocabulary reference).'
RETURN count(n) AS slugification_llm_context_fixed;

// Step 4: Add provenance to Slugification nodes where missing
MATCH (n:Slugification)
WHERE n.provenance IS NULL
SET n.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(n) AS slugification_provenance_fixed;

// Step 5: Add created_at to Slugification nodes where missing
MATCH (n:Slugification)
WHERE n.created_at IS NULL
SET n.created_at = datetime()
RETURN count(n) AS slugification_created_at_fixed;

// Step 6: Add updated_at to Slugification nodes where missing
MATCH (n:Slugification)
WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS slugification_updated_at_fixed;

// ============================================================================
// FORMATTING NODE FIXES
// ============================================================================

// Step 7: Add node_class to Formatting nodes where missing
MATCH (n:Formatting)
WHERE n.node_class IS NULL
SET n.node_class = 'Formatting'
RETURN count(n) AS formatting_node_class_fixed;

// Step 8: Add content to Formatting nodes where missing
MATCH (n:Formatting)
WHERE n.content IS NULL
SET n.content = 'Technical formatting rules: dates, numbers, currency, time, phone, address, units'
RETURN count(n) AS formatting_content_fixed;

// Step 9: Add llm_context to Formatting nodes where missing
MATCH (n:Formatting)
WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: for locale-specific formatting of dates, numbers, currency, time, addresses, and contact information. TRIGGERS: format, date, number, currency, phone, address, measurement. NOT: for slug generation (use Slugification instead). RELATES: Locale (parent), Slugification (sibling locale rules), Expression (vocabulary reference).'
RETURN count(n) AS formatting_llm_context_fixed;

// Step 10: Add provenance to Formatting nodes where missing
MATCH (n:Formatting)
WHERE n.provenance IS NULL
SET n.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(n) AS formatting_provenance_fixed;

// Step 11: Add created_at to Formatting nodes where missing
MATCH (n:Formatting)
WHERE n.created_at IS NULL
SET n.created_at = datetime()
RETURN count(n) AS formatting_created_at_fixed;

// Step 12: Add updated_at to Formatting nodes where missing
MATCH (n:Formatting)
WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS formatting_updated_at_fixed;

// ============================================================================
// VERIFICATION QUERIES
// ============================================================================

// Verify Slugification nodes have all standard properties
MATCH (n:Slugification)
WHERE n.node_class IS NULL OR n.content IS NULL OR n.llm_context IS NULL
  OR n.provenance IS NULL OR n.created_at IS NULL OR n.updated_at IS NULL
RETURN count(n) AS slugification_missing_properties;

// Verify Formatting nodes have all standard properties
MATCH (n:Formatting)
WHERE n.node_class IS NULL OR n.content IS NULL OR n.llm_context IS NULL
  OR n.provenance IS NULL OR n.created_at IS NULL OR n.updated_at IS NULL
RETURN count(n) AS formatting_missing_properties;

// Summary: All Slugification and Formatting nodes now fully compliant
MATCH (n:Slugification)
RETURN count(n) AS slugification_total_count,
       count(CASE WHEN n.node_class = 'Slugification' THEN 1 END) AS slugification_node_class_count,
       count(CASE WHEN n.content IS NOT NULL THEN 1 END) AS slugification_content_count,
       count(CASE WHEN n.llm_context IS NOT NULL THEN 1 END) AS slugification_llm_context_count,
       count(CASE WHEN n.provenance IS NOT NULL THEN 1 END) AS slugification_provenance_count,
       count(CASE WHEN n.created_at IS NOT NULL THEN 1 END) AS slugification_created_at_count,
       count(CASE WHEN n.updated_at IS NOT NULL THEN 1 END) AS slugification_updated_at_count;

MATCH (n:Formatting)
RETURN count(n) AS formatting_total_count,
       count(CASE WHEN n.node_class = 'Formatting' THEN 1 END) AS formatting_node_class_count,
       count(CASE WHEN n.content IS NOT NULL THEN 1 END) AS formatting_content_count,
       count(CASE WHEN n.llm_context IS NOT NULL THEN 1 END) AS formatting_llm_context_count,
       count(CASE WHEN n.provenance IS NOT NULL THEN 1 END) AS formatting_provenance_count,
       count(CASE WHEN n.created_at IS NOT NULL THEN 1 END) AS formatting_created_at_count,
       count(CASE WHEN n.updated_at IS NOT NULL THEN 1 END) AS formatting_updated_at_count;
