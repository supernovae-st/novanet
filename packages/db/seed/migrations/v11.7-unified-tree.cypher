// ============================================================================
// v11.7 Unified Tree Architecture - Neo4j Migration
// ============================================================================
//
// This migration adds structural relationships that enable the unified tree:
// - Realm -[:HAS_LAYER]-> Layer
// - Layer -[:HAS_KIND]-> Kind
// - ArcKind -[:BELONGS_TO_FAMILY]-> ArcFamily
//
// These relationships transform the flat meta-graph into a navigable hierarchy
// that mirrors the YAML source structure:
//
//   Realm (shared/org)
//     └── Layer (config/locale/geography/knowledge/foundation/structure/...)
//           └── Kind (Locale/Term/Page/Entity/...)
//
//   ArcFamily (ownership/localization/semantic/generation/mining)
//     └── ArcKind (HAS_PAGE/HAS_CONTENT/USES_ENTITY/...)
//
// Run with: cargo run -- db migrate
// Verify with: cargo run -- meta
// ============================================================================

// ----------------------------------------------------------------------------
// Phase 1: Create HAS_LAYER relationships (Realm → Layer)
// ----------------------------------------------------------------------------
// Links each Layer to its parent Realm based on the realm property.
// This enables tree navigation: Realm → Layers under that realm.
//
// Expected: 10 relationships total
//   - shared: config, locale, geography, knowledge (4)
//   - org: config, foundation, structure, semantic, instruction, output (6)

MATCH (r:Realm:Meta), (l:Layer:Meta)
WHERE l.realm = r.key
MERGE (r)-[:HAS_LAYER]->(l);

// ----------------------------------------------------------------------------
// Phase 2: Create HAS_KIND relationships (Layer → Kind)
// ----------------------------------------------------------------------------
// Links each Kind to its parent Layer based on realm + layer properties.
// Both realm and layer must match to ensure correct hierarchy placement.
//
// Expected: 60 relationships total
//   - shared/config: 3 kinds (EntityCategory, Locale, SEOKeywordFormat)
//   - shared/locale: 6 kinds (Culture, Style, Formatting, etc.)
//   - shared/geography: 6 kinds (Continent, Region, Country, etc.)
//   - shared/knowledge: 24 kinds (Terms, Expressions, SEO, GEO, etc.)
//   - org/config: 1 kind (OrgConfig)
//   - org/foundation: 3 kinds (Project, ProjectContent, BrandIdentity)
//   - org/structure: 3 kinds (Page, Block, ContentSlot)
//   - org/semantic: 4 kinds (Entity, EntityContent, AudiencePersona, ChannelSurface)
//   - org/instruction: 7 kinds (PageType, BlockType, prompts, etc.)
//   - org/output: 3 kinds (PageGenerated, BlockGenerated, OutputArtifact)

MATCH (l:Layer:Meta), (k:Kind:Meta)
WHERE k.layer = l.key AND k.realm = l.realm
MERGE (l)-[:HAS_KIND]->(k);

// ----------------------------------------------------------------------------
// Phase 3: Create BELONGS_TO_FAMILY relationships (ArcKind → ArcFamily)
// ----------------------------------------------------------------------------
// Links each ArcKind to its parent ArcFamily based on family property.
// This enables arc explorer navigation by family.
//
// Expected: 114 relationships total across 5 families
//   - ownership: 46 arc kinds (HAS_PAGE, HAS_BLOCK, HAS_ENTITY, etc.)
//   - localization: 15 arc kinds (HAS_CONTENT, HAS_GENERATED, etc.)
//   - semantic: 41 arc kinds (USES_ENTITY, REFERENCES, etc.)
//   - generation: 11 arc kinds (GENERATED_BY, PROMPTED_BY, etc.)
//   - mining: 1 arc kind (MINES_KEYWORD)

MATCH (af:ArcFamily:Meta), (ak:ArcKind:Meta)
WHERE ak.family = af.key
MERGE (ak)-[:BELONGS_TO_FAMILY]->(af);

// ----------------------------------------------------------------------------
// Phase 4: Create performance indexes for unified tree queries
// ----------------------------------------------------------------------------
// These indexes speed up common tree navigation patterns:
// - kind_label_idx: Fast Kind lookup by label (used in OF_KIND relationships)
// - node_key_idx: Fast node lookup by key (universal identifier pattern)
// - realm_key_idx: Fast Realm lookup for tree root
// - layer_key_idx: Fast Layer lookup for tree navigation

CREATE INDEX kind_label_idx IF NOT EXISTS FOR (k:Kind) ON (k.label);
CREATE INDEX node_key_idx IF NOT EXISTS FOR (n:Node) ON (n.key);
CREATE INDEX realm_key_idx IF NOT EXISTS FOR (r:Realm) ON (r.key);
CREATE INDEX layer_key_idx IF NOT EXISTS FOR (l:Layer) ON (l.key);
CREATE INDEX arc_family_key_idx IF NOT EXISTS FOR (af:ArcFamily) ON (af.key);

// ----------------------------------------------------------------------------
// Phase 5: Add constraint for unique node keys
// ----------------------------------------------------------------------------
// Ensures no two nodes share the same key (universal identifier).
// This is critical for key-based lookups and CRUD operations.

CREATE CONSTRAINT node_key_unique IF NOT EXISTS FOR (n:Node) REQUIRE n.key IS UNIQUE;

// ============================================================================
// Verification Queries
// ============================================================================
// Run these queries after migration to verify success.
// Execute with: cargo run -- query --cypher "..."
// ============================================================================

// ----------------------------------------------------------------------------
// V1. Check Realm → Layer relationships
// ----------------------------------------------------------------------------
// Expected output:
//   | realm  | layers |
//   |--------|--------|
//   | org    | 6      |
//   | shared | 4      |

// MATCH (r:Realm)-[hl:HAS_LAYER]->(l:Layer)
// RETURN r.key AS realm, count(l) AS layers
// ORDER BY r.key;

// ----------------------------------------------------------------------------
// V2. Check Layer → Kind relationships by realm and layer
// ----------------------------------------------------------------------------
// Expected output:
//   | realm  | layer       | kinds |
//   |--------|-------------|-------|
//   | org    | config      | 1     |
//   | org    | foundation  | 3     |
//   | org    | instruction | 7     |
//   | org    | output      | 3     |
//   | org    | semantic    | 4     |
//   | org    | structure   | 3     |
//   | shared | config      | 3     |
//   | shared | geography   | 6     |
//   | shared | knowledge   | 24    |
//   | shared | locale      | 6     |

// MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[hk:HAS_KIND]->(k:Kind)
// RETURN r.key AS realm, l.key AS layer, count(k) AS kinds
// ORDER BY r.key, l.key;

// ----------------------------------------------------------------------------
// V3. Check ArcKind → ArcFamily relationships
// ----------------------------------------------------------------------------
// Expected output:
//   | family       | arc_kinds |
//   |--------------|-----------|
//   | generation   | 11        |
//   | localization | 15        |
//   | mining       | 1         |
//   | ownership    | 46        |
//   | semantic     | 41        |

// MATCH (ak:ArcKind)-[:BELONGS_TO_FAMILY]->(af:ArcFamily)
// RETURN af.key AS family, count(ak) AS arc_kinds
// ORDER BY af.key;

// ----------------------------------------------------------------------------
// V4. Full tree verification (summary)
// ----------------------------------------------------------------------------
// Expected output:
//   | realm  | layers | kinds |
//   |--------|--------|-------|
//   | org    | 6      | 21    |
//   | shared | 4      | 39    |

// MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
// RETURN r.key AS realm, count(DISTINCT l) AS layers, count(DISTINCT k) AS kinds;

// ----------------------------------------------------------------------------
// V5. Check total relationship counts
// ----------------------------------------------------------------------------
// Expected output:
//   | relationship      | count |
//   |-------------------|-------|
//   | HAS_LAYER         | 10    |
//   | HAS_KIND          | 60    |
//   | BELONGS_TO_FAMILY | 114   |

// MATCH ()-[r:HAS_LAYER]->() RETURN 'HAS_LAYER' AS relationship, count(r) AS count
// UNION ALL
// MATCH ()-[r:HAS_KIND]->() RETURN 'HAS_KIND' AS relationship, count(r) AS count
// UNION ALL
// MATCH ()-[r:BELONGS_TO_FAMILY]->() RETURN 'BELONGS_TO_FAMILY' AS relationship, count(r) AS count;

// ============================================================================
// Rollback (if needed)
// ============================================================================
// To undo this migration, run these queries:
//
// MATCH ()-[r:HAS_LAYER]->() DELETE r;
// MATCH ()-[r:HAS_KIND]->() DELETE r;
// MATCH ()-[r:BELONGS_TO_FAMILY]->() DELETE r;
//
// DROP INDEX kind_label_idx IF EXISTS;
// DROP INDEX node_key_idx IF EXISTS;
// DROP INDEX realm_key_idx IF EXISTS;
// DROP INDEX layer_key_idx IF EXISTS;
// DROP INDEX arc_family_key_idx IF EXISTS;
// DROP CONSTRAINT node_key_unique IF EXISTS;
// ============================================================================
