// Migration 054: Fix missing Schema:Class and Schema:ArcClass nodes
//
// Problem: Term, TermSet classes and their arc classes were missing from Neo4j
// Root cause: Unknown - seed file contains them but they weren't created
//
// Executed: 2026-03-07

// Fix 1: Create Term Schema:Class
MERGE (c_Term:Schema:Class {label: 'Term'})
ON CREATE SET
  c_Term.key = 'term',
  c_Term.name = 'Term',
  c_Term.realm = 'shared',
  c_Term.layer = 'knowledge',
  c_Term.trait = 'imported',
  c_Term.display_name = 'Term',
  c_Term.description = 'Class: Term',
  c_Term.llm_context = 'Individual technical term belonging to a locale TermSet.'
ON MATCH SET
  c_Term.name = 'Term';

// Fix 2: Create TermSet Schema:Class
MERGE (c_TermSet:Schema:Class {label: 'TermSet'})
ON CREATE SET
  c_TermSet.key = 'term-set',
  c_TermSet.name = 'TermSet',
  c_TermSet.realm = 'shared',
  c_TermSet.layer = 'knowledge',
  c_TermSet.trait = 'defined',
  c_TermSet.display_name = 'TermSet',
  c_TermSet.description = 'Class: TermSet',
  c_TermSet.llm_context = 'Container for all Term atoms of a locale.'
ON MATCH SET
  c_TermSet.name = 'TermSet';

// Fix 3: Create CONTAINS_TERM ArcClass
MERGE (a1:Schema:ArcClass {key: 'CONTAINS_TERM'})
ON CREATE SET
  a1.name = 'CONTAINS_TERM',
  a1.family = 'ownership',
  a1.scope = 'intra_realm',
  a1.cardinality = 'one_to_many',
  a1.description = 'ArcClass: CONTAINS_TERM',
  a1.display_name = 'Contains Term',
  a1.llm_context = 'TermSet contains individual Term atoms.',
  a1.cypher_pattern = '(TermSet)-[:CONTAINS_TERM]->(Term)',
  a1.created_at = datetime(),
  a1.updated_at = datetime()
ON MATCH SET
  a1.name = 'CONTAINS_TERM';

// Fix 4: Create HAS_TERMS ArcClass
MERGE (a2:Schema:ArcClass {key: 'HAS_TERMS'})
ON CREATE SET
  a2.name = 'HAS_TERMS',
  a2.family = 'ownership',
  a2.scope = 'intra_realm',
  a2.cardinality = 'one_to_one',
  a2.description = 'ArcClass: HAS_TERMS',
  a2.display_name = 'Has Terms',
  a2.llm_context = 'Locale owns a TermSet container.',
  a2.cypher_pattern = '(Locale)-[:HAS_TERMS]->(TermSet)',
  a2.created_at = datetime(),
  a2.updated_at = datetime()
ON MATCH SET
  a2.name = 'HAS_TERMS';

// Fix 5: Create TERMS_OF ArcClass
MERGE (a3:Schema:ArcClass {key: 'TERMS_OF'})
ON CREATE SET
  a3.name = 'TERMS_OF',
  a3.family = 'ownership',
  a3.scope = 'intra_realm',
  a3.cardinality = 'many_to_one',
  a3.description = 'ArcClass: TERMS_OF',
  a3.display_name = 'Terms Of',
  a3.llm_context = 'Inverse of HAS_TERMS. TermSet belongs to Locale.',
  a3.cypher_pattern = '(TermSet)-[:TERMS_OF]->(Locale)',
  a3.created_at = datetime(),
  a3.updated_at = datetime()
ON MATCH SET
  a3.name = 'TERMS_OF';

// Fix 6: Ensure all Schema:Class have name property
MATCH (c:Schema:Class) WHERE c.name IS NULL AND c.description IS NOT NULL
SET c.name = REPLACE(c.description, 'Class: ', '');

// Fix 7: Ensure all Schema:ArcClass have name property
MATCH (a:Schema:ArcClass) WHERE a.name IS NULL AND a.key IS NOT NULL
SET a.name = a.key;
