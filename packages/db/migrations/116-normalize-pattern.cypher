// Migration 116: Normalize Pattern nodes with v0.19.0 standard properties
//
// Purpose: Adds missing standard properties to Pattern nodes
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

// ===== Pattern Normalization =====

// 1. Add node_class to Pattern nodes where missing
MATCH (p:Pattern)
WHERE p.node_class IS NULL
SET p.node_class = 'Pattern'
RETURN count(p) AS pattern_node_class_fixed;

// 2. Add content to Pattern nodes where missing (derive from description)
MATCH (p:Pattern)
WHERE p.content IS NULL AND p.description IS NOT NULL
SET p.content = p.description
RETURN count(p) AS pattern_content_from_description;

// 2b. Add placeholder content to Pattern nodes with no description
MATCH (p:Pattern)
WHERE p.content IS NULL AND p.description IS NULL
SET p.content = 'Text pattern or template for locale-specific formatting'
RETURN count(p) AS pattern_content_placeholder;

// 3. Add llm_context to Pattern nodes where missing
MATCH (p:Pattern)
WHERE p.llm_context IS NULL
SET p.llm_context = 'USE: for text patterns and templates in content generation. TRIGGERS: pattern, template, format, rule. NOT: for general expressions (use Expression), for terminology (use denomination_forms). RELATES: Locale (language rules), PatternSet (container).'
RETURN count(p) AS pattern_llm_context_fixed;

// 4. Add provenance to Pattern nodes where missing
MATCH (p:Pattern)
WHERE p.provenance IS NULL
SET p.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(p) AS pattern_provenance_fixed;

// 4b. Add created_at where missing on Pattern
MATCH (p:Pattern)
WHERE p.created_at IS NULL
SET p.created_at = datetime()
RETURN count(p) AS pattern_created_at_fixed;

// 4c. Add updated_at where missing on Pattern
MATCH (p:Pattern)
WHERE p.updated_at IS NULL
SET p.updated_at = datetime()
RETURN count(p) AS pattern_updated_at_fixed;

// ===== Verification Queries =====

// Verify all Pattern nodes have standard properties
MATCH (p:Pattern)
WHERE p.node_class IS NULL OR p.content IS NULL OR p.llm_context IS NULL OR p.provenance IS NULL OR p.created_at IS NULL OR p.updated_at IS NULL
RETURN count(p) AS pattern_missing_properties;

// Summary: Count normalized nodes
MATCH (p:Pattern)
RETURN 'Pattern' AS node_type, count(p) AS total_count;
