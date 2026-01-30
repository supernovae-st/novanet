// ═══════════════════════════════════════════════════════════════════════════════
// Migration 002: SEMANTIC_LINK Inverse Pairs (v7.8.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
// Ensures bidirectional SEMANTIC_LINK edges exist.
// Per ontology design patterns (ontologydesignpatterns.org):
// - Undefined Inverse Relationships is an anti-pattern
// - Each semantic relationship needs explicit inverse for bidirectional traversal
//
// SEMANTIC_LINK type pairs:
//   is_action_on (0.95) ↔ has_action (0.90)
//   includes (0.85) ↔ included_in (0.80)
//   type_of (0.90) ↔ has_type (0.85)
//   requires (0.80) ↔ required_by (0.75)
//   related (0.60) ↔ related (symmetric)
//   opposite (0.40) ↔ opposite (symmetric)
//
// Temperature asymmetry captures semantic directionality.
// ═══════════════════════════════════════════════════════════════════════════════

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for is_action_on → has_action
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'is_action_on'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'has_action'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'has_action',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 0.947 ELSE 0.90 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for has_action → is_action_on
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'has_action'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'is_action_on'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'is_action_on',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 1.056 ELSE 0.95 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for includes → included_in
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'includes'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'included_in'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'included_in',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 0.941 ELSE 0.80 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for included_in → includes
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'included_in'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'includes'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'includes',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 1.0625 ELSE 0.85 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for type_of → has_type
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'type_of'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'has_type'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'has_type',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 0.944 ELSE 0.85 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for has_type → type_of
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'has_type'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'type_of'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'type_of',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 1.059 ELSE 0.90 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for requires → required_by
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'requires'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'required_by'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'required_by',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 0.9375 ELSE 0.75 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Create inverse for required_by → requires
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'required_by'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'requires'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'requires',
  temperature: CASE WHEN r.temperature IS NOT NULL THEN r.temperature * 1.067 ELSE 0.80 END
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Ensure symmetric 'related' links are bidirectional
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'related'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'related'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'related',
  temperature: r.temperature
}]->(a);

// ───────────────────────────────────────────────────────────────────────────────
// Ensure symmetric 'opposite' links are bidirectional
// ───────────────────────────────────────────────────────────────────────────────
MATCH (a:Concept)-[r:SEMANTIC_LINK {type: 'opposite'}]->(b:Concept)
WHERE NOT (b)-[:SEMANTIC_LINK {type: 'opposite'}]->(a)
MERGE (b)-[:SEMANTIC_LINK {
  type: 'opposite',
  temperature: r.temperature
}]->(a);
