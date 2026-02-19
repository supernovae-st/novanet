# Research Report: GraphRAG and Knowledge Graph Enhancements for NovaNet

**Date**: 2026-02-19
**Author**: Claude (Research Agent)
**Focus**: Enhancing NovaNet's knowledge graph for better AI retrieval

---

## Executive Summary

This research synthesizes the latest developments in GraphRAG, knowledge graph ontologies, Neo4j+LLM integration patterns, and semantic knowledge graphs for content generation. The findings directly apply to NovaNet's architecture as a knowledge graph localization orchestrator that generates native content across 200+ locales.

**Key Takeaways**:
1. GraphRAG has matured significantly with Microsoft's v1.0 release and cost-efficient variants (LazyGraphRAG, LightRAG)
2. Community detection (Leiden algorithm) enables hierarchical retrieval - directly applicable to NovaNet's locale structure
3. MCP (Model Context Protocol) is the emerging standard for exposing knowledge graphs to LLMs
4. Neo4j's native vector embeddings + graph traversal creates a hybrid retrieval system
5. Locale-variant modeling follows established patterns: canonical entity + relationship-linked variants

---

## 1. GraphRAG: State of the Art (2024-2025)

### Microsoft GraphRAG Evolution

| Release | Feature | Impact |
|---------|---------|--------|
| July 2024 | Initial open-source release | 29,800+ GitHub stars |
| Sept 2024 | Auto-tuning | Rapid domain adaptation |
| Nov 2024 | Dynamic community selection | 77% cost reduction in global search |
| Nov 2024 | LazyGraphRAG | 0.1% computational cost vs full GraphRAG |
| Dec 2024 | Version 1.0 | Streamlined DX, 43% smaller storage |

### GraphRAG Variants for NovaNet

```
┌─────────────────────────────────────────────────────────────────────┐
│                        GraphRAG Landscape                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Full GraphRAG          LazyGraphRAG           LightRAG             │
│  ┌───────────┐         ┌───────────┐         ┌───────────┐          │
│  │ Entity    │         │ On-demand │         │ No        │          │
│  │ Extraction│         │ Loading   │         │ Community │          │
│  │ Community │         │ Local     │         │ Summaries │          │
│  │ Summaries │         │ Models    │         │           │          │
│  └───────────┘         └───────────┘         └───────────┘          │
│       │                     │                     │                 │
│  High quality          Low cost              Fast indexing          │
│  High cost             New quality standard  Lower accuracy         │
│                                                                     │
│  RECOMMENDATION FOR NOVANET: LazyGraphRAG approach                  │
│  - Entities already exist (curated)                                 │
│  - Community detection on locale clusters                           │
│  - On-demand summarization via MCP                                  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Community Detection with Leiden Algorithm

GraphRAG uses **Hierarchical Leiden** clustering for community detection:

1. Recursively clusters entities into communities
2. Generates community summaries at each level
3. Enables hierarchical retrieval (global -> local)

**NovaNet Application**:
- Level 0: Full knowledge graph
- Level 1: Entity categories (products, features, concepts)
- Level 2: Locale families (Romance, Germanic, CJK)
- Level 3: Individual locales

```cypher
// Example: Community structure for NovaNet
MATCH (e:Entity)-[:HAS_NATIVE]->(n:EntityNative)
WITH e, collect(n.locale) as locales
RETURN e.key, size(locales) as locale_coverage
ORDER BY locale_coverage DESC
```

---

## 2. Knowledge Graph Ontologies for AI Agents

### Recommended Ontology Patterns

| Pattern | Purpose | NovaNet Application |
|---------|---------|---------------------|
| **OWL (Web Ontology Language)** | Formal semantic modeling | Define Entity, EntityNative, Page relationships |
| **Schema.org** | Structured data vocabulary | SEO/GEO content schemas |
| **SKOS** | Taxonomy organization | Entity hierarchies, locale trees |
| **Custom Domain Ontology** | Application-specific | NovaNet's Block/BlockNative structure |

### Multi-Agent Ontology Design

```
┌─────────────────────────────────────────────────────────────────────┐
│               Ontology for NovaNet + Nika Integration               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  INVARIANT LAYER (NovaNet Core)                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Entity        Semantic concept, locale-independent          │   │
│  │  Page          Structure definition                          │   │
│  │  Block         Content container                             │   │
│  │  Arc           Relationship type                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           │                                        │
│                           │ HAS_NATIVE, HAS_BLOCK                  │
│                           ▼                                        │
│  VARIANT LAYER (Locale-Specific)                                   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  EntityNative  Authored locale content                       │   │
│  │  PageNative    Generated page (LLM output)                   │   │
│  │  BlockNative   Generated block content                       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  CONTEXT LAYER (For LLM Retrieval)                                 │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Community     Leiden cluster of related entities            │   │
│  │  Embedding     Vector representation (entity + text)         │   │
│  │  Summary       Pre-computed community/entity summaries       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Neo4j + LLM Integration Patterns

### Hybrid Retrieval Architecture

Neo4j in 2024-2025 supports **native vector embeddings** alongside graph traversal:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Hybrid Retrieval Pipeline                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  USER QUERY                                                         │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────┐                                                    │
│  │ Query       │  Extract entities, intent, locale                  │
│  │ Understanding│                                                   │
│  └─────────────┘                                                    │
│       │                                                             │
│       ├────────────────────┬────────────────────┐                   │
│       ▼                    ▼                    ▼                   │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐              │
│  │ Vector      │    │ Graph       │    │ Keyword     │              │
│  │ Search      │    │ Traversal   │    │ (BM25)      │              │
│  │ (Semantic)  │    │ (Structure) │    │ (Exact)     │              │
│  └─────────────┘    └─────────────┘    └─────────────┘              │
│       │                    │                    │                   │
│       └────────────────────┴────────────────────┘                   │
│                            │                                        │
│                            ▼                                        │
│                    ┌─────────────┐                                  │
│                    │ Re-ranking  │  LLM-based relevance scoring     │
│                    │ + Fusion    │                                  │
│                    └─────────────┘                                  │
│                            │                                        │
│                            ▼                                        │
│                    ┌─────────────┐                                  │
│                    │ Context     │  Format for LLM consumption      │
│                    │ Assembly    │                                  │
│                    └─────────────┘                                  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Neo4j Vector Index Implementation

```cypher
// Create vector index for entity embeddings
CREATE VECTOR INDEX entity_embeddings IF NOT EXISTS
FOR (e:Entity)
ON (e.embedding)
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine'
  }
}

// Hybrid query: vector + graph
CALL db.index.vector.queryNodes('entity_embeddings', 10, $queryEmbedding)
YIELD node AS entity, score
MATCH (entity)-[:HAS_NATIVE {locale: $locale}]->(native:EntityNative)
MATCH (entity)-[:RELATED_TO*1..2]-(related:Entity)
RETURN entity, native, collect(DISTINCT related) as related_entities, score
ORDER BY score DESC
```

### LangChain + Neo4j Pattern

```python
# Pseudo-code for Neo4j + LangChain integration
from langchain_neo4j import Neo4jGraph, GraphCypherQAChain

graph = Neo4jGraph(url="bolt://localhost:7687")

# Natural language to Cypher
chain = GraphCypherQAChain.from_llm(
    llm=claude,
    graph=graph,
    validate_cypher=True,
    allow_dangerous_requests=False
)

# Query with context
result = chain.invoke({
    "query": "What content exists for QR codes in French?",
    "locale": "fr-FR"
})
```

---

## 4. Semantic Knowledge Graphs for Content Generation

### Entity-Native Relationship Pattern

The **Variant Node Pattern** is optimal for 200+ locales:

```cypher
// Core Entity (invariant)
CREATE (entity:Entity {
  key: 'qr-code',
  category: 'feature',
  created_at: datetime()
})

// Locale Variants (authored content)
CREATE (native_en:EntityNative {
  locale: 'en-US',
  title: 'QR Code Generator',
  description: 'Create custom QR codes instantly',
  keywords: ['qr', 'barcode', 'scan']
})
CREATE (native_fr:EntityNative {
  locale: 'fr-FR',
  title: 'Generateur de Code QR',
  description: 'Creez des codes QR personnalises instantanement',
  keywords: ['qr', 'code-barres', 'scanner']
})

// Relationships with metadata
CREATE (entity)-[:HAS_NATIVE {
  authored_at: datetime(),
  quality_score: 0.95
}]->(native_en)
CREATE (entity)-[:HAS_NATIVE {
  authored_at: datetime(),
  quality_score: 0.88
}]->(native_fr)
```

### Content Generation Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│              NovaNet Content Generation Flow                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  1. CONTEXT RETRIEVAL (novanet_generate)                            │
│     ┌─────────────────────────────────────────────────────────────┐ │
│     │ Input: page_key, block_key, locale                          │ │
│     │ Output: structured context (entities, relationships, forms) │ │
│     └─────────────────────────────────────────────────────────────┘ │
│                            │                                        │
│                            ▼                                        │
│  2. GRAPH TRAVERSAL                                                 │
│     ┌─────────────────────────────────────────────────────────────┐ │
│     │ - Block -> references -> Entities                           │ │
│     │ - Entities -> HAS_NATIVE -> EntityNative (locale)           │ │
│     │ - Entities -> RELATED_TO -> Related Entities                │ │
│     │ - Page -> locale_config -> Style, Tone                      │ │
│     └─────────────────────────────────────────────────────────────┘ │
│                            │                                        │
│                            ▼                                        │
│  3. CONTEXT ASSEMBLY                                                │
│     ┌─────────────────────────────────────────────────────────────┐ │
│     │ - Structured JSON with entity data                          │ │
│     │ - Relationship context (what relates to what)               │ │
│     │ - Locale-specific style guidelines                          │ │
│     │ - Previous generation examples (few-shot)                   │ │
│     └─────────────────────────────────────────────────────────────┘ │
│                            │                                        │
│                            ▼                                        │
│  4. LLM GENERATION (via Nika agent)                                 │
│     ┌─────────────────────────────────────────────────────────────┐ │
│     │ - Prompt with full context                                  │ │
│     │ - Generate BlockNative content                              │ │
│     │ - Validate against schema                                   │ │
│     └─────────────────────────────────────────────────────────────┘ │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 5. MCP Integration Patterns

### Current MCP Landscape

| Implementation | Features | Relevance to NovaNet |
|----------------|----------|----------------------|
| **PuppyGraph MCP** | Graph as MCP endpoint, NL queries | Direct pattern for novanet-mcp |
| **Mem0/Memento** | Entity resolution, relationship traversal | Cross-session memory pattern |
| **Cognee MCP** | Document -> graph ingestion, semantic search | Content ingestion pipeline |
| **Memgraph MCP** | NL -> Cypher, 100M+ nodes scale | Large-scale query pattern |

### NovaNet MCP Enhancement Pattern

```typescript
// Enhanced MCP tool definitions for GraphRAG
const tools = [
  {
    name: "novanet_generate",
    description: "Generate context for content creation",
    inputSchema: {
      mode: "block" | "page" | "entity",
      page_key: string,
      block_key?: string,
      locale: string,
      include_related: boolean,  // NEW: include relationship context
      depth: number              // NEW: traversal depth
    }
  },
  {
    name: "novanet_search",  // NEW: semantic search tool
    description: "Search entities by semantic similarity",
    inputSchema: {
      query: string,
      locale: string,
      limit: number,
      filters: {
        category?: string,
        has_native?: boolean
      }
    }
  },
  {
    name: "novanet_community",  // NEW: community-based retrieval
    description: "Get entity community for global context",
    inputSchema: {
      entity_key: string,
      level: number  // community hierarchy level
    }
  }
]
```

---

## 6. Graph Neural Networks for Enhanced Retrieval

### GNN-RAG Architecture

GNNs provide **context-aware node scoring** based on query relevance:

```
┌─────────────────────────────────────────────────────────────────────┐
│                      GNN-RAG Pipeline                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  QUERY: "QR code features for French market"                        │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────────┐                                                │
│  │ Question Entity │  Extract: "QR code", "French"                  │
│  │ Recognition     │                                                │
│  └─────────────────┘                                                │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────────┐                                                │
│  │ Subgraph        │  2-hop neighborhood around matched entities   │
│  │ Extraction      │                                                │
│  └─────────────────┘                                                │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────────┐                                                │
│  │ GNN Reasoning   │  Score nodes by relevance + neighbor scores   │
│  │ (Message Pass)  │                                                │
│  └─────────────────┘                                                │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────────┐                                                │
│  │ Path Extraction │  Shortest paths: query entities -> answers    │
│  │ + Verbalization │                                                │
│  └─────────────────┘                                                │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────────┐                                                │
│  │ LLM Generation  │  With verbalized graph context                │
│  └─────────────────┘                                                │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Performance**: GNN-RAG achieves 8.9-15.5% improvement on multi-hop questions over baseline approaches.

---

## 7. Recommendations for NovaNet Enhancement

### Immediate Enhancements (Low Effort, High Impact)

1. **Add Vector Embeddings to Entities**
   ```cypher
   // Store embeddings on EntityNative nodes
   MATCH (n:EntityNative)
   SET n.embedding = $embedding  // 1536-dim from text-embedding-3-small
   ```

2. **Implement Hybrid Search MCP Tool**
   ```typescript
   // novanet_search: vector + graph hybrid
   async function searchEntities(query: string, locale: string) {
     const embedding = await embed(query);
     return neo4j.run(`
       CALL db.index.vector.queryNodes('entity_embeddings', 20, $embedding)
       YIELD node, score
       WHERE (node)-[:HAS_NATIVE {locale: $locale}]->()
       RETURN node, score
     `, { embedding, locale });
   }
   ```

3. **Add Relationship Context to novanet_generate**
   - Include 1-2 hop related entities
   - Provide relationship types for context

### Medium-Term Enhancements

4. **Community Detection Pipeline**
   - Run Leiden algorithm on entity relationships
   - Store community assignments as node property
   - Enable hierarchical retrieval

5. **Community Summaries**
   - Pre-compute LLM summaries for each community
   - Cache for fast global search

6. **Enhanced MCP Tools**
   - `novanet_community`: Get community context
   - `novanet_traverse`: Semantic graph navigation
   - `novanet_describe`: Entity with full relationship context

### Long-Term Vision

7. **GNN-Enhanced Retrieval**
   - Train lightweight GNN for relevance scoring
   - Multi-hop reasoning for complex queries

8. **Agentic RAG Integration**
   - Nika agents with full MCP tool access
   - Self-corrective retrieval (re-query on low confidence)
   - Dynamic context expansion

---

## 8. Implementation Priority Matrix

| Enhancement | Effort | Impact | Priority |
|-------------|--------|--------|----------|
| Vector embeddings on EntityNative | Low | High | P0 |
| Hybrid search MCP tool | Low | High | P0 |
| Relationship context in generate | Low | Medium | P1 |
| Community detection (Leiden) | Medium | High | P1 |
| Community summaries | Medium | High | P1 |
| Enhanced MCP tools | Medium | Medium | P2 |
| GNN-enhanced retrieval | High | High | P3 |
| Full agentic RAG | High | Very High | P3 |

---

## Sources

1. [Microsoft GraphRAG Project](https://www.microsoft.com/en-us/research/project/graphrag/) - Official documentation
2. [Graph RAG Guide 2025](https://salfati.group/topics/graph-rag) - Architecture and implementation
3. [RAGFlow 2024 Review](https://ragflow.io/blog/the-rise-and-evolution-of-rag-in-2024-a-year-in-review) - Evolution overview
4. [ThoughtWorks Technology Radar](https://www.thoughtworks.com/radar/techniques/graphrag) - Enterprise adoption
5. [IBM GraphRAG Overview](https://www.ibm.com/think/topics/graphrag) - Concepts and patterns
6. [arXiv Graph-RAG Overview](https://arxiv.org/html/2511.05297v1) - Academic research
7. [DataNucleus RAG Enterprise Guide](https://datanucleus.dev/rag-and-agentic-ai/what-is-rag-enterprise-guide-2025) - Enterprise patterns

---

## Methodology

- **Tools used**: Perplexity AI (sonar model) for web search
- **Queries executed**: 10 focused searches across 6 topic areas
- **Sources analyzed**: 40+ web pages, research papers, and documentation
- **Time period covered**: 2024-2025 developments

## Confidence Level

**High** - The research is based on well-documented, recent sources from Microsoft Research, Neo4j, IBM, and academic papers. The recommendations align with NovaNet's existing architecture and can be incrementally implemented.

---

## Further Research Suggestions

1. **LazyGraphRAG deep dive** - Detailed implementation patterns for cost optimization
2. **Neo4j GDS (Graph Data Science)** - Leiden algorithm implementation specifics
3. **MCP specification evolution** - Track Anthropic's MCP updates for new capabilities
4. **Entity embedding fine-tuning** - Domain-specific embeddings for localization
5. **Benchmarking** - Compare retrieval quality before/after enhancements

---

## 9. Addendum: Two-Stage Retrieval & OG-RAG Patterns (2026-02-19)

### Two-Stage Retrieval Implementation

**Research finding:** The optimal GraphRAG pipeline uses a two-stage approach:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  TWO-STAGE GRAPHRAG PIPELINE (NovaNet Implementation)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  STAGE 1: SEMANTIC SEARCH (Vector)                                          │
│  ├── Query → Embedding (fastembed or rig-core)                              │
│  ├── Vector similarity → Top-K candidates                                   │
│  └── Fast initial filtering (< 50ms)                                        │
│                                                                             │
│  STAGE 2: GRAPH EXPANSION (Structural)                                      │
│  ├── Candidates → Multi-hop neighbors (novanet_traverse)                    │
│  ├── Arc family filtering (ownership, localization, semantic)               │
│  └── Subgraph extraction for connected context                              │
│                                                                             │
│  STAGE 3: RANKING (PageRank + Relevance)                                    │
│  ├── Spreading activation with arc weights                                  │
│  ├── Query-relevance scoring                                                │
│  └── Combined rank = graph_centrality × semantic_similarity                │
│                                                                             │
│  STAGE 4: TOKEN-AWARE SELECTION                                             │
│  ├── Ranked nodes → Select within token_budget                              │
│  ├── Priority: direct evidence > related > background                       │
│  └── Output: structured context for LLM                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Ontology-Grounded RAG (OG-RAG)

**Key insight:** OG-RAG research validates NovaNet's knowledge atoms design:

| OG-RAG Concept | NovaNet Implementation |
|----------------|------------------------|
| **Hyperedge** (grouped facts) | TermSet → Terms via CONTAINS_TERM |
| **Ontology structure** | Realm → Layer → Class → Instance |
| **Spreading activation** | `novanet_traverse` with depth + arc_families |
| **Entity linking** | `novanet_search` hybrid mode |

### Spreading Activation via Arc Weights

**Proposal:** Add `retrieval_weight` to arc-class YAML:

```yaml
# arc-classes/ownership/has-native.yaml
arc:
  name: HAS_NATIVE
  family: ownership
  retrieval_weight: 1.0  # Always include in context

# arc-classes/semantic/relates-to.yaml
arc:
  name: RELATES_TO
  family: semantic
  retrieval_weight: 0.7  # Include if semantically relevant
```

**Cypher implementation:**

```cypher
// Weighted path scoring for context assembly
MATCH path = (seed:Entity {key: $key})-[r*1..3]-(target)
WITH path, target,
     reduce(score = 1.0, rel IN relationships(path) |
       score * CASE type(rel)
         WHEN 'HAS_NATIVE' THEN 1.0
         WHEN 'FOR_LOCALE' THEN 0.9
         WHEN 'RELATES_TO' THEN 0.7
         WHEN 'BELONGS_TO' THEN 0.5
         ELSE 0.3
       END
     ) AS activation_score
WHERE activation_score > 0.1
RETURN target, activation_score
ORDER BY activation_score DESC
LIMIT 50
```

### Multi-Hop Reasoning Depth Guide

| Depth | Token Cost | Use Case | Example |
|-------|------------|----------|---------|
| 1-hop | Low | Direct relationships | Entity → EntityNative |
| 2-hop | Medium | Contextual expansion | Entity → Category → Related Entities |
| 3-hop | High | Deep reasoning | Page → Block → Entity → Related → Native |

### Implementation Roadmap Update

| Enhancement | Priority | Sprint | Status |
|-------------|----------|--------|--------|
| Vector index on EntityNative | 🔴 High | v0.15 | PLANNED |
| Spreading activation weights | 🟡 Medium | v0.16 | PLANNED |
| Subgraph extraction API | 🟡 Medium | v0.16 | PLANNED |
| PageRank scoring | 🟢 Low | v0.17 | FUTURE |

### Research Sources (Addendum)

- **Microsoft GraphRAG v1.0**: Two-stage retrieval, community summarization
- **OG-RAG (Ontology-Grounded RAG)**: Hypergraph retrieval, spreading activation
- **kg-node (0xPlaygrounds)**: fastembed + neo4rs + rmcp pattern
- **Neo4j GenAI**: Native vector index + graph traversal integration

### MCP Library Research Summary

| Library | Version | Use Case | Recommendation |
|---------|---------|----------|----------------|
| **rmcp** | 0.16+ | Official MCP SDK | ⭐⭐⭐ Primary for both projects |
| **mcp-attr** | 0.0.7 | Declarative server macros | 🔄 Monitor for v0.1.0 (NovaNet) |
| **rust-mcp-schema** | 0.9.5 | Type-safe protocol types | ❌ Not needed (rmcp covers) |

**Conclusion:** NovaNet's architecture is well-aligned with GraphRAG best practices. The knowledge atoms design implements OG-RAG's hyperedge pattern. Adding vector indexing and arc weights will complete the hybrid retrieval pipeline.
