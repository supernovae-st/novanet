// ═══════════════════════════════════════════════════════════════════════════════
// VECTOR INDEXES for Hybrid OntologyRAG (v10.4.0)
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

// Entity embeddings (v10.3: replaces Concept, knowledge layer)
CREATE VECTOR INDEX entity_embedding IF NOT EXISTS
FOR (e:Entity) ON (e.embedding)
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine',
    `vector.quantization.enabled`: true,
    `vector.hnsw.m`: 16,
    `vector.hnsw.ef_construction`: 100
  }
};

// EntityContent embeddings (v10.3: replaces ConceptL10n, localized layer)
CREATE VECTOR INDEX entity_l10n_embedding IF NOT EXISTS
FOR (el:EntityContent) ON (el.embedding)
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

// Entity fulltext (v10.3: replaces Concept, knowledge layer)
CREATE FULLTEXT INDEX entity_fulltext IF NOT EXISTS
FOR (e:Entity) ON EACH [e.key, e.display_name, e.description, e.llm_context];

// EntityContent fulltext (v10.3: replaces ConceptL10n, localized content)
CREATE FULLTEXT INDEX entity_l10n_fulltext IF NOT EXISTS
FOR (el:EntityContent) ON EACH [el.title, el.definition, el.summary];

// ---------------------------------------------------------------------------
// VERIFICATION
// ---------------------------------------------------------------------------

// Show all indexes and their status
SHOW INDEXES
YIELD name, type, state, populationPercent
WHERE name CONTAINS 'entity' OR name CONTAINS 'page'
RETURN name, type, state, populationPercent
ORDER BY name;
