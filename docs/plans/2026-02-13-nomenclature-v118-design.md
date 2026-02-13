# NovaNet v0.12.0 — Complete Nomenclature Migration

**Date**: 2026-02-13
**Status**: Approved (pending implementation)
**Version**: v0.12.0 (SemVer)
**Codename**: "Class Act"
**Estimated Duration**: 8-12 hours (multi-session)
**GitHub Release**: v0.12.0

## Executive Summary

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🚀 v0.12.0 "CLASS ACT" — COMPLETE REPO MIGRATION                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  SCOPE: Full terminology migration + semantic validation + GitHub release     ║
║                                                                               ║
║  ADR-023: Kind → Class (NodeKind → NodeClass, :Meta: → :Schema:)              ║
║  ADR-024: Trait = "Data Origin" (invariant→defined, localized→authored, etc.) ║
║  ADR-025: Instruction layer renaming (PagePrompt → PageInstruction)           ║
║                                                                               ║
║  DELIVERABLES:                                                                ║
║  ├─ All YAML models updated (60 node kinds, 114 arc kinds)                    ║
║  ├─ All Rust code migrated (998 tests passing)                                ║
║  ├─ All TypeScript code migrated (type-check passing)                         ║
║  ├─ Neo4j fully reseeded with semantic validation                             ║
║  ├─ All documentation updated (CLAUDE.md, README, BOOK.md)                    ║
║  ├─ All skills tested (/novanet-arch, /schema:*, /codebase-audit)             ║
║  ├─ Ralph Wiggum semantic audit passed                                        ║
║  └─ GitHub release v0.12.0 published                                          ║
║                                                                               ║
║  VALIDATION: Multi-layer Ralph Wiggum audits (syntax, semantic, Socratic)     ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Overview

This document captures the nomenclature decisions made during the brainstorming session to simplify and clarify NovaNet's terminology for both humans and LLMs.

## Decision 1: Kind → Class (ADR-023)

### Problem
- "Kind" is non-standard graph theory terminology
- LLMs have less training data on "Kind" vs "Class"
- French translation "Genre" was awkward

### Solution
Rename schema-level terminology from "Kind" to "Class":

| Before | After | Context |
|--------|-------|---------|
| NodeKind | NodeClass | Rust/TypeScript struct |
| ArcKind | ArcClass | Rust/TypeScript struct |
| KindInfo | ClassInfo | TUI struct |
| TreeItem::Kind | TreeItem::Class | Rust enum variant |
| [:FROM_KIND] | [:FROM_CLASS] | Neo4j relationship |
| [:TO_KIND] | [:TO_CLASS] | Neo4j relationship |
| [:HAS_KIND] | [:HAS_CLASS] | Neo4j relationship |
| "Node Kinds" | "Classes" | UI label |

### Rationale
- `rdfs:Class`, `owl:Class` are in LLM training data millions of times
- "Class/Instance" is THE canonical OOP and ontology pairing
- Universal: "Classe/Instance" (FR), "Clase/Instancia" (ES), "Klasse/Instanz" (DE)

## Decision 2: Meta Elimination (ADR-023)

### Problem
- "Meta" is ambiguous (Facebook collision, Spanish "meta" = goal)
- Mixed usage: "Meta Node", "KindMeta", Neo4j `:Meta:` labels

### Solution
Eliminate "Meta" prefix/suffix entirely - use semantic names:

| Before | After | Context |
|--------|-------|---------|
| KindMeta | Classification | TypeScript interface (realm/layer/trait) |
| KIND_META | CLASS_TAXONOMY | TypeScript constant |
| :Meta:Kind | :Schema:Class | Neo4j label |
| :Meta:ArcKind | :Schema:ArcClass | Neo4j label |
| "Meta Node" | "Class" | Glossary |
| "Data Node" | "Instance" | Glossary |
| "Meta mode" | "Schema view" | Studio UI |
| "Data mode" | "Graph view" | Studio UI |

### Rationale
- `Classification` describes WHAT it contains (realm/layer/trait axes)
- `Schema` describes WHAT it is (the schema, not data)
- "Meta" described NOTHING

## Decision 3: Trait Redefinition (ADR-024)

### Problem
Current traits conflate multiple concerns and overlap with Layer:

1. **60% redundancy**: Most layers have a single trait (instruction=invariant, output=generated)
2. **Name collision**: "knowledge" trait vs "knowledge" layer
3. **Catch-all category**: 31 nodes are "invariant" but serve very different purposes
4. **Mixed semantics**: Traits mix "locale behavior" with "data origin"

Analysis by 5 brainstorming agents revealed:
- Layer already answers "WHAT functional category?"
- Trait should answer a DIFFERENT question to be truly orthogonal

### Solution: Redefine Trait as "Data Origin"

**New question**: "WHERE does this data come from?"

| Before | After | Definition | Examples |
|--------|-------|------------|----------|
| invariant | **defined** | Defined by human, created ONCE. Structure/template. | Page, Block, PageType, BlockType, Locale, OrgConfig |
| localized | **authored** | Written by human, PER locale. Editorial content. | EntityContent, ProjectContent |
| knowledge | **imported** | External data brought in. APIs, databases, corpora. | Term, Expression, SEOKeyword, GEOQuery |
| generated | **generated** | Produced by OUR LLM. NovaNet generates this. | PageGenerated, BlockGenerated, OutputArtifact |
| aggregated | **retrieved** | Retrieved from EXTERNAL APIs. Snapshots of third-party data. | GEOAnswer, SEOKeywordMetrics, GEOMetrics |

### Rationale: True Orthogonality

```
LAYER answers:  "WHAT functional category?"
                config, structure, semantic, instruction, output, knowledge...

TRAIT answers:  "WHERE does the data come from?"
                defined, authored, imported, generated, retrieved
```

These axes are NOW truly independent:
- A `knowledge` layer node can be `imported` (Term) or `defined` (TermSet container)
- A `semantic` layer node can be `defined` (Entity) or `authored` (EntityContent)
- An `output` layer node is always `generated` (but that's a valid pattern, not redundancy)

### New Trait Definitions

**defined** (was invariant):
- Human creates this ONCE, it doesn't vary by locale
- Templates, configurations, structural definitions
- "This is how things are set up"

**authored** (was localized):
- Human writes this content, PER locale
- Editorial content, curated descriptions
- "A human wrote this in French/Japanese/etc."

**imported** (was knowledge):
- Data brought in from external sources
- Linguistic corpora, market data, discovered keywords
- "We imported this from [source]"

**generated** (unchanged):
- Our LLM produces this output
- Final content for publication
- "NovaNet generated this"

**retrieved** (was aggregated):
- Snapshots from third-party APIs
- We don't create it, we capture it
- "We fetched this from Claude/GPT/Ahrefs/etc."

### Key Clarification: GEOAnswer

GEOAnswer is `retrieved`, NOT `generated`:
- It's a SNAPSHOT of what Claude/GPT/Perplexity returned
- We RETRIEVED it from their API, we didn't generate it
- It's evidence of how AI engines see our content
- Trait `retrieved` is correct (external API snapshot)

## Trait Distribution (60 nodes)

| Trait | Count | Nodes |
|-------|-------|-------|
| defined | 31 | Page, Block, Entity, Project, BrandIdentity, OrgConfig, PageType, BlockType, PagePrompt, BlockPrompt, Locale, EntityCategory, SEOKeywordFormat, TermSet, ExpressionSet, etc. |
| imported | 22 | Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait, Culture, Style, Formatting, Adaptation, Market, Slugification, SEOKeyword, GEOQuery, etc. |
| authored | 2 | EntityContent, ProjectContent |
| generated | 5 | PageGenerated, BlockGenerated, OutputArtifact, PromptArtifact |
| retrieved | 3 | GEOAnswer, SEOKeywordMetrics, GEOMetrics |

## Summary: Before → After

```
v11.7 (Current)                    v11.8 (Proposed)
─────────────────────────────────────────────────────────────────
SCHEMA LEVEL:
  NodeKind                    →    NodeClass
  ArcKind                     →    ArcClass
  KindMeta                    →    Classification
  :Meta:Kind                  →    :Schema:Class

DATA LEVEL:
  "Meta Node"                 →    "Class"
  "Data Node"                 →    "Instance"

TRAITS (redefined as "Data Origin"):
  invariant                   →    defined    (human-created once)
  localized                   →    authored   (human-written per locale)
  knowledge                   →    imported   (external data brought in)
  generated                   →    generated  (our LLM produces)
  aggregated                  →    retrieved  (fetched from external APIs)

INSTRUCTION LAYER:
  PageType                    →    PageStructure  (JSON defining block order)
  BlockType                   →    BlockType      (keep - JSON schema)
  PagePrompt                  →    PageInstruction (MD with @ refs)
  BlockPrompt                 →    BlockInstruction (MD with @ refs)

ARCS:
  [:OF_TYPE] (Page→PageType)     →    [:HAS_STRUCTURE]
  [:HAS_PROMPT] (Page→PagePrompt) →    [:HAS_INSTRUCTION]
  [:HAS_PROMPT] (Block→BlockPrompt) →  [:HAS_INSTRUCTION]
```

## Decision 4: Instruction Layer Renaming (ADR-025)

### Problem
Current instruction layer names don't reflect their FUNCTION:
- `PageType` → actually defines page STRUCTURE (JSON with headers, sections)
- `BlockType` → defines block JSON schema (this one is OK)
- `PagePrompt` → actually contains page INSTRUCTIONS (markdown with @ refs)
- `BlockPrompt` → contains block INSTRUCTIONS (markdown with @ refs)

The UI already uses the correct names: "Page Structures" and "Page Instructions".

### Solution
Rename to match function and existing UI:

| Before | After | Function |
|--------|-------|----------|
| PageType | **PageStructure** | JSON defining which BlockTypes in what order |
| BlockType | **BlockType** | (keep) JSON schema for a block |
| PagePrompt | **PageInstruction** | Markdown with LLM directives and @ references |
| BlockPrompt | **BlockInstruction** | Markdown with LLM directives and @ references |

### Pipeline

```
Page
├── [:HAS_STRUCTURE] → PageStructure
│   └── JSON compilé depuis l'ordre des blocks:
│       { "blocks": ["hero", "features", "pricing", "cta"] }
│
├── [:HAS_INSTRUCTION] → PageInstruction
│   └── Markdown compilé depuis BlockInstructions (dans l'ordre):
│       BlockInstruction₁ + BlockInstruction₂ + ... = PageInstruction
│
└── [:HAS_BLOCK {order: N}] → Block (l'ordre est sur l'arc!)
    │
    ├── [:OF_TYPE] → BlockType
    │   └── JSON Schema: { slug, title, description, cta_url, ... }
    │
    └── [:HAS_INSTRUCTION] → BlockInstruction
        └── Markdown avec @ références:
            ```markdown
            # Block: hero_pricing
            [TRANSLATE] title: Highlight @entity:tier-pro benefits
            [FIXED] cta_url: /signup
            ```
```

**L'ordre des blocs (propriété `order` sur [:HAS_BLOCK]) détermine:**
1. **PageStructure JSON** — L'ordre des BlockTypes dans le JSON
2. **PageInstruction** — La compilation séquentielle des BlockInstructions
3. **PageGenerated** — L'ordre final du contenu généré

### @ Reference System

Instructions support @ references that resolve at generation time:

```markdown
# PageInstruction example
Generate pricing page comparing @entity:tier-pro vs @entity:tier-basic
See @page:features for product consistency

# BlockInstruction example
[TRANSLATE] title: Highlight benefits of @entity:tier-pro
[FIXED] cta_url: /signup
```

At generation time:
- `@entity:tier-pro` → loads `EntityContent(tier-pro@{locale})`
- `@page:features` → loads `Page(features)` context
- `[TRANSLATE]` → field needs locale-native generation
- `[FIXED]` → field is invariant (URLs, technical values)

### Arc Changes

| Before | After |
|--------|-------|
| `[:OF_TYPE]` (Page→PageType) | `[:HAS_STRUCTURE]` (Page→PageStructure) |
| `[:HAS_PROMPT]` (Page→PagePrompt) | `[:HAS_INSTRUCTION]` (Page→PageInstruction) |
| `[:OF_TYPE]` (Block→BlockType) | `[:OF_TYPE]` (keep - BlockType unchanged) |
| `[:HAS_PROMPT]` (Block→BlockPrompt) | `[:HAS_INSTRUCTION]` (Block→BlockInstruction) |

### Rationale

1. **PageStructure**: Describes WHAT it is (the structure combining blocks)
2. **PageInstruction**: Describes WHAT it is (instructions for LLM)
3. **BlockType**: Already correct (defines the type/schema of a block)
4. **BlockInstruction**: Consistent with PageInstruction
5. **Aligned with UI**: The existing Studio UI uses these exact names

## Migration Impact (Detailed Analysis)

### Summary by Zone

| Zone | Files | Occurrences | Estimated Effort |
|------|-------|-------------|------------------|
| **Rust** (tools/novanet/) | 24 | ~170 | 6-8h |
| **YAML** (packages/core/models/) | 30 | ~60 | 3-4h |
| **TypeScript** (packages/ + apps/) | 67 | ~200 | 4-6h |
| **Neo4j** (packages/db/) | 13 | ~50 | 2h |
| **Documentation** | 3 | ~30 | 1-2h |
| **Total** | **137** | **~510** | **16-22h** |

### Rust Codebase (tools/novanet/)

| Category | Files | Key Changes |
|----------|-------|-------------|
| NodeKind → NodeClass | 10 | `NodeKindGenerator`, struct defs, comments |
| ArcKind → ArcClass | 24 | `ArcKindGenerator`, `ArcKindYaml`, `ArcKindInfo`, `ArcKindDetails` |
| :Meta: → :Schema: | 9 | Cypher patterns: `MERGE (k:Meta:Kind` → `MERGE (c:Schema:Class` |
| Trait enum | 24 | `NodeTrait::Invariant` → `NodeTrait::Defined`, etc. |
| Snapshots | 5 | Auto-regenerate after struct renames |

### YAML Models (packages/core/models/)

| Category | Files | Key Changes |
|----------|-------|-------------|
| **Node renames** | 2 | `page-type.yaml` → content: PageStructure, `page-prompt.yaml` → content: PageInstruction |
| **Trait updates** | 60 | `trait: invariant` → `trait: defined`, etc. |
| **Arc renames** | 3 | `of-type.yaml` (Page→PageStructure), `has-prompt.yaml` → `has-instruction.yaml` |
| **Taxonomy** | 1 | Update trait definitions in `taxonomy.yaml` |
| **Views** | 5 | Update Cypher queries: `PageType` → `PageStructure`, etc. |
| **Index** | 1 | Update `_index.yaml` node listings |

### TypeScript (packages/ + apps/)

| Category | Files | Status |
|----------|-------|--------|
| Core types | 28 | Mostly correct ✅, verify only |
| Studio UI | 26 | Mostly correct ✅, verify only |
| DB seeds | 13 | Regenerate from Rust |
| Medium complexity | 7 | Sync arc mappings, update Cypher |

### Neo4j Migration Script

```cypher
// Phase 1: Rename labels
MATCH (n:Meta:Kind) SET n:Schema:Class REMOVE n:Meta, n:Kind;
MATCH (n:Meta:ArcKind) SET n:Schema:ArcClass REMOVE n:Meta, n:ArcKind;

// Phase 2: Rename relationships (requires recreation)
MATCH (n)-[r:FROM_KIND]->(m)
CREATE (n)-[:FROM_CLASS]->(m)
DELETE r;

MATCH (n)-[r:TO_KIND]->(m)
CREATE (n)-[:TO_CLASS]->(m)
DELETE r;

MATCH (n)-[r:HAS_KIND]->(m)
CREATE (n)-[:HAS_CLASS]->(m)
DELETE r;

// Phase 3: Update trait property values
MATCH (n) WHERE n.trait = 'invariant' SET n.trait = 'defined';
MATCH (n) WHERE n.trait = 'localized' SET n.trait = 'authored';
MATCH (n) WHERE n.trait = 'knowledge' SET n.trait = 'imported';
MATCH (n) WHERE n.trait = 'aggregated' SET n.trait = 'retrieved';

// Phase 4: Instruction layer node renames
MATCH (n:PageType) SET n:PageStructure REMOVE n:PageType;
MATCH (n:PagePrompt) SET n:PageInstruction REMOVE n:PagePrompt;
MATCH (n:BlockPrompt) SET n:BlockInstruction REMOVE n:BlockPrompt;

// Phase 5: Instruction layer arc renames
MATCH (p:Page)-[r:OF_TYPE]->(ps:PageStructure)
CREATE (p)-[:HAS_STRUCTURE]->(ps)
DELETE r;

MATCH (p:Page)-[r:HAS_PROMPT]->(pi:PageInstruction)
CREATE (p)-[:HAS_INSTRUCTION]->(pi)
DELETE r;

MATCH (b:Block)-[r:HAS_PROMPT]->(bi:BlockInstruction)
CREATE (b)-[:HAS_INSTRUCTION]->(bi)
DELETE r;
```

## Implementation Order

### Phase 1: YAML Schema (3-4h)
- [ ] Update `taxonomy.yaml` with new trait names (defined/authored/imported/generated/retrieved)
- [ ] Update all 60 node YAML files (trait field)
- [ ] Rename `page-type.yaml` content → PageStructure
- [ ] Rename `page-prompt.yaml` content → PageInstruction
- [ ] Update arc definitions (OF_TYPE target, HAS_PROMPT → HAS_INSTRUCTION)
- [ ] Update `_index.yaml` listings
- [ ] Run `cargo run -- schema validate`

### Phase 2: Rust Generators (4-6h)
- [ ] Rename struct `NodeKindGenerator` → `NodeClassGenerator`
- [ ] Rename struct `ArcKindGenerator` → `ArcClassGenerator`
- [ ] Rename struct `ArcKindYaml` → `ArcClassYaml`
- [ ] Rename struct `ArcKindDef` → `ArcClassDef`
- [ ] Update Cypher templates (`:Meta:Kind` → `:Schema:Class`)
- [ ] Update relationship types (FROM_KIND → FROM_CLASS)
- [ ] Update trait enum values in `yaml_node.rs`
- [ ] Run `cargo run -- schema generate`
- [ ] Run `cargo test` (update snapshots with `cargo insta review`)

### Phase 3: TUI Updates (2-3h)
- [ ] Rename `ArcKindInfo` → `ArcClassInfo`
- [ ] Rename `ArcKindDetails` → `ArcClassDetails`
- [ ] Update `TreeItem::ArcKind` → `TreeItem::ArcClass`
- [ ] Update Cypher queries in `data.rs`
- [ ] Update glossary entries in `nexus/glossary.rs`
- [ ] Update i18n strings
- [ ] Update UI labels ("ArcKind" → "ArcClass")

### Phase 4: Neo4j Migration (1-2h)
- [ ] Create migration file `packages/db/migrations/v11.8-nomenclature.cypher`
- [ ] Test migration on dev database
- [ ] Verify with `cargo run -- meta` and `cargo run -- data`

### Phase 5: TypeScript Sync (2-3h)
- [ ] Regenerate types from YAML: `pnpm generate`
- [ ] Verify `packages/core/src/types/nodes.ts` has correct trait values
- [ ] Update `packages/core/src/filters/CypherGenerator.ts` arc mappings
- [ ] Sync `apps/studio/src/lib/filterAdapter.ts` with core
- [ ] Run `pnpm type-check && pnpm test`

### Phase 6: Studio UI (1-2h)
- [ ] Update ViewPicker labels
- [ ] Update component text (Schema view, Graph view)
- [ ] Verify Cypher queries in views work

### Phase 7: Documentation (1h)
- [x] Update `novanet-terminology.md` ✅
- [x] Update `novanet-decisions.md` with ADR-023, 024, 025 ✅
- [ ] Update CLAUDE.md files (monorepo + tools/novanet/)
- [ ] Update CHANGELOG.md

### Phase 8: Validation (1h)
- [ ] Full test suite: `cargo test && pnpm test`
- [ ] TUI smoke test: `cargo run -- tui`
- [ ] Studio smoke test: `pnpm dev`
- [ ] Neo4j schema check: `cargo run -- schema validate`

## Automation & Hooks (Claude Code Integration)

Based on Claude Code documentation, the migration can be automated with hooks, sub-agents, and headless execution.

### Hook Configuration

Create `.claude/hooks/v118-migration-hooks.json`:

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Migration Context: v11.8 nomenclature (Kind→Class, Trait=DataOrigin). Phase progress in /tmp/v118-progress.txt. Run /codebase-audit between phases.'"
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | grep -E '\\.(rs|yaml|ts|tsx)$' && echo \"[v11.8] Modified: $(jq -r '.tool_input.file_path')\" >> /tmp/v118-changes.log || true"
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Before stopping, verify: 1) Current migration phase is complete (check todo list), 2) Tests pass for modified files, 3) No Kind/Meta/invariant/localized terms remain in modified files. If incomplete, return {\"ok\": false, \"reason\": \"what remains\"}."
          }
        ]
      }
    ],
    "Notification": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "osascript -e 'display notification \"v11.8 Migration: Claude needs attention\" with title \"NovaNet Migration\"'"
          }
        ]
      }
    ]
  }
}
```

### Sub-agent Configuration

Create `.claude/agents/v118-validator.md`:

```markdown
---
name: v118-validator
description: Validates v11.8 nomenclature migration. Use after each migration phase to verify no old terminology remains.
tools: Read, Grep, Glob, Bash
model: haiku
---

You are a migration validator for NovaNet v11.8. Your job is to find OLD terminology that should have been migrated.

## Validation Rules

### Must NOT exist in migrated files:
- `NodeKind` (should be NodeClass)
- `ArcKind` (should be ArcClass)
- `:Meta:Kind` (should be :Schema:Class)
- `trait: invariant` (should be `trait: defined`)
- `trait: localized` (should be `trait: authored`)
- `trait: knowledge` (should be `trait: imported`)
- `trait: aggregated` (should be `trait: retrieved`)
- `PageType` in instruction layer (should be PageStructure)
- `PagePrompt` (should be PageInstruction)
- `BlockPrompt` (should be BlockInstruction)

## Workflow

1. Run Grep searches for each deprecated term
2. Filter out comments, documentation, and migration scripts
3. Report any remaining occurrences with file:line
4. Suggest specific fixes for each occurrence
```

### Headless Migration Script

Create `tools/scripts/v118-migration.sh`:

```bash
#!/bin/bash
# v11.8 Nomenclature Migration - Autonomous Execution
# Uses caffeinate to prevent sleep, tmux for session persistence

set -e

LOG_DIR="/tmp/v118-migration"
mkdir -p "$LOG_DIR"

# Prevent macOS sleep
caffeinate -s -w $$ &
CAFFEINATE_PID=$!
trap "kill $CAFFEINATE_PID 2>/dev/null || true" EXIT

# Migration phases with validation gates
PHASES=(
  "phase1:Update taxonomy.yaml and YAML models (trait values)"
  "phase2:Rename Rust structs (NodeKind→NodeClass, ArcKind→ArcClass)"
  "phase3:Update Rust generators and Cypher templates"
  "phase4:Update TUI components and UI labels"
  "phase5:Run Neo4j migration script"
  "phase6:Regenerate TypeScript types"
  "phase7:Update Studio UI and views"
  "phase8:Update documentation (CLAUDE.md, README)"
)

for phase in "${PHASES[@]}"; do
  PHASE_ID="${phase%%:*}"
  PHASE_DESC="${phase#*:}"

  echo "═══════════════════════════════════════════════════════════════"
  echo "Starting $PHASE_ID: $PHASE_DESC"
  echo "═══════════════════════════════════════════════════════════════"

  # Execute phase
  claude -p "$PHASE_DESC. Follow the plan in docs/plans/2026-02-13-nomenclature-v118-design.md. Mark todos as completed. Run tests after changes." \
    --allowedTools "Read,Edit,Write,Bash,Grep,Glob" \
    --output-format json \
    | tee "$LOG_DIR/$PHASE_ID.json"

  # Validation gate (Ralph Wiggum)
  echo "Running validation gate..."
  claude -p "Run /codebase-audit to validate $PHASE_ID is complete. Check for old terminology. Report issues." \
    --allowedTools "Read,Grep,Glob,Bash" \
    --max-turns 5 \
    | tee "$LOG_DIR/$PHASE_ID-validation.json"

  # Pause for review
  echo "Phase $PHASE_ID complete. Press Enter to continue or Ctrl+C to stop..."
  read -r
done

echo "Migration complete! Review logs in $LOG_DIR"
```

### Validation Gates (Ralph Wiggum Audits)

Insert between each phase:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🔍 RALPH WIGGUM VALIDATION GATE                                              ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Run: /codebase-audit                                                         ║
║                                                                               ║
║  Checklist:                                                                   ║
║  [ ] No deprecated terminology in modified files                              ║
║  [ ] cargo test passes                                                        ║
║  [ ] cargo clippy -- -D warnings passes                                       ║
║  [ ] pnpm type-check passes                                                   ║
║  [ ] Modified files match expected count for phase                            ║
║                                                                               ║
║  Only proceed to next phase if ALL checks pass.                               ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Parallel Agent Execution

For independent tasks, spawn multiple agents:

```
# Example: Parallel validation across zones
Use Task tool with 5 parallel agents:
1. Agent: Validate Rust files (tools/novanet/)
2. Agent: Validate YAML models (packages/core/models/)
3. Agent: Validate TypeScript (packages/core/, apps/studio/)
4. Agent: Validate Neo4j seeds (packages/db/)
5. Agent: Validate documentation (.claude/rules/, docs/)
```

### Session Persistence

For multi-hour migration, use tmux:

```bash
# Start tmux session
tmux new-session -d -s v118-migration

# Run migration in background
tmux send-keys -t v118-migration "cd /Users/thibaut/supernovae-st/novanet-hq && ./tools/scripts/v118-migration.sh" Enter

# Attach to monitor
tmux attach -t v118-migration

# Detach with Ctrl+B, D — migration continues
# Reattach with: tmux attach -t v118-migration
```

### Expanded Phase Details

#### Phase 3B: TUI Components (33 files, ~5,500 lines)

```
tools/novanet/src/tui/
├── Core modules (5): app.rs, data.rs, ui.rs, theme.rs, mod.rs
├── Nexus hub (8): mod.rs, intro.rs, tutorial.rs, glossary.rs, quiz.rs, stats.rs, help.rs, common.rs
├── Widgets (12): tree.rs, aside.rs, yaml.rs, panel_header.rs, effects.rs, search.rs, ...
├── Boot sequence (5): animation.rs, boot.rs, macropad.rs, startup.rs, onboarding.rs
└── Utilities (3): icons.rs, i18n.rs, accessibility.rs
```

#### Phase 6B: Studio Components (362 files)

```
apps/studio/
├── Components (180): graph/, sidebar/, panels/, controls/, overlays/
├── Stores (25): viewStore.ts, graphStore.ts, filterStore.ts, ...
├── Hooks (15): useGraph.ts, useFilter.ts, useSelection.ts, ...
├── API routes (30): /api/views/, /api/graph/, /api/filters/
├── Views (12): _registry.yaml, *.yaml definitions
└── Tests (100): *.test.ts, *.spec.ts, e2e/*.spec.ts
```

#### Phase 7B: Tests & DX

```
Test coverage:
├── Rust (990 tests, 91 modules): cargo nextest run
├── TypeScript (50 files, ~10K LOC): pnpm test
├── E2E Playwright (14 tests, 6 specs): pnpm test:e2e
└── Snapshots (5 generators): cargo insta review

Skills to verify:
├── /novanet-arch — Architecture diagram
├── /novanet-sync — Schema synchronization
├── /schema:add-node — Node creation flow
├── /schema:edit-node — Node editing flow
└── /codebase-audit — Ralph Wiggum audit
```

## Synchronization Checklist (CRITICAL)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🔄 SYNCHRONIZATION: SOURCES OF TRUTH                                         ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ⚠️  ALL these files MUST be updated TOGETHER for v11.8 consistency:          ║
║                                                                               ║
║  1. TAXONOMY (defines colors, traits, realms, layers)                         ║
║     └─ packages/core/models/taxonomy.yaml                                     ║
║        ├─ node_traits: invariant→defined, localized→authored, etc.            ║
║        ├─ Terminal palette: update trait_* keys                               ║
║        └─ kind_retrieval_defaults: rename trait keys                          ║
║                                                                               ║
║  2. VISUAL ENCODING (defines icons, border styles)                            ║
║     └─ packages/core/models/visual-encoding.yaml                              ║
║        ├─ icons.traits: update defined/authored/imported/retrieved            ║
║        ├─ trait_borders: rename keys to match new trait names                 ║
║        └─ channel_mapping: verify trait references                            ║
║                                                                               ║
║  3. NODE KIND YAMLS (60 files with trait: field)                              ║
║     └─ packages/core/models/node-kinds/**/*.yaml                              ║
║        └─ trait: invariant → defined, localized → authored, etc.              ║
║                                                                               ║
║  4. RUST GENERATORS (read YAML, write artifacts)                              ║
║     └─ tools/novanet/src/generators/                                          ║
║        ├─ organizing.rs: Trait enum mapping                                   ║
║        ├─ icons.rs: Icon constants for traits                                 ║
║        ├─ colors.rs: Color constants for traits                               ║
║        └─ visual_encoding.rs: Border/style mapping                            ║
║                                                                               ║
║  5. RUST TUI THEME (runtime colors, icons)                                    ║
║     └─ tools/novanet/src/tui/theme.rs                                         ║
║        ├─ trait_color(): match arms for new trait names                       ║
║        ├─ trait_icon(): icons for defined/authored/imported/retrieved         ║
║        └─ trait_border(): border styles                                       ║
║                                                                               ║
║  6. RUST TUI NEXUS (educational content)                                      ║
║     └─ tools/novanet/src/tui/nexus/                                           ║
║        ├─ intro.rs: Page 2 Meta→Class, Page 3 trait names                     ║
║        ├─ glossary.rs: All concept definitions                                ║
║        ├─ quiz.rs: Question answers                                           ║
║        └─ i18n.rs: French translations of new terms                           ║
║                                                                               ║
║  7. NEO4J SEEDS (Cypher scripts)                                              ║
║     └─ packages/db/seed/                                                      ║
║        ├─ 00.5-taxonomy.cypher: Trait node properties                         ║
║        └─ Migration script: MATCH (n:Meta:Kind) → :Schema:Class               ║
║                                                                               ║
║  8. TYPESCRIPT TYPES (generated from Rust)                                    ║
║     └─ packages/core/src/graph/                                               ║
║        ├─ taxonomy.ts: NodeTrait enum                                         ║
║        ├─ visual-encoding.ts: Icon/color exports                              ║
║        └─ hierarchy.ts: Realm/Layer/Trait tree                                ║
║                                                                               ║
║  9. DOCUMENTATION                                                             ║
║     └─ .claude/rules/                                                         ║
║        ├─ novanet-terminology.md: All term definitions                        ║
║        └─ novanet-decisions.md: ADR-023, ADR-024, ADR-025                     ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Synchronization Order (Dependency Chain)

```
taxonomy.yaml (1)
       ↓
visual-encoding.yaml (2)
       ↓
node-kinds/**/*.yaml (3)  ─────────────────────┐
       ↓                                       │
Rust generators (4)                            │
       ↓                                       │
┌──────┴──────┐                                │
│             │                                │
▼             ▼                                │
TUI theme (5) Neo4j seeds (7)                  │
│             │                                │
▼             │                                │
Nexus (6)     │                                │
              ▼                                │
       TypeScript (8) ◄────────────────────────┘
              │
              ▼
       Documentation (9)
```

### Verification Commands per Zone

```bash
# Zone 1: YAML validity
yq eval '.' packages/core/models/taxonomy.yaml > /dev/null
yq eval '.' packages/core/models/visual-encoding.yaml > /dev/null

# Zone 2: Rust compilation
cd tools/novanet && cargo check

# Zone 3: Schema generation
cargo run -- schema generate --dry-run
cargo run -- schema validate --strict

# Zone 4: TUI smoke test
cargo run -- tui

# Zone 5: TypeScript
pnpm type-check

# Zone 6: Full test suite
cargo nextest run && pnpm test

# Zone 7: Terminology audit
grep -r "NodeKind" --include="*.rs" --include="*.ts" | grep -v "test\|spec\|migration"
grep -r "invariant" packages/core/models/node-kinds/ | wc -l  # Should be 0 after migration
```

## Nexus LEARN Module Brainstorm

Based on research from Context7 (Ratatui patterns), Perplexity (TUI UX), and oxycards analysis:

### Intro Pages Improvements (v11.8)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  📚 INTRO PAGE ENHANCEMENTS                                                   ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Page 1: What is NovaNet?                                                     ║
║  ├─ ADD: Animated ASCII showing Translation vs Generation flow               ║
║  ├─ ADD: Progress dots indicator (● ○ ○)                                      ║
║  └─ UPDATE: Add page navigation hint ([h/l] or [←/→])                         ║
║                                                                               ║
║  Page 2: Two Types of Nodes (v11.8 terminology!)                              ║
║  ├─ RENAME: "META NODES" → "CLASS" (60 total)                                 ║
║  ├─ RENAME: "DATA NODES" → "INSTANCE" (200K+)                                 ║
║  ├─ RENAME: "Kind: Locale" → "Class: Locale"                                  ║
║  ├─ UPDATE: Neo4j label :Meta:Kind → :Schema:Class                            ║
║  └─ ADD: Cookie cutter metaphor (Class = cutter, Instance = cookies)          ║
║                                                                               ║
║  Page 3: Classification (v11.8 traits!)                                       ║
║  ├─ UPDATE: Trait names and definitions:                                      ║
║  │   ├─ invariant → defined (same everywhere, structure/template)             ║
║  │   ├─ localized → authored (human-written per locale)                       ║
║  │   ├─ knowledge → imported (external data)                                  ║
║  │   ├─ generated → generated (LLM output, keep name)                         ║
║  │   └─ aggregated → retrieved (external API snapshots)                       ║
║  ├─ UPDATE: Border style legend with new names                                ║
║  └─ ADD: "Layer = WHAT, Trait = WHERE FROM" summary                           ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Glossary Enhancements

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  📖 GLOSSARY ENHANCEMENTS                                                     ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  UX Improvements (from Perplexity research):                                  ║
║  ├─ Fuzzy search with highlighting (use fuzzy-matcher crate)                  ║
║  ├─ Collapsible categories with ▼/▶ icons                                     ║
║  ├─ Copy to clipboard with 'y' key (yank definition)                          ║
║  └─ Syntax highlighting for YAML/Cypher examples                              ║
║                                                                               ║
║  Terminology Updates (v11.8):                                                 ║
║  ├─ GRAPH_BASICS:                                                             ║
║  │   ├─ "Meta Node" → "Class" (schema definitions, :Schema:Class)             ║
║  │   └─ "Data Node" → "Instance" (actual data, :Locale, :Page)                ║
║  ├─ CLASSIFICATION:                                                           ║
║  │   └─ Trait full_desc: Update all 5 trait definitions                       ║
║  ├─ ADD new concepts:                                                         ║
║  │   ├─ "Classification" (realm/layer/trait axes)                             ║
║  │   ├─ "Data Origin" (what trait answers)                                    ║
║  │   └─ "Schema vs Graph" (Class definitions vs Instance data)                ║
║  └─ Navigation hints:                                                         ║
║      └─ [j/k] items, [h/l] panels, [/] search, [y] copy                       ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Quiz Gamification (from Perplexity research)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🎮 QUIZ GAMIFICATION PATTERNS                                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Question Types (from oxycards):                                              ║
║  ├─ Multiple choice: "Which trait applies to LLM output?"                     ║
║  ├─ Multiple answer: "Select all ownership arcs"                              ║
║  ├─ Flashcard: Term definitions                                               ║
║  ├─ Fill in blanks: "_____ realm is READ-ONLY" (shared)                       ║
║  └─ Ordering: Pipeline stages (Entity → generate → EntityContent)             ║
║                                                                               ║
║  Engagement Mechanics:                                                        ║
║  ├─ Immediate feedback with color + symbol (colorblind-safe)                  ║
║  │   ├─ Correct: [✓] green background                                         ║
║  │   ├─ Incorrect: [✗] red background + explanation                           ║
║  │   └─ Selected: [●] highlight                                               ║
║  ├─ Progress tracking:                                                        ║
║  │   ├─ Score counter: 7/10 correct                                           ║
║  │   ├─ Streak indicator: 🔥 5 in a row                                       ║
║  │   └─ High score persistence: ~/.novanet/quiz_progress.json                 ║
║  └─ Adaptive difficulty:                                                      ║
║      └─ Review incorrect answers (spaced repetition concept)                  ║
║                                                                               ║
║  v11.8 Quiz Updates:                                                          ║
║  ├─ Update all questions with new terminology                                 ║
║  ├─ Add "What does trait answer?" → "Where does data come from?"              ║
║  ├─ Add Class/Instance questions                                              ║
║  └─ Update trait mapping questions with new names                             ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Ratatui Patterns to Apply

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🦀 RATATUI PATTERNS (from Context7 + maintainer advice)                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  1. Screen Enum Pattern (joshka recommendation):                              ║
║     enum NexusScreen {                                                        ║
║         Intro(IntroState),    // 3-page carousel                              ║
║         Tutorial(TutorialState),                                              ║
║         Glossary(GlossaryState),                                              ║
║         Quiz(QuizState),                                                      ║
║     }                                                                         ║
║                                                                               ║
║  2. Component Architecture:                                                   ║
║     trait NexusComponent {                                                    ║
║         fn handle_key(&mut self, key: KeyEvent) -> NexusAction;               ║
║         fn update(&mut self, action: NexusAction);                            ║
║         fn render(&self, frame: &mut Frame, area: Rect);                      ║
║     }                                                                         ║
║                                                                               ║
║  3. Widget Recommendations:                                                   ║
║     ├─ Gauge: Tutorial completion percentage                                  ║
║     ├─ LineGauge: Quiz progress bar                                           ║
║     ├─ Tabs: Section navigation (LEARN/EXPLORE/PRACTICE)                      ║
║     └─ throbber-widgets-tui: Loading states, animations                       ║
║                                                                               ║
║  4. Keybinding Consistency:                                                   ║
║     ├─ j/k or ↑↓: Navigate items                                              ║
║     ├─ h/l or ←→: Navigate panels/pages                                       ║
║     ├─ Tab/BackTab: Next/Previous section                                     ║
║     ├─ Enter/Space: Select/Confirm                                            ║
║     ├─ /: Search                                                              ║
║     ├─ y: Yank (copy)                                                         ║
║     ├─ Esc: Back/Cancel                                                       ║
║     └─ q: Quit                                                                ║
║                                                                               ║
║  5. Accessibility:                                                            ║
║     ├─ Never rely on color alone (use symbols: ■□◇★⋆)                         ║
║     ├─ Support NO_COLOR env variable                                          ║
║     └─ Clear focus indicators                                                 ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Implementation Priority

| Priority | Task | Effort | Impact |
|----------|------|--------|--------|
| **P0** | Update taxonomy.yaml trait names | 30min | Blocks everything |
| **P0** | Update visual-encoding.yaml icons | 30min | Blocks TUI |
| **P1** | Update intro.rs (Class/Instance, traits) | 1h | User-facing |
| **P1** | Update glossary.rs (15 concepts) | 1h | User-facing |
| **P1** | Update quiz.rs questions | 1h | User-facing |
| **P2** | Add fuzzy search to glossary | 2h | Nice UX |
| **P2** | Add animated diagrams to intro | 2h | Polish |
| **P3** | Add spaced repetition to quiz | 4h | Future feature |

## Neo4j & Cypher Complete Reseed

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🗄️ NEO4J COMPLETE RESEED & SEMANTIC VALIDATION                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  PHASE 1: Update All Cypher Files                                             ║
║  ├─ packages/db/seed/                                                         ║
║  │   ├─ 00.0-cleanup.cypher     (drop old :Meta: labels)                      ║
║  │   ├─ 00.5-taxonomy.cypher    (Realm, Layer, Trait with new names)          ║
║  │   ├─ 01-schema-classes.cypher (renamed from 01-kinds.cypher)               ║
║  │   ├─ 02-arc-classes.cypher   (renamed from 02-arc-kinds.cypher)            ║
║  │   ├─ 03-instances.cypher     (sample data)                                 ║
║  │   └─ 04-relationships.cypher (connect all nodes)                           ║
║  │                                                                            ║
║  ├─ Migration script (run once):                                              ║
║  │   ├─ MATCH (n:Meta) REMOVE n:Meta SET n:Schema                             ║
║  │   ├─ MATCH (n:Kind) REMOVE n:Kind SET n:Class                              ║
║  │   ├─ MATCH (n:ArcKind) REMOVE n:ArcKind SET n:ArcClass                     ║
║  │   └─ Update all trait properties to new names                              ║
║  │                                                                            ║
║  PHASE 2: Full Reseed                                                         ║
║  ├─ pnpm infra:reset    (drop all data)                                       ║
║  ├─ pnpm infra:seed     (execute all seed files in order)                     ║
║  └─ Verify: cargo run -- meta --format=json                                   ║
║                                                                               ║
║  PHASE 3: Semantic Validation (Critical!)                                     ║
║  ├─ Check all nodes are connected (no orphans)                                ║
║  ├─ Check all Classes have instances OR are correctly empty                   ║
║  ├─ Check all arcs connect valid source → target types                        ║
║  ├─ Check realm isolation (org nodes can't link to other org's data)          ║
║  └─ Check trait consistency (generated nodes have generated trait)            ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Semantic Validation Queries

```cypher
// 1. Find orphan nodes (no relationships)
MATCH (n)
WHERE NOT (n)--()
AND NOT n:Schema
RETURN labels(n), n.key, n.display_name
LIMIT 100;

// 2. Check Class→Instance consistency
MATCH (c:Schema:Class)
OPTIONAL MATCH (c)<-[:INSTANCE_OF]-(i)
WITH c, count(i) AS instance_count
WHERE instance_count = 0
RETURN c.name AS class_without_instances
ORDER BY c.name;

// 3. Validate arc source/target types
MATCH (a:Schema:ArcClass)
MATCH (source)-[r]->(target)
WHERE type(r) = a.name
AND NOT labels(source) CONTAINS a.source_class
RETURN type(r), labels(source), a.source_class AS expected
LIMIT 50;

// 4. Check trait property consistency
MATCH (n)
WHERE n.trait IS NOT NULL
AND n.trait NOT IN ['defined', 'authored', 'imported', 'generated', 'retrieved']
RETURN labels(n), n.key, n.trait AS invalid_trait;

// 5. Check realm isolation
MATCH (org1:OrgConfig)-[*1..5]-(org2:OrgConfig)
WHERE org1 <> org2
RETURN org1.key, org2.key, 'CROSS-ORG LEAK!' AS warning;

// 6. Verify all HAS_* arcs have valid targets
MATCH (source)-[r:HAS_CLASS|HAS_LAYER|HAS_TRAIT]->(target)
WHERE NOT target:Schema
RETURN type(r), labels(source), labels(target) AS invalid_target;
```

## Full Repository Update Checklist

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  📁 COMPLETE REPOSITORY UPDATE                                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  1. ROOT FILES                                                                ║
║     ├─ [ ] CLAUDE.md — Update version to v0.12.0, terminology                 ║
║     ├─ [ ] README.md — Update terminology, version badge                      ║
║     ├─ [ ] CHANGELOG.md — Add v0.12.0 section with all changes                ║
║     ├─ [ ] ROADMAP.md — Mark v0.12.0 as current                               ║
║     └─ [ ] package.json — Update version field                                ║
║                                                                               ║
║  2. PACKAGES/CORE                                                             ║
║     ├─ [ ] models/taxonomy.yaml — Trait names                                 ║
║     ├─ [ ] models/visual-encoding.yaml — Icons, borders                       ║
║     ├─ [ ] models/node-kinds/**/*.yaml — 60 files                             ║
║     ├─ [ ] models/arc-kinds/**/*.yaml — 114 files                             ║
║     ├─ [ ] models/views/*.yaml — Query views                                  ║
║     ├─ [ ] src/graph/*.ts — Generated types                                   ║
║     └─ [ ] package.json — Version                                             ║
║                                                                               ║
║  3. PACKAGES/DB                                                               ║
║     ├─ [ ] seed/*.cypher — All seed files                                     ║
║     ├─ [ ] migrations/ — Add v0.12.0 migration                                ║
║     └─ [ ] README.md — Update schema docs                                     ║
║                                                                               ║
║  4. TOOLS/NOVANET (Rust)                                                      ║
║     ├─ [ ] Cargo.toml — version = "0.12.0"                                    ║
║     ├─ [ ] CLAUDE.md — Update terminology                                     ║
║     ├─ [ ] src/**/*.rs — All source files                                     ║
║     ├─ [ ] KEYBINDINGS.md — Verify accuracy                                   ║
║     └─ [ ] tests/**/*.rs — Update test assertions                             ║
║                                                                               ║
║  5. APPS/STUDIO                                                               ║
║     ├─ [ ] package.json — Version                                             ║
║     ├─ [ ] src/components/**/*.tsx — UI labels                                ║
║     ├─ [ ] src/stores/*.ts — Type imports                                     ║
║     └─ [ ] README.md — Update docs                                            ║
║                                                                               ║
║  6. .CLAUDE/ (DX)                                                             ║
║     ├─ [ ] rules/novanet-terminology.md — All terms (DONE)                    ║
║     ├─ [ ] rules/novanet-decisions.md — ADRs (DONE)                           ║
║     ├─ [ ] skills/*.md — All skill files                                      ║
║     ├─ [ ] commands/*.md — All command files                                  ║
║     ├─ [ ] hooks/*.json — Hook configurations                                 ║
║     └─ [ ] README.md — Index update                                           ║
║                                                                               ║
║  7. DOCS/                                                                     ║
║     ├─ [ ] BOOK.md — Full documentation book                                  ║
║     ├─ [ ] plans/*.md — Archive old plans, update references                  ║
║     └─ [ ] diagrams/*.mmd — Mermaid diagrams                                  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Skills Testing Matrix

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧪 SKILLS TESTING — ALL MUST PASS                                            ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  NOVANET SKILLS:                                                              ║
║  ├─ [ ] /novanet-arch — Verify ASCII diagram shows new terminology            ║
║  ├─ [ ] /novanet-sync — Test schema validation with new trait names           ║
║  ├─ [ ] /novanet-tui — Launch TUI, check Nexus content                        ║
║  │                                                                            ║
║  SCHEMA SKILLS:                                                               ║
║  ├─ [ ] /schema:add-node — Create test node, verify Class terminology         ║
║  ├─ [ ] /schema:edit-node — Edit node, check trait options                    ║
║  ├─ [ ] /schema:add-arc — Create arc, verify ArcClass references              ║
║  │                                                                            ║
║  AUDIT SKILLS:                                                                ║
║  ├─ [ ] /codebase-audit — Full Ralph Wiggum loop                              ║
║  ├─ [ ] /security-audit — Verify deps, no new vulnerabilities                 ║
║  │                                                                            ║
║  GIT SKILLS:                                                                  ║
║  ├─ [ ] /spn-powers:git:commit — Test commit with new version                 ║
║  └─ [ ] /spn-powers:git:push — Test push workflow                             ║
║                                                                               ║
║  DOCUMENTATION SKILLS:                                                        ║
║  ├─ [ ] /spn-powers:cc-docs:claude-master-docs — Search new terminology       ║
║  └─ [ ] /spn-writing:writing — Test documentation generation                  ║
║                                                                               ║
║  TEST PROCEDURE FOR EACH:                                                     ║
║  1. Invoke skill with representative input                                    ║
║  2. Verify output uses new terminology (Class, not Kind)                      ║
║  3. Verify no deprecated terms appear (Meta, invariant, localized)            ║
║  4. Verify functionality unchanged                                            ║
║  5. Document any issues found                                                 ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Ralph Wiggum Multi-Layer Audit

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🔍 RALPH WIGGUM AUDIT LAYERS                                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  LAYER 1: SYNTAX AUDIT (Automated)                                            ║
║  ├─ cargo fmt --check                                                         ║
║  ├─ cargo clippy -- -D warnings                                               ║
║  ├─ pnpm lint                                                                 ║
║  ├─ yq validation on all YAML                                                 ║
║  └─ No deprecated terms in source (grep audit)                                ║
║                                                                               ║
║  LAYER 2: SEMANTIC AUDIT (Semi-automated)                                     ║
║  ├─ All tests pass: cargo nextest run && pnpm test                            ║
║  ├─ Schema validation: cargo run -- schema validate --strict                  ║
║  ├─ Neo4j coherence: run semantic validation queries                          ║
║  ├─ TypeScript types match Rust: pnpm type-check                              ║
║  └─ Generated artifacts in sync: cargo run -- schema generate --dry-run       ║
║                                                                               ║
║  LAYER 3: SOCRATIC AUDIT (Agent-based)                                        ║
║  ├─ "Why is this node in this layer?" — Every node placement justified        ║
║  ├─ "Why is this trait correct?" — Every trait assignment validated           ║
║  ├─ "What could go wrong?" — Edge cases identified                            ║
║  ├─ "Is this terminology clear?" — LLM comprehension test                     ║
║  └─ "What did we miss?" — Final review                                        ║
║                                                                               ║
║  LAYER 4: INTEGRATION AUDIT (End-to-end)                                      ║
║  ├─ TUI full flow: boot → Graph → Nexus → Quiz → back                         ║
║  ├─ Studio full flow: load → filter → select → detail panel                   ║
║  ├─ CLI full flow: meta → data → query → search                               ║
║  └─ Skills full flow: invoke each skill, verify output                        ║
║                                                                               ║
║  LAYER 5: RELEASE AUDIT (Final gate)                                          ║
║  ├─ CHANGELOG complete with all changes                                       ║
║  ├─ Version numbers consistent across all files                               ║
║  ├─ Git history clean (no WIP commits)                                        ║
║  ├─ All documentation accurate                                                ║
║  └─ Ready for GitHub release                                                  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Ralph Wiggum Automated Script

```bash
#!/bin/bash
# Ralph Wiggum v0.12.0 Audit Script
# Run after each phase and before release

set -e
LOG="/tmp/ralph-wiggum-v012.log"
echo "🔍 Ralph Wiggum Audit — $(date)" | tee "$LOG"

echo "═══════════════════════════════════════════════════════════════" | tee -a "$LOG"
echo "LAYER 1: SYNTAX" | tee -a "$LOG"
echo "═══════════════════════════════════════════════════════════════" | tee -a "$LOG"

# Rust
cd tools/novanet
cargo fmt --check 2>&1 | tee -a "$LOG"
cargo clippy -- -D warnings 2>&1 | tee -a "$LOG"
cd ../..

# TypeScript
pnpm lint 2>&1 | tee -a "$LOG"

# YAML
find packages/core/models -name "*.yaml" -exec yq eval '.' {} \; > /dev/null 2>&1 || echo "YAML ERROR" | tee -a "$LOG"

# Deprecated terms (should return 0 matches)
echo "Checking for deprecated terms..." | tee -a "$LOG"
DEPRECATED=$(grep -rE "(NodeKind|ArcKind|:Meta:|trait:\s*(invariant|localized|knowledge|aggregated))" \
  --include="*.rs" --include="*.ts" --include="*.yaml" \
  packages/ tools/novanet/src/ 2>/dev/null | grep -v "test\|spec\|migration\|plans" | wc -l)
echo "Deprecated terms found: $DEPRECATED" | tee -a "$LOG"

echo "═══════════════════════════════════════════════════════════════" | tee -a "$LOG"
echo "LAYER 2: SEMANTIC" | tee -a "$LOG"
echo "═══════════════════════════════════════════════════════════════" | tee -a "$LOG"

# Tests
cd tools/novanet
cargo nextest run 2>&1 | tail -20 | tee -a "$LOG"
cd ../..
pnpm test 2>&1 | tail -20 | tee -a "$LOG"

# Schema
cd tools/novanet
cargo run -- schema validate --strict 2>&1 | tee -a "$LOG"
cd ../..

# Type check
pnpm type-check 2>&1 | tee -a "$LOG"

echo "═══════════════════════════════════════════════════════════════" | tee -a "$LOG"
echo "AUDIT COMPLETE — Review $LOG" | tee -a "$LOG"
echo "═══════════════════════════════════════════════════════════════" | tee -a "$LOG"
```

## GitHub Release Process

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  📦 GITHUB RELEASE v0.12.0                                                    ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  PRE-RELEASE CHECKLIST:                                                       ║
║  [ ] All 5 Ralph Wiggum audit layers pass                                     ║
║  [ ] CHANGELOG.md updated with full v0.12.0 section                           ║
║  [ ] Version numbers consistent:                                              ║
║      ├─ Cargo.toml: version = "0.12.0"                                        ║
║      ├─ package.json (root): "version": "0.12.0"                              ║
║      ├─ packages/*/package.json: "version": "0.12.0"                          ║
║      └─ CLAUDE.md: **Version**: v0.12.0                                       ║
║  [ ] Git working tree clean (no uncommitted changes)                          ║
║  [ ] Main branch up to date with remote                                       ║
║                                                                               ║
║  RELEASE STEPS:                                                               ║
║  1. Create release branch: git checkout -b release/v0.12.0                    ║
║  2. Final commit: git commit -m "chore(release): v0.12.0 Class Act"           ║
║  3. Create annotated tag:                                                     ║
║     git tag -a v0.12.0 -m "v0.12.0 — Class Act                                ║
║                                                                               ║
║     Complete nomenclature migration:                                          ║
║     - ADR-023: Kind → Class, Meta → Schema                                    ║
║     - ADR-024: Trait = Data Origin                                            ║
║     - ADR-025: Instruction layer renaming                                     ║
║                                                                               ║
║     60 node classes, 114 arc classes, 998 tests passing.                      ║
║     "                                                                         ║
║  4. Push with tags: git push origin release/v0.12.0 --tags                    ║
║  5. Create GitHub Release with auto-generated notes                           ║
║  6. Merge to main: gh pr create --fill && gh pr merge                         ║
║                                                                               ║
║  RELEASE NOTES TEMPLATE:                                                      ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │  ## v0.12.0 "Class Act" — Complete Nomenclature Migration               │  ║
║  │                                                                         │  ║
║  │  ### Breaking Changes                                                   │  ║
║  │  - `NodeKind` → `NodeClass`, `ArcKind` → `ArcClass`                     │  ║
║  │  - Neo4j labels: `:Meta:Kind` → `:Schema:Class`                         │  ║
║  │  - Trait values: `invariant` → `defined`, `localized` → `authored`, etc.│  ║
║  │                                                                         │  ║
║  │  ### Migration Required                                                 │  ║
║  │  - Run `pnpm infra:reset && pnpm infra:seed` for fresh Neo4j            │  ║
║  │  - Run `cargo run -- schema generate` for updated artifacts             │  ║
║  │                                                                         │  ║
║  │  ### New Features                                                       │  ║
║  │  - Clearer terminology aligned with OWL/RDF standards                   │  ║
║  │  - Trait now answers "WHERE does data come from?"                       │  ║
║  │  - Nexus LEARN updated with new terminology                             │  ║
║  │                                                                         │  ║
║  │  ### Full Changelog                                                     │  ║
║  │  See CHANGELOG.md for complete list of changes.                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Complete Phase Timeline (8-12 hours)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ⏱️ COMPLETE MIGRATION TIMELINE                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  HOUR 1-2: YAML & Source of Truth                                             ║
║  ├─ [ ] taxonomy.yaml (traits, colors, borders)                               ║
║  ├─ [ ] visual-encoding.yaml (icons)                                          ║
║  ├─ [ ] node-kinds/**/*.yaml (60 files)                                       ║
║  └─ [ ] arc-kinds/**/*.yaml (if trait references)                             ║
║  └─ GATE: yq validation + Ralph Layer 1                                       ║
║                                                                               ║
║  HOUR 2-4: RUST CODEBASE                                                      ║
║  ├─ [ ] Struct renames (NodeKind→NodeClass, etc.)                             ║
║  ├─ [ ] Generators (organizing, icons, colors, visual_encoding)               ║
║  ├─ [ ] TUI theme (trait_color, trait_icon, trait_border)                     ║
║  ├─ [ ] TUI Nexus (intro, glossary, quiz, tutorial)                           ║
║  ├─ [ ] All other modules                                                     ║
║  └─ GATE: cargo check + cargo nextest run                                     ║
║                                                                               ║
║  HOUR 4-5: NEO4J & CYPHER                                                     ║
║  ├─ [ ] Rename seed files                                                     ║
║  ├─ [ ] Update all Cypher content                                             ║
║  ├─ [ ] Create migration script                                               ║
║  ├─ [ ] Full reseed                                                           ║
║  └─ GATE: Semantic validation queries pass                                    ║
║                                                                               ║
║  HOUR 5-6: TYPESCRIPT & STUDIO                                                ║
║  ├─ [ ] Generate TypeScript from Rust                                         ║
║  ├─ [ ] Update Studio components                                              ║
║  ├─ [ ] Update stores and hooks                                               ║
║  └─ GATE: pnpm type-check + pnpm test                                         ║
║                                                                               ║
║  HOUR 6-7: DOCUMENTATION                                                      ║
║  ├─ [ ] CLAUDE.md (root + tools/novanet/)                                     ║
║  ├─ [ ] README.md files                                                       ║
║  ├─ [ ] CHANGELOG.md                                                          ║
║  ├─ [ ] BOOK.md                                                               ║
║  └─ GATE: Documentation review                                                ║
║                                                                               ║
║  HOUR 7-8: DX & SKILLS                                                        ║
║  ├─ [ ] Update all skills                                                     ║
║  ├─ [ ] Update all commands                                                   ║
║  ├─ [ ] Test each skill                                                       ║
║  └─ GATE: Skills testing matrix complete                                      ║
║                                                                               ║
║  HOUR 8-10: TESTING & AUDIT                                                   ║
║  ├─ [ ] Full test suite                                                       ║
║  ├─ [ ] TUI smoke test                                                        ║
║  ├─ [ ] Studio smoke test                                                     ║
║  ├─ [ ] Ralph Wiggum all 5 layers                                             ║
║  └─ GATE: All audits pass                                                     ║
║                                                                               ║
║  HOUR 10-12: RELEASE                                                          ║
║  ├─ [ ] Version numbers updated                                               ║
║  ├─ [ ] Final commit                                                          ║
║  ├─ [ ] Create tag                                                            ║
║  ├─ [ ] GitHub release                                                        ║
║  └─ DONE: v0.12.0 shipped! 🎉                                                 ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## CRITICAL: Items Previously FORGOTTEN (Socratic Audit)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🚨 FORGOTTEN ITEMS — ADDED FROM SOCRATIC AUDIT                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  P0 - BLOCKING (Must fix or migration fails)                                  ║
║  ├─ [ ] novanet-mcp server (SEPARATE RUST CRATE!) — 21 files, 351 MCP refs    ║
║  │       └─ /tools/novanet-mcp/src/resources/mod.rs                           ║
║  │       └─ /tools/novanet-mcp/src/tools/describe.rs                          ║
║  │       └─ /tools/novanet-mcp/CLAUDE.md                                      ║
║  │       └─ /tools/novanet-mcp/tests/integration_tests.rs                     ║
║  │                                                                            ║
║  ├─ [ ] Legacy query files — 4 files, ACTIVE USE                              ║
║  │       └─ /packages/db/queries/block-generation.cypher                      ║
║  │       └─ /packages/db/queries/page-generation-context.cypher               ║
║  │       └─ /packages/db/queries/project-layer.cypher                         ║
║  │                                                                            ║
║  ├─ [ ] Snapshot tests — 5 files, WILL BREAK CI                               ║
║  │       └─ src/generators/snapshots/*arc_kind*.snap                          ║
║  │       └─ src/generators/snapshots/*node_kind*.snap                         ║
║  │       └─ src/generators/snapshots/*organizing*.snap                        ║
║  │       └─ src/generators/snapshots/*layers*.snap                            ║
║  │       └─ src/generators/snapshots/*hierarchy*.snap                         ║
║  │       └─ Fix: cargo insta review                                           ║
║  │                                                                            ║
║  ├─ [ ] Arc renames (OF_TYPE/HAS_PROMPT) — 73 files, 316 occurrences          ║
║  │       └─ [:OF_TYPE] → [:HAS_STRUCTURE] (Page→PageStructure)                ║
║  │       └─ [:HAS_PROMPT] → [:HAS_INSTRUCTION]                                ║
║  │       └─ Affects: views/*.yaml, queries/*.cypher, Studio configs           ║
║  │                                                                            ║
║  P1 - HIGH (User-facing, must fix before release)                             ║
║  ├─ [ ] i18n.rs content — 780 lines, ALL old terminology                      ║
║  │       └─ Quiz questions: "What does 'invariant' trait mean?"               ║
║  │       └─ Glossary definitions with old trait names                         ║
║  │       └─ French translations need parallel updates                         ║
║  │                                                                            ║
║  ├─ [ ] Studio design system — 6 files                                        ║
║  │       └─ /apps/studio/src/design/traitStyles.ts                            ║
║  │       └─ /apps/studio/src/design/colors/generated.ts                       ║
║  │       └─ /apps/studio/src/design/colors/palette.ts                         ║
║  │       └─ /apps/studio/src/design/icons/nodeIcons.generated.ts              ║
║  │                                                                            ║
║  ├─ [ ] E2E Playwright tests — 6 spec files                                   ║
║  │       └─ /apps/studio/e2e/schema-mode.spec.ts                              ║
║  │       └─ /apps/studio/e2e/focus-mode.spec.ts                               ║
║  │       └─ pnpm test:e2e must pass                                           ║
║  │                                                                            ║
║  P2 - MEDIUM (DX, should fix)                                                 ║
║  ├─ [ ] .claude/skills/*.md — 10+ files with old terminology                  ║
║  │       └─ novanet-architecture/SKILL.md (9 Kind occurrences)                ║
║  │       └─ novanet-sync/SKILL.md (4 Kind occurrences)                        ║
║  │       └─ novanet-mcp/SKILL.md (1 Kind occurrence)                          ║
║  │                                                                            ║
║  ├─ [ ] .claude/commands/*.md — schema commands                               ║
║  │       └─ schema-add-arc.md (1 Kind occurrence)                             ║
║  │       └─ novanet-arch.md (6 Kind occurrences)                              ║
║  │                                                                            ║
║  ├─ [ ] Error messages in Rust — 28 log calls                                 ║
║  │       └─ error!/warn!/info!/debug! may contain "NodeKind"                  ║
║  │                                                                            ║
║  P3 - LOW (Technical debt)                                                    ║
║  ├─ [ ] TODO/FIXME comments — 58 occurrences                                  ║
║  │       └─ Some may reference old terminology                                ║
║  │                                                                            ║
║  ├─ [ ] Console logs in TypeScript — 55 calls                                 ║
║  │       └─ Audit for terminology in log messages                             ║
║  │                                                                            ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Neo4j Backup BEFORE Migration (CRITICAL)

```bash
# MANDATORY: Backup before any migration
# Run this BEFORE starting Phase 4 (Neo4j)

# Option 1: Docker volume backup
docker exec novanet-neo4j neo4j-admin dump --database=neo4j --to=/backups/pre-v012.dump
docker cp novanet-neo4j:/backups/pre-v012.dump ./backups/

# Option 2: Cypher export (data only)
CALL apoc.export.cypher.all("/backups/pre-v012-full.cypher", {format: "plain"})

# Option 3: Full Docker volume backup
docker run --rm -v novanet_neo4j_data:/data -v $(pwd)/backups:/backup \
  alpine tar czf /backup/neo4j-data-pre-v012.tar.gz /data
```

## Rollback Procedure

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ↩️ ROLLBACK PROCEDURE (If migration fails)                                   ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  STEP 1: Git Rollback                                                         ║
║  ├─ git stash (if uncommitted changes)                                        ║
║  ├─ git checkout v0.11.7  (last known good tag)                               ║
║  └─ git branch -D release/v0.12.0 (delete failed branch)                      ║
║                                                                               ║
║  STEP 2: Neo4j Rollback                                                       ║
║  ├─ docker-compose down                                                       ║
║  ├─ docker volume rm novanet_neo4j_data                                       ║
║  ├─ docker run --rm -v novanet_neo4j_data:/data -v $(pwd)/backups:/backup \   ║
║  │    alpine tar xzf /backup/neo4j-data-pre-v012.tar.gz -C /                  ║
║  └─ docker-compose up -d                                                      ║
║                                                                               ║
║  STEP 3: Verify Rollback                                                      ║
║  ├─ cargo run -- meta --format=json | jq '.nodes | length'  # Should be 60   ║
║  ├─ cargo test                                                                ║
║  └─ pnpm test                                                                 ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Rust Refactoring Patterns (from Context7)

```rust
// Pattern 1: Type alias for backward compatibility
// src/types.rs
#[deprecated(since = "0.12.0", note = "Use NodeClass instead")]
pub type NodeKind = NodeClass;

// Pattern 2: Serde alias for YAML backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeTrait {
    #[serde(alias = "invariant")]  // Accept old value during deserialize
    Defined,
    #[serde(alias = "localized")]
    Authored,
    #[serde(alias = "knowledge")]
    Imported,
    Generated,  // No change
    #[serde(alias = "aggregated")]
    Retrieved,
}

// Pattern 3: Exhaustive match linting
#![warn(clippy::match_wildcard_for_single_variants)]
```

## Snapshot Test Strategy

```bash
# After Rust changes, snapshots will fail
# Run interactively to review and accept changes:

cd tools/novanet

# Step 1: Run tests to see failures
cargo nextest run

# Step 2: Review snapshot diffs
cargo insta review

# Step 3: Accept all changes (after visual review)
cargo insta accept

# Step 4: Commit updated snapshots
git add src/generators/snapshots/
git commit -m "test(snapshots): update for v0.12.0 terminology"
```

## Long-Running Session Protocol

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ⚡ MULTI-HOUR SESSION PROTOCOL                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  COMMITMENT: Work continuously until v0.12.0 is complete and released.        ║
║                                                                               ║
║  SESSION MANAGEMENT:                                                          ║
║  ├─ Use tmux for session persistence (detach/reattach)                        ║
║  ├─ Use caffeinate to prevent macOS sleep                                     ║
║  ├─ Log progress to /tmp/v012-progress.log                                    ║
║  └─ Checkpoint after each phase                                               ║
║                                                                               ║
║  PROGRESS TRACKING:                                                           ║
║  ├─ TodoWrite for real-time task tracking                                     ║
║  ├─ Update this plan document with [x] checkmarks                             ║
║  ├─ Commit after each major phase                                             ║
║  └─ Ralph Wiggum audit between phases                                         ║
║                                                                               ║
║  ERROR HANDLING:                                                              ║
║  ├─ If tests fail: fix immediately, don't skip                                ║
║  ├─ If blocked: document issue, try alternative approach                      ║
║  ├─ If context limit: continue in new session, resume from checkpoint         ║
║  └─ If ambiguous: ask user, don't assume                                      ║
║                                                                               ║
║  QUALITY GATES:                                                               ║
║  ├─ NO phase is complete without passing tests                                ║
║  ├─ NO deprecated terms allowed in committed code                             ║
║  ├─ NO skipping validation steps                                              ║
║  └─ FULL Ralph Wiggum before release                                          ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## References

- ADR-023: Class/Instance Terminology + Meta Elimination (novanet-decisions.md)
- ADR-024: Trait Redefinition as "Data Origin" (novanet-decisions.md)
- ADR-025: Instruction Layer Renaming (novanet-decisions.md)
- Brainstorming session: 2026-02-12/13
- 5-agent analysis: Pipeline flow, GEO/SEO, Trait×Layer matrix, Devil's advocate, Industry research
- Research: FRBR, SKOS, ISO 25964, Drupal Entity API, Sanity CMS, Contentful, Neo4j best practices
- Claude Code documentation: hooks, headless mode, sub-agents, GitHub Actions
- Ratatui patterns: Context7 research, joshka (maintainer) recommendations
- TUI UX research: Perplexity (educational TUIs, gamification, glossary patterns)
- Oxycards: https://github.com/BrookJeynes/oxycards (quiz TUI reference)
