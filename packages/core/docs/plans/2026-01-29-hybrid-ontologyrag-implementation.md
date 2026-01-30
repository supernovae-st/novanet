# NovaNet Hybrid OntologyRAG Implementation Plan

> **For Claude/LLMs:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task.
>
> **For Developers:** This is the master plan for implementing Hybrid OntologyRAG in NovaNet. Each task is self-contained with acceptance criteria.

**Document Version:** 1.1.0
**Created:** 2026-01-29
**Updated:** 2026-01-29
**Status:** APPROVED FOR IMPLEMENTATION

---

## Executive Summary

### What We're Building

A **Hybrid OntologyRAG** system that combines:
1. **Vector Search** - Semantic similarity on Concept/ConceptL10n embeddings
2. **Graph Traversal** - Spreading activation via SEMANTIC_LINK
3. **Schema Constraints** - Ontology-guided filtering (llm_context, priority, freshness)
4. **Learning Loop** - Temperature adjustment from PageMetrics feedback

### Why Hybrid (Not OWL, Not Pure GraphRAG)

| Approach | NovaNet Fit | Reason |
|----------|-------------|--------|
| **OWL Formal Ontology** | 50/100 | Unnecessary complexity. YAML + SEMANTIC_LINK.temperature provides graduated inference. |
| **Microsoft GraphRAG** | 65/100 | Designed for TEXT → KG extraction. NovaNet already HAS its ontology. |
| **Pure OntologyRAG** | 85/100 | Good fit but lacks cross-locale semantic search |
| **Hybrid OntologyRAG** | 95/100 | Best of both: schema-guided + vector + graph |

**Research findings (2025-2026):**
- GraphRAG: 30% better than vanilla RAG on multi-hop queries
- OntologyRAG: 15-30% better than GraphRAG on structured domains
- Hybrid: 40-50% hallucination reduction with schema constraints

### References

| Document | Location | Relevance |
|----------|----------|-----------|
| Technical Stack | `docs/STACK.md` | Architecture decisions |
| NovaNet Spec | `docs/spec.md` | Current graph structure |
| Model Index | `models/_index.yaml` | 37 nodes, 43 relations |
| Relations | `models/relations.yaml` | SEMANTIC_LINK definition |
| Orchestrator | `docs/orchestrator.md` | Orchestrator-subagent pattern |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                        NOVANET HYBRID RAG ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─────────────┐    ┌─────────────────────────────────────────────────────┐    │
│  │   QUERY     │    │                    RETRIEVAL LAYER                   │    │
│  │             │    ├─────────────────────────────────────────────────────┤    │
│  │  "Generate  │───▶│                                                     │    │
│  │   CTA for   │    │   1. VECTOR SEARCH (semantic similarity)            │    │
│  │   tier-pro  │    │      └── Concept.embedding HNSW index               │    │
│  │   fr-FR"    │    │      └── ConceptL10n.embedding HNSW index           │    │
│  │             │    │                        ↓                            │    │
│  └─────────────┘    │   2. GRAPH TRAVERSAL (spreading activation)         │    │
│                     │      └── SEMANTIC_LINK with temperature             │    │
│                     │      └── Task-aware cutoff (0.2-0.5)                │    │
│                     │      └── 2-hop max                                  │    │
│                     │                        ↓                            │    │
│                     │   3. SCHEMA CONSTRAINTS (ontology filtering)        │    │
│                     │      └── llm_context validation                     │    │
│                     │      └── priority budgeting                         │    │
│                     │      └── freshness check                            │    │
│                     │                        ↓                            │    │
│                     │   4. CONTEXT ASSEMBLY                               │    │
│                     │      └── Ranked by: activation × vector_sim         │    │
│                     │      └── Top-K (token budget)                       │    │
│                     │      └── LocaleKnowledge injection                  │    │
│                     └─────────────────────────────────────────────────────┘    │
│                                              ↓                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                         LLM GENERATION                                   │   │
│  │   Context: Concepts + ConceptL10n + Expressions + LocaleVoice + ...     │   │
│  │   Schema hints: llm_context format "USE:... TRIGGERS:... NOT:..."       │   │
│  │   Output: BlockL10n with INFLUENCED_BY provenance                       │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                              ↓                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                         FEEDBACK LOOP (Phase 4)                          │   │
│  │   PageMetrics → Adjust SEMANTIC_LINK.temperature                        │   │
│  │   Human edits → Learn from corrections                                  │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Implementation Phases

| Phase | Name | Tasks | Dependencies |
|-------|------|-------|--------------|
| 1 | Vector Infrastructure | 1.1 - 1.4 | None |
| 2 | Task-Aware Retrieval | 2.1 - 2.2 | Phase 1 |
| 3 | Hybrid Retriever | 3.1 - 3.2 | Phase 2 |
| 4 | Learning Loop | 4.1 - 4.2 | Phase 3 + PageMetrics |

---

# PHASE 1: Vector Infrastructure

> **Goal:** Add embeddings to Concept, ConceptL10n, Page with HNSW indexes

## Task 1.1: Update Model Definitions

**Files to modify:**
- `models/nodes/content/concept.yaml`
- `models/nodes/content/concept-l10n.yaml`
- `models/nodes/content/page.yaml`

**Changes for each file:**
```yaml
properties:
  # ... existing properties ...

  # NEW: Vector embedding for semantic search
  embedding:
    type: vector
    dimensions: 1536  # OpenAI text-embedding-3-small
    required: false
    description: "Semantic embedding for hybrid retrieval"

  embedding_source:
    type: string
    required: false
    description: "Text used to generate embedding (for debugging)"

  embedding_updated_at:
    type: datetime
    required: false
```

**Acceptance Criteria:**
- [ ] `concept.yaml` has embedding properties
- [ ] `concept-l10n.yaml` has embedding properties
- [ ] `page.yaml` has embedding properties
- [ ] `npm run validate` passes

---

## Task 1.2: Create Vector Indexes

**File:** `neo4j/seed/02-vector-indexes.cypher`

**Content:**
```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// VECTOR INDEXES for Hybrid OntologyRAG
// ═══════════════════════════════════════════════════════════════════════════════

// Concept embeddings (invariant layer) - WITH QUANTIZATION
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

// ConceptL10n embeddings (localized layer) - WITH QUANTIZATION
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

// Page embeddings (structure layer) - WITH QUANTIZATION
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

// ═══════════════════════════════════════════════════════════════════════════════
// FULLTEXT INDEXES (fallback for keyword search)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE FULLTEXT INDEX concept_fulltext IF NOT EXISTS
FOR (c:Concept) ON EACH [c.key, c.display_name, c.description];

CREATE FULLTEXT INDEX concept_l10n_fulltext IF NOT EXISTS
FOR (cl:ConceptL10n) ON EACH [cl.title, cl.definition];
```

**Optimization Notes:**
- `vector.quantization.enabled: true` provides 40-60% storage savings
- `vector.hnsw.m: 16` is optimal for recall/speed balance
- `vector.hnsw.ef_construction: 100` balances build time and accuracy
- Fulltext indexes provide fallback for exact keyword matches

**Acceptance Criteria:**
- [ ] All 3 vector indexes created successfully
- [ ] All 2 fulltext indexes created successfully
- [ ] `SHOW INDEXES` shows all indexes as "ONLINE"
- [ ] Test quantization is active: `SHOW INDEXES YIELD * WHERE type = 'VECTOR'`

---

## Task 1.3: Embedding Generation Script

**File:** `scripts/generate-embeddings.ts`

```typescript
import { OpenAI } from 'openai';
import { getDriver } from '../src/db/client';

const openai = new OpenAI();
const BATCH_SIZE = 100;
const MODEL = 'text-embedding-3-small';

interface EmbeddingTarget {
  nodeType: 'Concept' | 'ConceptL10n' | 'Page';
  keyField: string;
  textBuilder: (node: Record<string, unknown>) => string;
}

const TARGETS: EmbeddingTarget[] = [
  {
    nodeType: 'Concept',
    keyField: 'key',
    textBuilder: (n) => `${n.key}: ${n.llm_context || ''} ${n.description || ''}`,
  },
  {
    nodeType: 'ConceptL10n',
    keyField: 'id',
    textBuilder: (n) => `${n.title || ''}. ${n.definition || ''}`,
  },
  {
    nodeType: 'Page',
    keyField: 'key',
    textBuilder: (n) => `${n.key}: ${n.llm_context || ''} ${n.description || ''}`,
  },
];

async function generateEmbeddings(target: EmbeddingTarget): Promise<void> {
  const driver = getDriver();
  const session = driver.session();

  try {
    // Load nodes without embeddings
    const result = await session.run(`
      MATCH (n:${target.nodeType})
      WHERE n.embedding IS NULL
      RETURN n.${target.keyField} AS key, n
      LIMIT 1000
    `);

    const nodes = result.records.map(r => ({
      key: r.get('key'),
      data: r.get('n').properties,
    }));

    if (nodes.length === 0) {
      console.log(`[${target.nodeType}] All nodes have embeddings`);
      return;
    }

    console.log(`[${target.nodeType}] Generating embeddings for ${nodes.length} nodes`);

    // Batch embed
    for (let i = 0; i < nodes.length; i += BATCH_SIZE) {
      const batch = nodes.slice(i, i + BATCH_SIZE);
      const texts = batch.map(n => target.textBuilder(n.data));

      const response = await openai.embeddings.create({
        model: MODEL,
        input: texts,
      });

      // Update nodes
      for (let j = 0; j < batch.length; j++) {
        const node = batch[j];
        const embedding = response.data[j].embedding;
        const source = texts[j];

        await session.run(`
          MATCH (n:${target.nodeType} {${target.keyField}: $key})
          SET n.embedding = $embedding,
              n.embedding_source = $source,
              n.embedding_updated_at = datetime()
        `, { key: node.key, embedding, source });
      }

      console.log(`[${target.nodeType}] Processed ${Math.min(i + BATCH_SIZE, nodes.length)}/${nodes.length}`);
    }
  } finally {
    await session.close();
  }
}

async function main() {
  console.log('Starting embedding generation...');
  for (const target of TARGETS) {
    await generateEmbeddings(target);
  }
  console.log('Done!');
}

main().catch(console.error);
```

**Acceptance Criteria:**
- [ ] Script runs without errors
- [ ] All Concept nodes have embeddings
- [ ] All ConceptL10n nodes have embeddings
- [ ] All Page nodes have embeddings
- [ ] `embedding_source` populated for debugging

---

## Task 1.4: Vector Search Service

**File:** `src/services/vector-search.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { OpenAI } from 'openai';

export interface VectorMatch {
  key: string;
  score: number;
  nodeType: string;
  properties: Record<string, unknown>;
}

export interface VectorSearchOptions {
  limit?: number;
  threshold?: number;
  locale?: string;
}

export class VectorSearchService {
  private openai = new OpenAI();

  constructor(private driver: Driver) {}

  async embedQuery(text: string): Promise<number[]> {
    const response = await this.openai.embeddings.create({
      model: 'text-embedding-3-small',
      input: text,
    });
    return response.data[0].embedding;
  }

  async searchConcepts(
    queryOrEmbedding: string | number[],
    options: VectorSearchOptions = {}
  ): Promise<VectorMatch[]> {
    const { limit = 10, threshold = 0.7 } = options;
    const embedding = typeof queryOrEmbedding === 'string'
      ? await this.embedQuery(queryOrEmbedding)
      : queryOrEmbedding;

    const session = this.driver.session();
    try {
      const result = await session.run(`
        CALL db.index.vector.queryNodes('concept_embedding', $limit, $embedding)
        YIELD node AS c, score
        WHERE score >= $threshold
        RETURN c.key AS key,
               score,
               'Concept' AS nodeType,
               c { .key, .display_name, .llm_context, .priority } AS properties
        ORDER BY score DESC
      `, { limit, threshold, embedding });

      return result.records.map(r => ({
        key: r.get('key'),
        score: r.get('score'),
        nodeType: r.get('nodeType'),
        properties: r.get('properties'),
      }));
    } finally {
      await session.close();
    }
  }

  async searchConceptL10n(
    queryOrEmbedding: string | number[],
    locale: string,
    options: VectorSearchOptions = {}
  ): Promise<VectorMatch[]> {
    const { limit = 10, threshold = 0.7 } = options;
    const embedding = typeof queryOrEmbedding === 'string'
      ? await this.embedQuery(queryOrEmbedding)
      : queryOrEmbedding;

    const session = this.driver.session();
    try {
      const result = await session.run(`
        CALL db.index.vector.queryNodes('concept_l10n_embedding', $limit * 2, $embedding)
        YIELD node AS cl, score
        MATCH (cl)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        WHERE score >= $threshold
        RETURN cl.title AS key,
               score,
               'ConceptL10n' AS nodeType,
               cl { .title, .definition, .summary } AS properties
        ORDER BY score DESC
        LIMIT $limit
      `, { limit, threshold, embedding, locale });

      return result.records.map(r => ({
        key: r.get('key'),
        score: r.get('score'),
        nodeType: r.get('nodeType'),
        properties: r.get('properties'),
      }));
    } finally {
      await session.close();
    }
  }
}
```

**Acceptance Criteria:**
- [ ] `VectorSearchService` class implemented
- [ ] `embedQuery()` works with OpenAI
- [ ] `searchConcepts()` returns ranked results
- [ ] `searchConceptL10n()` filters by locale
- [ ] Unit tests pass

---

# PHASE 2: Task-Aware Retrieval

> **Goal:** Add task-type modifiers that adjust spreading activation behavior

## Task 2.1: Define Task Types and Modifiers

**File:** `src/types/task-types.ts`

```typescript
export enum TaskType {
  CTA = 'cta',
  FAQ = 'faq',
  HERO = 'hero',
  TESTIMONIAL = 'testimonial',
  PRICING = 'pricing',
  DEFAULT = 'default',
}

export interface TaskModifier {
  semanticBoost: Record<string, number>;
  cutoff: number;
  maxHops: number;
  priorityFilter: string[];
}

export const TASK_MODIFIERS: Record<TaskType, TaskModifier> = {
  [TaskType.CTA]: {
    semanticBoost: { urgency: 1.3, value: 1.2, action: 1.1 },
    cutoff: 0.25,
    maxHops: 2,
    priorityFilter: ['critical', 'high'],
  },
  [TaskType.FAQ]: {
    semanticBoost: { definition: 1.3, type_of: 1.2 },
    cutoff: 0.4,
    maxHops: 2,
    priorityFilter: ['critical', 'high', 'medium'],
  },
  [TaskType.HERO]: {
    semanticBoost: { is_action_on: 1.2, includes: 1.1 },
    cutoff: 0.3,
    maxHops: 2,
    priorityFilter: ['critical', 'high'],
  },
  [TaskType.TESTIMONIAL]: {
    semanticBoost: { related: 1.2 },
    cutoff: 0.35,
    maxHops: 1,
    priorityFilter: ['critical', 'high', 'medium'],
  },
  [TaskType.PRICING]: {
    semanticBoost: { includes: 1.3, type_of: 1.2, opposite: 1.1 },
    cutoff: 0.2,
    maxHops: 2,
    priorityFilter: ['critical', 'high'],
  },
  [TaskType.DEFAULT]: {
    semanticBoost: {},
    cutoff: 0.3,
    maxHops: 2,
    priorityFilter: ['critical', 'high', 'medium'],
  },
};
```

**Acceptance Criteria:**
- [ ] `TaskType` enum defined
- [ ] `TaskModifier` interface defined
- [ ] `TASK_MODIFIERS` configuration complete
- [ ] Exported from `src/index.ts`

---

## Task 2.2: Graph Traversal Service

**File:** `src/services/graph-traversal.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { TaskType, TaskModifier, TASK_MODIFIERS } from '../types/task-types';

export interface ActivationResult {
  key: string;
  activation: number;
  path: string[];
  conceptData: Record<string, unknown>;
}

export class GraphTraversalService {
  constructor(private driver: Driver) {}

  async spreadingActivation(
    seedConcepts: string[],
    taskType: TaskType = TaskType.DEFAULT
  ): Promise<ActivationResult[]> {
    const modifier = TASK_MODIFIERS[taskType];
    const session = this.driver.session();

    try {
      const result = await session.run(`
        UNWIND $seeds AS seedKey
        MATCH (seed:Concept {key: seedKey})
        MATCH path = (seed)-[r:SEMANTIC_LINK*1..$maxHops]->(c2:Concept)
        WHERE ALL(rel IN r WHERE rel.temperature >= $cutoff)
          AND c2.priority IN $priorityFilter
        WITH c2,
             [node IN nodes(path) | node.key] AS pathKeys,
             reduce(a = 1.0, rel IN r |
               a * rel.temperature *
               CASE
                 WHEN rel.type IN keys($boosts) THEN $boosts[rel.type]
                 ELSE 1.0
               END
             ) AS activation
        WHERE activation >= $cutoff
        RETURN c2.key AS key,
               activation,
               pathKeys AS path,
               c2 { .key, .display_name, .llm_context, .priority } AS conceptData
        ORDER BY activation DESC
      `, {
        seeds: seedConcepts,
        maxHops: modifier.maxHops,
        cutoff: modifier.cutoff,
        priorityFilter: modifier.priorityFilter,
        boosts: modifier.semanticBoost,
      });

      return result.records.map(r => ({
        key: r.get('key'),
        activation: r.get('activation'),
        path: r.get('path'),
        conceptData: r.get('conceptData'),
      }));
    } finally {
      await session.close();
    }
  }
}
```

**Acceptance Criteria:**
- [ ] `GraphTraversalService` class implemented
- [ ] `spreadingActivation()` uses task modifiers
- [ ] Semantic boosts applied correctly
- [ ] Priority filtering works
- [ ] Unit tests pass

---

# PHASE 3: Hybrid Retriever

> **Goal:** Combine vector search and graph traversal into unified retrieval

## Task 3.1: Hybrid Retriever

**File:** `src/services/hybrid-retriever.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { VectorSearchService, VectorMatch } from './vector-search';
import { GraphTraversalService, ActivationResult } from './graph-traversal';
import { TaskType, TASK_MODIFIERS } from '../types/task-types';

export interface HybridResult {
  key: string;
  score: number;
  source: 'vector' | 'graph' | 'both';
  vectorScore?: number;
  graphActivation?: number;
  conceptData: Record<string, unknown>;
}

export interface HybridRetrieverOptions {
  alpha?: number; // Weight for vector vs graph (0.5 = equal)
  vectorLimit?: number;
  vectorThreshold?: number;
}

export class HybridRetriever {
  private vectorService: VectorSearchService;
  private graphService: GraphTraversalService;

  constructor(driver: Driver) {
    this.vectorService = new VectorSearchService(driver);
    this.graphService = new GraphTraversalService(driver);
  }

  async retrieve(
    query: string,
    seedConcepts: string[],
    locale: string,
    taskType: TaskType = TaskType.DEFAULT,
    options: HybridRetrieverOptions = {}
  ): Promise<HybridResult[]> {
    const { alpha = 0.5, vectorLimit = 20, vectorThreshold = 0.6 } = options;

    // 1. Vector search for semantic entry points
    const vectorResults = await this.vectorService.searchConcepts(query, {
      limit: vectorLimit,
      threshold: vectorThreshold,
    });

    // 2. Graph traversal from seed concepts
    const graphResults = await this.graphService.spreadingActivation(
      seedConcepts,
      taskType
    );

    // 3. Merge and score
    const merged = this.mergeResults(vectorResults, graphResults, alpha);

    return merged;
  }

  private mergeResults(
    vectorResults: VectorMatch[],
    graphResults: ActivationResult[],
    alpha: number
  ): HybridResult[] {
    const resultMap = new Map<string, HybridResult>();

    // Add vector results
    for (const vr of vectorResults) {
      resultMap.set(vr.key, {
        key: vr.key,
        score: alpha * vr.score,
        source: 'vector',
        vectorScore: vr.score,
        conceptData: vr.properties,
      });
    }

    // Merge graph results
    for (const gr of graphResults) {
      const existing = resultMap.get(gr.key);
      if (existing) {
        existing.score += (1 - alpha) * gr.activation;
        existing.source = 'both';
        existing.graphActivation = gr.activation;
      } else {
        resultMap.set(gr.key, {
          key: gr.key,
          score: (1 - alpha) * gr.activation,
          source: 'graph',
          graphActivation: gr.activation,
          conceptData: gr.conceptData,
        });
      }
    }

    // Sort by combined score
    return Array.from(resultMap.values())
      .sort((a, b) => b.score - a.score);
  }
}
```

**Acceptance Criteria:**
- [ ] `HybridRetriever` class implemented
- [ ] Combines vector and graph results
- [ ] Alpha parameter controls weighting
- [ ] Results sorted by combined score
- [ ] Unit tests pass

---

## Task 3.2: Context Assembly

**File:** `src/services/context-assembly.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { HybridResult } from './hybrid-retriever';

export interface AssembledContext {
  concepts: ConceptContext[];
  localeKnowledge: LocaleKnowledge;
  totalTokens: number;
}

interface ConceptContext {
  key: string;
  title: string;
  definition: string;
  llmContext: string;
  relevanceScore: number;
}

interface LocaleKnowledge {
  voice: Record<string, unknown>;
  expressions: string[];
  culturalNotes: string[];
}

export class ContextAssemblyService {
  constructor(
    private driver: Driver,
    private maxTokens: number = 8000
  ) {}

  async assembleContext(
    hybridResults: HybridResult[],
    locale: string
  ): Promise<AssembledContext> {
    const session = this.driver.session();

    try {
      // 1. Get ConceptL10n for each concept
      const conceptKeys = hybridResults.map(r => r.key);
      const conceptsResult = await session.run(`
        UNWIND $keys AS key
        MATCH (c:Concept {key: key})-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN c.key AS key,
               cl.title AS title,
               cl.definition AS definition,
               c.llm_context AS llmContext
      `, { keys: conceptKeys, locale });

      // 2. Get Locale Knowledge
      const localeResult = await session.run(`
        MATCH (l:Locale {key: $locale})
        OPTIONAL MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
        OPTIONAL MATCH (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
        OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:LocaleCulture)
        RETURN v AS voice,
               collect(DISTINCT e.text) AS expressions,
               collect(DISTINCT c.key_values) AS culturalNotes
      `, { locale });

      // 3. Build context with token budgeting
      const concepts: ConceptContext[] = [];
      let tokenCount = 0;

      for (const record of conceptsResult.records) {
        const concept: ConceptContext = {
          key: record.get('key'),
          title: record.get('title') || '',
          definition: record.get('definition') || '',
          llmContext: record.get('llmContext') || '',
          relevanceScore: hybridResults.find(r => r.key === record.get('key'))?.score || 0,
        };

        const conceptTokens = this.estimateTokens(concept);
        if (tokenCount + conceptTokens > this.maxTokens) break;

        concepts.push(concept);
        tokenCount += conceptTokens;
      }

      const localeRecord = localeResult.records[0];
      const localeKnowledge: LocaleKnowledge = {
        voice: localeRecord?.get('voice')?.properties || {},
        expressions: localeRecord?.get('expressions') || [],
        culturalNotes: localeRecord?.get('culturalNotes') || [],
      };

      return {
        concepts,
        localeKnowledge,
        totalTokens: tokenCount,
      };
    } finally {
      await session.close();
    }
  }

  private estimateTokens(concept: ConceptContext): number {
    const text = `${concept.title} ${concept.definition} ${concept.llmContext}`;
    return Math.ceil(text.length / 4); // Rough estimate
  }
}
```

**Acceptance Criteria:**
- [ ] `ContextAssemblyService` class implemented
- [ ] Respects token budget
- [ ] Includes locale knowledge
- [ ] Concepts sorted by relevance
- [ ] Unit tests pass

---

# PHASE 4: Learning Loop

> **Goal:** Adjust SEMANTIC_LINK.temperature based on PageMetrics feedback

## Task 4.1: Temperature Adjustment

**File:** `src/services/temperature-learner.ts`

```typescript
import { Driver } from 'neo4j-driver';

export interface LearningSignal {
  pageKey: string;
  locale: string;
  signal: 'positive' | 'negative';
  strength: number; // 0.0 - 1.0
}

export class TemperatureLearner {
  private learningRate = 0.01;
  private minTemperature = 0.1;
  private maxTemperature = 1.0;

  constructor(private driver: Driver) {}

  async adjustFromPageMetrics(signal: LearningSignal): Promise<void> {
    const session = this.driver.session();

    try {
      // 1. Get concepts that influenced this page's output
      const influencedConcepts = await session.run(`
        MATCH (p:Page {key: $pageKey})-[:HAS_OUTPUT]->(pl:PageL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        MATCH (pl)-[:ASSEMBLES]->(bl:BlockL10n)-[:INFLUENCED_BY]->(cl:ConceptL10n)
        MATCH (c:Concept)-[:HAS_L10N]->(cl)
        RETURN DISTINCT c.key AS conceptKey
      `, { pageKey: signal.pageKey, locale: signal.locale });

      const conceptKeys = influencedConcepts.records.map(r => r.get('conceptKey'));

      // 2. Get SEMANTIC_LINK edges between these concepts
      const links = await session.run(`
        MATCH (c1:Concept)-[r:SEMANTIC_LINK]->(c2:Concept)
        WHERE c1.key IN $keys AND c2.key IN $keys
        RETURN id(r) AS linkId, r.temperature AS currentTemp
      `, { keys: conceptKeys });

      // 3. Adjust temperature based on signal
      const adjustment = signal.signal === 'positive'
        ? this.learningRate * signal.strength
        : -this.learningRate * signal.strength;

      for (const record of links.records) {
        const linkId = record.get('linkId');
        const currentTemp = record.get('currentTemp');
        const newTemp = Math.max(
          this.minTemperature,
          Math.min(this.maxTemperature, currentTemp + adjustment)
        );

        await session.run(`
          MATCH ()-[r:SEMANTIC_LINK]->()
          WHERE id(r) = $linkId
          SET r.temperature = $newTemp,
              r.last_adjusted = datetime()
        `, { linkId, newTemp });
      }
    } finally {
      await session.close();
    }
  }
}
```

**Acceptance Criteria:**
- [ ] `TemperatureLearner` class implemented
- [ ] Respects min/max temperature bounds
- [ ] Learning rate configurable
- [ ] Tracks adjustment history
- [ ] Unit tests pass

---

## Task 4.2: PageMetrics Integration

**File:** `src/services/metrics-processor.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { TemperatureLearner, LearningSignal } from './temperature-learner';

export class MetricsProcessor {
  private learner: TemperatureLearner;

  constructor(driver: Driver) {
    this.learner = new TemperatureLearner(driver);
  }

  async processMetricsUpdate(
    pageKey: string,
    locale: string,
    metrics: {
      bounceRate: number;
      avgTimeOnPage: number;
      conversions: number;
      previousBounceRate?: number;
      previousConversions?: number;
    }
  ): Promise<void> {
    // Determine signal from metrics changes
    const bounceImproved = metrics.previousBounceRate !== undefined
      && metrics.bounceRate < metrics.previousBounceRate;
    const conversionsImproved = metrics.previousConversions !== undefined
      && metrics.conversions > metrics.previousConversions;

    if (bounceImproved || conversionsImproved) {
      await this.learner.adjustFromPageMetrics({
        pageKey,
        locale,
        signal: 'positive',
        strength: this.calculateStrength(metrics),
      });
    } else if (metrics.previousBounceRate !== undefined) {
      const bounceDegraded = metrics.bounceRate > metrics.previousBounceRate * 1.1;
      if (bounceDegraded) {
        await this.learner.adjustFromPageMetrics({
          pageKey,
          locale,
          signal: 'negative',
          strength: this.calculateStrength(metrics),
        });
      }
    }
  }

  private calculateStrength(metrics: { bounceRate: number; conversions: number }): number {
    // Higher conversions = stronger positive signal
    // Lower bounce rate = stronger positive signal
    const bounceScore = Math.max(0, 1 - metrics.bounceRate);
    const conversionScore = Math.min(1, metrics.conversions / 100);
    return (bounceScore + conversionScore) / 2;
  }
}
```

**Acceptance Criteria:**
- [ ] `MetricsProcessor` class implemented
- [ ] Integrates with TemperatureLearner
- [ ] Handles metric comparisons correctly
- [ ] Strength calculation reasonable
- [ ] Unit tests pass

---

# Appendix: Production Optimizations

## A. Consider Neo4j GraphRAG Python

The `neo4j-graphrag-python` library provides `HybridCypherRetriever` out-of-box:

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

**Consider wrapping this instead of building from scratch if using Python backend.**

## B. Quantization Benefits

With `vector.quantization.enabled: true`:
- 40-60% storage reduction
- Minimal accuracy impact (~1-2% recall drop)
- Faster similarity search

## C. Index Maintenance

```cypher
-- Check index health
SHOW INDEXES YIELD name, type, state, populationPercent
WHERE type = 'VECTOR' OR type = 'FULLTEXT';

-- Reindex if needed (after bulk updates)
CALL db.index.fulltext.awaitIndexes();
```

---

## Summary

This plan implements **Hybrid OntologyRAG** in 4 phases:

1. **Vector Infrastructure** - Embeddings + HNSW indexes with quantization
2. **Task-Aware Retrieval** - Dynamic cutoffs per task type
3. **Hybrid Retriever** - Vector + Graph fusion
4. **Learning Loop** - Self-improving temperature adjustment

**Stack Decision:** YAML schema + Neo4j (NOT OWL). See `docs/STACK.md` for architecture rationale.

**Next Step:** Begin Phase 1, Task 1.1 - Update model definitions.
