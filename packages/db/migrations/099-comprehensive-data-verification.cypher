// ============================================================================
// PLAN C - Migration 099: Comprehensive Data Quality Verification
// ============================================================================
// Purpose: Final verification after all migrations
// Run this AFTER all other migrations to validate completeness
// CSR Target: >= 95% across all metrics
// ============================================================================

// ============================================================================
// SECTION 1: FOUNDATION LAYER VERIFICATION
// ============================================================================

// 1.1 OrgConfig verification
MATCH (oc:OrgConfig {key: 'supernovae'})
OPTIONAL MATCH (oc)-[:HAS_PROJECT]->(p:Project)
RETURN 'OrgConfig' AS check,
       CASE WHEN oc IS NOT NULL THEN 'PASS' ELSE 'FAIL' END AS status,
       count(p) AS project_count;

// 1.2 Project completeness
MATCH (p:Project)
OPTIONAL MATCH (p)-[:HAS_DEFAULT_LOCALE]->(dl:Locale)
OPTIONAL MATCH (p)-[:HAS_BRAND]->(b:Brand)
OPTIONAL MATCH (p)-[:HAS_SEO_SCOPE]->(seo:ProjectSEOScope)
OPTIONAL MATCH (p)-[:HAS_GEO_SCOPE]->(geo:ProjectGEOScope)
RETURN p.key AS project,
       CASE WHEN dl IS NOT NULL THEN 'PASS' ELSE 'FAIL' END AS has_default_locale,
       CASE WHEN b IS NOT NULL THEN 'PASS' ELSE 'FAIL' END AS has_brand,
       CASE WHEN seo IS NOT NULL THEN 'PASS' ELSE 'FAIL' END AS has_seo_scope,
       CASE WHEN geo IS NOT NULL THEN 'PASS' ELSE 'FAIL' END AS has_geo_scope;

// 1.3 Schema timestamps
MATCH (n)
WHERE n:Schema OR n:Realm OR n:Layer OR n:Trait OR n:ArcFamily
WITH count(*) AS total,
     count(n.updated_at) AS with_timestamp
RETURN 'Schema Timestamps' AS check,
       with_timestamp AS complete,
       total AS total,
       CASE WHEN with_timestamp = total THEN 'PASS' ELSE 'FAIL' END AS status;

// ============================================================================
// SECTION 2: STRUCTURAL INTEGRITY VERIFICATION
// ============================================================================

// 2.1 EntityCategory completeness
MATCH (c:EntityCategory)
WITH count(c) AS category_count
MATCH (e:Entity)
OPTIONAL MATCH (e)-[:BELONGS_TO_CATEGORY]->(c:EntityCategory)
WITH category_count,
     count(e) AS entity_count,
     count(c) AS linked_count
RETURN 'Entity-Category Links' AS check,
       linked_count AS linked,
       entity_count AS total_entities,
       category_count AS category_count,
       CASE WHEN linked_count = entity_count THEN 'PASS' ELSE 'PARTIAL' END AS status;

// 2.2 BlockType completeness
MATCH (bt:BlockType)
WITH count(bt) AS type_count
MATCH (b:Block)
OPTIONAL MATCH (b)-[:OF_TYPE]->(bt:BlockType)
WITH type_count,
     count(b) AS block_count,
     count(bt) AS linked_count
RETURN 'Block-Type Links' AS check,
       linked_count AS linked,
       block_count AS total_blocks,
       type_count AS type_count,
       CASE WHEN linked_count = block_count THEN 'PASS' ELSE 'PARTIAL' END AS status;

// 2.3 Locale BCP47 property
MATCH (l:Locale)
WITH count(*) AS total,
     count(l.bcp47) AS with_bcp47
RETURN 'Locale BCP47' AS check,
       with_bcp47 AS complete,
       total AS total,
       CASE WHEN with_bcp47 = total THEN 'PASS' ELSE 'FAIL' END AS status;

// 2.4 LanguageBranch connections
MATCH (l:Locale)
OPTIONAL MATCH (l)-[:OF_BRANCH]->(lb:LanguageBranch)
WITH count(l) AS total,
     count(lb) AS linked
RETURN 'Locale-Branch Links' AS check,
       linked AS linked,
       total AS total,
       CASE WHEN linked >= total * 0.9 THEN 'PASS' ELSE 'PARTIAL' END AS status;

// 2.5 Entity descriptions
MATCH (e:Entity)
WITH count(*) AS total,
     count(e.description) AS with_desc,
     count(e.llm_context) AS with_context
RETURN 'Entity Descriptions' AS check,
       with_desc AS with_description,
       with_context AS with_llm_context,
       total AS total,
       CASE WHEN with_desc = total AND with_context = total THEN 'PASS' ELSE 'PARTIAL' END AS status;

// ============================================================================
// SECTION 3: DATA QUALITY VERIFICATION
// ============================================================================

// 3.1 FOR_LOCALE integrity (no null targets)
MATCH (n)-[r:FOR_LOCALE]->(target)
WHERE target IS NULL
WITH count(*) AS broken_count
RETURN 'FOR_LOCALE Integrity' AS check,
       broken_count AS broken_arcs,
       CASE WHEN broken_count = 0 THEN 'PASS' ELSE 'FAIL' END AS status;

// 3.2 SEOKeyword completeness
MATCH (kw:SEOKeyword)
WITH count(*) AS total,
     count(kw.locale_key) AS with_locale,
     count(CASE WHEN kw.volume > 0 THEN 1 END) AS with_volume,
     count(kw.search_intent) AS with_intent
RETURN 'SEOKeyword Data' AS check,
       with_locale AS with_locale_key,
       with_volume AS with_actual_volume,
       with_intent AS with_search_intent,
       total AS total,
       CASE WHEN with_locale = total THEN 'PASS' ELSE 'FAIL' END AS locale_status;

// 3.3 Expression locale property
MATCH (e:Expression)
WITH count(*) AS total,
     count(e.locale) AS with_locale
RETURN 'Expression Locale' AS check,
       with_locale AS with_locale_property,
       total AS total,
       CASE WHEN total > 0 THEN round(100.0 * with_locale / total, 1) ELSE 0 END AS coverage_pct,
       CASE WHEN total = 0 OR with_locale >= total * 0.95 THEN 'PASS' ELSE 'PARTIAL' END AS status;

// 3.4 Pattern completeness (fr-FR)
MATCH (p:Pattern)
WHERE p.locale = 'fr-FR'
WITH count(*) AS total,
     count(CASE WHEN p.template IS NOT NULL AND p.template <> '' THEN 1 END) AS with_template
RETURN 'fr-FR Patterns' AS check,
       with_template AS with_templates,
       total AS total,
       CASE WHEN with_template = total THEN 'PASS' ELSE 'PARTIAL' END AS status;

// ============================================================================
// SECTION 4: OVERALL CSR CALCULATION
// ============================================================================

// Calculate overall Constraint Satisfaction Rate
CALL {
  // Foundation checks
  OPTIONAL MATCH (oc:OrgConfig) WITH count(oc) > 0 AS has_orgconfig
  RETURN has_orgconfig AS foundation_check
}
CALL {
  // Schema timestamps
  MATCH (n) WHERE n:Schema OR n:Realm OR n:Layer
  WITH count(*) AS total, count(n.updated_at) AS with_ts
  RETURN CASE WHEN total = 0 THEN 1.0 WHEN with_ts = total THEN 1.0 ELSE toFloat(with_ts) / total END AS timestamp_rate
}
CALL {
  // Entity categorization
  MATCH (e:Entity)
  OPTIONAL MATCH (e)-[:BELONGS_TO_CATEGORY]->(c:EntityCategory)
  WITH count(e) AS total, count(c) AS linked
  RETURN CASE WHEN total = 0 THEN 1.0 ELSE toFloat(linked) / total END AS entity_category_rate
}
CALL {
  // FOR_LOCALE integrity
  MATCH (n)-[r:FOR_LOCALE]->(target)
  WITH count(*) AS total, count(CASE WHEN target IS NOT NULL THEN 1 END) AS valid
  RETURN CASE WHEN total = 0 THEN 1.0 ELSE toFloat(valid) / total END AS for_locale_rate
}
CALL {
  // Expression locale
  MATCH (e:Expression)
  WITH count(*) AS total, count(e.locale) AS with_locale
  RETURN CASE WHEN total = 0 THEN 1.0 ELSE toFloat(with_locale) / total END AS expression_locale_rate
}
WITH foundation_check, timestamp_rate, entity_category_rate, for_locale_rate, expression_locale_rate
RETURN 'OVERALL CSR' AS metric,
       round((timestamp_rate + entity_category_rate + for_locale_rate + expression_locale_rate) / 4 * 100, 2) AS csr_percentage,
       CASE
         WHEN (timestamp_rate + entity_category_rate + for_locale_rate + expression_locale_rate) / 4 >= 0.95 THEN 'HEALTHY ✓'
         WHEN (timestamp_rate + entity_category_rate + for_locale_rate + expression_locale_rate) / 4 >= 0.85 THEN 'WARNING ⚠'
         ELSE 'CRITICAL ✗'
       END AS health_status;

// ============================================================================
// SECTION 5: SUMMARY REPORT
// ============================================================================

// Generate final summary
RETURN '═══════════════════════════════════════════════════════' AS separator
UNION ALL
RETURN '  NOVANET DATA QUALITY VERIFICATION COMPLETE' AS separator
UNION ALL
RETURN '  Run novanet_audit(target: "all") for detailed report' AS separator
UNION ALL
RETURN '═══════════════════════════════════════════════════════' AS separator;
