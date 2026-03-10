// ============================================================================
// PLAN B - Migration 089: Link Entities to Categories
// ============================================================================
// Priority: STRUCTURE (Complete entity taxonomy)
// Fixes: 0/9 entities have BELONGS_TO_CATEGORY arcs
// CSR Impact: Enables semantic grouping and category-based queries
// Note: Entity keys use "entity:" prefix (e.g., entity:qr-code)
// ============================================================================

// Link qr-code to THING (core product concept)
MATCH (e:Entity {key: 'entity:qr-code'})
MATCH (c:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link custom-qr-code to FEATURE (product capability)
MATCH (e:Entity {key: 'entity:custom-qr-code'})
MATCH (c:EntityCategory {key: 'FEATURE'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link qr-code-art to FEATURE (AI art generation capability)
MATCH (e:Entity {key: 'entity:qr-code-art'})
MATCH (c:EntityCategory {key: 'FEATURE'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link dynamic-qr-code to THING (product type)
MATCH (e:Entity {key: 'entity:dynamic-qr-code'})
MATCH (c:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link static-qr-code to THING (product type)
MATCH (e:Entity {key: 'entity:static-qr-code'})
MATCH (c:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link smart-link to FEATURE (link management capability)
MATCH (e:Entity {key: 'entity:smart-link'})
MATCH (c:EntityCategory {key: 'FEATURE'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link landing-page to CONTENT_TYPE (content format)
MATCH (e:Entity {key: 'entity:landing-page'})
MATCH (c:EntityCategory {key: 'CONTENT_TYPE'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link barcode to THING (legacy product concept)
MATCH (e:Entity {key: 'entity:barcode'})
MATCH (c:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Link qr-code-generator to TOOL (the main tool)
MATCH (e:Entity {key: 'entity:qr-code-generator'})
MATCH (c:EntityCategory {key: 'TOOL'})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(c);

// Verify all entities are linked
MATCH (e:Entity)
OPTIONAL MATCH (e)-[:BELONGS_TO_CATEGORY]->(c:EntityCategory)
RETURN e.key AS entity,
       c.key AS category,
       CASE WHEN c IS NOT NULL THEN 'LINKED' ELSE 'ORPHAN' END AS status
ORDER BY e.key;
