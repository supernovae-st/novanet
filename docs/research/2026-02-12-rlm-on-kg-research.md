# Research Report: RLM-on-KG (Recursive Language Models on Knowledge Graphs)

**Date**: 2026-02-12
**Researcher**: Claude Opus 4.5
**Context**: NovaNet knowledge graph architecture for native content generation

## Summary

RLM-on-KG (Recursive Language Models on Knowledge Graphs) represents a paradigm shift from traditional RAG approaches. Instead of dumping large context windows into LLMs, RLM-on-KG treats knowledge graphs as explorable environments where models navigate hop-by-hop to gather targeted evidence. This approach delivers 4-6x more evidence while maintaining citation quality, making it ideal for NovaNet's multi-locale content generation architecture.

## Key Findings

### 1. WordLift RLM-on-KG Implementation

WordLift adapted the MIT RLM framework to use knowledge graphs (via GraphQL) as the exploration environment instead of a Python REPL.

**Architecture**:
```
Question -> entitySearch (seed) -> Hop Loop -> Evidence Packets -> Synthesis -> Citations + Judge
```

**Core Algorithm**:
1. Seed entity from `entitySearch(question)`
2. For each hop (budget = 5):
   - Gather EvidencePacket (~200 bytes vs 50KB articles)
   - Expand candidates via `schema:about` and `schema:mentions` relations
   - Select next entity using overlap-plus-score policy
   - Avoid revisiting same entities
3. Build compact context from all evidence packets
4. Generate answer constrained to evidence
5. Cite URLs discovered hop-by-hop

**MDP Formalization**:
- **State**: Current focus entity (IRI/name), visited set, hop index
- **Actions**: Select next entity among neighbors/candidates
- **Transition**: Next entity becomes focus
- **Budget**: Fixed hop limit (5 hops)
- **Objective**: Maximize grounded answer quality

**Benchmark Results** (150-question evaluation):

| Metric | RLM-on-KG | Simple RAG | Vanilla LLM |
|--------|-----------|------------|-------------|
| Overall | **5.813** | 4.981 | 2.897 |
| Faithfulness | 3.347 | **4.513** | 0.753 |
| On-Intent | 9.273 | 8.780 | **9.993** |
| Citation Coverage | **7.987** | 2.107 | 0.147 |
| Citation Precision | **6.233** | 2.133 | 0.000 |

**Key Delta**: +5.88 citation coverage (p < 1e-17), +4.10 citation precision (p < 1e-10)

**Source**: [WordLift Blog - Recursive Language Models on KG](https://wordlift.io/blog/en/recursive-language-models-on-kg/)

### 2. Academic Papers on Recursive KG Reasoning

#### Recursive Self-Aggregation (RSA) - arXiv 2509.26626v1 (2025)
- Evolutionary reasoning over graphs of thoughts
- Recursively aggregates multiple candidate reasoning chains
- Outperforms single-trajectory approaches via cross-proposal error correction
- Optimal aggregation size: K=2-3

#### RTQA (Recursive Temporal KG QA) - EMNLP 2025
- Decomposes temporal queries into sub-questions for recursive bottom-up reasoning
- **+1400% gain** vs RAG on complex cases
- Handles multi-hop and temporal constraints (day/month/year granularities)

#### KG as Implicit Reward Models - arXiv 2601.15160v1
- Knowledge graphs used as implicit reward models in RL pipelines
- Models learn from 1-3 hop paths and generalize to 4-5 hops
- **7.5-11.1% performance gains** on unseen multi-hop tasks

**Source**: [LLM+KG Survey arXiv 2406.08223v1](https://arxiv.org/html/2406.08223v1)

### 3. RLM vs GraphRAG Comparison

| Aspect | RLM-on-KG | GraphRAG (Microsoft) |
|--------|-----------|----------------------|
| **Graph Source** | Native RDF KG (pre-curated) | LLM-extracted from documents |
| **Query Process** | Multi-hop traversal (5 hops) | Community summaries + local retrieval |
| **Provenance** | Entity URIs + page URLs (granular) | Text chunks / community summaries |
| **Context** | ~200 bytes per hop (thin evidence) | Community-level abstractions |
| **Scale** | 10M+ tokens in enterprise | Large corpora with global queries |
| **Best For** | Pre-curated KGs, SEO, FAQs | Unstructured document collections |

**Key Insight**: RLM-on-KG excels when you have a pre-curated semantic knowledge graph. GraphRAG excels when you need to build a graph from unstructured documents.

**Source**: [GraphRAG Architecture Comparison](https://atalupadhyay.wordpress.com/2026/02/10/9-rag-architectures-every-ai-developer-must-know/)

### 4. Multi-Hop Reasoning Patterns

**CompactRAG (2025)** - arXiv 2602.05728v1:
- Decomposes multi-hop questions into dependency graphs
- Pre-generates compact QA pairs from corpora
- Traces reasoning chains via graph-guided retrieval
- Reduces token overhead vs iterative RAG

**Path-Derived Rewards**:
- Learn from 1-3 hop paths, generalize to 4-5 hops
- Rewards derived directly from KG traversals
- Provides verifiable supervision for multi-hop reasoning

**Reasoning Path Structure**:
```json
{
  "reasoningPaths": [
    {
      "path": ["entity_A", "-> relation_1 ->", "entity_B", "-> relation_2 ->", "entity_C"],
      "strength": 0.85,
      "evidence": ["Source document 1", "Source document 2"]
    }
  ]
}
```

### 5. Citation and Provenance Tracking

**TrustGraph Context Graphs** architecture for provenance:

```json
{
  "triple": {
    "subject": "Entity_A",
    "predicate": "relation_type",
    "object": "Entity_B",
    "sources": [
      {
        "type": "knowledge_base",
        "timestamp": "2024-12-01",
        "confidence": 1.0,
        "verified": true
      }
    ],
    "overallConfidence": 0.98,
    "lastVerified": "2024-12-01"
  }
}
```

**Key Provenance Properties**:
- Source attribution (database, document, API)
- Confidence scores (0.0-1.0)
- Temporal validity (validFrom, validTo)
- Verification status

**Source**: [TrustGraph Context Graphs](https://trustgraph.ai/guides/key-concepts/context-graphs/)

## RLM-on-KG Failure Mode: "Overreach"

**Critical Discovery**: RLM-on-KG accumulates 4-6x more evidence than Simple RAG, which improves coverage BUT can lead to "over-synthesis":

- **High intent coverage** (answers the question well)
- **Low faithfulness** (conclusions not entirely supported by evidence)

> "The structure gives you more material to synthesize, which is powerful, but that same richness can enable more sophisticated hallucinations if you're not careful."

**Mitigation**: Implement a "Grounding Judge" that validates faithfulness of generated content against source evidence.

## MCP Server Options for Knowledge Graphs

### 1. Neo4j MCP Server (Recommended for NovaNet)

**GitHub**: https://github.com/neo4j-contrib/mcp-neo4j

**Tools**:
- `get-neo4j-schema` - Read existing schema
- `read-neo4j-cypher` - Execute read queries
- `write-neo4j-cypher` - Execute write queries

**Setup**:
```json
{
  "mcpServers": {
    "neo4j": {
      "command": "uvx",
      "args": [
        "mcp-neo4j-cypher",
        "--db-url", "bolt://localhost:7687",
        "--username", "neo4j",
        "--password", "novanetpassword"
      ]
    }
  }
}
```

### 2. KnowledgeGraph MCP Server (n-r-w)

**GitHub**: https://github.com/n-r-w/knowledgegraph-mcp

**Features**:
- Multi-backend: PostgreSQL + SQLite
- Entities + Relations + Observations
- Fuzzy search with pagination
- Project separation (isolation)

### 3. Memory MCP Server (Anthropic Official)

**GitHub**: https://github.com/modelcontextprotocol/servers/tree/main/src/memory

**Features**:
- Official Anthropic implementation
- Simple: Entities + Relations + Observations
- JSONL storage

## NovaNet Integration Recommendations

### Architecture Alignment

NovaNet's existing architecture aligns naturally with RLM-on-KG:

| RLM-on-KG Concept | NovaNet Equivalent |
|-------------------|-------------------|
| Entity navigation | NodeKind traversal |
| Hop relations | ArcKind (114 types) |
| Evidence packets | Knowledge Atoms (Term, Expression, Pattern) |
| Thin context | Selective loading from TermSet, ExpressionSet |
| Citations | Composite keys (entity:key@locale) |

### Proposed Multi-Hop Context Loading

Instead of loading ALL entity data (50KB+), implement hop-by-hop:

```
Hop 1: Entity (invariant)           -> ~200 bytes
Hop 2: EntityContent (fr-FR)        -> ~500 bytes
Hop 3: Related Terms from TermSet   -> ~300 bytes
Hop 4: Related Expressions          -> ~300 bytes
Hop 5: Cultural context (CultureRef) -> ~200 bytes
────────────────────────────────────────────────
Total: ~1.5KB instead of 50KB
```

### Provenance Arc Properties

Add provenance tracking to generation arcs:

```yaml
arc:
  name: USES_TERM
  source: BlockGenerated
  target: Term
  properties:
    confidence: float
    hop_distance: int
    timestamp: datetime
    verified: boolean
```

### Generation Audit Trail

Track which knowledge atoms influenced generated content:

```cypher
MATCH path = (bg:BlockGenerated)-[:USES_TERM]->(t:Term)
             -[:CONTAINED_IN]->(ts:TermSet)
             -[:BELONGS_TO]->(l:Locale)
RETURN path,
       collect(t.text) as terms_used,
       collect(ts.domain) as domains
```

### Grounding Judge Implementation

Validate generated content against source evidence:

```typescript
interface GroundingJudge {
  // Validate that generated content is grounded in evidence
  validateFaithfulness(
    generated: BlockGenerated,
    evidence: EvidencePacket[]
  ): GroundingScore;

  // Check citation accuracy
  validateCitations(
    generated: BlockGenerated,
    sources: Entity[]
  ): CitationScore;
}
```

## Sources

1. [WordLift: Recursive Language Models on KG](https://wordlift.io/blog/en/recursive-language-models-on-kg/) - Primary RLM-on-KG implementation
2. [MIT RLM Paper (arXiv 2512.24601)](https://arxiv.org/abs/2512.24601) - Original RLM framework
3. [Recursive Self-Aggregation (arXiv 2509.26626v1)](https://arxiv.org/html/2509.26626v1) - RSA reasoning
4. [RTQA EMNLP 2025](https://aclanthology.org/2025.emnlp-main.499.pdf) - Temporal KG QA
5. [LLM+KG Survey (arXiv 2406.08223v1)](https://arxiv.org/html/2406.08223v1) - Comprehensive survey
6. [KG as Implicit Reward Models (arXiv 2601.15160v1)](https://arxiv.org/html/2601.15160v1) - RL with KG
7. [CompactRAG (arXiv 2602.05728v1)](https://arxiv.org/html/2602.05728v1) - Dependency graph decomposition
8. [TrustGraph Context Graphs](https://trustgraph.ai/guides/key-concepts/context-graphs/) - Provenance architecture
9. [GraphRAG Architecture](https://graphwise.ai/use-cases/graph-rag/) - Microsoft GraphRAG
10. [Neo4j MCP Server](https://github.com/neo4j-contrib/mcp-neo4j) - MCP implementation
11. [KnowledgeGraph MCP](https://github.com/n-r-w/knowledgegraph-mcp) - PostgreSQL-backed MCP
12. [Prime Intellect RLM](https://www.primeintellect.ai/blog/rlm) - RLM paradigm overview

## Methodology

- Tools used: Web search (Perplexity-style), academic paper search
- Sources analyzed: 12 primary sources, 20+ referenced papers
- Time period covered: 2024-2026 (focus on latest advances)

## Confidence Level

**High** - Multiple independent sources confirm RLM-on-KG as an emerging paradigm with proven benchmarks. WordLift implementation provides concrete evidence of effectiveness. Academic papers validate multi-hop reasoning improvements.

## Further Research Suggestions

1. **Grounding Judge Implementation**: Research RAGAS and similar evaluation frameworks for faithfulness scoring
2. **Adaptive Hop Budget**: Dynamic hop limits based on query complexity
3. **Cross-Locale Evidence**: How to aggregate evidence across multiple locales efficiently
4. **Streaming RLM**: Real-time hop exploration for interactive applications
5. **Agent-Standards-MCP**: Alternative approach for user-controlled dynamic context
