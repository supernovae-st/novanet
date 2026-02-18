// =============================================================================
// migration-coherence-08-geography.cypher
// NovaNet v0.13.1 coherence — Continent, GeoRegion, GeoSubRegion, Country
// =============================================================================
//
// SCOPE: 296 geographic nodes across 4 classes
//   - Continent    (6 instances)
//   - GeoRegion   (22 instances)
//   - GeoSubRegion (19 instances)
//   - Country     (249 instances)
//
// ALL FOUR CLASSES had the same two problems:
//   A. Orphan prop: 'name' (should be 'display_name' per YAML schema)
//   B. Missing required: 'display_name', 'description', 'created_at', 'updated_at'
//
// GeoRegion additionally had:
//   A. Orphan props: 'wikidata_id', 'cultural_notes', 'dominant_languages'
//
// Continent additionally had:
//   A. Orphan prop: 'wikidata_id'
//
// IDEMPOTENT: safe to run multiple times (all steps use WHERE guards)
//
// Rollback: If needed:
//   SET n.name = n.display_name WHERE n:Country (or :GeoRegion, :Continent)
//   REMOVE n.display_name, n.description
// =============================================================================

// =============================================================================
// STEP 1 (ALL): Rename 'name' → 'display_name'
// The 'name' property was the human-readable label in old seed files.
// YAML schema uses 'display_name' as the standard prop.
// =============================================================================

MATCH (n)
WHERE (n:Continent OR n:GeoRegion OR n:Country)
  AND n.name IS NOT NULL
  AND n.display_name IS NULL
SET n.display_name = n.name
RETURN count(n) AS display_name_set
;

// =============================================================================
// STEP 2 (ALL): Add missing 'description'
// Derive from available data (display_name + geographic context)
// =============================================================================

MATCH (n:Continent)
WHERE n.description IS NULL
SET n.description = 'Continental region: ' + n.display_name
RETURN count(n) AS continent_description_added
;

MATCH (n:GeoRegion)
WHERE n.description IS NULL
SET n.description = 'Geographic region: ' + n.display_name
RETURN count(n) AS georegion_description_added
;

MATCH (n:Country)
WHERE n.description IS NULL
SET n.description = n.display_name + ' (' + coalesce(n.alpha3, n.key) + '), ' + coalesce(n.region, 'unclassified region')
RETURN count(n) AS country_description_added
;

// =============================================================================
// STEP 3 (ALL): Add missing timestamps
// =============================================================================

MATCH (n)
WHERE (n:Continent OR n:GeoRegion OR n:Country)
  AND n.created_at IS NULL
SET n.created_at = datetime(), n.updated_at = datetime()
RETURN count(n) AS timestamps_added
;

// =============================================================================
// STEP 3B (GeoSubRegion): Add missing props
// =============================================================================

MATCH (n:GeoSubRegion)
WHERE n.description IS NULL
SET n.description = 'Geographic sub-region: ' + n.display_name,
    n.created_at  = datetime(),
    n.updated_at  = datetime()
RETURN count(n) AS georegion_sub_description_added
;

// =============================================================================
// STEP 4A (Continent, GeoRegion, GeoSubRegion): Remove orphan 'wikidata_id'
// Not in YAML schema (could be added as optional if needed)
// =============================================================================

MATCH (n)
WHERE (n:Continent OR n:GeoRegion OR n:GeoSubRegion)
  AND n.wikidata_id IS NOT NULL
REMOVE n.wikidata_id
RETURN count(n) AS wikidata_id_removed
;

// =============================================================================
// STEP 4B (GeoRegion): Remove orphan 'cultural_notes', 'dominant_languages'
// Old-format props from v10.x seed — replaced by 'cultural_style' JSON in v0.12.4
// =============================================================================

MATCH (n:GeoRegion)
WHERE n.cultural_notes IS NOT NULL OR n.dominant_languages IS NOT NULL
REMOVE n.cultural_notes, n.dominant_languages
RETURN count(n) AS georegion_extra_orphans_removed
;

// =============================================================================
// STEP 5: Remove orphan 'name' (after rename in STEP 1)
// =============================================================================

MATCH (n)
WHERE (n:Continent OR n:GeoRegion OR n:GeoSubRegion OR n:Country)
  AND n.name IS NOT NULL
REMOVE n.name
RETURN count(n) AS name_removed
;

// =============================================================================
// STEP 6: Verification — should return 0 rows if clean
// =============================================================================

MATCH (n)
WHERE n:Continent OR n:GeoRegion OR n:GeoSubRegion OR n:Country
WITH labels(n)[0] AS label, n,
  [k IN keys(n) WHERE k IN ['name', 'wikidata_id', 'cultural_notes', 'dominant_languages']] AS remaining_orphans,
  [k IN ['key', 'display_name', 'description', 'created_at', 'updated_at']
   WHERE NOT k IN keys(n)] AS still_missing
WHERE size(remaining_orphans) > 0 OR size(still_missing) > 0
RETURN label, n.key AS still_dirty, remaining_orphans, still_missing
LIMIT 10
;
