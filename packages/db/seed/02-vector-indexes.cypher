// ═══════════════════════════════════════════════════════════════════════════════
// VECTOR INDEXES for Hybrid OntologyRAG (v7.8.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
// Prerequisites:
//   - Neo4j 5.11+ (vector index support)
//   - Nodes with embedding property (1536-dimensional vectors)
//
// RUN: docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < seed/02-vector-indexes.cypher
// ═══════════════════════════════════════════════════════════════════════════════

// ---------------------------------------------------------------------------
// VECTOR INDEXES (HNSW with quantization)
// ---------------------------------------------------------------------------

// Concept embeddings (invariant layer)
CREATE VECTOR INDEX concept_embedding IF NOT EXISTS
FOR (c:Concept) ON (c.embedding)
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine',
    `vector.quantization.enabled`: true,
    `vector.hnsw.m`: 16,
    `vector.hnsw.ef_construction`: 100
  }
};

// ConceptL10n embeddings (localized layer)
CREATE VECTOR INDEX concept_l10n_embedding IF NOT EXISTS
FOR (cl:ConceptL10n) ON (cl.embedding)
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine',
    `vector.quantization.enabled`: true,
    `vector.hnsw.m`: 16,
    `vector.hnsw.ef_construction`: 100
  }
};

// Page embeddings (structure layer)
CREATE VECTOR INDEX page_embedding IF NOT EXISTS
FOR (p:Page) ON (p.embedding)
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine',
    `vector.quantization.enabled`: true,
    `vector.hnsw.m`: 16,
    `vector.hnsw.ef_construction`: 100
  }
};

// ---------------------------------------------------------------------------
// FULLTEXT INDEXES (fallback for keyword search)
// ---------------------------------------------------------------------------

// Concept fulltext (invariant - English keys and context)
CREATE FULLTEXT INDEX concept_fulltext IF NOT EXISTS
FOR (c:Concept) ON EACH [c.key, c.display_name, c.description, c.llm_context];

// ConceptL10n fulltext (localized content)
CREATE FULLTEXT INDEX concept_l10n_fulltext IF NOT EXISTS
FOR (cl:ConceptL10n) ON EACH [cl.title, cl.definition, cl.summary];

// ---------------------------------------------------------------------------
// VERIFICATION
// ---------------------------------------------------------------------------

// Show all indexes and their status
SHOW INDEXES
YIELD name, type, state, populationPercent
WHERE name CONTAINS 'concept' OR name CONTAINS 'page'
RETURN name, type, state, populationPercent
ORDER BY name;
