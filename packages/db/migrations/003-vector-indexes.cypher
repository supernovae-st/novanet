// ═══════════════════════════════════════════════════════════════════════════════
// Migration 003: HNSW Vector Indexes (v7.8.0 - Hybrid OntologyRAG)
// ═══════════════════════════════════════════════════════════════════════════════
//
// Creates HNSW (Hierarchical Navigable Small World) vector indexes for
// semantic search over Concept, ConceptL10n, and Page embeddings.
//
// Index Configuration:
//   - Dimensions: 1536 (OpenAI text-embedding-3-small)
//   - Similarity: cosine (normalized vectors)
//   - M: 16 (connections per node, balance between recall and speed)
//   - efConstruction: 200 (build quality, higher = better recall)
//
// Usage:
//   CALL db.index.vector.queryNodes('concept_embedding', 10, $embedding)
//   YIELD node, score
//   WHERE score >= 0.7
//   RETURN node.key, score
//
// ═══════════════════════════════════════════════════════════════════════════════

// ───────────────────────────────────────────────────────────────────────────────
// Concept Embedding Index
// ───────────────────────────────────────────────────────────────────────────────
// Invariant semantic units - language-independent concepts
// Embedding source: key | display_name | description | llm_context
// ───────────────────────────────────────────────────────────────────────────────
CREATE VECTOR INDEX concept_embedding IF NOT EXISTS
FOR (c:Concept)
ON c.embedding
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine'
  }
};

// ───────────────────────────────────────────────────────────────────────────────
// ConceptL10n Embedding Index
// ───────────────────────────────────────────────────────────────────────────────
// Localized concept definitions - per-locale semantic content
// Embedding source: concept_key | title | definition | summary | purpose
// Query pattern: filter by locale after vector search
// ───────────────────────────────────────────────────────────────────────────────
CREATE VECTOR INDEX concept_l10n_embedding IF NOT EXISTS
FOR (cl:ConceptL10n)
ON cl.embedding
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine'
  }
};

// ───────────────────────────────────────────────────────────────────────────────
// Page Embedding Index
// ───────────────────────────────────────────────────────────────────────────────
// Page structures - for matching queries to page context
// Embedding source: key | display_name | description | llm_context
// ───────────────────────────────────────────────────────────────────────────────
CREATE VECTOR INDEX page_embedding IF NOT EXISTS
FOR (p:Page)
ON p.embedding
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine'
  }
};

// ───────────────────────────────────────────────────────────────────────────────
// Verification Queries
// ───────────────────────────────────────────────────────────────────────────────
// Run these to verify indexes were created:
//
// SHOW INDEXES WHERE type = 'VECTOR'
// YIELD name, labelsOrTypes, properties, state
// RETURN name, labelsOrTypes, properties, state;
//
// Check embedding coverage:
//
// MATCH (c:Concept)
// RETURN count(*) AS total,
//        count(c.embedding) AS with_embedding,
//        round(100.0 * count(c.embedding) / count(*), 1) AS coverage_pct;
