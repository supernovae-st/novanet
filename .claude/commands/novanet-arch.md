---
description: Display NovaNet meta-graph architecture diagrams in ASCII. Use when user asks about architecture, node taxonomy, arc taxonomy, layer structure, realm organization, or system overview. ALWAYS reads YAML source of truth first.
argument-hint: [section]
allowed-tools: Bash, Read, Glob, Grep
---

# NovaNet Architecture Diagram Generator

Generate accurate ASCII diagrams of the NovaNet meta-graph **from YAML source of truth**.

---

## CRITICAL: Source of Truth Protocol

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  RULE #1: NEVER GENERATE FROM MEMORY                                          ║
║  RULE #2: ALWAYS READ YAML FILES FIRST                                        ║
║  RULE #3: EXTRACT DATA FROM YAML, THEN DRAW                                   ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

**WHY:** Training data is outdated. YAML is the single source of truth.
Diagrams from memory WILL contain errors (wrong realms, missing nodes, incorrect traits).

---

## Mandatory Workflow

### Step 1: Validate Schema Sync

```bash
cargo run --quiet -- schema validate
```

If validation fails → run `cargo run -- schema generate` first.

### Step 2: Read Source Files (REQUIRED)

**BEFORE drawing ANY diagram, you MUST read:**

| Section | Required YAML Reads |
|---------|---------------------|
| `all` | taxonomy.yaml + ALL node-kinds + ALL arc-kinds |
| `nodes` | taxonomy.yaml + node-kinds/**/*.yaml |
| `arcs` | taxonomy.yaml + arc-kinds/**/*.yaml |
| `seo` | node-kinds/global/seo/*.yaml |
| `knowledge` | node-kinds/global/knowledge/*.yaml |
| `layers` | meta/layers/*.yaml |

**Quick commands to list files:**

```bash
# List all node kinds with realm/layer/trait
grep -r "^  trait:" packages/core/models/node-kinds --include="*.yaml" | sed 's|.*/node-kinds/||' | sort

# List all arc kinds
find packages/core/models/arc-kinds -name "*.yaml" -not -name "_index.yaml" | sort

# Count nodes per layer
find packages/core/models/node-kinds -name "*.yaml" | xargs -I{} dirname {} | sort | uniq -c
```

### Step 3: Extract Data

From YAML files, extract:
- **Node names** from `node.name`
- **Realms** from `node.realm` (global | tenant)
- **Layers** from `node.layer` (config | knowledge | seo | foundation | structure | semantic | instruction | output)
- **Traits** from `node.trait` (invariant | localized | knowledge | derived | job)
- **Colors** from taxonomy.yaml `node_realms[].color`, `node_layers[].color`

### Step 4: Generate ASCII Diagram

Only AFTER reading YAML, generate the diagram using extracted data.

### Step 5: Validate Output

Verify diagram matches YAML:
- [ ] Node count matches file count
- [ ] Realms are correct (from YAML, not memory)
- [ ] Layers are correct
- [ ] Traits are correct (check each node's trait in YAML)
- [ ] No invented nodes (every node has a YAML file)

---

## Available Sections

Use `$ARGUMENTS` to focus on specific section:

| Argument | Description |
|----------|-------------|
| `all` or empty | Complete meta-graph taxonomy (default) |
| `nodes` | Node taxonomy by realm/layer |
| `arcs` | Arc taxonomy by family/scope |
| `seo` | SEO layer nodes and arcs |
| `locale-knowledge` | Locale-knowledge layer (Sets, Atoms) |
| `tenant` | Tenant realm (Organization + Projects + business content) |
| `pipeline` | YAML → TypeScript → Neo4j sync flow |
| `visual` | Visual encoding (colors, borders, strokes) |

---

## ASCII Template

Use this structure for consistency:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                           TITLE (from YAML version)                           ║
╚═══════════════════════════════════════════════════════════════════════════════╝

┌───────────────────────────────────────────────────────────────────────────────┐
│  SECTION TITLE                                                    color      │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  LAYER NAME ──────────────────────────────────────────────────── color       │
│  ├── NodeName ▣ (trait)        description                                   │
│  └── NodeName ▢ (trait)        description                                   │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│  TRAIT LEGEND (from taxonomy.yaml node_traits)                                │
├───────────────────────────────────────────────────────────────────────────────┤
│  ▣ invariant   ─── solid        ▢ localized   ┄┄┄ dashed                     │
│  ┈ knowledge   ┈┈┈ dotted       ═ derived     ═══ double                     │
│    job             none                                                       │
└───────────────────────────────────────────────────────────────────────────────┘
```

---

## Section: nodes

**Required reads:**
1. `packages/core/models/taxonomy.yaml` → realms, layers, colors
2. `packages/core/models/node-kinds/**/*.yaml` → all node definitions

**Extract from each YAML:**
```yaml
node:
  name: SEOKeyword      # ← Node name
  realm: global         # ← WHERE (global | tenant)
  layer: seo            # ← WHAT layer
  trait: knowledge      # ← HOW (invariant | localized | knowledge | derived | job)
```

**Output format:**
```
REALM (count) ──────────────────────────────────────────── realm_color
  LAYER (count) ────────────────────────────────────────── layer_color
    ├── NodeName (trait)    description
    └── NodeName (trait)    description
```

---

## Section: arcs

**Required reads:**
1. `packages/core/models/taxonomy.yaml` → arc_families, arc_scopes
2. `packages/core/models/arc-kinds/**/*.yaml` → all arc definitions

**Extract from each YAML:**
```yaml
arc:
  name: EXPRESSES       # ← Arc name (UPPER_SNAKE_CASE)
  family: semantic      # ← ownership | localization | semantic | generation | mining
  scope: cross_realm    # ← intra_realm | cross_realm
  source: SEOKeyword
  target: Entity
  cardinality: many_to_one
```

---

## Section: seo

**Required reads:**
- `packages/core/models/node-kinds/global/seo/*.yaml`
- `packages/core/models/arc-kinds/mining/*.yaml`

Show: SEOKeyword, SEOKeywordMetrics, SEOMiningRun and their relationships.

---

## Section: knowledge

**Required reads:**
- `packages/core/models/node-kinds/global/locale-knowledge/*.yaml`
- `packages/core/models/node-kinds/tenant/semantic/*.yaml` (Entity, EntityContent)

Show: All Sets (TermSet, etc.), all Atoms (Term, etc.), Entity/EntityContent from tenant realm.

---

## Examples

```bash
/novanet-arch              # Full architecture (reads ALL YAML)
/novanet-arch nodes        # Node taxonomy from node-kinds/**
/novanet-arch arcs         # Arc taxonomy from arc-kinds/**
/novanet-arch seo          # SEO layer from global/seo/
/novanet-arch locale-knowledge  # Locale-knowledge layer from global/locale-knowledge/
/novanet-arch tenant       # Tenant realm (v10.6)
```

---

## Anti-Patterns (DO NOT)

- **DO NOT** draw diagrams from memory
- **DO NOT** assume node locations (always check realm/layer in YAML)
- **DO NOT** invent nodes that don't have YAML files
- **DO NOT** guess traits (always read from YAML)
- **DO NOT** use outdated color values (always from taxonomy.yaml)

---

## Validation Checklist

Before presenting the diagram:

```
PRE-FLIGHT
──────────
[x] Ran schema validate (passed)
[x] Read taxonomy.yaml
[x] Read relevant node-kinds/**/*.yaml
[x] Read relevant arc-kinds/**/*.yaml

ACCURACY
────────
[ ] Every node in diagram has a YAML file
[ ] Realm matches YAML (not assumed)
[ ] Layer matches YAML (not assumed)
[ ] Trait matches YAML (not assumed)
[ ] Colors from taxonomy.yaml
[ ] Node count matches file count

OUTPUT
──────
[ ] Used standard ASCII template
[ ] Included trait legend
[ ] Included statistics
```
