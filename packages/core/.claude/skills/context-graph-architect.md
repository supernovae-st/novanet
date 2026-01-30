---
name: context-graph-architect
description: Use when designing knowledge graphs for AI content generation, planning graph schemas, or architecting context retrieval systems for LLMs
---

# Context Graph Architect Skill

Design graph architectures that enable precise context retrieval for AI content generation.

## ⚠️ CRITICAL: Generation, NOT Translation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET = NATIVE GENERATION                                                   │
│                                                                              │
│  ❌ WRONG: Source → Translate → Target                                      │
│  ✅ RIGHT: Concept (invariant) → Generate natively → ConceptL10n (local)    │
└─────────────────────────────────────────────────────────────────────────────┘
```

Each locale content is **generated natively** from the invariant entity:

```
Concept (invariant EN)          ConceptL10n (generated natively per locale)
───────────────────────         ──────────────────────────────────────────────
key: "qr-code"           ──►    locale: "fr-FR"
llm_context: "Represents:          title: "QR Code"           ← NOT a translation
  QR code technology..."         definition: "Un code-barres 2D lisible..."
                                 cultural_notes: "'QR Code' > 'Code QR' en France"

                         ──►    locale: "ja-JP"
                                 title: "QRコード"          ← Native generation
                                 definition: "スマホで読み取れる2Dコード..."
                                 cultural_notes: "Invented by Denso Wave (1994)"
```

**Why Generation > Translation:**
- Cultural authenticity (idioms, tone, local references)
- Native SEO (keywords users actually search, not translations)
- Scalability (parallel generation, no source bottleneck)

## When to Use

- Designing new graph schemas for content systems
- Planning entity types, relationships, and properties
- Architecting context retrieval for LLM generation
- Extending existing graphs with new domains
- Optimizing graph structure for RAG/GEO patterns

## Core Principles

### 1. Ontology Encodes Epistemology

**The graph structure determines what the LLM can retrieve and reason over.**

```
BAD:  Flat document store → LLM searches blindly
GOOD: Rich graph → LLM navigates precisely to relevant context
```

Design questions:
- What entities exist in the domain?
- How do entities relate to each other?
- What context does each entity need for generation?
- How will the LLM traverse to find relevant information?

### 2. Multi-Layer Graph Architecture

Separate concerns into distinct graph layers:

```
┌─────────────────────────────────────────────────────────────┐
│                    SEMANTIC LAYER                           │
│  Concepts, Meanings, Definitions, Relationships             │
│  (Concept)--[:SEMANTIC_LINK {type, temperature}]-->(Concept)│
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    STRUCTURAL LAYER                         │
│  Pages, Blocks, Templates, Composition                      │
│  (Page)--[:HAS_BLOCK]-->(Block {position})--[:OF_TYPE]-->() │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    CONTENT LAYER                            │
│  Localized content, Generated outputs                       │
│  (Concept)--[:HAS_L10N]-->(ConceptL10n {locale})            │
│  (Page/Block)--[:HAS_OUTPUT]-->(PageL10n/BlockL10n)     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    METADATA LAYER                           │
│  SEO/GEO queries, Audit logs                                │
│  Locale → Knowledge nodes (v7.0.0)                          │
└─────────────────────────────────────────────────────────────┘
```

### 3. Entity-Content Pattern

**Separate invariant structure from localized content:**

```
┌──────────────┐     :HAS_L10N         ┌──────────────────┐
│   Concept    │ ──────────────────────▶│ ConceptL10n      │
│              │                        │ {locale: "fr-FR"}│
│  key (EN)    │                        │                  │
│  llm_context   │                        │  title           │
│  type        │                        │  definition      │
└──────────────┘                        │  benefits[]      │
       │                                │  use_cases[]     │
       │ :HAS_L10N                      └──────────────────┘
       ▼
┌──────────────────┐
│ ConceptL10n      │
│ {locale: "en-US"}│
└──────────────────┘
```

Benefits:
- Single source of truth for structure
- N locales without entity duplication
- Clean separation of concerns
- Easy to add new locales

### 4. Reference by Key, Not by ID

**Use semantic keys for graph references:**

```cypher
-- GOOD: Semantic, readable, stable
(Block)-[:USES_CONCEPT]->(Concept {key: "tier-pro"})

-- Instructions use @key syntax
instructions: "[GENERATE] Use @tier-pro and @analytics features"

-- Query by key
MATCH (c:Concept {key: $conceptKey})
```

Benefits:
- Human-readable queries and logs
- Stable across environments
- Self-documenting relationships
- Easy debugging

## Graph Design Process

### Step 1: Identify Core Entities

Ask: "What are the main objects in this domain?"

```yaml
# NovaNet example (v7.0.0)
entities:
  - Concept: Reusable semantic unit (action, object, feature)
  - Page: Web page template
  - Block: Content section within page
  - BlockType: Template rules
  - Locale: First-class locale node with Knowledge nodes
  - LocaleVoice, LocaleCulture, LocaleMarket, LocaleLexicon
```

### Step 2: Define Relationships

Ask: "How do entities connect? What properties describe the connection?"

```yaml
relationships:
  USES_CONCEPT:
    from: Block
    to: Concept
    properties:
      purpose: primary | secondary | example
      temperature: 0.0-1.0  # relevance weight

  SEMANTIC_LINK:
    from: Concept
    to: Concept
    properties:
      type: is_action_on | includes | type_of | related_to
      temperature: 0.0-1.0  # activation decay
      source: manual | llm_inferred
```

### Step 3: Plan Traversal Patterns

Ask: "How will the LLM gather context for generation?"

```
Orchestrator traversal (v7.0.0):
1. Page → Blocks (structure)
2. Block → BlockType (rules)
3. Block → Concept → ConceptL10n → Locale (semantics)

Sub-agent traversal:
4. Concept → SEMANTIC_LINK*2 → Concept (spreading)
5. Locale → Knowledge nodes (voice, culture, market, lexicon)
6. Concept → SEOKeywordL10n/GEOSeedL10n (SEO/GEO targeting)
```

> **v7.0.0 Note**: All nodes have standard properties: key, display_name, icon, description, llm_context.
> Locale Knowledge accessed via graph: (Locale)-[:HAS_VOICE]->(LocaleVoice)

### Step 4: Design for Query Efficiency

```cypher
-- Single query loads complete block context (v7.0.0)
MATCH (b:Block {key: $blockKey})

-- Structure
OPTIONAL MATCH (b)-[:OF_TYPE]->(bt:BlockType)

-- Semantics with Locale resolution
OPTIONAL MATCH (b)-[:USES_CONCEPT]->(c:Concept)
OPTIONAL MATCH (c)-[:HAS_L10N]->(cc:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})

-- Locale Knowledge
OPTIONAL MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)

-- Spreading (sub-agent loads separately)
-- SEO/GEO (sub-agent loads separately)

RETURN b, bt, collect(DISTINCT {c: c, cc: cc}), v, lex
```

## Context Assembly Patterns

### Pattern: Layered Context Loading

```
┌─────────────────────────────────────────┐
│            ORCHESTRATOR                 │
│  Loads: ADN + Structure + Core Concepts │
│  Dispatches per block with context      │
└─────────────────────────────────────────┘
                    │
    ┌───────────────┼───────────────┐
    ▼               ▼               ▼
┌─────────┐   ┌─────────┐   ┌─────────┐
│Sub-Agent│   │Sub-Agent│   │Sub-Agent│
│         │   │         │   │         │
│ Loads:  │   │ Loads:  │   │ Loads:  │
│ Spread  │   │ Spread  │   │ Spread  │
│ SEO/GEO │   │ SEO/GEO │   │ SEO/GEO │
└─────────┘   └─────────┘   └─────────┘
```

### Pattern: Temperature-Weighted Activation

```
Seed concept → activation = 1.0
  │
  ├──[temp: 0.85]──▶ Related concept → activation = 0.85
  │                       │
  │                       └──[temp: 0.80]──▶ Distant → activation = 0.68
  │
  └──[temp: 0.60]──▶ Weak link → activation = 0.60
                          │
                          └──[temp: 0.70]──▶ (0.42 < cutoff, STOP)
```

### Pattern: Block-Scoped Concepts

```cypher
-- Concepts can be scoped to specific blocks
(:Block {key: "faq"})-[:USES_CONCEPT {scope: "block:faq"}]->(:Concept)

-- Or global to entire page
(:Block)-[:USES_CONCEPT {scope: "global"}]->(:Concept)
```

## Anti-Patterns to Avoid

### 1. Document Dump

```
BAD: Store entire documents as node properties
     (:Page {content: "10KB of HTML..."})

GOOD: Decompose into traversable structure
     (Page)-[:HAS_BLOCK]->(Block)-[:HAS_OUTPUT]->(BlockL10n)
```

### 2. God Node

```
BAD: Single node with hundreds of relationships
     (SiteConfig)--[*500 relationships]-->()

GOOD: Hierarchical structure with intermediate nodes
     (Site)-[:HAS_PAGE]->(Page)-[:HAS_BLOCK]->(Block)
```

### 3. Missing Inverse Navigation

```
BAD: Can only traverse one direction
     (Block)-[:USES_CONCEPT]->(Concept)
     -- Cannot find "which blocks use this concept?"

GOOD: Design for bi-directional queries
     -- Relationship direction + query both ways
     MATCH (c:Concept)<-[:USES_CONCEPT]-(b:Block)
```

### 4. Overloaded Properties

```
BAD: JSON in properties
     (Block {config: '{"a":1,"b":{"c":2}}'})

GOOD: Model as graph structure
     (Block)-[:HAS_CONFIG]->(Config)-[:HAS_SETTING]->(Setting)
```

## Checklist: New Graph Feature

- [ ] Entity identified with clear purpose
- [ ] Relationships defined with properties
- [ ] Traversal pattern documented
- [ ] Query examples written
- [ ] Indexes planned for query patterns
- [ ] Content pattern applied (if localized)
- [ ] Spreading config defined (if semantic)
