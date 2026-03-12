// Migration 115: Normalize AudienceTrait and Taboo nodes with v0.19.0 standard properties
//
// Purpose: Adds missing standard properties to AudienceTrait and Taboo nodes
//   - node_class (string): must match the node label
//   - content (string): descriptive text
//   - llm_context (string): USE/TRIGGERS/NOT/RELATES pattern
//   - provenance (string): JSON with source, created_by, version
//   - created_at/updated_at: timestamps where missing
//
// These properties ensure CSR (Constraint Satisfaction Rate) compliance
// and enable proper LLM context injection for knowledge atoms.
//
// Related: ADR-033 (denomination forms), ADR-036 (@ pattern)

// ===== AudienceTrait Normalization =====

// 1. Add node_class to AudienceTrait nodes where missing
MATCH (a:AudienceTrait)
WHERE a.node_class IS NULL
SET a.node_class = 'AudienceTrait'
RETURN count(a) AS audiencetrait_node_class_fixed;

// 2. Add content to AudienceTrait nodes where missing (derive from description)
MATCH (a:AudienceTrait)
WHERE a.content IS NULL AND a.description IS NOT NULL
SET a.content = a.description
RETURN count(a) AS audiencetrait_content_from_description;

// 2b. Add placeholder content to AudienceTrait nodes with no description
MATCH (a:AudienceTrait)
WHERE a.content IS NULL AND a.description IS NULL
SET a.content = 'Audience characteristic for locale-specific content generation'
RETURN count(a) AS audiencetrait_content_placeholder;

// 3. Add llm_context to AudienceTrait nodes where missing
MATCH (a:AudienceTrait)
WHERE a.llm_context IS NULL
SET a.llm_context = 'USE: for audience characteristics in locale. TRIGGERS: audience, demographic, trait. NOT: for geographic data (use GEOQuery). RELATES: Locale (target market), AudienceSet (container).'
RETURN count(a) AS audiencetrait_llm_context_fixed;

// 4. Add provenance to AudienceTrait nodes where missing
MATCH (a:AudienceTrait)
WHERE a.provenance IS NULL
SET a.provenance = '{\"source\": \"seed\", \"created_by\": \"system\", \"version\": \"0.19.0\"}'
RETURN count(a) AS audiencetrait_provenance_fixed;

// 4b. Add created_at where missing on AudienceTrait
MATCH (a:AudienceTrait)
WHERE a.created_at IS NULL
SET a.created_at = datetime()
RETURN count(a) AS audiencetrait_created_at_fixed;

// 4c. Add updated_at where missing on AudienceTrait
MATCH (a:AudienceTrait)
WHERE a.updated_at IS NULL
SET a.updated_at = datetime()
RETURN count(a) AS audiencetrait_updated_at_fixed;

// ===== Taboo Normalization =====

// 5. Add node_class to Taboo nodes where missing
MATCH (t:Taboo)
WHERE t.node_class IS NULL
SET t.node_class = 'Taboo'
RETURN count(t) AS taboo_node_class_fixed;

// 6. Add content to Taboo nodes where missing (derive from description)
MATCH (t:Taboo)
WHERE t.content IS NULL AND t.description IS NOT NULL
SET t.content = t.description
RETURN count(t) AS taboo_content_from_description;

// 6b. Add placeholder content to Taboo nodes with no description
MATCH (t:Taboo)
WHERE t.content IS NULL AND t.description IS NULL
SET t.content = 'Cultural taboo to avoid in locale-specific content'
RETURN count(t) AS taboo_content_placeholder;

// 7. Add llm_context to Taboo nodes where missing
MATCH (t:Taboo)
WHERE t.llm_context IS NULL
SET t.llm_context = 'USE: for cultural taboos to avoid in content generation. TRIGGERS: taboo, avoid, sensitive, forbidden. NOT: for general constraints (use BlockNative restrictions). RELATES: Locale (cultural context), TabooSet (container).'
RETURN count(t) AS taboo_llm_context_fixed;

// 8. Add provenance to Taboo nodes where missing
MATCH (t:Taboo)
WHERE t.provenance IS NULL
SET t.provenance = '{\"source\": \"seed\", \"created_by\": \"system\", \"version\": \"0.19.0\"}'
RETURN count(t) AS taboo_provenance_fixed;

// 8b. Add created_at where missing on Taboo
MATCH (t:Taboo)
WHERE t.created_at IS NULL
SET t.created_at = datetime()
RETURN count(t) AS taboo_created_at_fixed;

// 8c. Add updated_at where missing on Taboo
MATCH (t:Taboo)
WHERE t.updated_at IS NULL
SET t.updated_at = datetime()
RETURN count(t) AS taboo_updated_at_fixed;

// ===== Verification Queries =====

// Verify all AudienceTrait nodes have standard properties
MATCH (a:AudienceTrait)
WHERE a.node_class IS NULL OR a.content IS NULL OR a.llm_context IS NULL OR a.provenance IS NULL OR a.created_at IS NULL OR a.updated_at IS NULL
RETURN count(a) AS audiencetrait_missing_properties;

// Verify all Taboo nodes have standard properties
MATCH (t:Taboo)
WHERE t.node_class IS NULL OR t.content IS NULL OR t.llm_context IS NULL OR t.provenance IS NULL OR t.created_at IS NULL OR t.updated_at IS NULL
RETURN count(t) AS taboo_missing_properties;

// Summary: Count normalized nodes
MATCH (a:AudienceTrait)
RETURN 'AudienceTrait' AS node_type, count(a) AS total_count;

MATCH (t:Taboo)
RETURN 'Taboo' AS node_type, count(t) AS total_count;
