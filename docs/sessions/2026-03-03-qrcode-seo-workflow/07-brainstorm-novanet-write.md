# Brainstorm: novanet_write MCP Tools

**Date**: 2026-03-03
**Status**: Design Complete - Ready for Implementation
**Decision**: Single `novanet_write` tool with 3 operations

---

## Table of Contents

1. [Context](#context)
2. [Research Findings](#research-findings)
3. [Architecture Design](#architecture-design)
4. [The Virtuous Cycle](#the-virtuous-cycle)
5. [API Design](#api-design)
6. [Implementation Plan](#implementation-plan)
7. [Scale Considerations](#scale-considerations)

---

## Context

### The Problem

Le MCP NovaNet est actuellement **READ-ONLY**. Nika (workflow engine) a besoin d'ecrire dans Neo4j pour le workflow SEO:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SITUATION ACTUELLE                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│   NIKA (Workflow)              MCP               NOVANET (Brain)                │
│   ┌───────────────┐           ┌───┐           ┌───────────────┐                │
│   │  "Je veux     │  READ OK  │ R │  QUERY    │   Neo4j       │                │
│   │   creer un    │◄──────────│ E │◄──────────│   Graph       │                │
│   │   SEOKeyword" │           │ A │           │               │                │
│   │               │     ❌     │ D │     ❌     │               │                │
│   │               │──────────►│ O │──────────►│               │                │
│   │               │  WRITE    │ N │  BLOCKED! │               │                │
│   │               │  BLOCKED  │ L │           │               │                │
│   └───────────────┘           │ Y │           └───────────────┘                │
│                               └───┘                                             │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### What Nika Needs to Write

1. **SEOKeyword** - Keywords avec vrais volumes de recherche (Ahrefs)
2. **EntityNative** - Mettre a jour `denomination_forms.url` apres pipeline SEO
3. **BlockNative** - Contenu genere par LLM
4. **Term** - Termes decouverts via recherche SEO
5. **Arcs** - TARGETS, USES_TERM, FOR_LOCALE, INFLUENCED_BY

---

## Research Findings

### 1. NovaNet Schema Philosophy

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  DEUX COUCHES DISTINCTES                                                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  COUCHE SCHEMA (Meta)                 COUCHE DATA (Instances)                   │
│  ═════════════════════                ═══════════════════════                   │
│                                                                                 │
│  :Class, :ArcClass                    :Entity, :EntityNative, :SEOKeyword       │
│  ├── Definis par Thibaut (YAML)       ├── Crees par Nika (MCP write)            │
│  ├── Versionnes dans Git              ├── Stockes dans Neo4j                    │
│  └── IMMUTABLES par Nika              └── MUTABLES par Nika                     │
│                                                                                 │
│  REALMS:                                                                        │
│  ├── shared (40 nodes) → READ-ONLY sauf trait "imported"                        │
│  └── org (21 nodes) → WRITABLE (authored, generated)                            │
│                                                                                 │
│  TRAITS (Data Origin):                                                          │
│  ├── defined    → Schema-level (Entity, Page)                                   │
│  ├── authored   → Human-written (EntityNative) ← NIKA CAN WRITE                 │
│  ├── imported   → External API (SEOKeyword, Term) ← NIKA CAN WRITE              │
│  ├── generated  → LLM output (BlockNative) ← NIKA CAN WRITE                     │
│  └── retrieved  → Fetched on-demand                                             │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Key Insight**: Le trait determine ce que Nika peut ecrire:
- `imported` (SEOKeyword, Term) → Nika importe des donnees externes (Ahrefs)
- `authored` (EntityNative) → Nika peut creer/modifier du contenu
- `generated` (BlockNative) → Nika stocke les outputs LLM

### 2. Multiple Keywords per EntityNative (N:M)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  RELATION N:M: EntityNative ──[:TARGETS]──> SEOKeyword                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  EntityNative:qr-code@fr-FR                                                     │
│  │                                                                              │
│  ├──[:TARGETS {rank: "primary", is_slug_source: true}]──►                       │
│  │   SEOKeyword:qr-code@fr-FR (volume: 110,000)                                 │
│  │                                                                              │
│  ├──[:TARGETS {rank: "secondary", is_slug_source: false}]──►                    │
│  │   SEOKeyword:creer-qr-code@fr-FR (volume: 18,000)                            │
│  │                                                                              │
│  ├──[:TARGETS {rank: "secondary"}]──►                                           │
│  │   SEOKeyword:qr-code-gratuit@fr-FR (volume: 14,000)                          │
│  │                                                                              │
│  └──[:TARGETS {rank: "tertiary"}]──►                                            │
│      SEOKeyword:code-qr@fr-FR (volume: 8,500)                                   │
│                                                                                 │
│  IMPORTANT:                                                                     │
│  ├── Une EntityNative target PLUSIEURS keywords (10 typiquement)                │
│  ├── rank: primary | secondary | tertiary                                       │
│  ├── is_slug_source: true sur UN SEUL keyword (celui qui donne le slug URL)     │
│  └── Le keyword primary n'est PAS forcement le slug source!                     │
│      Ex: "creer qr code gratuit" est primary (plus de volume)                   │
│          mais "qr-code" est slug source (plus court, canonique)                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 3. Keyword Scoring Formula

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  FORMULE DE SCORING (decidee par Nika workflow)                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  score = volume × sem_coef × intent_boost × trend_factor                        │
│                                                                                 │
│  sem_coef (relation semantique avec l'Entity):                                  │
│  ├── same_as:      1.00  ("qr code" = Entity)                                   │
│  ├── action_for:   0.95  ("creer qr code" = action sur Entity)                  │
│  ├── produces:     0.85  ("generateur qr" = produit un QR)                      │
│  ├── subtopic_of:  0.70  ("qr code wifi" = sous-topic)                          │
│  ├── related_to:   0.50  ("flashcode" = lie)                                    │
│  └── attribute_of: 0.30  ("qr code couleur" = attribut)                         │
│                                                                                 │
│  intent_boost:                                                                  │
│  ├── transactional:  1.20  (user veut agir)                                     │
│  ├── commercial:     1.10  (user compare)                                       │
│  ├── informational:  1.00  (user apprend)                                       │
│  └── navigational:   0.80  (user cherche site)                                  │
│                                                                                 │
│  trend_factor:                                                                  │
│  ├── rising:   1.15                                                             │
│  ├── stable:   1.00                                                             │
│  └── declining: 0.85                                                            │
│                                                                                 │
│  EXEMPLE fr-FR:                                                                 │
│  "qr code"       → 110,000 × 1.00 × 1.00 × 1.00 = 110,000 ← GAGNANT             │
│  "creer qr code" → 18,000 × 0.95 × 1.20 × 1.00 = 20,520                         │
│  "code qr"       → 8,500 × 1.00 × 1.00 × 0.85 = 7,225 (declining)               │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 4. USES_TERM Selective Loading Pattern

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SELECTIVE LOADING via USES_TERM                                                │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  AVANT (4 hops, charge TOUT):                                                   │
│  EntityNative → FOR_LOCALE → Locale → HAS_TERMS → TermSet → CONTAINS → Term    │
│  └── Charge 20,000 Terms = explosion de tokens!                                 │
│                                                                                 │
│  APRES (1 hop, selectif):                                                       │
│  EntityNative ──[:USES_TERM {purpose, temperature, semantic_field}]──► Term    │
│  └── Charge ~50 Terms pertinents                                                │
│                                                                                 │
│  Arc USES_TERM properties:                                                      │
│  ├── purpose: primary | secondary | contextual | avoid                          │
│  ├── temperature: 0.0-1.0 (spreading activation weight)                         │
│  └── semantic_field: action | benefit | feature | comparison | technical        │
│                                                                                 │
│  QUERY EXEMPLE:                                                                 │
│  MATCH (en:EntityNative {key: $key})-[ut:USES_TERM]->(t:Term)                   │
│  WHERE ut.purpose IN ['primary', 'secondary']                                   │
│    AND ut.temperature > 0.5                                                     │
│  RETURN t.value, ut.purpose                                                     │
│  └── Retourne ~20 terms, pas 20,000!                                            │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 5. Schema Validation via Introspect

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VALIDATION "INTELLIGENTE" VIA NEO4J                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Les schemas sont DEJA dans Neo4j comme nodes :Class et :ArcClass!              │
│  Pas besoin de parser les YAML au runtime.                                      │
│                                                                                 │
│  FLOW DE VALIDATION:                                                            │
│                                                                                 │
│  novanet_write({node_type: "SEOKeyword", ...})                                  │
│       │                                                                         │
│       ▼                                                                         │
│  1. INTROSPECT CLASS                                                            │
│     MATCH (c:Class {name: "SEOKeyword"})                                        │
│     RETURN c.realm, c.trait, c.layer                                            │
│     → {realm: "shared", trait: "imported", layer: "knowledge"}                  │
│       │                                                                         │
│       ▼                                                                         │
│  2. VALIDATE REALM + TRAIT                                                      │
│     shared + imported = OK ✓                                                    │
│     shared + authored = BLOCKED ✗                                               │
│       │                                                                         │
│       ▼                                                                         │
│  3. GET MANDATORY ARCS                                                          │
│     MATCH (c:Class {name: $type})<-[:FROM_CLASS]-(a:ArcClass)                   │
│     WHERE a.cardinality = "many_to_one"                                         │
│     RETURN a.name, a.target                                                     │
│       │                                                                         │
│       ▼                                                                         │
│  4. EXECUTE MERGE (idempotent)                                                  │
│  5. CREATE MANDATORY ARCS (if auto_arcs: true)                                  │
│  6. INVALIDATE CACHE                                                            │
│  7. RETURN result                                                               │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Architecture Design

### Separation of Concerns

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  QUI FAIT QUOI?                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  THIBAUT (Schema Owner):                                                        │
│  ├── Definit les node-classes en YAML                                           │
│  ├── Definit les arc-classes en YAML                                            │
│  ├── Run: novanet schema generate                                               │
│  ├── Run: novanet db seed                                                       │
│  └── Controle la STRUCTURE du graphe                                            │
│                                                                                 │
│  NIKA (Data Producer):                                                          │
│  ├── Cree des INSTANCES de nodes existants                                      │
│  ├── Cree des ARCS entre nodes existants                                        │
│  ├── Update des proprietes sur nodes existants                                  │
│  └── Controle le CONTENU du graphe                                              │
│                                                                                 │
│  NIKA NE PEUT PAS:                                                              │
│  ├── Creer un nouveau type (Class)                                              │
│  ├── Creer un nouveau arc type (ArcClass)                                       │
│  ├── Modifier le schema                                                         │
│  └── Inventer des arcs qui n'existent pas                                       │
│                                                                                 │
│  ANALOGIE SQL:                                                                  │
│  ├── Thibaut = DBA (CREATE TABLE, ALTER TABLE)                                  │
│  └── Nika = Application (INSERT, UPDATE, SELECT)                                │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### The 3 Nodes Nika Writes

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NODES QUE NIKA ECRIT                                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. SEOKeyword (shared/knowledge, trait: imported)                              │
│  ───────────────────────────────────────────────────                            │
│  {                                                                              │
│    key: "seo:qr-code@fr-FR",        ← Auto: seo:{slug}@{locale}                 │
│    value: "qr code",                ← Le VRAI terme Ahrefs                      │
│    slug_form: "qr-code",            ← Forme URL-safe                            │
│    volume: 110000,                  ← Data Ahrefs                               │
│    difficulty: 45,                                                              │
│    traffic_potential: 85000,                                                    │
│    intent: "transactional",                                                     │
│    trend: "stable",                                                             │
│    source: "ahrefs",                                                            │
│    source_date: "2026-03-03"                                                    │
│  }                                                                              │
│                                                                                 │
│  2. EntityNative (org/semantic, trait: authored)                                │
│  ───────────────────────────────────────────────────                            │
│  {                                                                              │
│    key: "entity:qr-code@fr-FR",     ← Auto: entity:{key}@{locale}               │
│    entity_key: "qr-code",                                                       │
│    locale_key: "fr-FR",                                                         │
│    denomination_forms: [                                                        │
│      { type: "text", value: "qr code", priority: 1 },                           │
│      { type: "title", value: "QR Code", priority: 1 },                          │
│      { type: "abbrev", value: "qr", priority: 1 },                              │
│      { type: "url", value: "qr-code", priority: 1 }  ← POST-SEO pipeline        │
│    ],                                                                           │
│    curation_status: "ai_generated",                                             │
│    version: 1                                                                   │
│  }                                                                              │
│  ARCS: HAS_NATIVE (from Entity), FOR_LOCALE (to Locale), TARGETS (to Keywords)  │
│                                                                                 │
│  3. BlockNative (org/output, trait: generated)                                  │
│  ───────────────────────────────────────────────────                            │
│  {                                                                              │
│    key: "block:qr-code-hero@fr-FR", ← Auto: block:{key}@{locale}                │
│    block_key: "qr-code-hero",                                                   │
│    locale_key: "fr-FR",                                                         │
│    block_type: "hero",                                                          │
│    content: { title: "...", cta: "..." },  ← JSON genere par LLM                │
│    status: "draft",                                                             │
│    version: 1                                                                   │
│  }                                                                              │
│  ARCS: HAS_NATIVE (from Block), FOR_LOCALE, INFLUENCED_BY (to EntityNative)     │
│                                                                                 │
│  4. Term (shared/knowledge, trait: imported)                                    │
│  ───────────────────────────────────────────────────                            │
│  {                                                                              │
│    key: "term:qr-code@fr-FR",                                                   │
│    value: "qr code",                ← Le VRAI terme decouvert                   │
│    domain: "technical",                                                         │
│    register: "neutral",                                                         │
│    semantic_field: "domain",                                                    │
│    source: "ahrefs"                                                             │
│  }                                                                              │
│  ARCS: USES_TERM (from EntityNative, with purpose/temperature)                  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## The Virtuous Cycle

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CYCLE VERTUEUX: Nika ECRIT → NovaNet STOCKE → Nika LIT → LLM GENERE            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  PHASE 1: RECHERCHE (Ahrefs MCP)                                                │
│  ─────────────────────────────────                                              │
│  ahrefs_keywords(seed: "QR Code", country: "fr")                                │
│  └── Retourne 20 keywords avec volumes REELS                                    │
│      "qr code" (110K), "creer qr code" (18K), "code qr" (8.5K)...               │
│                    │                                                            │
│                    ▼                                                            │
│  PHASE 2: SCORING (Nika infer)                                                  │
│  ─────────────────────────────────                                              │
│  score = volume × sem_coef × intent_boost × trend_factor                        │
│  └── Determine le keyword gagnant et le slug source                             │
│                    │                                                            │
│                    ▼                                                            │
│  PHASE 3: ECRITURE (NovaNet MCP write) ← CE QU'ON IMPLEMENTE                    │
│  ─────────────────────────────────                                              │
│  ├── Creer SEOKeywords (10 par Entity×Locale)                                   │
│  ├── Creer/MAJ EntityNative avec denomination_forms                             │
│  ├── Creer Terms decouverts                                                     │
│  └── Creer arcs TARGETS, USES_TERM                                              │
│                    │                                                            │
│                    ▼                                                            │
│  PHASE 4: LECTURE FUTURE (Cycle vertueux)                                       │
│  ─────────────────────────────────                                              │
│  Quand Nika genere du contenu pour une AUTRE page:                              │
│  ├── novanet_atoms() → Retourne Terms avec value: "qr code" (pas "code qr"!)    │
│  ├── novanet_generate() → denomination_forms.text = "qr code" (CORRECT!)        │
│  └── Le LLM utilise UNIQUEMENT ces formes → PAS d'erreur                        │
│                                                                                 │
│  ════════════════════════════════════════════════════════════════════════════   │
│                                                                                 │
│  BOOTSTRAP PROBLEM:                                                             │
│  La premiere execution du workflow REMPLIT le graphe.                           │
│  Ensuite le graphe GUIDE les generations futures.                               │
│                                                                                 │
│  C'est pourquoi novanet_write est CRITIQUE:                                     │
│  Sans ecriture, le graphe reste vide et le LLM invente n'importe quoi.          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## API Design

### Single Tool with 3 Operations

```rust
// novanet_write - Single unified tool
pub struct WriteParams {
    pub operation: WriteOperation,
}

pub enum WriteOperation {
    /// Create or update a node (idempotent via MERGE)
    UpsertNode {
        node_type: String,       // Class name: "SEOKeyword", "EntityNative", etc.
        key: Option<String>,     // Auto-generated if not provided
        properties: JsonMap,     // Node properties
        auto_arcs: Option<bool>, // Create mandatory arcs (default: true)
    },

    /// Create an arc between two nodes
    CreateArc {
        arc_type: String,        // ArcClass name: "TARGETS", "FOR_LOCALE", etc.
        from_key: String,        // Source node key
        to_key: String,          // Target node key
        properties: Option<JsonMap>, // Arc properties (rank, is_slug_source, etc.)
    },

    /// Update properties on an existing node (partial update)
    UpdateProps {
        node_key: String,        // Existing node key
        properties: JsonMap,     // Properties to update (merged with existing)
    },
}

pub struct WriteResult {
    pub success: bool,
    pub key: Option<String>,           // Created/updated node key
    pub was_created: bool,             // true if new, false if updated
    pub created_arcs: Vec<String>,     // Auto-created arcs
    pub token_estimate: usize,
}
```

### Example Nika Workflow

```yaml
workflow: seo-research
mcp:
  servers:
    novanet: { command: "novanet-mcp" }
    ahrefs: { command: "ahrefs-mcp" }

tasks:
  # 1. Search keywords with Ahrefs
  - id: search_keywords
    invoke: ahrefs_keywords_search
    params:
      query: "qr code"
      locale: "fr-FR"
    use.result: keywords

  # 2. Create winning SEOKeyword
  - id: create_keyword
    invoke: novanet_write
    params:
      operation: upsert_node
      node_type: SEOKeyword
      properties:
        value: "{{use.keywords[0].keyword}}"
        volume: "{{use.keywords[0].volume}}"
        locale_key: "fr-FR"
        intent: "transactional"
        source: "ahrefs"
    use.result: keyword

  # 3. Link keyword to EntityNative
  - id: link_keyword
    invoke: novanet_write
    params:
      operation: create_arc
      arc_type: TARGETS
      from_key: "entity:qr-code@fr-FR"
      to_key: "{{use.keyword.key}}"
      properties:
        rank: primary
        is_slug_source: true

  # 4. Update EntityNative denomination_forms
  - id: update_entity
    invoke: novanet_write
    params:
      operation: update_props
      node_key: "entity:qr-code@fr-FR"
      properties:
        denomination_forms:
          - { type: "text", value: "qr code", priority: 1 }
          - { type: "title", value: "QR Code", priority: 1 }
          - { type: "url", value: "qr-code", priority: 1 }
```

---

## Implementation Plan

### Phase 1: Infrastructure (~500 lines)

**pool.rs** - Add write support
- [ ] `execute_write()` method with allowlist validation
- [ ] Allowlist: MERGE, CREATE (for arcs), SET
- [ ] Blocklist: DELETE, DROP, REMOVE
- [ ] Write audit logging

**error.rs** - Add write-specific errors
- [ ] `WriteBlocked { reason }` (realm/trait violation)
- [ ] `CardinalityViolation { arc, expected, actual }`
- [ ] `MissingMandatoryArc { arc_type }`
- [ ] `ClassNotFound { name }`
- [ ] `ArcClassNotFound { name }`

### Phase 2: Schema Validation (~300 lines)

**tools/write/schema.rs** - Schema validation helpers
- [ ] `validate_node_type()` - Check Class exists, get realm/trait
- [ ] `check_write_permission()` - Validate realm + trait allows write
- [ ] `get_mandatory_arcs()` - Query ArcClass with cardinality many_to_one
- [ ] `generate_key()` - Pattern-based key generation

### Phase 3: Write Tool (~600 lines)

**tools/write/mod.rs** - Main write tool
- [ ] `WriteParams` / `WriteResult` structs with JsonSchema
- [ ] `execute()` dispatcher for operation types
- [ ] Cache invalidation after write

**tools/write/upsert.rs** - Upsert node implementation
- [ ] MERGE Cypher generation
- [ ] Auto-arc creation based on mandatory arcs
- [ ] Key auto-generation from pattern

**tools/write/arc.rs** - Create arc implementation
- [ ] Arc property validation
- [ ] Source/target node existence check
- [ ] Cardinality validation (optional)

**tools/write/update.rs** - Update props implementation
- [ ] Partial property update via SET
- [ ] Node existence check

### Phase 4: Integration (~200 lines)

**server/handler.rs** - Register tool
- [ ] `#[tool(name = "novanet_write", description = "...")]`

**tests/** - Comprehensive test suite
- [ ] Unit tests for validation logic
- [ ] Integration tests with Neo4j
- [ ] Idempotency tests (call twice, same result)

**Total: ~1600 lines**

---

## Scale Considerations

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ECHELLE: 2000 Entities × 200 Locales                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Nodes a creer:                                                                 │
│  ├── EntityNative:     2,000 × 200 = 400,000 nodes                              │
│  ├── SEOKeyword:       2,000 × 200 × 10 = 4,000,000 nodes                       │
│  ├── Term:             Variable (~50 par EntityNative)                          │
│  ├── USES_TERM arcs:   400,000 × 50 = 20,000,000 arcs                           │
│  └── BlockNative:      500 pages × 200 × ~5 blocks = 500,000 nodes              │
│                                                                                 │
│  STRATEGIES:                                                                    │
│  ├── Batch processing par Locale (traiter fr-FR complet avant de-DE)            │
│  ├── Keyword convergence (1 SEOKeyword → N EntityNatives via TARGETS)           │
│  ├── Term reuse (1 Term → N EntityNatives via USES_TERM)                        │
│  └── novanet_batch pour operations paralleles                                   │
│                                                                                 │
│  PERFORMANCE:                                                                   │
│  ├── MERGE est O(1) avec index sur key                                          │
│  ├── Batch writes: grouper 100 operations par call                              │
│  └── Cache invalidation: par pattern (SEOKeyword:fr-FR:*)                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Decisions Made

| Question | Decision | Rationale |
|----------|----------|-----------|
| Single vs multiple tools? | Single `novanet_write` with operation enum | Simpler API, consistent patterns |
| YAML vs Neo4j schema? | Neo4j (via introspect pattern) | Already implemented, no new deps |
| Auto-arcs by default? | Yes, `auto_arcs: true` | Intelligent writes = less error-prone |
| Cache invalidation? | By node type pattern | `SEOKeyword:*` when writing SEOKeyword |
| YAML parser if needed? | `serde-saphyr` (like Nika) | `serde_yaml` is deprecated |

---

## Session References

- Briefing: `NEXT-SESSION-NOVANET-WRITE.md`
- Architecture: `02-architecture.md`
- Schema files: `packages/core/models/node-classes/`
- Introspect tool: `tools/novanet-mcp/src/tools/introspect.rs`
- Other terminal research: Keyword scoring formula, USES_TERM pattern
