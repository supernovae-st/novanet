# Standard Properties Migration Plan

**Version**: v0.19.0
**Date**: 2026-03-11
**Status**: ✅ VALIDATED (brainstorming complete)
**ADRs**: ADR-037 (Standard Properties), ADR-042 (Provenance Tracking)

---

## Executive Summary

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  v0.19.0 STANDARD PROPERTIES — VALIDATED FINAL SPEC                           ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  8 PROPRIÉTÉS STANDARD — TOUTES LES 61 CLASSES (DATA + SCHEMA)                ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  │ # │ Property      │ Type     │ Groupe    │ Description                   │ ║
║  │───┼───────────────┼──────────┼───────────┼───────────────────────────────│ ║
║  │ 1 │ key           │ string   │ IDENTITÉ  │ Identifiant unique            │ ║
║  │ 2 │ display_name  │ string   │ IDENTITÉ  │ Nom lisible                   │ ║
║  │ 3 │ node_class    │ string   │ IDENTITÉ  │ Classe du nœud                │ ║
║  │ 4 │ content       │ string   │ CONTENU   │ Markdown: WHAT the node IS    │ ║
║  │ 5 │ llm_context   │ string   │ CONTENU   │ Markdown: HOW to USE it       │ ║
║  │ 6 │ provenance    │ string   │ METADATA  │ JSON: {"source","file",...}   │ ║
║  │ 7 │ created_at    │ datetime │ METADATA  │ Création                      │ ║
║  │ 8 │ updated_at    │ datetime │ METADATA  │ Modification                  │ ║
║                                                                               ║
║  DÉCISIONS VALIDÉES:                                                          ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  ✅ 8 props pour TOUT (data + schema) — zéro exception                        ║
║  ✅ TOUT required — zéro optionnel                                            ║
║  ✅ content + llm_context = markdown strings (simple, LLM-readable)           ║
║  ✅ provenance = JSON string (Neo4j ne supporte pas les maps)                 ║
║  ✅ Ordre: IDENTITÉ → CONTENU → METADATA                                      ║
║  ✅ content AVANT llm_context (QUOI avant COMMENT)                            ║
║  ✅ node_class case: PascalCase = DATA, lowercase = SCHEMA                    ║
║                                                                               ║
║  provenance.source enum:                                                      ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  • "seed" — fichier Cypher (seed files)                                       ║
║  • "nika" — généré par workflow Nika                                          ║
║  • "mcp"  — créé via MCP tools                                                ║
║                                                                               ║
║  PHILOSOPHIE NovaNet:                                                         ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  • content = WHAT the node IS (définition, connaissance, données)             ║
║  • llm_context = HOW to USE it (routing, triggers, disambiguation)            ║
║  • Le graphe gère les relations — pas besoin de "relates" dans llm_context    ║
║  • Entity: triggers EN pour routing système                                   ║
║  • EntityNative: triggers natifs pour compréhension utilisateur               ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Part 1: Semantic Distinction — content vs llm_context

### The Problem

Currently, the codebase conflates two distinct concerns:
- **WHAT** a node contains (its data/knowledge)
- **HOW** an LLM should use that node (operational instructions)

This creates hallucination risks when LLMs confuse operational metadata with actual content.

### The Solution

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  SEMANTIC DISTINCTION: content vs llm_context                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  content = WHAT the node IS                                                   ║
║  ───────────────────────────────────────────────────────────────────────────  ║
║  • The actual data, knowledge, or information stored in the node              ║
║  • For Entity: definition, examples, relationships to other concepts          ║
║  • For EntityNative: denomination_forms, locale-specific content              ║
║  • For Expression: the actual expression text, usage examples                 ║
║  • For Page: page structure, purpose, target audience                         ║
║  • Can be string (simple) or object (structured)                              ║
║                                                                               ║
║  Examples:                                                                    ║
║    Entity.content: "A QR code is a two-dimensional barcode..."                ║
║    Expression.content: "C'est du gâteau" (idiomatic expression)               ║
║    Page.content: { purpose: "landing", sections: [...] }                      ║
║                                                                               ║
║  llm_context = HOW to USE the node                                            ║
║  ───────────────────────────────────────────────────────────────────────────  ║
║  • Operational instructions for LLM consumption (ADR-027 pattern)             ║
║  • NOT the node's data — metadata about how to interpret/use it               ║
║  • Structured format: USE / TRIGGERS / NOT / RELATES                          ║
║  • Enables routing, disambiguation, and context selection                     ║
║                                                                               ║
║  Pattern (ADR-027):                                                           ║
║    USE: when [primary use case for this element].                             ║
║    TRIGGERS: "keyword1", "keyword2", "keyword3".                              ║
║    NOT: for [what NOT to use this for] (use [alternative] instead).           ║
║    RELATES: [related elements with roles].                                    ║
║                                                                               ║
║  Example (Entity: qr-code):                                                   ║
║    llm_context: |                                                             ║
║      USE: when generating content about QR codes, scanning, or mobile apps.   ║
║      TRIGGERS: "QR", "code", "scan", "barcode", "mobile", "2D".               ║
║      NOT: for barcodes (use entity:barcode), NFC (use entity:nfc).            ║
║      RELATES: EntityNative (locale content), Page (landing pages).            ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Why This Matters for LLM

```
WITHOUT clear distinction:
─────────────────────────────────────────────────────────────────────────────────
LLM sees: description = "A QR code is a two-dimensional barcode..."
LLM also sees: llm_context = "USE: when generating content about QR codes..."

Problem: LLM might quote the llm_context as content, or ignore content
         because it looks like metadata.

WITH clear distinction:
─────────────────────────────────────────────────────────────────────────────────
LLM sees: content = "A QR code is a two-dimensional barcode..."  ← USE THIS
LLM sees: llm_context = "USE: when generating..."                ← ROUTING INFO

Pattern: content is ALWAYS the authoritative source for generation.
         llm_context is ALWAYS routing/disambiguation metadata.
```

---

## Part 2: LLM Context Assembly Flow

### How novanet_generate Works (7-Phase Pipeline)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  novanet_generate: RLM-on-KG Context Assembly Pipeline                        ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  INPUT: focus_key, locale, mode (block|page), token_budget                    ║
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ Phase 1: FOCUS NODE RESOLUTION                                          │  ║
║  │ ─────────────────────────────────────────────────────────────────────── │  ║
║  │ Query: MATCH (n {key: $focus_key}) RETURN n                             │  ║
║  │ Extract: key, display_name, content, node_class, llm_context            │  ║
║  │                                                                         │  ║
║  │ 🔴 CURRENT GAP: Uses `description` instead of `content`                 │  ║
║  │ 🔴 CURRENT GAP: Missing `node_class` query                              │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                          │                                                    ║
║                          ▼                                                    ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ Phase 2: SPREADING ACTIVATION                                           │  ║
║  │ ─────────────────────────────────────────────────────────────────────── │  ║
║  │ Traverse graph from focus node with exponential decay:                  │  ║
║  │   relevance = base_weight × decay^depth                                 │  ║
║  │                                                                         │  ║
║  │ Arc family weights (from taxonomy.yaml):                                │  ║
║  │   ownership: 1.0      (HAS_BLOCK, HAS_PAGE)                             │  ║
║  │   localization: 0.95  (HAS_NATIVE, FOR_LOCALE)                          │  ║
║  │   semantic: 0.9       (USES_ENTITY, REPRESENTS)                         │  ║
║  │   generation: 0.8     (GENERATED_BY)                                    │  ║
║  │   mining: 0.7         (TARGETS, ANSWERS)                                │  ║
║  │                                                                         │  ║
║  │ Task-specific boosts (by block_type):                                   │  ║
║  │   CTA: boost semantic arcs (entities matter for calls-to-action)        │  ║
║  │   FAQ: boost mining arcs (SEO keywords, GEO answers)                    │  ║
║  │   HERO: boost localization (locale voice matters for headers)           │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                          │                                                    ║
║                          ▼                                                    ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ Phase 3: LOCALE CONTEXT LOADING                                         │  ║
║  │ ─────────────────────────────────────────────────────────────────────── │  ║
║  │ Load locale-specific atoms:                                             │  ║
║  │   - LocaleVoice: tone, formality, style                                 │  ║
║  │   - LocaleCulture: cultural references, taboos                          │  ║
║  │   - Expressions: idiomatic phrases (selective, not all 20K)             │  ║
║  │   - Patterns: text templates                                            │  ║
║  │                                                                         │  ║
║  │ Query pattern:                                                          │  ║
║  │   MATCH (l:Locale {key: $locale})-[:HAS_EXPRESSIONS]->(es)              │  ║
║  │   MATCH (es)-[:CONTAINS_EXPRESSION]->(e:Expression)                     │  ║
║  │   WHERE e.domain IN $relevant_domains                                   │  ║
║  │   RETURN e.content, e.llm_context LIMIT 50                              │  ║
║  │                                                                         │  ║
║  │ 🔴 CURRENT GAP: Uses `text` instead of `content` for atoms              │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                          │                                                    ║
║                          ▼                                                    ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ Phase 4: ENTITY NATIVE RESOLUTION                                       │  ║
║  │ ─────────────────────────────────────────────────────────────────────── │  ║
║  │ For each Entity in context, load locale-specific EntityNative:          │  ║
║  │   - denomination_forms (ADR-033): text, title, abbrev, url              │  ║
║  │   - content: locale-specific definition                                 │  ║
║  │                                                                         │  ║
║  │ Query:                                                                  │  ║
║  │   MATCH (e:Entity {key: $entity_key})-[:HAS_NATIVE]->(en:EntityNative)  │  ║
║  │   MATCH (en)-[:FOR_LOCALE]->(l:Locale {key: $locale})                   │  ║
║  │   RETURN en.content, en.denomination_forms                              │  ║
║  │                                                                         │  ║
║  │ 🔴 CURRENT GAP: Uses `definition` instead of `content`                  │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                          │                                                    ║
║                          ▼                                                    ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ Phase 5: TOKEN BUDGET MANAGEMENT                                        │  ║
║  │ ─────────────────────────────────────────────────────────────────────── │  ║
║  │ Prioritize context by relevance score:                                  │  ║
║  │   1. Focus node (always included)                                       │  ║
║  │   2. EntityNatives with denomination_forms                              │  ║
║  │   3. Locale context (voice, culture)                                    │  ║
║  │   4. Related entities (by spreading activation score)                   │  ║
║  │   5. Knowledge atoms (expressions, patterns)                            │  ║
║  │                                                                         │  ║
║  │ Truncation strategy:                                                    │  ║
║  │   - Never truncate focus node                                           │  ║
║  │   - Truncate lowest-relevance items first                               │  ║
║  │   - Mark truncation in response: truncated: true                        │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                          │                                                    ║
║                          ▼                                                    ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ Phase 6: CONTEXT ANCHORS                                                │  ║
║  │ ─────────────────────────────────────────────────────────────────────── │  ║
║  │ Build provenance-aware anchors for citation:                            │  ║
║  │                                                                         │  ║
║  │ context_anchors: [                                                      │  ║
║  │   {                                                                     │  ║
║  │     ref: "entity:qr-code",                                              │  ║
║  │     source: "EntityNative",                                             │  ║
║  │     locale: "fr-FR",                                                    │  ║
║  │     provenance: { source: "nika", workflow: "entity-native-gen" }       │  ║
║  │   },                                                                    │  ║
║  │   {                                                                     │  ║
║  │     ref: "expr:c-est-du-gateau",                                        │  ║
║  │     source: "Expression",                                               │  ║
║  │     provenance: { source: "seed", file: "expressions-fr.cypher" }       │  ║
║  │   }                                                                     │  ║
║  │ ]                                                                       │  ║
║  │                                                                         │  ║
║  │ 🔴 CURRENT GAP: provenance not queried or returned                      │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                          │                                                    ║
║                          ▼                                                    ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ Phase 7: RESPONSE ASSEMBLY                                              │  ║
║  │ ─────────────────────────────────────────────────────────────────────── │  ║
║  │ OUTPUT:                                                                 │  ║
║  │   prompt: string           # Assembled context for LLM                  │  ║
║  │   evidence_summary: {...}  # What was included and why                  │  ║
║  │   locale_context: {...}    # Voice, culture, expressions                │  ║
║  │   context_anchors: [...]   # Citation references with provenance        │  ║
║  │   denomination_forms: {...} # ADR-033 canonical entity names            │  ║
║  │   context_build_log: [...]  # DX-11 debugging trace                     │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Anti-Hallucination Safeguards

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  HALLUCINATION RISK MITIGATION                                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  RISK 1: Missing Property Source                                              ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Problem: LLM invents content when `content` field is missing                 ║
║  Solution: v0.19.0 makes `content` REQUIRED on all 57 data nodes              ║
║                                                                               ║
║  RISK 2: Semantic Confusion                                                   ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Problem: LLM confuses `llm_context` (routing) with `content` (data)          ║
║  Solution: Clear naming + consistent position (content=3, llm_context=4)      ║
║                                                                               ║
║  RISK 3: Node Type Misidentification                                          ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Problem: LLM doesn't know if node is schema vs data                          ║
║  Solution: `node_class` with case convention (lowercase=schema, Pascal=data)  ║
║                                                                               ║
║  RISK 4: Unprovenance Content                                                 ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Problem: LLM can't assess content reliability                                ║
║  Solution: `provenance` object tracks source (seed/authored/imported/mcp)     ║
║                                                                               ║
║  RISK 5: Locale Mismatch                                                      ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Problem: Content from wrong locale leaks into generation                     ║
║  Solution: Strict locale filtering in spreading activation                    ║
║                                                                               ║
║  RISK 6: Stale Content                                                        ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Problem: Outdated generated content used for new generation                  ║
║  Solution: `updated_at` comparison + freshness audit in novanet_audit         ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Part 3: Current State Audit

### YAML Model Audit Results (57 files)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  YAML AUDIT RESULTS — 57 Node Class Files                                     ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Property          │ Present │ Missing │ Status                               ║
║  ──────────────────┼─────────┼─────────┼──────────────────────────────────── ║
║  key               │  57/57  │   0     │ ✅ 100% - No action needed           ║
║  display_name      │  57/57  │   0     │ ✅ 100% - No action needed           ║
║  description       │  52/57  │   5     │ 🔄 91% - Rename to content           ║
║  content (object)  │   5/57  │  52     │ 🔄  9% - Already correct             ║
║  llm_context       │  57/57  │   0     │ ✅ 100% - No action needed           ║
║  node_class        │   0/57  │  57     │ 🔴   0% - ADD TO ALL FILES           ║
║  provenance        │   6/57  │  51     │ 🔴  11% - ADD TO 51 FILES            ║
║  created_at        │  57/57  │   0     │ ✅ 100% - No action needed           ║
║  updated_at        │  57/57  │   0     │ ✅ 100% - No action needed           ║
║                                                                               ║
║  FILES USING content (object) — 5 files, already correct:                     ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  • org/semantic/entity.yaml                                                   ║
║  • org/semantic/entity-native.yaml                                            ║
║  • org/instruction/block-instruction.yaml                                     ║
║  • org/output/block-native.yaml                                               ║
║  • shared/knowledge/atoms/term.yaml (if exists)                               ║
║                                                                               ║
║  FILES WITH provenance — 6 files, verify schema matches:                      ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  • shared/knowledge/atoms/expression.yaml                                     ║
║  • shared/knowledge/atoms/pattern.yaml                                        ║
║  • shared/knowledge/atoms/culture-ref.yaml                                    ║
║  • shared/knowledge/atoms/taboo.yaml                                          ║
║  • shared/knowledge/atoms/audience-trait.yaml                                 ║
║  • shared/knowledge/containers/term-set.yaml                                  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### MCP Server Audit Results

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  MCP SERVER AUDIT — novanet_generate (tools/novanet-mcp/src/tools/)           ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  FILE: generate.rs                                                            ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║                                                                               ║
║  ISSUE 1: Wrong property names in queries                                     ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Line ~85:  Uses `n.description` instead of `n.content`                       ║
║  Line ~142: Uses `en.definition` instead of `en.content`                      ║
║  Line ~198: Uses `e.text` instead of `e.content` for atoms                    ║
║                                                                               ║
║  ISSUE 2: Missing standard properties in queries                              ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Missing: n.node_class (no class discrimination in context)                   ║
║  Missing: n.llm_context (routing info not included)                           ║
║  Missing: n.provenance (no source tracking in response)                       ║
║                                                                               ║
║  ISSUE 3: Response structure incomplete                                       ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Missing: context_anchors[].provenance                                        ║
║  Missing: node_class in evidence items                                        ║
║                                                                               ║
║  FILE: write.rs                                                               ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  ISSUE: No provenance object handling                                         ║
║  ISSUE: No node_class validation                                              ║
║                                                                               ║
║  FILE: check.rs                                                               ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  ISSUE: No provenance schema validation                                       ║
║  ISSUE: No content vs description validation                                  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Part 4: Migration Phases

### Phase 1: Schema Templates ✅ DONE

- [x] Update `_standard-properties-template.yaml` with 8 properties
- [x] Create `_schema-properties-template.yaml` for Schema nodes (5 props)
- [x] Update `_index.yaml` documentation
- [x] Update TUI constants in `yaml_panel.rs`

### Phase 2: Add node_class to ALL 57 Files

**Scope**: ALL 57 YAML files need `node_class` added to standard_properties

```yaml
# Add to every node class YAML:
standard_properties:
  # ... existing properties ...
  node_class:
    type: string
    required: true
    indexed: true
    default: "{NodeClassName}"  # e.g., "Entity", "Page", "Expression"
    description: "Node class discriminator (PascalCase = DATA node)"
```

**Script approach**: Automated via jq/yq transformation

```bash
# For each file in packages/core/models/node-classes/**/*.yaml
# Insert node_class after llm_context (position 5)
```

### Phase 3: Rename description → content (52 files)

**Scope**: 52 files using `description` as string

```yaml
# BEFORE
standard_properties:
  description:
    type: string
    required: true
    description: "What this node is"

# AFTER
standard_properties:
  content:
    type: string
    required: true
    description: "Node content (data/knowledge)"
```

**Files**:
- `org/config/org-config.yaml`
- `org/foundation/*.yaml` (8 files)
- `org/structure/*.yaml` (3 files)
- `org/instruction/*.yaml` (3 files, except block-instruction)
- `org/output/*.yaml` (2 files, except block-native)
- `shared/config/*.yaml` (3 files)
- `shared/locale/*.yaml` (5 files)
- `shared/geography/*.yaml` (7 files)
- `shared/knowledge/**/*.yaml` (21 files, containers + most atoms)

### Phase 4: Verify content Object Nodes (5 files)

**Scope**: 5 files already using `content` as object — verify structure

**Files**:
- `org/semantic/entity.yaml`
- `org/semantic/entity-native.yaml`
- `org/instruction/block-instruction.yaml`
- `org/output/block-native.yaml`
- `shared/knowledge/atoms/term.yaml` (if exists)

**Action**: Add `node_class`, verify content object schema

### Phase 5: Add provenance to ALL 57 Files

**Scope**: 51 files missing `provenance`

```yaml
standard_properties:
  # ... existing properties ...
  provenance:
    type: object
    required: true
    description: "Data origin tracking"
    properties:
      source:
        type: string
        enum: [seed, authored, imported, generated, mcp]
        required: true
      file:
        type: string
        description: "Source file (for seed/imported)"
      author:
        type: string
        description: "Author identifier (for authored)"
      timestamp:
        type: datetime
        description: "When data was created/imported"
      workflow:
        type: string
        description: "Nika workflow ID (for generated/mcp)"
```

**Already have provenance** (verify schema matches): 6 files
- `shared/knowledge/atoms/expression.yaml`
- `shared/knowledge/atoms/pattern.yaml`
- `shared/knowledge/atoms/culture-ref.yaml`
- `shared/knowledge/atoms/taboo.yaml`
- `shared/knowledge/atoms/audience-trait.yaml`
- `shared/knowledge/containers/term-set.yaml`

### Phase 6: Schema Nodes (Layers + Realms)

**Scope**: 12 files in layers/ and realms/

**Action**:
1. Keep `description` (Schema nodes don't use `content`)
2. Add `node_class` with lowercase value
3. No `provenance` (Schema nodes are defined, not authored)

```yaml
# layers/semantic.yaml
schema_properties:
  key:
    type: string
    required: true
    default: "semantic"
  display_name:
    type: string
    required: true
    default: "Semantic"
  description:
    type: string
    required: true
    default: "Meaning and knowledge relationships"
  llm_context:
    type: string
    required: true
  node_class:
    type: string
    required: true
    default: "layer"  # lowercase = SCHEMA node
```

### Phase 7: MCP Server Updates

#### 7.1: generate.rs

```rust
// BEFORE (line ~85)
let query = r#"
    MATCH (n {key: $focus_key})
    RETURN n.key, n.display_name, n.description
"#;

// AFTER
let query = r#"
    MATCH (n {key: $focus_key})
    RETURN n.key, n.display_name, n.content, n.node_class,
           n.llm_context, n.provenance
"#;
```

#### 7.2: write.rs

```rust
// Add provenance validation
fn validate_provenance(provenance: &Value) -> Result<(), ValidationError> {
    let source = provenance.get("source")
        .ok_or(ValidationError::MissingField("provenance.source"))?;

    let valid_sources = ["seed", "authored", "imported", "generated", "mcp"];
    if !valid_sources.contains(&source.as_str().unwrap_or("")) {
        return Err(ValidationError::InvalidEnum("provenance.source", valid_sources));
    }
    Ok(())
}
```

#### 7.3: check.rs

```rust
// Add to validation checks
fn check_standard_properties(node: &Node) -> Vec<ValidationIssue> {
    let mut issues = vec![];

    // Check node_class exists and matches expected case
    if let Some(nc) = node.get("node_class") {
        let is_schema = nc.chars().next().map(|c| c.is_lowercase()).unwrap_or(false);
        if is_schema && !SCHEMA_CLASSES.contains(&nc.as_str()) {
            issues.push(ValidationIssue::warning("node_class",
                "Lowercase node_class should be realm/layer/class/arc_class"));
        }
    } else {
        issues.push(ValidationIssue::error("node_class", "Missing required property"));
    }

    // Check provenance for data nodes
    if !is_schema_node(node) && node.get("provenance").is_none() {
        issues.push(ValidationIssue::error("provenance", "Missing required property"));
    }

    issues
}
```

### Phase 8: Rust TUI Updates

#### 8.1: yaml_panel.rs (Already done in Phase 1)

#### 8.2: info.rs

```rust
// Update property display to use new names
fn render_standard_properties(node: &NodeData, area: Rect, buf: &mut Buffer) {
    let props = if is_schema_node(node) {
        SCHEMA_PROPERTIES
    } else {
        STANDARD_PROPERTIES
    };

    for prop in props {
        let value = node.get(prop).unwrap_or(&Value::Null);
        render_property(prop, value, area, buf);
    }
}
```

### Phase 9: Generators Updates

#### 9.1: Cypher Generator

```rust
// Update CREATE statements to include all standard properties
fn generate_create_statement(node: &ParsedNode) -> String {
    format!(r#"
CREATE (n:{class} {{
    key: $key,
    display_name: $display_name,
    content: $content,
    llm_context: $llm_context,
    node_class: '{class}',
    provenance: $provenance,
    created_at: datetime(),
    updated_at: datetime()
}})
"#, class = node.name)
}
```

#### 9.2: TypeScript Generator

```typescript
// Update generated types
interface StandardProperties {
    key: string;
    display_name: string;
    content: string | ContentObject;
    llm_context: string;
    node_class: string;
    provenance: ProvenanceObject;
    created_at: string;
    updated_at: string;
}

interface ProvenanceObject {
    source: 'seed' | 'nika' | 'mcp';  // v0.19.0: logical source, NOT trait names
    file?: string;           // For seed: the cypher file name
    workflow?: string;       // For nika: the workflow id
    workflow_run?: string;   // For nika: the workflow run id
}
```

### Phase 10: Seed Migration

```cypher
// BEFORE
CREATE (e:Entity {
    key: 'qr-code',
    display_name: 'QR Code',
    description: 'A two-dimensional barcode...',
    created_at: datetime(),
    updated_at: datetime()
})

// AFTER
CREATE (e:Entity {
    key: 'qr-code',
    display_name: 'QR Code',
    content: 'A two-dimensional barcode...',
    llm_context: 'USE: when generating content about QR codes...',
    node_class: 'Entity',
    provenance: {source: 'seed', file: '01-entities.cypher'},
    created_at: datetime(),
    updated_at: datetime()
})
```

---

## Part 5: ADRs to Create

### ADR-037: Standard Properties Schema

**Status**: To Create
**Scope**: Defines the 8 standard properties for DATA nodes and 5 for SCHEMA nodes

**Content outline**:
1. Context: Current inconsistent property names
2. Decision: Unified standard properties
3. Consequences: Migration required, LLM-friendly structure

### ADR-038: Provenance Tracking

**Status**: To Create
**Scope**: Defines the provenance object schema and usage

**Content outline**:
1. Context: Need to track data origin for trust/audit
2. Decision: Provenance object with source enum
3. Consequences: All writes must include provenance

### ADR-034 Update: Auto-Fix Property Order

**Status**: To Update
**Scope**: Add node_class and provenance to canonical property order

**Current order**: key, display_name, description, created_at, updated_at
**New order**: key, display_name, content, llm_context, node_class, provenance, created_at, updated_at

---

## Part 6: Validation Checklist

After migration:

- [ ] `cargo run -- schema validate` — All 57 YAMLs valid
- [ ] `cargo run -- schema validate --strict` — Zero warnings
- [ ] `cargo test` — All 1210 tests pass
- [ ] `cargo clippy -- -D warnings` — Zero warnings
- [ ] TUI shows STANDARD: 8 (not 6) for data nodes
- [ ] TUI shows SCHEMA: 5 for schema nodes
- [ ] MCP `novanet_check` validates node_class
- [ ] MCP `novanet_check` validates provenance
- [ ] MCP `novanet_generate` uses `content` not `description`
- [ ] MCP `novanet_generate` returns `provenance` in context_anchors
- [ ] MCP `novanet_audit` checks for missing properties
- [ ] Seed files use new property names
- [ ] Neo4j nodes have all 8 standard properties

---

## Part 7: Rollback Plan

If issues arise:

1. **Git revert**: `git revert HEAD~N` to pre-migration commit
2. **Backward compatibility**: Keep `description` as alias for `content` temporarily
3. **Migration flag**: Add `--legacy-properties` flag to generators
4. **Database migration**: Cypher script to rename properties back

---

## References

- ADR-027: llm_context Pattern (USE/TRIGGERS/NOT/RELATES)
- ADR-029: *Native Pattern
- ADR-033: Denomination Forms
- ADR-024: Trait = Data Origin
- Templates: `packages/core/models/_standard-properties-template.yaml`
- Templates: `packages/core/models/_schema-properties-template.yaml`
