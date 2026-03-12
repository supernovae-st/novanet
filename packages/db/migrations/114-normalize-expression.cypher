// Migration 114: Normalize Expression nodes to v0.19.0 standard properties
// Purpose: Add missing standard properties (node_class, display_name, content, llm_context, provenance, created_at, updated_at)
// to Expression nodes to achieve v0.19.0 compliance
// Date: 2026-03-12
// Status: Pending review and application

// ============================================================================
// Phase 1: Add node_class = 'Expression' where missing
// ============================================================================

MATCH (e:Expression)
WHERE e.node_class IS NULL
SET e.node_class = 'Expression'
RETURN count(e) AS expression_node_class_fixed;

// ============================================================================
// Phase 2: Add display_name from text property where missing
// ============================================================================

MATCH (e:Expression)
WHERE e.display_name IS NULL AND e.text IS NOT NULL
SET e.display_name = e.text
RETURN count(e) AS expression_display_name_fixed;

// ============================================================================
// Phase 3: Add content from text property where missing
// ============================================================================

MATCH (e:Expression)
WHERE e.content IS NULL AND e.text IS NOT NULL
SET e.content = e.text
RETURN count(e) AS expression_content_fixed;

// ============================================================================
// Phase 4: Add llm_context where missing
// ============================================================================

MATCH (e:Expression)
WHERE e.llm_context IS NULL
SET e.llm_context = 'USE: for idiomatic expressions in locale. TRIGGERS: idiom, phrase, expression. RELATES: Locale (language), ExpressionSet (container).'
RETURN count(e) AS expression_llm_context_fixed;

// ============================================================================
// Phase 5: Add provenance where missing
// ============================================================================

MATCH (e:Expression)
WHERE e.provenance IS NULL
SET e.provenance = '{"source": "seed", "created_by": "system", "version": "0.19.0"}'
RETURN count(e) AS expression_provenance_fixed;

// ============================================================================
// Phase 6: Add created_at where missing
// ============================================================================

MATCH (e:Expression)
WHERE e.created_at IS NULL
SET e.created_at = datetime()
RETURN count(e) AS expression_created_at_fixed;

// ============================================================================
// Phase 7: Add updated_at where missing
// ============================================================================

MATCH (e:Expression)
WHERE e.updated_at IS NULL
SET e.updated_at = datetime()
RETURN count(e) AS expression_updated_at_fixed;

// ============================================================================
// Verification Queries
// ============================================================================

// Verify all Expression nodes have required properties
MATCH (e:Expression)
WHERE e.node_class IS NULL OR e.display_name IS NULL OR e.content IS NULL
  OR e.llm_context IS NULL OR e.provenance IS NULL
  OR e.created_at IS NULL OR e.updated_at IS NULL
RETURN count(e) AS expression_nodes_with_missing_properties;

// Show sample of normalized Expression nodes
MATCH (e:Expression)
RETURN
  e.key,
  e.node_class,
  e.display_name,
  e.locale,
  e.text,
  e.content,
  e.created_at,
  e.updated_at
LIMIT 10;

// Count Expression nodes by locale (verify data integrity)
MATCH (e:Expression)
RETURN e.locale, count(e) AS count
ORDER BY count DESC;

// Final verification: count Expression nodes with ALL required properties
MATCH (e:Expression)
WHERE e.node_class IS NOT NULL
  AND e.display_name IS NOT NULL
  AND e.content IS NOT NULL
  AND e.llm_context IS NOT NULL
  AND e.provenance IS NOT NULL
  AND e.created_at IS NOT NULL
  AND e.updated_at IS NOT NULL
RETURN count(e) AS expression_nodes_fully_normalized;
