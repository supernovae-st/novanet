# NovaNet - Orchestrator Flow v7.0.0

> **v7.0.0**: Standard node properties (key, display_name, icon, description, llm_context).
>
> **v7.0.0**: Unified relations: `PAGE_USES_CONCEPT`/`BLOCK_USES_CONCEPT` → `USES_CONCEPT`, `HAS_PAGE_OUTPUT`/`HAS_BLOCK_OUTPUT` → `HAS_OUTPUT`
>
> **v7.0.0**: Locale.code → Locale.key, llm_hints → llm_context

## Vue d'ensemble

```
                              { page_key, locale }
                                      │
                                      ▼
┌──────────────────────────────────────────────────────────────────────┐
│                         ORCHESTRATOR AGENT                           │
│                                                                      │
│   ✓ ADN projet                                                       │
│   ✓ Page + Blocks + Instructions                                     │
│   ✓ Concepts (USES_CONCEPT) + ConceptL10n                            │
│   ✓ Locale Knowledge (via :FOR_LOCALE → Locale → :HAS_*)            │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
                                      │
            ┌─────────────────────────┼─────────────────────────┐
            ▼                         ▼                         ▼
┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐
│     SUB-AGENT 1     │  │     SUB-AGENT 2     │  │     SUB-AGENT N     │
│    Block: SEO       │  │    Block: Hero      │  │    Block: FAQ       │
│                     │  │                     │  │                     │
│ Reçoit:             │  │ Reçoit:             │  │ Reçoit:             │
│ • ADN, Concepts     │  │ • ADN, Concepts     │  │ • ADN, Concepts     │
│ • Locale Knowledge  │  │ • Locale Knowledge  │  │ • Locale Knowledge  │
│ • Page+Blocks (awr) │  │ • Page+Blocks (awr) │  │ • Page+Blocks (awr) │
│                     │  │                     │  │                     │
│ Charge:             │  │ Charge:             │  │ Charge:             │
│ • Spreading         │  │ • Spreading         │  │ • Spreading         │
│ • SEO, GEO          │  │ • SEO, GEO          │  │ • SEO, GEO          │
└─────────────────────┘  └─────────────────────┘  └─────────────────────┘
            │                         │                         │
            └─────────────────────────┼─────────────────────────┘
                                      ▼
                    ┌────────────────────────────────┐
                    │        ORCHESTRATEUR           │
                    │     Assemble + Cohérence       │
                    │                                │
                    │  ✓ INFLUENCED_BY provenance    │
                    └────────────────────────────────┘
                                      │
                                      ▼
                    ┌────────────────────────────────┐
                    │      HOOK: JSON SCHEMA         │
                    │   ✓ Pass → Save   ✗ Fail → Retry
                    └────────────────────────────────┘
                                      │
                                      ▼
                              ┌────────────┐
                              │ PageL10n │
                              │  → Neo4j   │
                              └────────────┘
```

---

## Locale Knowledge Architecture (v7.0.0)

```
Locale {key: "fr-FR"}
    │
    ├──[:HAS_IDENTITY]──▶ LocaleIdentity {script, timezone, formats}
    ├──[:HAS_VOICE]─────▶ LocaleVoice {formality, tone, pronouns}
    ├──[:HAS_CULTURE]───▶ LocaleCulture {norms, taboos, references}
    ├──[:HAS_MARKET]────▶ LocaleMarket {context, competitors}
    └──[:HAS_LEXICON]───▶ LocaleLexicon
                              │
                              └──[:HAS_EXPRESSION]──▶ Expression (x90)

Content ──[:FOR_LOCALE]──▶ Locale
```

---

## Qui charge quoi ?

| Neo4j                                               | Orchestrateur | Sub-Agent (reçoit) | Sub-Agent (charge) |
|-----------------------------------------------------|:-------------:|:------------------:|:------------------:|
| ADN projet                                          |       ✓       |         ✓          |                    |
| Page + Blocks                                       |       ✓       |    ✓ (awareness)   |                    |
| Concepts (via :USES_CONCEPT)                        |       ✓       |         ✓          |                    |
| ConceptL10n (via :FOR_LOCALE)                       |       ✓       |         ✓          |                    |
| Locale Knowledge (Identity, Voice, Culture, etc.)   |       ✓       |    ✓ (ce block)    |                    |
| Spreading (max 2 hops, cutoff 0.3)                  |               |                    |         ✓          |
| SEO (:TARGETS_SEO)                                  |               |                    |         ✓          |
| GEO (:TARGETS_GEO)                                  |               |                    |         ✓          |

**Orchestrateur** = compile tout, dispatch avec le contexte pertinent par block.
**Sub-Agent** = reçoit son contexte, enrichit via Neo4j (spreading, SEO/GEO), génère son block.

---

## Queries Neo4j (v7.0.0)

### Locale Knowledge Query (graph-native)

```cypher
// Load full Locale Knowledge for content generation
MATCH (l:Locale {key: $locale})
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(lm:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
RETURN l, li, lv, lc, lm, ll, collect(e) AS expressions
```

### Page Context (orchestrator)

```cypher
// Load full page context for orchestrator
MATCH (p:Page {key: $pageKey})-[:HAS_BLOCK]->(b:Block)
MATCH (b)-[:OF_TYPE]->(bt:BlockType)
MATCH (l:Locale {key: $locale})
OPTIONAL MATCH (p)-[:USES_CONCEPT]->(c:Concept)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l)
RETURN p,
       collect(DISTINCT {block: b, type: bt, position: b.position}) AS blocks,
       collect(DISTINCT {concept: c.key, title: cl.title, definition: cl.definition}) AS concepts
ORDER BY b.position
```

### Block Context with Locale Knowledge (sub-agent)

```cypher
// Load Locale Knowledge context for a specific block
MATCH (b:Block {key: $blockKey})-[:OF_TYPE]->(bt:BlockType)
MATCH (l:Locale {key: $locale})
MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)
OPTIONAL MATCH (ll)-[:HAS_EXPRESSION]->(e:Expression)
WHERE e.semantic_field IN bt.semantic_fields
RETURN bt.key AS blockType,
       lv.formality, lv.tone, lv.pronouns,
       collect(e) AS expressions
```

### Spreading Activation (sub-agent)

```cypher
// Spreading activation from block concepts (v7.0.0: USES_CONCEPT unified)
MATCH (b:Block {key: $blockKey})-[:USES_CONCEPT]->(c:Concept)
MATCH (c)-[r:SEMANTIC_LINK*1..2]->(c2:Concept)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH c2, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
MATCH (c2)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN c2.key, cl.title, cl.definition, activation
ORDER BY activation DESC
```

### SEO Keywords (sub-agent)

```cypher
// Load SEO keywords for block concepts (v7.0.0: USES_CONCEPT unified)
MATCH (b:Block {key: $blockKey})-[:USES_CONCEPT]->(c:Concept)
      -[:TARGETS_SEO {status: "active"}]->(kw:SEOKeywordL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN c.key AS concept, kw.value AS keyword, kw.volume, kw.difficulty
ORDER BY kw.volume DESC
LIMIT 10
```

### GEO Seeds (sub-agent)

```cypher
// Load GEO seeds for block concepts (v7.0.0: USES_CONCEPT unified)
MATCH (b:Block {key: $blockKey})-[:USES_CONCEPT]->(c:Concept)
      -[:TARGETS_GEO {status: "active"}]->(gs:GEOSeedL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN c.key AS concept, gs.value AS query, gs.format, gs.target_answer
```

---

## Phase 3: Sub-Agent travaille

```
REÇU DE L'ORCHESTRATEUR          CHARGÉ DEPUIS NEO4J
─────────────────────────        ─────────────────────
• ADN projet                     • Spreading (concepts liés)
• Page + Blocks (awareness)      • SEOKeywordL10n (via :TARGETS_SEO)
• Concepts de la page            • GEOSeedL10n (via :TARGETS_GEO)
• Locale Knowledge (Voice, etc.)
• Instructions block
• Rules SEO/UX

Le sub-agent CONNAÎT tous les blocks mais ne GÉNÈRE que le sien.

                    │
                    ▼

┌─────────────────────────────────────────────────────────────────┐
│  ÉTAPE 1: SPREADING                                             │
│  tier-pro ──[:SEMANTIC_LINK]──▶ bulk-create (0.85)              │
│  tier-pro ──[:SEMANTIC_LINK]──▶ custom-qr (0.75)                │
├─────────────────────────────────────────────────────────────────┤
│  ÉTAPE 2: SEO/GEO (v7.0.0 - explicit relations + :FOR_LOCALE)  │
│  Concept ─[:TARGETS_SEO]─▶ SEOKeywordL10n ─[:FOR_LOCALE]─▶ Locale   │
│  Concept ─[:TARGETS_GEO]─▶ GEOSeedL10n ─[:FOR_LOCALE]─▶ Locale      │
├─────────────────────────────────────────────────────────────────┤
│  ÉTAPE 3: GÉNÉRATION                                            │
│  → BlockL10n (JSON) using LocaleVoice + LocaleLexicon         │
├─────────────────────────────────────────────────────────────────┤
│  ÉTAPE 4: PROVENANCE (v7.0.0)                                   │
│  BlockL10n ─[:INFLUENCED_BY {weight}]─▶ ConceptL10n           │
└─────────────────────────────────────────────────────────────────┘
```

---

## Provenance Tracking (v7.0.0)

Quand un sub-agent génère un BlockL10n, il track les ConceptL10n qui ont influencé:

```cypher
// After generation, record provenance (v7.0.0: HAS_OUTPUT, USES_CONCEPT unified)
MATCH (bo:BlockL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
MATCH (bo)<-[:HAS_OUTPUT]-(b:Block {key: $blockKey})
MATCH (cl:ConceptL10n)-[:FOR_LOCALE]->(l)
MATCH (cl)<-[:HAS_L10N]-(c:Concept)<-[:USES_CONCEPT]-(b)
CREATE (bo)-[:INFLUENCED_BY {weight: $weight, generated_at: datetime()}]->(cl)
```

Utilité:
- **Debugging**: "Pourquoi ce contenu parle de X?" → suivre INFLUENCED_BY
- **Régénération**: Si ConceptL10n change → identifier BlockL10n à regénérer
- **Analytics**: Quels concepts sont les plus utilisés?

---

## Prompts

### Orchestrateur

```
Tu es un orchestrateur de génération de contenu.
Ta mission: dispatcher et coordonner les sub-agents pour générer
la page '{page_key}' en '{locale}'.

Tu NE génères PAS de contenu toi-même.
Tu dispatches, tu valides, tu assembles.

Relations v7.0.0 (graph-native Locale):
- USES_CONCEPT pour les concepts (Page ou Block)
- HAS_OUTPUT pour le contenu généré (PageL10n, BlockL10n)
- FOR_LOCALE pour lier content → Locale
- HAS_IDENTITY, HAS_VOICE, HAS_CULTURE, HAS_MARKET, HAS_LEXICON pour Locale Knowledge
```

### Sub-Agent

```
Tu es un générateur de contenu localisé.
Ta mission: générer le contenu du block '{block_key}' pour la page
'{page_key}' en '{locale}'.

Tu fais partie d'un plan où {N} agents génèrent chacun un block.
Toi tu gères: {block_key} (block {position}/{total}).
Les autres agents gèrent: {autres_blocks}.

Tu GÉNÈRES uniquement le contenu de TON block.
Tu ne dois PAS générer de contenu pour les autres blocks.

Utilise LocaleVoice pour le ton et la formalité.
Utilise LocaleLexicon/Expression pour le vocabulaire préféré.

Après génération, track INFLUENCED_BY pour les ConceptL10n utilisés.
```

---

## Hot Paths (indexes critiques - v7.0.0)

| Query Pattern | Index | Fréquence |
|---------------|-------|-----------|
| Locale by key | `locale_key` (UNIQUE) | Très haute |
| Content via FOR_LOCALE | `for_locale_idx` | Très haute |
| ConceptL10n via FOR_LOCALE | `cl10n_for_locale` | Haute |
| SEOKeywordL10n via FOR_LOCALE | `seo_for_locale` | Haute |
| Spreading activation | `sl_temp` (relation) | Haute |
| LocaleLexicon expressions | `expr_semantic_field` | Moyenne |
