// Migration 064: Fix data nodes missing OF_CLASS arcs to Schema:Class
// Issue: Some data nodes don't have OF_CLASS relationship to their class definition
// Solution: Match nodes by label and create OF_CLASS arc to corresponding Schema:Class

// Step 1: Fix BlockNative nodes
MATCH (n:BlockNative)
WHERE NOT (n)-[:OF_CLASS]->(:Schema:Class)
MATCH (c:Schema:Class {label: 'BlockNative'})
MERGE (n)-[:OF_CLASS]->(c)
RETURN 'BlockNative' as fixed_type, count(n) as count;

// Step 2: Fix SEOKeywordSet nodes
MATCH (n:SEOKeywordSet)
WHERE NOT (n)-[:OF_CLASS]->(:Schema:Class)
MATCH (c:Schema:Class {label: 'SEOKeywordSet'})
MERGE (n)-[:OF_CLASS]->(c)
RETURN 'SEOKeywordSet' as fixed_type, count(n) as count;

// Step 3: Fix EntityNative nodes
MATCH (n:EntityNative)
WHERE NOT (n)-[:OF_CLASS]->(:Schema:Class)
MATCH (c:Schema:Class {label: 'EntityNative'})
MERGE (n)-[:OF_CLASS]->(c)
RETURN 'EntityNative' as fixed_type, count(n) as count;

// Step 4: Fix Page nodes
MATCH (n:Page)
WHERE NOT (n)-[:OF_CLASS]->(:Schema:Class)
MATCH (c:Schema:Class {label: 'Page'})
MERGE (n)-[:OF_CLASS]->(c)
RETURN 'Page' as fixed_type, count(n) as count;

// Step 5: Fix Block nodes
MATCH (n:Block)
WHERE NOT (n)-[:OF_CLASS]->(:Schema:Class)
MATCH (c:Schema:Class {label: 'Block'})
MERGE (n)-[:OF_CLASS]->(c)
RETURN 'Block' as fixed_type, count(n) as count;

// Step 6: Verify no data nodes without OF_CLASS remain
MATCH (n)
WHERE NOT n:Schema
  AND NOT (n)-[:OF_CLASS]->(:Schema:Class)
RETURN labels(n)[0] as node_type, count(n) as still_missing
ORDER BY still_missing DESC;
