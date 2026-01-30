# NovaNet Technical Stack v7.6.0

> **Architecture Decision Record**: Hybrid OntologyRAG for Native Content Generation

---

## Executive Summary

NovaNet uses a **Hybrid OntologyRAG** architecture that combines:
- **Neo4j Property Graph** (NOT RDF/OWL) for structured knowledge
- **Vector Search** (HNSW indexes) for semantic similarity
- **Graph Traversal** (Spreading Activation) for context expansion
- **Schema-Guided Retrieval** (YAML + Zod) for constraints

**Why this stack?**
- 30% better accuracy than standard RAG (GraphRAG research)
- 40-50% hallucination reduction with schema constraints
- Scalable to 200+ locales
- No external dependencies (vector DB, reasoner)

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                        NOVANET HYBRID ONTOLOGYRAG                               │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│   ┌─────────────────────────────────────────────────────────────────────────┐   │
│   │                         1. SCHEMA LAYER                                 │   │
│   │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                     │   │
│   │  │ models/*.yaml│  │ Zod Schemas │  │ TypeScript  │                     │   │
│   │  │ 37 nodes    │  │ Validation  │  │ Types       │                     │   │
│   │  │ 43 relations│  │             │  │             │                     │   │
│   │  └─────────────┘  └─────────────┘  └─────────────┘                     │   │
│   │                                                                         │   │
│   │  NOT OWL. Schema-guided, not reasoner-based.                           │   │
│   └─────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                            │
│                                    ▼                                            │
│   ┌─────────────────────────────────────────────────────────────────────────┐   │
│   │                         2. STORAGE LAYER                                │   │
│   │                                                                         │   │
│   │  ┌─────────────────────────────────────────────────────────────────┐   │   │
│   │  │                        NEO4J (Single DB)                        │   │   │
│   │  │                                                                 │   │   │
│   │  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐       │   │   │
│   │  │  │ Property Graph│  │ HNSW Indexes  │  │ Fulltext Index│       │   │   │
│   │  │  │ (Nodes/Edges) │  │ (Embeddings)  │  │ (Fallback)    │       │   │   │
│   │  │  └───────────────┘  └───────────────┘  └───────────────┘       │   │   │
│   │  │                                                                 │   │   │
│   │  └─────────────────────────────────────────────────────────────────┘   │   │
│   │                                                                         │   │
│   │  NOT separate vector DB. All in Neo4j.                                 │   │
│   └─────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                            │
│                                    ▼                                            │
│   ┌─────────────────────────────────────────────────────────────────────────┐   │
│   │                         3. RETRIEVAL LAYER                              │   │
│   │                                                                         │   │
│   │  Query ──► ┌────────────────┐  ┌────────────────┐  ┌────────────────┐  │   │
│   │            │ 1. Vector      │  │ 2. Graph       │  │ 3. Schema      │  │   │
│   │            │ Similarity     │──►│ Traversal     │──►│ Filtering     │  │   │
│   │            │ (Entry Point)  │  │ (Expansion)   │  │ (Constraints) │  │   │
│   │            └────────────────┘  └────────────────┘  └────────────────┘  │   │
│   │                                                                         │   │
│   │  Dual-mode: Embeddings find entry, graph expands context.              │   │
│   └─────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                            │
│                                    ▼                                            │
│   ┌─────────────────────────────────────────────────────────────────────────┐   │
│   │                         4. GENERATION LAYER                             │   │
│   │                                                                         │   │
│   │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │   │
│   │  │ Context Assembly│──►│ LLM Generation  │──►│ Output + Prov.  │         │   │
│   │  │ (Token Budget)  │  │ (Claude/GPT)    │  │ (INFLUENCED_BY) │         │   │
│   │  └─────────────────┘  └─────────────────┘  └─────────────────┘         │   │
│   │                                                                         │   │
│   │  Native generation per locale, NOT translation.                        │   │
│   └─────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                            │
│                                    ▼                                            │
│   ┌─────────────────────────────────────────────────────────────────────────┐   │
│   │                         5. FEEDBACK LAYER                               │   │
│   │                                                                         │   │
│   │  PageMetrics ──► Learning Loop ──► Adjust SEMANTIC_LINK.temperature    │   │
│   │                                                                         │   │
│   │  Self-improving: Good outputs strengthen links, bad weaken them.       │   │
│   └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Why NOT These Alternatives?

### 1. OWL (Web Ontology Language)

| Feature | OWL | NovaNet (YAML + Neo4j) |
|---------|-----|------------------------|
| **Schema Definition** | RDF/Turtle formal ontology | YAML files + Zod validation |
| **Reasoning** | External reasoner (Jena, Pellet) | Spreading activation (temperature) |
| **Inferences** | Pre-materialized | Real-time traversal |
| **Complexity** | High (2 sources of truth) | Low (single Neo4j) |
| **Performance** | Slower (reasoning overhead) | Fast (native queries) |

**Decision**: OWL's formal reasoning is unnecessary. `SEMANTIC_LINK.temperature` provides graduated inference that OWL's all-or-nothing transitivity cannot match.

### 2. Microsoft GraphRAG

| Feature | MS GraphRAG | NovaNet Hybrid |
|---------|-------------|----------------|
| **KG Source** | Auto-extracted from text | Pre-defined ontology (37 nodes) |
| **Best For** | Unknown corpus | Known domain (content gen) |
| **Extraction** | LLM-based entity extraction | Manual schema design |
| **Overhead** | High (extraction pipeline) | Low (schema already exists) |

**Decision**: NovaNet already HAS its knowledge graph. Auto-extraction would duplicate existing structure.

### 3. Separate Vector DB (Pinecone, Weaviate)

| Feature | External Vector DB | Neo4j HNSW |
|---------|-------------------|------------|
| **Sync** | Requires ETL pipeline | Same DB |
| **Consistency** | Eventually consistent | ACID |
| **Operations** | 2 systems to manage | 1 system |
| **Cost** | Additional service | Included |

**Decision**: Neo4j's built-in HNSW indexes eliminate the need for external vector storage.

---

## Core Technologies

### 1. Neo4j (Primary Database)

```
Version: 5.x+
Purpose: Property graph + Vector indexes + Fulltext
```

**Indexes**:
```cypher
-- Vector indexes (HNSW)
CREATE VECTOR INDEX concept_embedding FOR (c:Concept) ON c.embedding
OPTIONS { indexConfig: { `vector.dimensions`: 1536, `vector.similarity_function`: 'cosine' }};

-- Fulltext index (fallback)
CREATE FULLTEXT INDEX concept_fulltext FOR (c:Concept) ON EACH [c.key, c.display_name];
```

**Key Query Pattern (Spreading Activation)**:
```cypher
MATCH (c:Concept {key: $key})-[r:SEMANTIC_LINK*1..2]->(c2:Concept)
WHERE ALL(rel IN r WHERE rel.temperature >= $cutoff)
WITH c2, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= $cutoff
RETURN c2.key, activation ORDER BY activation DESC
```

### 2. OpenAI Embeddings

```
Model: text-embedding-3-small
Dimensions: 1536
Use: Concept, ConceptL10n, Page embeddings
```

### 3. LLM (Generation)

```
Primary: Claude (Anthropic)
Fallback: GPT-4 (OpenAI)
Use: Native content generation per locale
```

### 4. TypeScript + Zod

```
Runtime: Node.js 20+
Validation: Zod schemas for all node types
Type Safety: Full TypeScript coverage
```

---

## Retrieval Strategy

### Dual-Mode Retrieval

NovaNet uses **two retrieval modes** that complement each other:

| Mode | Purpose | Technology |
|------|---------|------------|
| **Semantic Entry** | Find relevant starting points | Vector similarity (HNSW) |
| **Graph Expansion** | Expand context through relationships | Graph traversal (Cypher) |

**Flow**:
1. **Vector Search** finds K most similar Concept nodes
2. **Graph Traversal** expands via `SEMANTIC_LINK` relationships
3. **Schema Filter** applies `priority`, `freshness`, `llm_context` constraints
4. **Context Assembly** builds LLM prompt within token budget

### Task-Aware Cutoffs

Different task types use different spreading activation parameters:

| TaskType | Cutoff | MaxHops | Semantic Boost |
|----------|--------|---------|----------------|
| CTA | 0.25 | 2 | urgency: 1.3, value: 1.2 |
| FAQ | 0.40 | 2 | definition: 1.3, type_of: 1.2 |
| HERO | 0.30 | 2 | is_action_on: 1.2, includes: 1.1 |
| PRICING | 0.20 | 2 | includes: 1.3, opposite: 1.1 |
| DEFAULT | 0.30 | 2 | (none) |

---

## Schema Design Principles

### 1. Invariant vs Localized

```
INVARIANT (English)     → Defined once, no locale
LOCALIZED (*L10n)       → Generated per locale, has :FOR_LOCALE
LOCALE KNOWLEDGE        → Attached TO locale (LocaleVoice, LocaleCulture)
```

### 2. Relationship Semantics

```
:HAS_L10N      → Human-curated (ConceptL10n, ProjectL10n)
:HAS_OUTPUT    → LLM-generated (PageL10n, BlockL10n)
:SEMANTIC_LINK → Concept relationships with temperature
:FOR_LOCALE    → Links localized content to target locale
```

### 3. Standard Node Properties

Every node has:
- `key`: Semantic identifier
- `display_name`: Human-readable name
- `llm_context`: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
- `priority`: critical | high | medium | low
- `freshness`: realtime | hourly | daily | static

---

## Production Optimizations

### 1. Vector Quantization (40-60% storage savings)

```cypher
CREATE VECTOR INDEX concept_embedding IF NOT EXISTS
FOR (c:Concept) ON (c.embedding)
OPTIONS {indexConfig: {
  `vector.dimensions`: 1536,
  `vector.similarity_function`: 'cosine',
  `vector.quantization.enabled`: true,
  `vector.hnsw.m`: 16,
  `vector.hnsw.ef_construction`: 100
}};
```

### 2. Fulltext Index Fallback

```cypher
CREATE FULLTEXT INDEX concept_fulltext IF NOT EXISTS
FOR (c:Concept) ON EACH [c.key, c.display_name, c.description];
```

### 3. Neo4j GraphRAG Python Integration

Consider wrapping `HybridCypherRetriever` from `neo4j-graphrag-python`:

```python
from neo4j_graphrag.retrievers import HybridCypherRetriever

retriever = HybridCypherRetriever(
    driver,
    index_name="concept_embedding",
    fulltext_index_name="concept_fulltext",
    retrieval_query="""
        MATCH (node)-[:SEMANTIC_LINK*1..2]->(related:Concept)
        WHERE ALL(rel IN relationships(path) WHERE rel.temperature >= 0.3)
        RETURN related.key, related.llm_context
    """,
    embedder=embedder,
)
```

---

## Future Roadmap

### Phase 1: Vector Infrastructure (Current)
- [ ] Add embeddings to Concept, ConceptL10n, Page
- [ ] Create HNSW indexes
- [ ] Implement VectorSearchService

### Phase 2: Task-Aware Retrieval
- [ ] TaskType enum and modifiers
- [ ] Dynamic cutoff per task
- [ ] GraphTraversalService

### Phase 3: Hybrid Retriever
- [ ] Combine vector + graph
- [ ] Score fusion (α × vector + (1-α) × graph)
- [ ] Context assembly

### Phase 4: Learning Loop
- [ ] PageMetrics feedback integration
- [ ] Temperature adjustment algorithm
- [ ] A/B testing infrastructure

---

## References

### Internal Docs
- `docs/plans/2026-01-29-hybrid-ontologyrag-implementation.md` - Full implementation plan
- `docs/orchestrator.md` - Orchestrator-subagent architecture
- `docs/spec.md` - NovaNet specification
- `models/_index.yaml` - Schema index

### External Research
- [Neo4j GraphRAG Python](https://github.com/neo4j/neo4j-graphrag-python) - Official library
- [GraphRAG Production Patterns](https://www.decodingai.com/p/designing-production-engineer-agent-graphrag) - Decoding AI
- [Context Graphs: AI's Opportunity](https://foundationcapital.com/context-graphs-ais-trillion-dollar-opportunity/) - Foundation Capital
- [Awesome-GraphRAG](https://github.com/DEEP-PolyU/Awesome-GraphRAG) - Research collection

### Validation Sources (Context7)
- `/neo4j/neo4j-graphrag-python` - 182 code snippets, Score 78.3
- `/websites/neo4j_cypher-manual_25` - 2032 code snippets, Score 89.2
- `/websites/langchain` - 20299 code snippets, Score 82

---

## Summary

**NovaNet uses Hybrid OntologyRAG:**

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                                                                 │
│   HYBRID = Vector Search + Graph Traversal + Schema Constraints                 │
│   ONTOLOGY = Our schema (37 nodes, 43 relations) guides retrieval              │
│   RAG = Retrieval-Augmented Generation for native content                      │
│                                                                                 │
│   NOT OWL (too complex, unnecessary reasoning)                                 │
│   NOT GraphRAG pure (we already have the KG)                                   │
│   NOT Vector RAG pure (missing structured context)                             │
│                                                                                 │
│   This stack is SCALABLE and FUTURE-PROOF.                                     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```
