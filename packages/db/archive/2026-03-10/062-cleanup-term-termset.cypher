// Migration 062: Cleanup orphan Term/TermSet nodes (v0.17.2 YAGNI)
// Term and TermSet were removed from the schema in v0.17.2 but data persisted
// This migration removes:
// - Term data instances (35)
// - TermSet data instances (15)
// - Term/TermSet Schema:Class nodes
// - Related ArcClass nodes (CONTAINS_TERM, HAS_TERMS, TERMS_OF)
// - Related arc data

// Step 1: Delete all Term data instances and their relationships
MATCH (t:Term)
DETACH DELETE t;

// Step 2: Delete all TermSet data instances and their relationships
MATCH (ts:TermSet)
DETACH DELETE ts;

// Step 3: Delete Term Schema:Class node
MATCH (c:Schema:Class {label: 'Term'})
DETACH DELETE c;

// Step 4: Delete TermSet Schema:Class node
MATCH (c:Schema:Class {label: 'TermSet'})
DETACH DELETE c;

// Step 5: Delete CONTAINS_TERM ArcClass
MATCH (a:Schema:ArcClass {key: 'CONTAINS_TERM'})
DETACH DELETE a;

// Step 6: Delete HAS_TERMS ArcClass
MATCH (a:Schema:ArcClass {key: 'HAS_TERMS'})
DETACH DELETE a;

// Step 7: Delete TERMS_OF ArcClass
MATCH (a:Schema:ArcClass {key: 'TERMS_OF'})
DETACH DELETE a;
