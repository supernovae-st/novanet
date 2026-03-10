// Migration 053: Invert REPRESENTS direction (ADR-028)
// Before: (Entity)-[:REPRESENTS]->(Page) - WRONG
// After:  (Page)-[:REPRESENTS]->(Entity) - CORRECT per ADR-028 Page-Entity 1:1
//
// Executed: 2026-03-07

// Step 1: Create correct arcs (Page)-[:REPRESENTS]->(Entity)
MATCH (e:Entity)-[r:REPRESENTS]->(p:Page)
MERGE (p)-[:REPRESENTS]->(e);

// Step 2: Delete incorrect arcs (Entity)-[:REPRESENTS]->(Page)
MATCH (e:Entity)-[r:REPRESENTS]->(p:Page)
DELETE r;
