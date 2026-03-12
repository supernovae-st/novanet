// Migration 113: Normalize Culture and CultureRef nodes to v0.19.0 standard
// Purpose: Add missing standard properties (node_class, content, llm_context, provenance)
//          to Culture and CultureRef nodes for CSR compliance and data quality
// Version: v0.19.0
// Date: 2026-03-12

// ==================== CULTURE NODES ====================

// Culture: Add node_class where missing
MATCH (c:Culture)
WHERE c.node_class IS NULL
SET c.node_class = 'Culture'
RETURN count(c) AS culture_node_class_fixed;

// Culture: Add content where missing (derive from display_name if available)
MATCH (c:Culture)
WHERE c.content IS NULL
SET c.content = COALESCE(c.display_name, c.key, 'Cultural knowledge node')
RETURN count(c) AS culture_content_fixed;

// Culture: Add llm_context where missing
MATCH (c:Culture)
WHERE c.llm_context IS NULL
SET c.llm_context = 'USE: for providing cultural context and local knowledge. TRIGGERS: culture, tradition, customs, heritage. NOT: for translations (use Expressions), for individual traits (use AudienceTrait). RELATES: Locale (cultural context), CultureRef (specific references).'
RETURN count(c) AS culture_llm_context_fixed;

// Culture: Add provenance where missing
MATCH (c:Culture)
WHERE c.provenance IS NULL
SET c.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(c) AS culture_provenance_fixed;

// Culture: Add timestamps where missing
MATCH (c:Culture)
WHERE c.created_at IS NULL
SET c.created_at = datetime()
RETURN count(c) AS culture_created_at_fixed;

MATCH (c:Culture)
WHERE c.updated_at IS NULL
SET c.updated_at = datetime()
RETURN count(c) AS culture_updated_at_fixed;

// ==================== CULTUREREF NODES ====================

// CultureRef: Add node_class where missing
MATCH (cr:CultureRef)
WHERE cr.node_class IS NULL
SET cr.node_class = 'CultureRef'
RETURN count(cr) AS cultureref_node_class_fixed;

// CultureRef: Add content where missing (derive from display_name or text if available)
MATCH (cr:CultureRef)
WHERE cr.content IS NULL
SET cr.content = COALESCE(cr.display_name, cr.text, cr.key, 'Cultural reference node')
RETURN count(cr) AS cultureref_content_fixed;

// CultureRef: Add llm_context where missing
MATCH (cr:CultureRef)
WHERE cr.llm_context IS NULL
SET cr.llm_context = 'USE: for cultural references in localized content. TRIGGERS: culture, reference, local knowledge, tradition, custom. NOT: for translations (use Expressions), for general audience traits (use AudienceTrait). RELATES: Locale (localization context), Culture (cultural category).'
RETURN count(cr) AS cultureref_llm_context_fixed;

// CultureRef: Add provenance where missing
MATCH (cr:CultureRef)
WHERE cr.provenance IS NULL
SET cr.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(cr) AS cultureref_provenance_fixed;

// CultureRef: Add timestamps where missing
MATCH (cr:CultureRef)
WHERE cr.created_at IS NULL
SET cr.created_at = datetime()
RETURN count(cr) AS cultureref_created_at_fixed;

MATCH (cr:CultureRef)
WHERE cr.updated_at IS NULL
SET cr.updated_at = datetime()
RETURN count(cr) AS cultureref_updated_at_fixed;

// ==================== VERIFICATION QUERIES ====================

// Verify Culture nodes have all required properties
MATCH (c:Culture)
WHERE c.node_class IS NULL OR c.content IS NULL OR c.llm_context IS NULL OR c.provenance IS NULL
RETURN count(c) AS culture_incomplete_count;

// Verify CultureRef nodes have all required properties
MATCH (cr:CultureRef)
WHERE cr.node_class IS NULL OR cr.content IS NULL OR cr.llm_context IS NULL OR cr.provenance IS NULL
RETURN count(cr) AS cultureref_incomplete_count;

// Summary statistics
MATCH (c:Culture)
RETURN 'Culture' AS node_type, count(c) AS total_count;

MATCH (cr:CultureRef)
RETURN 'CultureRef' AS node_type, count(cr) AS total_count;
