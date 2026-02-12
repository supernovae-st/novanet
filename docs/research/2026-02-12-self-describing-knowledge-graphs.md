# Research Report: Self-Describing Knowledge Graphs for AI Agents

**Date**: 2026-02-12
**Researcher**: Claude Opus 4.5
**Focus**: Meta-node patterns, introspection capabilities, agent navigation

## Summary

Self-describing knowledge graphs enable AI agents to autonomously discover, understand, and navigate graph structures through embedded schema information. This research identifies patterns from industry implementations (Neo4j, Graphiti, Microsoft GraphRAG) and academic work, comparing them to NovaNet's current Kind/ArcKind meta-node architecture.

**Key Finding**: NovaNet's meta-node pattern is well-aligned with industry best practices. The primary enhancement opportunity is adding agent-specific introspection queries and LLM context optimization.

## Key Findings

### 1. How Graphs Can Describe Their Own Schema

**Meta-Node Pattern (NovaNet's Approach)**
NovaNet implements the canonical "Kind/Type meta-node" pattern:

```
(:Meta:Kind {label: 'Locale'})
     |
     | [:OF_KIND]
     v
(:Locale {key: 'fr-FR'})
```

This is equivalent to RDF's `rdf:type` but with richer schema information stored directly on the meta-node:
- `properties`: List of all properties the node type can have
- `required_properties`: Validation rules
- `schema_hint`: Formatted hint for LLM context
- `llm_context`: Natural language description for agents
- `context_budget`: Token allocation guidance (low/medium/high)
- `visibility`: Whether agents should surface this data

**Industry Comparison**:

| System | Schema Pattern | Self-Description Mechanism |
|--------|----------------|---------------------------|
| NovaNet | Meta-nodes with double-label `:Meta` | `llm_context`, `schema_hint` on Kind nodes |
| Neo4j | Runtime inference via `db.schema()` | Aggregates labels, properties, indexes |
| RDF/OWL | Reification, ontology classes | `owl:Class`, `rdfs:domain` as triples |
| Graphiti | Hierarchical subgraphs | Episode, semantic, community subgraphs |
| Microsoft GraphRAG | Leiden community detection | Community reports summarizing structure |

**NovaNet Strengths**:
1. Schema is queryable as first-class data (not just metadata)
2. `llm_context` provides agent-friendly descriptions
3. YAML-first architecture ensures single source of truth
4. Hierarchy: Realm -> Layer -> Kind mirrors RDF ontology depth

Source: Neo4j documentation, RDF specifications, NovaNet YAML files

### 2. Meta-Node Patterns in Production Systems

**The CRYSTAL Approach (MIT 2025)**
Agents iteratively generate subgraphs from reasoning traces, which self-organize into:
- Conceptual communities (clustered topics)
- Bridge nodes (connecting concepts)
- Scale-free networks (hub-and-spoke)

This emergent structure describes the graph's own evolution without predefined schemas.

**NovaNet's Explicit vs. CRYSTAL's Emergent**

| Aspect | NovaNet | CRYSTAL |
|--------|---------|---------|
| Schema definition | YAML upfront | Emergent from reasoning |
| Agent discovery | Query `:Meta` nodes | Traverse self-organized clusters |
| Evolution | Version-controlled YAML | Dynamic reorganization |
| Use case | Structured content generation | Exploratory reasoning |

**Recommendation**: NovaNet should add support for emergent semantic links between entities based on co-occurrence in generation contexts, similar to CRYSTAL's self-organizing patterns.

### 3. Introspection Capabilities for Agents

**What Agents Need**

1. **Schema Discovery**: "What types of nodes exist?"
2. **Property Discovery**: "What properties does a Locale have?"
3. **Relationship Discovery**: "How are Pages connected to Blocks?"
4. **Context Assembly**: "What data should I load for this generation task?"

**NovaNet's Current Introspection Queries**

```cypher
-- Schema discovery: All meta-nodes
MATCH (n:Meta) RETURN labels(n), n.key, n.llm_context

-- Property discovery: Kind properties
MATCH (k:Kind {label: 'Locale'})
RETURN k.properties, k.required_properties, k.schema_hint

-- Relationship discovery: Arc patterns
MATCH (ak:ArcKind)-[:FROM_KIND]->(source:Kind)
MATCH (ak)-[:TO_KIND]->(target:Kind)
RETURN ak.key, source.label, target.label, ak.cardinality

-- Context assembly: Spreading activation
MATCH (b:Block {key: $blockKey})
MATCH (b)-[:USES_ENTITY]->(e:Entity)
MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN e, ec
```

**Industry Best Practice: APOC Meta Schema**

```cypher
CALL apoc.meta.schema() YIELD value
RETURN value
-- Returns JSON with labels, relationships, properties, counts
```

**Recommendation**: Add a `novanet introspect` command that generates agent-friendly schema summaries combining:
- Kind/ArcKind metadata
- Instance counts per type
- Token budget estimates
- Suggested traversal patterns

### 4. RDF vs Property Graph for Self-Description

| Aspect | RDF (RDFS/OWL) | Property Graph (NovaNet) |
|--------|----------------|-------------------------|
| **Schema Storage** | Triples (s-p-o statements) | Meta-nodes with properties |
| **Inference** | Native (subclass, transitivity) | Query-time only |
| **Self-Description** | Built-in vocabulary (`rdf:type`, `owl:Class`) | Custom `:Meta` labels |
| **Agent Suitability** | Formal reasoning | Flexible traversal |
| **Evolution** | Ontology versioning complexity | YAML version control |
| **Query Language** | SPARQL | Cypher |

**RDF Strengths for Agents**:
- Reification: Statements about statements (`<s,p,o> rdf:type rdf:Statement`)
- OWL inference: Automatic subclass reasoning
- SHACL validation: Schema shapes as data

**Property Graph Strengths for Agents**:
- Direct property access (no triple decomposition)
- Relationship properties (weights, timestamps)
- Index-backed traversal (Neo4j performance)

**NovaNet's Hybrid Approach**:
NovaNet effectively implements RDF-like semantics in a Property Graph:
- `:Meta:Kind` = `owl:Class`
- `[:OF_KIND]` = `rdf:type`
- `:Meta:Trait` = `owl:ObjectProperty` constraints
- `llm_context` = `rdfs:comment` but structured for LLMs

**Recommendation**: Add SHACL-like validation rules as properties on Kind nodes:

```yaml
node:
  name: Locale
  validation_rules:
    - property: key
      pattern: "^[a-z]{2}-[A-Z]{2}$"
      message: "Key must be BCP-47 format"
```

### 5. How Agents Discover and Navigate Unknown Graphs

**Agent Discovery Patterns**

1. **Schema-First**: Query meta-nodes to understand structure before data
2. **Example-Driven**: Sample nodes, infer patterns
3. **Question-Driven**: LLM generates exploratory queries iteratively

**Microsoft GraphRAG Approach**
1. Extract entities and relationships from documents
2. Cluster with Leiden algorithm into communities
3. Generate community reports (summaries)
4. Query either locally (entity-focused) or globally (community-focused)

**Graphiti (Zep) Approach**
1. Ingest conversational + structured data
2. Organize into temporal subgraphs
3. Dual timestamps (event time + ingestion time)
4. Token-efficient retrieval formatted for LLMs

**NovaNet Current Approach**
1. Query Kind nodes for available types
2. Follow ownership arcs for structural traversal
3. Follow semantic arcs with spreading activation
4. Assembly via `context_budget` hints

**Recommended Enhancements**

```cypher
-- Agent bootstrap query: Get overview of entire schema
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
WITH r, l, collect({
  kind: k.label,
  trait: k.trait,
  llm_context: k.llm_context,
  context_budget: k.context_budget
}) AS kinds
RETURN r.key AS realm, l.key AS layer, kinds
ORDER BY r.key, l.key

-- Agent navigation: What can I traverse from here?
MATCH (k:Kind {label: $nodeType})
MATCH (ak:ArcKind)-[:FROM_KIND]->(k)
MATCH (ak)-[:TO_KIND]->(target:Kind)
RETURN ak.key AS arc, target.label AS target_type,
       ak.llm_context AS description,
       target.context_budget AS cost
ORDER BY target.context_budget DESC
```

## Patterns to Apply to NovaNet

### Pattern 1: Agent Bootstrap Schema

**Current**: Agents must query multiple meta-node types separately.

**Proposed**: Single introspection endpoint that returns complete schema context:

```yaml
# New command: novanet schema introspect --for-agent
{
  "schema_version": "11.7.0",
  "realms": ["shared", "org"],
  "layers": {...},
  "kinds": [
    {
      "label": "Locale",
      "realm": "shared",
      "layer": "config",
      "trait": "invariant",
      "llm_hint": "BCP-47 locale codes (fr-FR, en-US, ja-JP)",
      "properties_summary": "key, language_code, region, script",
      "token_budget": "~200 tokens per instance",
      "traversal_suggestions": [
        "HAS_STYLE -> Style (voice/tone)",
        "HAS_TERMS -> TermSet -> Term (vocabulary)"
      ]
    }
  ],
  "arc_families": {...},
  "total_estimated_tokens": 15000
}
```

### Pattern 2: Context Budget Optimization

**Current**: `context_budget: low/medium/high` is informative but not actionable.

**Proposed**: Add explicit token estimates and pruning strategies:

```yaml
node:
  name: EntityContent
  context_budget:
    estimate_per_instance: 500  # tokens
    pruning_strategy: "truncate_to_2000_chars"
    always_include: ["key", "title", "summary"]
    include_if_room: ["body", "metadata"]
```

### Pattern 3: Traversal Hints for Agents

**Current**: Agents must learn traversal patterns from examples.

**Proposed**: Add explicit traversal hints on ArcKind nodes:

```yaml
arc:
  name: USES_ENTITY
  traversal_hints:
    agent_recommendation: "always_follow"
    max_hops: 2
    token_multiplier: 3.5  # Each entity adds ~3.5x base tokens
    use_case: "Load entities referenced by this block for context"
```

### Pattern 4: Community Detection for Semantic Clustering

**Inspired by**: Microsoft GraphRAG's Leiden clustering.

**Proposed**: Add semantic communities that group related entities:

```cypher
// New meta-node type
(:Meta:Community {
  key: 'qr-code-generation',
  summary: 'Entities related to QR code creation and customization',
  entity_count: 45,
  llm_summary: 'This cluster covers QR code types, customization options...'
})

// Link entities to communities
(e:Entity)-[:BELONGS_TO_COMMUNITY]->(c:Community)
```

### Pattern 5: Temporal Provenance (Inspired by Graphiti)

**Current**: Nodes have `created_at`, `updated_at` but no generation history.

**Proposed**: Add generation provenance for audit and debugging:

```yaml
node:
  name: BlockGenerated
  temporal_properties:
    event_time: "When the content event occurred"
    ingestion_time: "When ingested into graph"
    generation_time: "When LLM generated this content"
    model_version: "claude-opus-4-5-20251101"
    prompt_hash: "sha256 of prompt used"
```

## Implementation Priorities

### High Priority (v11.8)

1. **Agent Bootstrap Query**: Single Cypher query returning complete schema for agent onboarding
2. **Token Budget Estimates**: Add numeric estimates to `context_budget`
3. **Traversal Hints on ArcKind**: Guide agents on which relationships to follow

### Medium Priority (v12.0)

4. **Schema Introspect Command**: `novanet schema introspect --for-agent --format=json`
5. **SHACL-like Validation Properties**: Store validation rules as meta-node properties
6. **Community Detection**: Group related entities for overview queries

### Low Priority (Future)

7. **Emergent Semantic Links**: Track entity co-occurrence in generation contexts
8. **Full Provenance Tracking**: Generation history with model versions
9. **GraphRAG-style Summaries**: Auto-generated community reports

## Sources

1. **Neo4j Documentation**: `db.schema()`, `apoc.meta.schema()`
2. **RDF Specifications**: RDF 1.1, RDFS, OWL 2, SHACL
3. **Microsoft GraphRAG**: Community detection, Leiden algorithm, local/global search
4. **Zep Graphiti**: Temporal knowledge graphs, episodic/semantic/community subgraphs
5. **MIT CRYSTAL**: Emergent graph structures from reasoning traces
6. **NovaNet Codebase**: `taxonomy.yaml`, `01-kinds.cypher`, Kind YAML files

## Confidence Level

**High** - Patterns are well-documented in production systems. NovaNet's architecture is already aligned with best practices. Recommendations are incremental improvements rather than architectural changes.

## Further Research Suggestions

1. **Graph Neural Networks on Meta-Schemas**: How can GNNs leverage Kind/ArcKind structure for embeddings?
2. **Dynamic Schema Evolution**: How do agents handle schema changes during long conversations?
3. **Multi-Agent Graph Coordination**: How do multiple agents share graph state without conflicts?
4. **Token-Aware Graph Sampling**: Algorithms for optimal subgraph selection within token budgets
