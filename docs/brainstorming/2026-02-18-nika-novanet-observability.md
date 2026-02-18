# Brainstorming: Nika-NovaNet Observability Layer

**Date:** 2026-02-18
**Participants:** Thibaut, Claude
**Status:** Design validé

---

## Contexte

On développe une couche d'observabilité pour voir ce que Nika fait quand il exécute des workflows avec NovaNet via MCP.

## Problème

Quand on exécute un workflow Nika avec une tâche `agent:` qui utilise NovaNet via MCP :
- On ne voit pas les appels MCP effectués
- On ne voit pas comment l'agent parcourt le graphe ontologique
- On ne peut pas debugger ni optimiser les workflows
- On ne peut pas garantir la reproductibilité

## Objectifs

1. **Reproducibilité** → même YAML = même comportement
2. **Debugging live** → voir en temps réel ce qui se passe
3. **Post-mortem** → analyser et ajuster après exécution
4. **Compréhension** → voir le "chemin" dans le knowledge graph

## Ce qu'on veut voir

### 1. Les appels MCP
- Quels tools appelés (`novanet_generate`, `novanet_traverse`...)
- Avec quels paramètres
- Ce qui a été retourné
- Durée, tokens, cache hit/miss

### 2. Le parcours dans le graphe ontologique
- Par quels noeuds l'agent est passé (Entity → EntityNative → Block → Terms...)
- Quels arcs ont été traversés (HAS_NATIVE, REPRESENTS, USES_TERM...)
- Visualiser le "chemin" dans le knowledge graph
- **Pourquoi** certains noeuds ont été exclus (budget, relevance, depth)

### 3. Le raisonnement de l'agent
- Pourquoi il a fait tel appel MCP
- Ce qu'il a "compris" du contexte retourné
- Ses décisions à chaque tour (turn)

---

## Design: Trace Data Model (FULL Granularity)

### ExecutionTrace Structure

```yaml
trace:
  version: "1.0"
  generation_id: "2026-02-18T14-32-00-a3f9"  # Unique par run
  workflow_hash: "sha256:a1b2c3d4..."        # Pour reproductibilité

  execution:
    started_at: "2026-02-18T14:32:00.000Z"
    completed_at: "2026-02-18T14:32:12.450Z"
    duration_ms: 12450
    status: success

  workflow:
    source_file: "workflows/generate-page.yaml"
    params:
      entity: "qr-code"
      locale: "fr-FR"
      block: "head-seo-meta"

  turns: [AgentTurn]
```

### AgentTurn Structure

```yaml
turn:
  turn_number: 1
  timestamp: "2026-02-18T14:32:00.100Z"
  reasoning: "Need entity context for qr-code"
  decision: "Query NovaNet for page description"

  mcp_calls:
    - tool: "novanet_generate"
      call_id: "uuid-1234"
      params: { mode: block, page_key: qr-code, locale: fr-FR }
      timing:
        started_at: "2026-02-18T14:32:00.120Z"
        duration_ms: 120
      response:
        size_bytes: 4200
        cached: false

  graph_traversal:
    start_node: { label: Page, key: "qr-code" }
    steps:
      - arc: HAS_BLOCK
        direction: outgoing
        props_matched: { order: 0 }
        target:
          label: Block
          key: "qr-code:head-seo-meta:1"
          properties_read: [key, block_type_key]

      - arc: HAS_NATIVE
        direction: outgoing
        props_matched: { locale: "fr-FR" }
        target:
          label: EntityNative
          key: "entity:qr-code@fr-FR"
          properties_read: [denomination_forms, llm_context]
          properties_values:
            denomination_forms:
              - { type: text, value: "qr code" }
              - { type: title, value: "QR Code" }
              - { type: url, value: "créer-qr-code" }

  context_assembled:
    total_tokens: 4200
    budget_used_pct: 0.84

    sources:
      - node: "EntityNative:qr-code@fr-FR"
        tokens: 850
        fields: [denomination_forms, llm_context]
        traversal_path:
          - "Page:qr-code"
          - "[:REPRESENTS]"
          - "Entity:qr-code"
          - "[:HAS_NATIVE {locale: fr-FR}]"
          - "EntityNative:qr-code@fr-FR"

      - node: "SEOKeyword:creer-qr-code@fr-FR"
        tokens: 120
        fields: [value, volume, slug_form]
        traversal_path:
          - "EntityNative:qr-code@fr-FR"
          - "[:TARGETS {rank: primary}]"
          - "SEOKeyword:creer-qr-code@fr-FR"

      - node: "Slugification:fr-FR"
        tokens: 340
        fields: [rule, stop_words]

    # NOUVEAU: Ce qui a été EXCLU et pourquoi
    excluded:
      - node: "Term:code-barre@fr-FR"
        reason: "relevance_below_threshold"
        relevance_score: 0.23
        threshold: 0.50
        tokens_would_add: 180

      - node: "Expression:générer-qr@fr-FR"
        reason: "budget_exhausted"
        tokens_would_add: 340
        budget_remaining: 150

    # NOUVEAU: Log de troncature si budget dépassé
    truncation:
      truncated: true
      budget_exhausted_at_node: "EntityNative:barcode@fr-FR"
      nodes_dropped: 2
      tokens_saved: 1010

    transformations:
      - type: stop_word_removal
        input: "créer un qr code"
        output: "créer qr code"
        rule_applied: "fr-FR.stop_words contains 'un'"
        source_node: "Slugification:fr-FR"

      - type: slug_generation
        input: "créer qr code"
        output: "créer-qr-code"
        rule_applied: "latin_preserve (accents kept, spaces → hyphens)"

  llm_generation:
    request_id: "req_018EeWyXxfu5pfWkrYcMdjWG"  # Anthropic support
    model: "claude-sonnet-4-20250514"
    prompt_hash: "xxh3:f8e2a1b4"  # Pour reproductibilité (pas le texte)

    tokens:
      input: 4200
      output: 380
      cache_read: 2100
      cache_creation: 0
      total: 4580

    cost_usd: 0.0127
    latency_ms: 8500
    ttft_ms: 120  # Time-to-first-token
    finish_reason: "end_turn"  # end_turn | max_tokens | tool_use | refusal

    output:
      slug: "créer-qr-code"
      meta_title: "Créer un QR Code - Générateur Gratuit"
      meta_description: "Créez des QR codes personnalisés gratuitement..."

  arcs_created:
    - arc: DERIVED_SLUG_FROM
      from: "BlockNative:qr-code:head-seo-meta:1@fr-FR"
      to: "EntityNative:qr-code@fr-FR"
      props: { field: "denomination_forms.url" }

    - arc: SLUGIFIED_BY
      from: "BlockNative:qr-code:head-seo-meta:1@fr-FR"
      to: "Slugification:fr-FR"
      props: { applied_rule: "latin_preserve" }
```

### Summary Block

```yaml
summary:
  turns_count: 3
  mcp_calls_count: 2
  nodes_traversed: 8
  nodes_included: 6
  nodes_excluded: 2
  arcs_traversed: 7
  arcs_created: 2
  total_tokens:
    context: 4200
    completion: 380
  total_cost_usd: 0.0127
  transformations_applied: 2
```

---

## Design: TUI Visualization (FULL Granularity)

Layout principal - 4 panels synchronisés:

```
┌─────────────────────────────────────────────────────────────────────────┐
│ [1] WORKFLOW PROGRESS                                    ▶ RUNNING 12s │
├─────────────────────────────────────────────────────────────────────────┤
│ ✅ step:1 invoke:novanet_describe    45ms   2.3K tokens  $0.003       │
│ ✅ step:2 invoke:novanet_generate   120ms   4.2K tokens  $0.008       │
│ ▶️ step:3 infer:generate-content     ...    ~8K tokens (streaming)     │
│ ⏳ step:4 invoke:novanet_persist                                       │
└─────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────┐ ┌────────────────────────────────────────┐
│ [2] GRAPH TRAVERSAL         │ │ [3] CONTEXT ASSEMBLED                  │
├──────────────────────────────┤ ├────────────────────────────────────────┤
│                              │ │ Sources (4.2K tokens, 84% budget):     │
│  Page:qr-code                │ │ ┌────────────────────────────────────┐ │
│    │                         │ │ │ EntityNative:qr-code@fr-FR  850t  │ │
│    ├─[:HAS_BLOCK {order:0}]  │ │ │ ├─ denomination_forms:            │ │
│    │                         │ │ │ │  text: "qr code"                │ │
│    ▼                         │ │ │ │  title: "QR Code"               │ │
│  Block:head-seo-meta:1       │ │ │ │  url: "créer-qr-code" ✓         │ │
│    │                         │ │ │ └─ llm_context: "..."             │ │
│    ├─[:OF_TYPE]──▶ BlockType │ │ └────────────────────────────────────┘ │
│    │   schema: {slug,        │ │ ┌────────────────────────────────────┐ │
│    │    meta_title,          │ │ │ SEOKeyword:creer-qr-code@fr  120t │ │
│    │    meta_description}    │ │ │ ├─ value: "créer qr code"         │ │
│    │                         │ │ │ ├─ volume: 14000                  │ │
│    └─[:HAS_INSTRUCTION]      │ │ │ └─ slug_form: "créer-qr-code"     │ │
│        ▼                     │ │ └────────────────────────────────────┘ │
│      BlockInstruction        │ │                                        │
│        (markdown+@refs)      │ │ Excluded (2 nodes):                    │
│                              │ │ ┌────────────────────────────────────┐ │
│  Page:qr-code                │ │ │ ❌ Term:code-barre@fr-FR          │ │
│    │                         │ │ │    relevance: 0.23 < 0.50         │ │
│    └─[:REPRESENTS]           │ │ │ ❌ Expression:générer-qr@fr-FR    │ │
│        ▼                     │ │ │    budget exhausted (340t needed) │ │
│      Entity:qr-code          │ │ └────────────────────────────────────┘ │
│        │                     │ │                                        │
│        └─[:HAS_NATIVE]       │ │ Transformations:                       │
│            {locale:fr-FR}    │ │ ┌────────────────────────────────────┐ │
│            ▼                 │ │ │ 🔄 stop_word_removal               │ │
│          EntityNative ◀──────┼─┼─│   "créer un qr code"               │ │
│            @fr-FR            │ │ │         ↓                          │ │
│                              │ │ │   "créer qr code"                  │ │
└──────────────────────────────┘ └────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│ [4] AGENT REASONING                                          Turn 2/3  │
├─────────────────────────────────────────────────────────────────────────┤
│ 💭 "EntityNative has denomination_forms with url='créer-un-qr-code'    │
│     but Slugification:fr-FR defines stop_words=['un','une','le'...]    │
│     Applying latin_preserve rule: remove stop word 'un'                │
│     → Final slug: 'créer-qr-code'"                                     │
│                                                                         │
│ 📤 Decision: Generate head-seo-meta with slug="créer-qr-code"          │
│              meta_title="Créer un QR Code - Générateur Gratuit"        │
│                                                                         │
│ 💰 Tokens: 4200 in / 380 out | Cost: $0.0127 | Latency: 8.5s           │
│ 🔑 Request ID: req_018EeWyXxfu5pfWkrYcMdjWG                            │
└─────────────────────────────────────────────────────────────────────────┘

Keyboard: [←→] Navigate turns  [Tab] Focus panel  [g] Graph zoom
          [c] Context details  [r] Raw JSON       [s] Save trace
          [x] Show excluded    [t] Token breakdown [e] Export YAML
```

---

## Design: Trace File Format

### Format Principal: NDJSON (Newline-Delimited JSON)

```
.nika/traces/
├── 2026-02-18T14-32-00-a3f9.ndjson   ← Primary (append-only, jq-compatible)
├── 2026-02-18T14-32-00-a3f9.yaml     ← Export on-demand (human-readable)
└── index.yaml                         ← Metadata de tous les runs
```

**Pourquoi NDJSON:**
- `tail -f` streaming pendant l'exécution
- `jq` post-processing sans charger tout le fichier
- Append-only (pas de réécriture à la fermeture)
- TUI peut `watch` le fichier indépendamment

### CLI Commands

```bash
nika trace list                    # Liste les traces récentes
nika trace show <generation_id>    # Affiche un trace dans TUI
nika trace export --format yaml    # Export YAML pour lecture humaine
nika trace diff <id1> <id2>        # Compare deux exécutions
nika trace replay <id>             # Rejoue avec même params
```

---

## Design: Architecture Rust

### Dual-Track Observability

```
EventLog (Arc<mpsc::UnboundedSender>)
     │
     ├──► Vec<Event> snapshot  (queries: filter_task, events())
     │
     ├──► BufWriter<File>      (.nika/traces/{generation_id}.ndjson)
     │
     └──► broadcast::Sender    (TUI real-time, lag-tolerant)
```

### Nouveaux EventKind (6 variants à ajouter)

```rust
// src/event/log.rs

// MCP Level
McpToolCalled {
    task_id: Arc<str>,
    call_id: Uuid,
    tool: Arc<str>,
    params: Arc<Value>,
},
McpToolResponded {
    call_id: Uuid,
    task_id: Arc<str>,
    tool: Arc<str>,
    duration_ms: u64,
    response_size_bytes: u32,
    cached: bool,
    is_error: bool,
},

// Context Assembly Level
ContextAssembled {
    task_id: Arc<str>,
    sources: SmallVec<[ContextSource; 4]>,
    excluded: SmallVec<[ExcludedNode; 4]>,
    total_tokens: u32,
    budget_used_pct: f32,
    truncated: bool,
},
ContextTruncated {
    task_id: Arc<str>,
    budget_tokens: u32,
    actual_tokens: u32,
    nodes_dropped: u32,
},

// Agent Turn Level
AgentTurnStarted {
    task_id: Arc<str>,
    turn_index: u32,
    context_tokens: u32,
},
AgentTurnCompleted {
    task_id: Arc<str>,
    turn_index: u32,
    mcp_calls_made: u32,
    duration_ms: u64,
},
```

### Extensions à ProviderCalled/ProviderResponded

```rust
// Extend existing ProviderCalled
ProviderCalled {
    task_id: Arc<str>,
    call_id: Uuid,
    provider: String,
    model: String,
    prompt_len: usize,
    prompt_hash: u64,              // xxhash (pas le texte complet)
    context_source_task_ids: Vec<Arc<str>>,  // Provenance
},

// Extend existing ProviderResponded
ProviderResponded {
    task_id: Arc<str>,
    call_id: Uuid,
    request_id: String,            // Anthropic support correlation

    // Token split
    input_tokens: u32,
    output_tokens: u32,
    cache_read_tokens: u32,
    cache_creation_tokens: u32,

    // Quality signals
    finish_reason: FinishReason,   // EndTurn | MaxTokens | ToolUse | Refusal
    latency_ms: u64,
    ttft_ms: Option<u64>,          // Time-to-first-token (streaming)

    // Cost
    cost_usd: f64,
},
```

### Extension WorkflowStarted

```rust
WorkflowStarted {
    task_count: usize,
    generation_id: Arc<str>,       // Unique per run
    workflow_hash: String,         // SHA-256 du YAML
    nika_version: String,
    timestamp_utc: String,
},
```

---

## Design: NovaNet MCP Response Extensions

### Ajouter à novanet_assemble response

```rust
pub struct AssembleResult {
    // Existing fields...
    pub evidence: Vec<EvidencePacket>,
    pub total_tokens: usize,
    pub truncated: bool,

    // NEW: Context build log
    pub context_build_log: Vec<ContextBuildEntry>,
    pub truncation_log: Option<TruncationLog>,
}

pub struct ContextBuildEntry {
    pub node_key: String,
    pub node_kind: String,
    pub included: bool,
    pub exclusion_reason: Option<ExclusionReason>,
    pub tokens_allocated: usize,
    pub traversal_path: Vec<String>,
    pub relevance_score: f32,
    pub cache_hit: bool,
}

pub enum ExclusionReason {
    BudgetExceeded { tokens_would_add: usize, budget_remaining: usize },
    RelevanceBelowThreshold { score: f32, threshold: f32 },
    DepthLimitReached { depth: usize, max_depth: usize },
    AlreadyVisited,
    FilteredByKind,
}

pub struct TruncationLog {
    pub budget_exhausted_at_node: String,
    pub nodes_dropped: Vec<DroppedNode>,
    pub tokens_saved: usize,
}

pub struct DroppedNode {
    pub key: String,
    pub kind: String,
    pub estimated_tokens: usize,
    pub reason: String,
}
```

---

## Use Case Concret (avec vraies données Neo4j)

> **Note**: Données extraites de `novanet-dev/packages/db/seed/54-denomination-forms.cypher`
> Entity: `qr-code`, locale: `fr-FR`, slugification rule: `latin_preserve`

```
Demande: "Crée le block head-seo-meta de la page qr-code en fr-FR"

Agent Turn 1:
  └── Appel: novanet_describe(target: page, filters: {key: qr-code})
  └── call_id: uuid-001
  └── Parcours: (recherche dans l'index)
  └── Résultat: Page trouvée avec 5 blocks (order 0 = head-seo-meta)
  └── duration_ms: 45, cached: false

Agent Turn 2:
  └── Appel: novanet_generate(mode: block, page_key: qr-code,
                              block_key: head-seo-meta, locale: fr-FR)
  └── call_id: uuid-002
  └── Parcours dans le graphe:

      Page:qr-code
        │
        ├──[:HAS_BLOCK {order: 0}]──> Block:qr-code:head-seo-meta:1
        │                                  │
        │                                  ├──[:OF_TYPE]──> BlockType:head-seo-meta
        │                                  │                   schema: {slug, meta_title, meta_description}
        │                                  │
        │                                  └──[:HAS_INSTRUCTION]──> BlockInstruction
        │                                                           (markdown avec @ refs)
        │
        └──[:REPRESENTS]──> Entity:qr-code
                                │
                                ├── denomination_forms (invariant EN):
                                │   ├── text:   "qr code"
                                │   ├── title:  "QR Code"
                                │   └── abbrev: "qr"
                                │
                                └──[:HAS_NATIVE {locale: fr-FR}]──> EntityNative:qr-code@fr-FR
                                                                        │
                                                                        ├── denomination_forms:
                                                                        │   ├── text:   "qr code"       (prose)
                                                                        │   ├── title:  "QR Code"       (H1/H2)
                                                                        │   ├── abbrev: "qr"            (short)
                                                                        │   └── url:    "créer-qr-code" ← stop word removed!
                                                                        │
                                                                        └──[:TARGETS {rank: primary}]──> SEOKeyword
                                                                                                           value: "créer qr code"
                                                                                                           slug_form: "créer-qr-code"
                                                                                                           volume: 14000

  └── Context assemblé:
      ├── Sources incluses (6 nodes, 4200 tokens)
      ├── Exclusions:
      │   ├── Term:code-barre@fr-FR (relevance 0.23 < 0.50)
      │   └── Expression:générer-qr@fr-FR (budget exhausted)
      └── Transformations:
          └── stop_word_removal: "créer un qr code" → "créer qr code"

Agent Turn 3:
  └── Génération du contenu head-seo-meta avec le contexte
  └── LLM Call:
      ├── request_id: req_018EeWyXxfu5pfWkrYcMdjWG
      ├── prompt_hash: xxh3:f8e2a1b4
      ├── tokens: 4200 in / 380 out / 2100 cache_read
      ├── cost: $0.0127
      ├── latency: 8.5s (ttft: 120ms)
      └── finish_reason: end_turn

  └── Output (BlockNative:qr-code:head-seo-meta:1@fr-FR):
      {
        "slug": "créer-qr-code",              ← stop word "un" removed!
        "meta_title": "Créer un QR Code - Générateur Gratuit",
        "meta_description": "Créez des QR codes personnalisés gratuitement..."
      }

  └── Arcs créés:
      BlockNative ──[:DERIVED_SLUG_FROM]──> EntityNative (provenance)
      BlockNative ──[:SLUGIFIED_BY {applied_rule: 'latin_preserve'}]──> Slugification:fr-FR (validation)
```

---

## Implementation Roadmap

### Phase 1: Foundation (Nika)
- [ ] Add `generation_id` to `WorkflowStarted`
- [ ] Extend `ProviderCalled` with `prompt_hash`, `call_id`
- [ ] Extend `ProviderResponded` with token split, `finish_reason`, `request_id`, `cost_usd`
- [ ] Switch EventLog to channel-based drain (mpsc::UnboundedSender)

### Phase 2: MCP Observability (Nika)
- [ ] Add `McpToolCalled` variant
- [ ] Add `McpToolResponded` variant
- [ ] Implement when `invoke:` verb lands in AST

### Phase 3: Context Assembly (Nika + NovaNet)
- [ ] Add `ContextAssembled` variant
- [ ] Add `ContextTruncated` variant
- [ ] NovaNet: Add `context_build_log` to `novanet_assemble` response
- [ ] NovaNet: Add `truncation_log` with dropped nodes

### Phase 4: Agent Turns (Nika)
- [ ] Add `AgentTurnStarted` variant
- [ ] Add `AgentTurnCompleted` variant
- [ ] Implement when `agent:` verb lands in AST

### Phase 5: TUI (Nika)
- [ ] Implement 4-panel layout
- [ ] Add broadcast::Sender for real-time updates
- [ ] Keyboard navigation between turns
- [ ] Graph visualization with ratatui

### Phase 6: Trace Files (Nika)
- [ ] NDJSON writer (BufWriter<File>)
- [ ] `nika trace list` command
- [ ] `nika trace show` command (TUI replay)
- [ ] `nika trace export --format yaml` command

---

## Research Sources

Cette conception a été validée par 6 agents spécialisés:

1. **NovaNet Explore Agent** → Patterns MCP existants, generate 7-phase pipeline
2. **Nika Explore Agent** → EventLog architecture, verbs disponibles
3. **Rust Architect Agent** → Channel-based drain, NDJSON, SmallVec patterns
4. **Web Research Agent** → LangSmith/LangFuse/W&B best practices
5. **Claude API/SDK Agent** → Token accounting, finish_reason, request_id
6. **Code Architect Agent** → Context assembly, exclusion reasons, truncation

---

## Notes de session

- **Granularité choisie**: FULL (maximum d'informations)
- **Format principal**: NDJSON (append-only, jq-compatible)
- **TUI**: 4 panels synchronisés avec navigation par turn
- **Stop words**: Règle de slugification retire "un" → "créer qr code" pas "créer un qr code"
- **Prompt storage**: Hash seulement (xxhash), pas le texte complet par défaut
