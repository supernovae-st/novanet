# NovaNet Consolidated Implementation Plan

> **Created:** 2026-01-29
> **Source:** Merged from `hybrid-ontologyrag-implementation.md` + `ontological-analysis-plan.md`
> **Author:** Claude (consolidation & reordering)
> **Status:** AWAITING APPROVAL

---

## Executive Summary

This plan consolidates two plans into an **optimally-ordered execution sequence**:

| Original Plan | Focus | Tasks |
|---------------|-------|-------|
| Ontological Analysis | Schema improvements (inverse rels, PROV-O, SKOS-XL) | 32 phases |
| Hybrid OntologyRAG | Implementation (vector, services, retrieval) | 12 tasks |

### Key Reordering Decisions

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  REORDERING RATIONALE                                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. SCHEMA BEFORE CODE                                                          │
│     Inverse relationships (L10N_OF, BLOCK_OF) MUST come before services         │
│     Reason: Services will use these relationships in queries                    │
│                                                                                 │
│  2. YAML BEFORE INDEXES                                                         │
│     Add embedding properties to YAML before creating Neo4j indexes              │
│     Reason: Indexes reference properties that must exist in schema              │
│                                                                                 │
│  3. INFRASTRUCTURE BEFORE SERVICES                                              │
│     Create vector indexes before VectorSearchService                            │
│     Reason: Service code assumes indexes exist                                  │
│                                                                                 │
│  4. RETRIEVAL BEFORE PROVENANCE                                                 │
│     Get hybrid retrieval working before PROV-O                                  │
│     Reason: Provenance tracks WHAT was retrieved, needs retrieval first         │
│                                                                                 │
│  5. CORE BEFORE ADVANCED                                                        │
│     Learning loop, SHACL, SKOS-XL are lower priority                           │
│     Reason: System works without them; add incrementally                        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Batch Overview

| Batch | Name | Tasks | Dependencies | Priority |
|-------|------|-------|--------------|----------|
| 1 | Schema Foundation | 6 | None | CRITICAL |
| 2 | Neo4j Infrastructure | 4 | Batch 1 | CRITICAL |
| 3 | Core Services | 4 | Batch 2 | HIGH |
| 4 | Hybrid Retriever | 3 | Batch 3 | HIGH |
| 5 | Provenance System | 3 | Batch 4 | MEDIUM |
| 6 | Learning Loop | 2 | Batch 4 | MEDIUM |
| 7 | Advanced Features | 4 | Batch 5 | LOW |

**Total: 26 tasks** (reduced from 44 by merging overlapping tasks)

---

# BATCH 1: Schema Foundation

> **Goal:** Fix schema issues before writing any implementation code
> **Source:** Ontological Plan Milestone A + Hybrid Plan Task 1.1

## Task 1.1: Add Inverse Relationships to relations.yaml

**File:** `models/relations.yaml`

**Add:**
```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# INVERSE RELATIONSHIPS (v7.8.0)
# ═══════════════════════════════════════════════════════════════════════════════

L10N_OF:
  from: "[ConceptL10n, ProjectL10n]"
  to: "[Concept, Project]"
  inverse_of: HAS_L10N
  description: "Inverse of HAS_L10N - localized content points to parent"

OUTPUT_OF:
  from: "[PageL10n, BlockL10n]"
  to: "[Page, Block]"
  inverse_of: HAS_OUTPUT
  description: "Inverse of HAS_OUTPUT - generated content points to structure"

BLOCK_OF:
  from: Block
  to: Page
  inverse_of: HAS_BLOCK
  props: [position]
  description: "Inverse of HAS_BLOCK - block points to its page"

USED_BY:
  from: Concept
  to: "[Page, Block]"
  inverse_of: USES_CONCEPT
  description: "Inverse of USES_CONCEPT - concept knows who uses it"

HAS_LOCALIZED_CONTENT:
  from: Locale
  to: "[ProjectL10n, ConceptL10n, PageL10n, BlockL10n, SEOKeywordL10n, GEOSeedL10n]"
  inverse_of: FOR_LOCALE
  description: "Inverse of FOR_LOCALE - locale knows all its content"
```

**Acceptance Criteria:**
- [ ] `relations.yaml` updated with 5 inverse relationships
- [ ] `npm run validate` passes
- [ ] Documentation updated in `GRAPH-DETAILED.md`

---

## Task 1.2: Add Embedding Properties to Concept YAML

**File:** `models/nodes/content/concept.yaml`

**Add to properties section:**
```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# VECTOR EMBEDDING (v7.8.0 - Hybrid OntologyRAG)
# ═══════════════════════════════════════════════════════════════════════════════

embedding:
  type: vector
  dimensions: 1536
  required: false
  description: "OpenAI text-embedding-3-small vector for semantic search"

embedding_source:
  type: string
  required: false
  description: "Text used to generate embedding (for debugging)"

embedding_updated_at:
  type: datetime
  required: false
  description: "Timestamp of last embedding update"
```

**Acceptance Criteria:**
- [ ] `concept.yaml` has embedding properties
- [ ] `npm run validate` passes

---

## Task 1.3: Add Embedding Properties to ConceptL10n YAML

**File:** `models/nodes/content/concept-l10n.yaml`

**Add same embedding properties as Task 1.2**

**Acceptance Criteria:**
- [ ] `concept-l10n.yaml` has embedding properties
- [ ] `npm run validate` passes

---

## Task 1.4: Add Embedding Properties to Page YAML

**File:** `models/nodes/content/page.yaml`

**Add same embedding properties as Task 1.2**

**Acceptance Criteria:**
- [ ] `page.yaml` has embedding properties
- [ ] `npm run validate` passes

---

## Task 1.5: Update TypeScript Types

**File:** `src/types/content.ts` (NEW)

```typescript
// Base embedding fields for all embeddable nodes
export interface EmbeddableNode {
  embedding?: number[];
  embedding_source?: string;
  embedding_updated_at?: Date;
}

export interface Concept extends EmbeddableNode {
  key: string;
  display_name: string;
  icon: string;
  description: string;
  llm_context: string;
  priority: 'critical' | 'high' | 'medium' | 'low';
  freshness: 'realtime' | 'hourly' | 'daily' | 'static';
  // ... other existing fields
}

export interface ConceptL10n extends EmbeddableNode {
  title: string;
  definition?: string;
  summary?: string;
  // ... other existing fields
}

export interface Page extends EmbeddableNode {
  key: string;
  display_name: string;
  // ... other existing fields
}
```

**Acceptance Criteria:**
- [ ] `src/types/content.ts` created with all types
- [ ] Types exported from `src/types/index.ts`
- [ ] `npm run build` passes

---

## Task 1.6: Create Migration Script for Inverse Relationships

**File:** `neo4j/migrations/001-inverse-relationships.cypher`

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 001: Create Inverse Relationships
// Run ONCE after schema update
// ═══════════════════════════════════════════════════════════════════════════════

// L10N_OF from HAS_L10N
MATCH (parent)-[:HAS_L10N]->(l10n)
WHERE NOT (l10n)-[:L10N_OF]->(parent)
MERGE (l10n)-[:L10N_OF]->(parent);

// OUTPUT_OF from HAS_OUTPUT
MATCH (parent)-[:HAS_OUTPUT]->(output)
WHERE NOT (output)-[:OUTPUT_OF]->(parent)
MERGE (output)-[:OUTPUT_OF]->(parent);

// BLOCK_OF from HAS_BLOCK
MATCH (page:Page)-[r:HAS_BLOCK]->(block:Block)
WHERE NOT (block)-[:BLOCK_OF]->(page)
MERGE (block)-[:BLOCK_OF {position: r.position}]->(page);

// USED_BY from USES_CONCEPT
MATCH (user)-[r:USES_CONCEPT]->(concept:Concept)
WHERE NOT (concept)-[:USED_BY]->(user)
MERGE (concept)-[:USED_BY]->(user);

// HAS_LOCALIZED_CONTENT from FOR_LOCALE
MATCH (content)-[:FOR_LOCALE]->(locale:Locale)
WHERE NOT (locale)-[:HAS_LOCALIZED_CONTENT]->(content)
MERGE (locale)-[:HAS_LOCALIZED_CONTENT]->(content);

// Verify counts
MATCH ()-[r:L10N_OF]->() RETURN 'L10N_OF' AS rel, count(r) AS count
UNION ALL
MATCH ()-[r:OUTPUT_OF]->() RETURN 'OUTPUT_OF' AS rel, count(r) AS count
UNION ALL
MATCH ()-[r:BLOCK_OF]->() RETURN 'BLOCK_OF' AS rel, count(r) AS count
UNION ALL
MATCH ()-[r:USED_BY]->() RETURN 'USED_BY' AS rel, count(r) AS count
UNION ALL
MATCH ()-[r:HAS_LOCALIZED_CONTENT]->() RETURN 'HAS_LOCALIZED_CONTENT' AS rel, count(r) AS count;
```

**Acceptance Criteria:**
- [ ] Migration script created
- [ ] Script is idempotent (can run multiple times)
- [ ] Verification query shows correct counts

---

# BATCH 2: Neo4j Infrastructure

> **Goal:** Create indexes and configuration before services
> **Source:** Hybrid Plan Task 1.2 + Ontological Plan Milestone B

## Task 2.1: Create Vector Indexes

**File:** `neo4j/seed/02-vector-indexes.cypher`

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// VECTOR INDEXES for Hybrid OntologyRAG
// ═══════════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════════
// FULLTEXT INDEXES (fallback for keyword search)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE FULLTEXT INDEX concept_fulltext IF NOT EXISTS
FOR (c:Concept) ON EACH [c.key, c.display_name, c.description, c.llm_context];

CREATE FULLTEXT INDEX concept_l10n_fulltext IF NOT EXISTS
FOR (cl:ConceptL10n) ON EACH [cl.title, cl.definition, cl.summary];
```

**Acceptance Criteria:**
- [ ] Vector indexes created (3)
- [ ] Fulltext indexes created (2)
- [ ] `SHOW INDEXES` shows all as ONLINE

---

## Task 2.2: Create Spreading Activation Config

**File:** `models/config/spreading-activation.yaml`

```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# SPREADING ACTIVATION CONFIGURATION v1.0.0
# ═══════════════════════════════════════════════════════════════════════════════

default:
  # Core Parameters (ρ, δ, T from cognitive science)
  decay_factor: 0.01           # ρ - exponential decay over steps
  retention_factor: 0.5        # δ - activation retained between steps
  propagation_steps: 2         # T - maximum hops
  initial_activation: 1.0      # A₀ - starting activation

  # Thresholds
  activation_threshold: 0.3    # Minimum to continue spreading
  output_threshold: 0.1        # Minimum to include in results

  # Fan Effect Control
  max_fan_out: 10              # Limit outgoing edges per node
  fan_penalty: 0.1             # Reduce activation for high-degree nodes

# Task-specific overrides
task_modifiers:
  CTA:
    activation_threshold: 0.25
    propagation_steps: 2
    semantic_boosts:
      urgency: 1.3
      value: 1.2
      action: 1.15
    priority_filter: [critical, high]

  FAQ:
    activation_threshold: 0.40
    propagation_steps: 2
    semantic_boosts:
      definition: 1.3
      type_of: 1.2
    priority_filter: [critical, high, medium]

  HERO:
    activation_threshold: 0.30
    propagation_steps: 2
    semantic_boosts:
      is_action_on: 1.2
      includes: 1.1
    priority_filter: [critical, high]

  PRICING:
    activation_threshold: 0.20
    propagation_steps: 2
    semantic_boosts:
      includes: 1.3
      type_of: 1.2
      value: 1.1
    priority_filter: [critical, high]

  DEFAULT:
    activation_threshold: 0.30
    propagation_steps: 2
    semantic_boosts: {}
    priority_filter: [critical, high, medium]
```

**Acceptance Criteria:**
- [ ] Config file created
- [ ] Default parameters defined
- [ ] 5 task modifiers defined (CTA, FAQ, HERO, PRICING, DEFAULT)

---

## Task 2.3: Create TypeScript Config Loader

**File:** `src/config/spreading-activation.ts`

```typescript
import { readFileSync } from 'fs';
import { parse } from 'yaml';
import { join } from 'path';

export interface SemanticBoosts {
  [key: string]: number;
}

export interface TaskModifier {
  activation_threshold: number;
  propagation_steps: number;
  semantic_boosts: SemanticBoosts;
  priority_filter: string[];
}

export interface SpreadingActivationConfig {
  decay_factor: number;
  retention_factor: number;
  propagation_steps: number;
  initial_activation: number;
  activation_threshold: number;
  output_threshold: number;
  max_fan_out: number;
  fan_penalty: number;
  task_modifiers: Record<string, TaskModifier>;
}

let cachedConfig: SpreadingActivationConfig | null = null;

export function loadSpreadingActivationConfig(): SpreadingActivationConfig {
  if (cachedConfig) return cachedConfig;

  const configPath = join(__dirname, '../../models/config/spreading-activation.yaml');
  const raw = readFileSync(configPath, 'utf-8');
  const parsed = parse(raw);

  cachedConfig = {
    ...parsed.default,
    task_modifiers: parsed.task_modifiers,
  };

  return cachedConfig;
}

export function getTaskModifier(taskType: string): TaskModifier {
  const config = loadSpreadingActivationConfig();
  return config.task_modifiers[taskType] || config.task_modifiers.DEFAULT;
}
```

**Acceptance Criteria:**
- [ ] Config loader implemented
- [ ] Caching for performance
- [ ] `getTaskModifier()` helper function
- [ ] `npm run build` passes

---

## Task 2.4: Update seed.sh to Include New Scripts

**File:** `neo4j/seed.sh` (UPDATE)

Add to the execution list:
```bash
# ... existing seed scripts ...

echo "Creating vector indexes..."
cypher-shell -f seed/02-vector-indexes.cypher

echo "Running inverse relationship migration..."
cypher-shell -f migrations/001-inverse-relationships.cypher
```

**Acceptance Criteria:**
- [ ] seed.sh updated
- [ ] Full seed runs without errors

---

# BATCH 3: Core Services

> **Goal:** Implement embedding generation and vector search
> **Source:** Hybrid Plan Tasks 1.3, 1.4, 2.1, 2.2

## Task 3.1: Create Embedding Generation Script

**File:** `scripts/generate-embeddings.ts`

(Use implementation from Hybrid Plan Task 1.3)

**Acceptance Criteria:**
- [ ] Script generates embeddings for Concept, ConceptL10n, Page
- [ ] Batch processing (100 at a time)
- [ ] `embedding_source` populated
- [ ] Progress logging

---

## Task 3.2: Create Vector Search Service

**File:** `src/services/vector-search.ts`

(Use implementation from Hybrid Plan Task 1.4)

**Acceptance Criteria:**
- [ ] `VectorSearchService` class
- [ ] `embedQuery()` method
- [ ] `searchConcepts()` method
- [ ] `searchConceptL10n()` with locale filter

---

## Task 3.3: Create Task Types

**File:** `src/types/task-types.ts`

(Use implementation from Hybrid Plan Task 2.1)

**Acceptance Criteria:**
- [ ] `TaskType` enum
- [ ] `TaskModifier` interface
- [ ] `TASK_MODIFIERS` constant

---

## Task 3.4: Create Graph Traversal Service

**File:** `src/services/graph-traversal.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { getTaskModifier, SpreadingActivationConfig } from '../config/spreading-activation';

export interface ActivatedNode {
  key: string;
  activation: number;
  depth: number;
  nodeType: string;
  properties: Record<string, unknown>;
}

export class GraphTraversalService {
  constructor(
    private driver: Driver,
    private config: SpreadingActivationConfig
  ) {}

  async spreadingActivation(
    startKey: string,
    taskType: string = 'DEFAULT'
  ): Promise<ActivatedNode[]> {
    const modifier = getTaskModifier(taskType);
    const session = this.driver.session();

    try {
      const result = await session.run(`
        WITH $startKey AS startKey
        MATCH (start:Concept {key: startKey})

        // Direct connections (depth 1)
        OPTIONAL MATCH (start)-[r1:SEMANTIC_LINK]->(c1:Concept)
        WHERE r1.temperature >= $threshold

        WITH start, collect({
          node: c1,
          activation: r1.temperature * $initial,
          depth: 1,
          sem_type: r1.semantic_type
        }) AS depth1

        // Two-hop connections (depth 2)
        OPTIONAL MATCH (start)-[r1:SEMANTIC_LINK]->(c1:Concept)-[r2:SEMANTIC_LINK]->(c2:Concept)
        WHERE r1.temperature >= $threshold
          AND r2.temperature >= $threshold
          AND c2 <> start

        WITH depth1, collect({
          node: c2,
          activation: r1.temperature * r2.temperature * $retention,
          depth: 2,
          sem_type: r2.semantic_type
        }) AS depth2

        // Combine and dedupe
        UNWIND (depth1 + depth2) AS item
        WITH item.node AS node, max(item.activation) AS activation, min(item.depth) AS depth
        WHERE activation >= $outputThreshold AND node IS NOT NULL

        RETURN node.key AS key,
               activation,
               depth,
               labels(node)[0] AS nodeType,
               node { .key, .display_name, .llm_context, .priority } AS properties
        ORDER BY activation DESC
        LIMIT 20
      `, {
        startKey,
        threshold: modifier.activation_threshold,
        initial: this.config.initial_activation,
        retention: this.config.retention_factor,
        outputThreshold: this.config.output_threshold
      });

      return result.records.map(r => ({
        key: r.get('key'),
        activation: r.get('activation'),
        depth: r.get('depth'),
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
- [ ] `GraphTraversalService` implemented
- [ ] `spreadingActivation()` method
- [ ] Uses config from YAML
- [ ] Unit tests pass

---

# BATCH 4: Hybrid Retriever

> **Goal:** Combine vector + graph for optimal retrieval
> **Source:** Hybrid Plan Tasks 3.1, 3.2 + Ontological Plan Milestone D

## Task 4.1: Create Hybrid Retriever

**File:** `src/services/hybrid-retriever.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { VectorSearchService, VectorMatch } from './vector-search';
import { GraphTraversalService, ActivatedNode } from './graph-traversal';

export interface HybridResult {
  key: string;
  score: number;
  vectorScore: number;
  activationScore: number;
  depth: number;
  nodeType: string;
  properties: Record<string, unknown>;
}

export interface HybridSearchOptions {
  limit?: number;
  alpha?: number;  // 0 = pure graph, 1 = pure vector
  locale?: string;
  taskType?: string;
}

export class HybridRetriever {
  private vectorService: VectorSearchService;
  private graphService: GraphTraversalService;

  constructor(driver: Driver, config: any) {
    this.vectorService = new VectorSearchService(driver);
    this.graphService = new GraphTraversalService(driver, config);
  }

  async search(
    query: string,
    seedConcepts: string[],
    options: HybridSearchOptions = {}
  ): Promise<HybridResult[]> {
    const { limit = 10, alpha = 0.7, taskType = 'DEFAULT' } = options;

    // 1. Vector search for semantic entry points
    const vectorResults = await this.vectorService.searchConcepts(query, { limit: limit * 2 });

    // 2. Graph traversal from seed concepts + vector results
    const allSeeds = [...seedConcepts, ...vectorResults.slice(0, 5).map(r => r.key)];
    const uniqueSeeds = [...new Set(allSeeds)];

    const graphResults: ActivatedNode[] = [];
    for (const seed of uniqueSeeds) {
      const activated = await this.graphService.spreadingActivation(seed, taskType);
      graphResults.push(...activated);
    }

    // 3. Merge and score
    const merged = this.mergeResults(vectorResults, graphResults, alpha);

    // 4. Sort and limit
    return merged
      .sort((a, b) => b.score - a.score)
      .slice(0, limit);
  }

  private mergeResults(
    vectorResults: VectorMatch[],
    graphResults: ActivatedNode[],
    alpha: number
  ): HybridResult[] {
    const resultMap = new Map<string, HybridResult>();

    // Add vector results
    for (const v of vectorResults) {
      resultMap.set(v.key, {
        key: v.key,
        score: v.score * alpha,
        vectorScore: v.score,
        activationScore: 0,
        depth: 0,
        nodeType: v.nodeType,
        properties: v.properties,
      });
    }

    // Merge graph results
    for (const g of graphResults) {
      const existing = resultMap.get(g.key);
      if (existing) {
        existing.activationScore = Math.max(existing.activationScore, g.activation);
        existing.score = (existing.vectorScore * alpha) + (existing.activationScore * (1 - alpha));
        existing.depth = Math.min(existing.depth || 99, g.depth);
      } else {
        resultMap.set(g.key, {
          key: g.key,
          score: g.activation * (1 - alpha),
          vectorScore: 0,
          activationScore: g.activation,
          depth: g.depth,
          nodeType: g.nodeType,
          properties: g.properties,
        });
      }
    }

    return Array.from(resultMap.values());
  }
}
```

**Acceptance Criteria:**
- [ ] `HybridRetriever` combines vector + graph
- [ ] Alpha parameter controls blend
- [ ] Deduplication works correctly
- [ ] Integration tests pass

---

## Task 4.2: Create Context Assembly Service

**File:** `src/services/context-assembly.ts`

```typescript
import { Driver } from 'neo4j-driver';
import { HybridRetriever, HybridResult } from './hybrid-retriever';

export interface AssembledContext {
  concepts: ConceptContext[];
  localeKnowledge: LocaleKnowledge;
  seoKeywords: string[];
  totalTokens: number;
}

export interface ConceptContext {
  key: string;
  title: string;
  definition: string;
  llmContext: string;
  score: number;
}

export interface LocaleKnowledge {
  voice: Record<string, unknown>;
  expressions: string[];
  formatting: Record<string, unknown>;
}

export class ContextAssemblyService {
  constructor(
    private driver: Driver,
    private retriever: HybridRetriever
  ) {}

  async assembleContext(
    query: string,
    seedConcepts: string[],
    locale: string,
    taskType: string,
    tokenBudget: number = 4000
  ): Promise<AssembledContext> {
    // 1. Get hybrid results
    const results = await this.retriever.search(query, seedConcepts, {
      taskType,
      limit: 20
    });

    // 2. Load localized content
    const concepts = await this.loadConceptL10n(results, locale);

    // 3. Load locale knowledge
    const localeKnowledge = await this.loadLocaleKnowledge(locale);

    // 4. Load SEO keywords if available
    const seoKeywords = await this.loadSEOKeywordL10ns(seedConcepts, locale);

    // 5. Trim to token budget
    const trimmed = this.trimToTokenBudget(concepts, tokenBudget);

    return {
      concepts: trimmed,
      localeKnowledge,
      seoKeywords,
      totalTokens: this.estimateTokens(trimmed, localeKnowledge),
    };
  }

  private async loadConceptL10n(
    results: HybridResult[],
    locale: string
  ): Promise<ConceptContext[]> {
    const session = this.driver.session();
    try {
      const keys = results.map(r => r.key);
      const result = await session.run(`
        UNWIND $keys AS key
        MATCH (c:Concept {key: key})-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN c.key AS key, cl.title AS title, cl.definition AS definition, c.llm_context AS llmContext
      `, { keys, locale });

      const l10nMap = new Map(result.records.map(r => [r.get('key'), r.toObject()]));

      return results
        .filter(r => l10nMap.has(r.key))
        .map(r => ({
          ...l10nMap.get(r.key) as any,
          score: r.score,
        }));
    } finally {
      await session.close();
    }
  }

  private async loadLocaleKnowledge(locale: string): Promise<LocaleKnowledge> {
    const session = this.driver.session();
    try {
      const result = await session.run(`
        MATCH (l:Locale {key: $locale})
        OPTIONAL MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
        OPTIONAL MATCH (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
        OPTIONAL MATCH (l)-[:HAS_RULES_FORMATTING]->(fmt:LocaleRulesFormatting)
        RETURN v AS voice, collect(DISTINCT e.text) AS expressions, fmt AS formatting
      `, { locale });

      const record = result.records[0];
      return {
        voice: record?.get('voice')?.properties || {},
        expressions: record?.get('expressions') || [],
        formatting: record?.get('formatting')?.properties || {},
      };
    } finally {
      await session.close();
    }
  }

  private async loadSEOKeywordL10ns(conceptKeys: string[], locale: string): Promise<string[]> {
    const session = this.driver.session();
    try {
      const result = await session.run(`
        UNWIND $keys AS key
        MATCH (c:Concept {key: key})-[:HAS_L10N]->(cl:ConceptL10n)-[:HAS_SEO_TARGET]->(sk:SEOKeywordL10n)
        MATCH (sk)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN DISTINCT sk.value AS keyword
        LIMIT 10
      `, { keys: conceptKeys, locale });

      return result.records.map(r => r.get('keyword'));
    } finally {
      await session.close();
    }
  }

  private trimToTokenBudget(concepts: ConceptContext[], budget: number): ConceptContext[] {
    // Simple estimation: ~4 chars per token
    let currentTokens = 0;
    const result: ConceptContext[] = [];

    for (const concept of concepts) {
      const tokens = this.estimateConceptTokens(concept);
      if (currentTokens + tokens > budget) break;
      result.push(concept);
      currentTokens += tokens;
    }

    return result;
  }

  private estimateConceptTokens(concept: ConceptContext): number {
    const text = `${concept.title} ${concept.definition} ${concept.llmContext}`;
    return Math.ceil(text.length / 4);
  }

  private estimateTokens(concepts: ConceptContext[], locale: LocaleKnowledge): number {
    const conceptTokens = concepts.reduce((sum, c) => sum + this.estimateConceptTokens(c), 0);
    const localeTokens = Math.ceil(JSON.stringify(locale).length / 4);
    return conceptTokens + localeTokens;
  }
}
```

**Acceptance Criteria:**
- [ ] Assembles concepts + locale knowledge + SEO
- [ ] Token budget respected
- [ ] Uses HAS_SEO_TARGET (v7.7.0 pattern)

---

## Task 4.3: Create Service Index

**File:** `src/services/index.ts`

```typescript
export { VectorSearchService } from './vector-search';
export { GraphTraversalService } from './graph-traversal';
export { HybridRetriever } from './hybrid-retriever';
export { ContextAssemblyService } from './context-assembly';
```

**Acceptance Criteria:**
- [ ] All services exported
- [ ] `npm run build` passes

---

# BATCH 5: Provenance System

> **Goal:** Track what was used to generate content (PROV-O aligned)
> **Source:** Ontological Plan Milestone C

## Task 5.1: Add GenerationTrace Node

**File:** `models/nodes/generation/generation-trace.yaml` (NEW)

```yaml
node:
  name: GenerationTrace
  category: generation
  icon: "📋"
  description: "PROV-O aligned trace of content generation"

  properties:
    key:
      type: string
      required: true
      pattern: "^trace-[a-z0-9-]+$"

    # PROV-O Core
    prov_started_at: { type: datetime, required: true }
    prov_ended_at: { type: datetime, required: true }
    prov_type: { type: string, enum: [ContentGeneration, Regeneration, Edit] }

    # LLM Specifics
    model: { type: string, required: true }
    temperature: { type: float }
    prompt_template_key: { type: string }

    # Token Accounting
    context_tokens: { type: int }
    output_tokens: { type: int }

    # Quality
    confidence_score: { type: float, description: "0.0-1.0" }
```

---

## Task 5.2: Add Provenance Relationships

**File:** `models/relations.yaml` (UPDATE)

```yaml
# PROVENANCE (PROV-O aligned)
GENERATED_BY:
  from: "[BlockL10n, PageL10n]"
  to: GenerationTrace
  description: "prov:wasGeneratedBy - output was generated by trace"

USED_CONTEXT:
  from: GenerationTrace
  to: "[ConceptL10n, LocaleVoice, Expression, SEOKeywordL10n, BlockPrompt]"
  props: [token_count, relevance_score]
  description: "prov:used - trace used this context"
```

---

## Task 5.3: Create Provenance Service

**File:** `src/services/provenance.ts`

```typescript
export class ProvenanceService {
  constructor(private driver: Driver) {}

  async createTrace(params: CreateTraceParams): Promise<string> {
    // Generate trace key
    // Create GenerationTrace node
    // Create GENERATED_BY relationship
    // Create USED_CONTEXT relationships for all context items
  }

  async getTraceForOutput(outputKey: string): Promise<GenerationTrace | null> {
    // Query trace + all used context
  }

  async getOutputHistory(outputKey: string, limit: number = 5): Promise<GenerationTrace[]> {
    // Get all traces for an output (regeneration history)
  }
}
```

**Acceptance Criteria:**
- [ ] `ProvenanceService` creates traces
- [ ] Links outputs to traces
- [ ] Links traces to context used

---

# BATCH 6: Learning Loop

> **Goal:** Adjust temperatures based on performance
> **Source:** Hybrid Plan Phase 4

## Task 6.1: Create Metrics Processor

**File:** `src/services/metrics-processor.ts`

Processes PageMetrics to identify high/low performing content.

---

## Task 6.2: Create Temperature Learner

**File:** `src/services/temperature-learner.ts`

Adjusts SEMANTIC_LINK.temperature based on PageMetrics feedback.

---

# BATCH 7: Advanced Features (MEDIUM PRIORITY)

## Task 7.1: SHACL Shapes

**File:** `models/shapes/concept.shacl.ttl`

Basic validation shapes for Concept nodes.

---

## Task 7.2: SKOS-XL Properties

Add `skos_prefLabel`, `skos_altLabel`, `skos_hiddenLabel` to ConceptL10n.

---

## Task 7.3: ExternalConceptMapping Node

For interoperability with Schema.org, Wikidata.

---

## Task 7.4: Two Clocks Pattern

Add `valid_from`, `valid_until` to SEOKeywordL10n, GEOSeedL10n for temporal tracking.

---

# Execution Notes

## Running with execute-plans skill

```bash
# Claude will use:
/spn-powers:planning:execute-plan

# Execute in batches, stopping for review after each batch
```

## Review Checkpoints

| After Batch | Review Focus |
|-------------|--------------|
| 1 | Schema changes correct? Inverse rels make sense? |
| 2 | Indexes created? Config loads? |
| 3 | Embeddings generate? Vector search works? |
| 4 | Hybrid retrieval returns good results? |
| 5 | Provenance tracking complete? |
| 6 | Learning loop adjusts temperatures? |

## Rollback Strategy

Each batch is independent. If a batch fails:
1. Revert files in that batch
2. Re-run from start of batch
3. Inverse relationships can be deleted with: `MATCH ()-[r:L10N_OF|OUTPUT_OF|BLOCK_OF|USED_BY|HAS_LOCALIZED_CONTENT]->() DELETE r`
