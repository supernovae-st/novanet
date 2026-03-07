# Research Report: LLM-Knowledge Graph Integration Best Practices (2024-2025)

## Summary

This report synthesizes recent (2024-2025) best practices for integrating Large Language Models with Knowledge Graphs, focusing on edge/arc weighting, self-describing ontologies, relevance scoring, and GraphRAG patterns. The research reveals a convergence toward hybrid neuro-symbolic systems where KGs provide structured grounding to reduce LLM hallucinations, with bidirectional feedback loops for mutual improvement.

## Key Findings

### 1. Edge/Arc Weight Modeling for LLMs

Modern approaches emphasize **learnable confidence scores** on edges to filter noise, with three primary patterns emerging:

| Pattern | Mechanism | Source |
|---------|-----------|--------|
| **Embedding-based weighting** | Vector clustering + LLM deduplication for dynamic weights | LKD-KGC (Sun et al., 2025) |
| **Confidence-aware propagation** | MOE (Mixture-of-Experts) per-edge confidence scoring | CKG-LLMA (arXiv 2025) |
| **Prompt-driven fusion** | Multi-stage relation prompts for iterative weight evolution | AutoSchemaKG (Bai et al., 2025) |

**Key insight**: Edge weights represent relationship strength, confidence, or relevance derived from multiple sources. CKG-LLMA uses MOE to compute per-edge confidence, enabling dropout of low-confidence triplets from LLM hallucinations.

**Implementation pattern**:
```yaml
arc:
  name: HAS_NATIVE
  confidence: 0.95        # Learnable weight
  provenance: "llm"       # Source tracking
  decay_factor: 0.8       # Per-hop attenuation
```

### 2. GraphRAG Edge Scoring Patterns

Microsoft's GraphRAG (2024) constructs knowledge graphs where **edges represent relationship strength via normalized count values**, reflecting co-occurrence frequency across text units.

**Core patterns**:

| Pattern | Scoring Mechanism | Key Benefit |
|---------|-------------------|-------------|
| **Embedding Clustering** | Vector-based similarity + LLM dedup | Domain flexibility |
| **Soft Pruning** | Per-edge MLPs for scalar factors | GNN message passing optimization |
| **Cached Retrieval** | Key-value + async queries | Speed/cost efficiency |
| **Synergized RAG** | KG-LLM feedback loops | Accuracy + freshness |

**Deep GraphRAG (arXiv 2026)** introduces **DW-GRPO** with softmax temperature T for adaptive reward weights, balancing relevance, faithfulness, and efficiency in hierarchical retrieval.

**Temperature-based scoring formula**:
```
w_j(t) = softmax(slope_j / T)
```

Where `T` controls the sharpness of weight distribution across objectives.

### 3. Self-Describing Ontologies

Modern ontologies embed natural language descriptions, constraints, and examples directly into schemas, allowing LLMs to interpret structure without external mapping.

**Key patterns**:

1. **llm_context fields**: Embed textual embeddings and semantic properties directly in ontology definitions
2. **Common Metadata Ontology (CMO)**: Adds multimodal properties including text and vector embeddings of entity names
3. **SKOS-based usage hints**: USE/TRIGGERS/NOT patterns via OWL restrictions or custom properties

**Example pattern (NovaNet-compatible)**:
```yaml
arc:
  name: HAS_NATIVE
  llm_context: |
    USE: when loading locale-specific content for a defined node.
    TRIGGERS: content, native, locale, localized, l10n.
    NOT: for structure (use HAS_BLOCK), for definitions (read the invariant).
    RELATES: Entity (parent), EntityNative (locale content), FOR_LOCALE (locale link).
```

### 4. Spreading Activation for Context Assembly

The **Synapse architecture** (Jiang et al., 2026) models spreading activation through directed graphs where activation vectors propagate across episodic and semantic nodes.

**Implementation details**:

| Component | Function | Decay Mechanism |
|-----------|----------|-----------------|
| **Anchor nodes** | High-relevance starting points | Initial energy injection |
| **Lateral inhibition** | Suppress less-relevant neighbors | Priority implicit relevance |
| **Uncertainty gating** | Reject hallucinations | Confidence thresholding |
| **Distance decay** | Energy dissipation across hops | Exponential attenuation |

**Academic benchmark**: NetworkX implementation with 132 nodes, 802 edges achieved 44% F1 on 45-query benchmark.

**Token-aware optimization**:
- Retrieve only semantically relevant information
- Expand retrieved graph elements by one hop
- Combine texts into compact context for LLM

### 5. Quality Metrics (MMKG-RDS)

**MMKG-RDS** (360AI Lab, 2025) introduces multidimensional quality assessment:

| Metric | Description | Application |
|--------|-------------|-------------|
| **Complexity** | Path depth and branching factor | Training data difficulty |
| **Fidelity** | Accuracy to source knowledge | Hallucination prevention |
| **Difficulty** | Reasoning steps required | Model capability assessment |

**CSR (Constraint Satisfaction Rate)** pattern for validation:
```
CSR = satisfied_constraints / (satisfied_constraints + violated_constraints)

Thresholds:
- >= 0.95: Healthy (green)
- 0.85-0.95: Warning (yellow)
- < 0.85: Critical (red)
```

## Relevance to NovaNet

These findings directly apply to NovaNet's architecture:

| NovaNet Feature | Research Pattern | Implementation Path |
|-----------------|------------------|---------------------|
| `llm_context` in ArcClass | Self-describing ontologies | Already implemented (ADR-027) |
| `novanet_audit` CSR | MMKG-RDS quality metrics | Implemented in v0.17.0 |
| `novanet_generate` spreading | Synapse activation patterns | Partial (spreading_depth param) |
| Arc weighting | CKG-LLMA confidence scoring | Not yet implemented |

### Recommended Enhancements

1. **Arc Confidence Scores**: Add `confidence` field to ArcClass YAML
2. **Decay Factors**: Implement per-arc `decay_factor` for multi-hop traversal
3. **Temperature-based Retrieval**: Add temperature param to `novanet_assemble`
4. **Provenance Tracking**: Track data origin for confidence weighting

## Sources

### Academic Papers

1. **CKG-LLMA** (2025) - Confidence-aware KG-based Recommendation Framework with LLM Augmentation
   - URL: https://arxiv.org/html/2502.03715v1
   - Key: Learnable per-edge confidence via MOE

2. **Synapse** (Jiang et al., 2026) - Episodic-Semantic Memory Graphs with Spreading Activation
   - URL: https://arxiv.org/html/2601.02744v3
   - Key: Lateral inhibition, uncertainty gating

3. **Deep GraphRAG** (2026) - Balanced Hierarchical Retrieval with RL
   - URL: https://arxiv.org/html/2601.11144v1
   - Key: Temperature-based adaptive reward weights

4. **MMKG-RDS** (360AI Lab, 2025) - Reasoning Data Synthesis via Deep Mining
   - URL: https://arxiv.org/abs/2602.23632
   - Key: Multidimensional quality scoring (complexity, fidelity, difficulty)

5. **Spreading Activation for Document Retrieval** (2025)
   - URL: https://arxiv.org/html/2512.15922v1
   - Key: LLM-constructed text-attributed KGs with community detection

6. **LLM-empowered KG Construction Survey** (2025)
   - URL: https://arxiv.org/pdf/2510.20345
   - Key: End-to-end workflows like AutoSchemaKG, LKD-KGC

### Industry Implementations

7. **Microsoft GraphRAG** (2024)
   - URL: https://github.com/microsoft/graphrag
   - Key: Normalized count-based edge weights, Leiden community detection

8. **NVIDIA LLM-Driven KGs** (2024)
   - URL: https://developer.nvidia.com/blog/insights-techniques-and-evaluation-for-llm-driven-knowledge-graphs/
   - Key: GraphRAG vs VectorRAG comparison, cuGraph acceleration

9. **TigerGraph KG-LLM Integration** (2025)
   - URL: https://www.tigergraph.com/blog/reducing-ai-hallucinations-why-llms-need-knowledge-graphs-for-accuracy/
   - Key: Fan-in/fan-out features for edge importance

10. **FalkorDB GraphRAG** (2025)
    - URL: https://www.falkordb.com/blog/glossary/knowledge-graph-llms-graphrag/
    - Key: Real-time edge traversal with quality scores

### Conferences & Workshops

11. **TEXT2KG 2025** - LLM-Integrated Knowledge Graph Generation
    - URL: https://aiisc.ai/text2kg2025/
    - Focus: Relation extraction, hallucination mitigation

12. **KGC 2024** - Knowledge Graphs in RAG
    - URL: https://www.youtube.com/watch?v=cNui87jWYRU
    - Key: WhyHow.AI patterns for structured retrieval

## Methodology

- **Tools used**: Perplexity AI (sonar model), 6 targeted searches
- **Pages analyzed**: 25+ academic papers, industry blogs, and GitHub repos
- **Time period covered**: 2024-2025 (with some 2026 preprints)
- **Search queries**:
  1. "knowledge graph LLM integration weights"
  2. "GraphRAG edge scoring techniques"
  3. "self-describing ontologies LLM"
  4. "arc weight modeling knowledge graphs"
  5. "spreading activation context assembly"
  6. "MMKG-RDS quality metrics"

## Confidence Level

**High** - Multiple authoritative sources (arXiv, NVIDIA, Microsoft, TigerGraph) converge on similar patterns. The research represents cutting-edge work with some 2026 preprints indicating ongoing evolution.

## Further Research Suggestions

1. **ReKnoS Framework** (2025) - Super-relations for multi-hop reasoning
   - URL: https://openreview.net/forum?id=rTCJ29pkuA

2. **Agent Semantic Memory (AgentSM)** - Graph-based semantic memory with momentum-aware consolidation
   - URL: https://www.emergentmind.com/topics/agent-semantic-memory-agentsm

3. **LLM4VKG** (IJCAI 2025) - LLMs for Virtual Knowledge Graph construction
   - URL: https://www.ijcai.org/proceedings/2025/0525.pdf

4. **Semantic-KG** (NeurIPS 2025) - KG benchmarks for semantic similarity
   - URL: https://neurips.cc/virtual/2025/poster/121506
