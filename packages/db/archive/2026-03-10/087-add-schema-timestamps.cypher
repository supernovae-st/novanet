// ============================================================================
// PLAN A - Migration 087: Add updated_at to Schema Nodes
// ============================================================================
// Priority: FOUNDATION (Metadata completeness)
// Fixes: 227 Schema nodes missing updated_at timestamp
// CSR Impact: Enables freshness auditing and change tracking
// ============================================================================

// Add updated_at to all Schema:Class nodes
MATCH (c:Schema:Class)
WHERE c.updated_at IS NULL
SET c.updated_at = datetime();

// Add updated_at to all Schema:ArcClass nodes
MATCH (ac:Schema:ArcClass)
WHERE ac.updated_at IS NULL
SET ac.updated_at = datetime();

// Add updated_at to Realm nodes
MATCH (r:Realm)
WHERE r.updated_at IS NULL
SET r.updated_at = datetime();

// Add updated_at to Layer nodes
MATCH (l:Layer)
WHERE l.updated_at IS NULL
SET l.updated_at = datetime();

// Add updated_at to Trait nodes
MATCH (t:Trait)
WHERE t.updated_at IS NULL
SET t.updated_at = datetime();

// Add updated_at to ArcFamily nodes
MATCH (af:ArcFamily)
WHERE af.updated_at IS NULL
SET af.updated_at = datetime();

// Add created_at where missing (bootstrap completeness)
MATCH (n)
WHERE (n:Schema OR n:Realm OR n:Layer OR n:Trait OR n:ArcFamily)
  AND n.created_at IS NULL
SET n.created_at = datetime();

// Verify timestamp coverage
MATCH (n)
WHERE n:Schema OR n:Realm OR n:Layer OR n:Trait OR n:ArcFamily
WITH labels(n)[0] AS label,
     count(*) AS total,
     count(n.updated_at) AS with_updated_at,
     count(n.created_at) AS with_created_at
RETURN label,
       total,
       with_updated_at,
       with_created_at,
       CASE WHEN with_updated_at = total THEN 'COMPLETE' ELSE 'INCOMPLETE' END AS status;
