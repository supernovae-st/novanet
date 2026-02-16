---
description: Display NovaNet schema graph architecture diagrams in ASCII. Use when user asks about architecture, node taxonomy, arc taxonomy, layer structure, realm organization, or system overview. ALWAYS reads YAML source of truth first.
argument-hint: [section]
allowed-tools: Bash, Read, Glob, Grep
---

# NovaNet Architecture Diagram Generator

Generate accurate ASCII diagrams of the NovaNet meta-graph **from YAML source of truth**.

---

## CRITICAL: Source of Truth Protocol

```
+===============================================================================+
|  RULE #1: NEVER GENERATE FROM MEMORY                                          |
|  RULE #2: ALWAYS READ YAML FILES FIRST                                        |
|  RULE #3: EXTRACT DATA FROM YAML, THEN DRAW                                   |
+===============================================================================+
```

**WHY:** Training data is outdated. YAML is the single source of truth.
Diagrams from memory WILL contain errors (wrong realms, missing nodes, incorrect traits).

---

## Mandatory Workflow

### Step 1: Validate Schema Sync

```bash
cargo run --quiet -- schema validate
```

If validation fails -> run `cargo run -- schema generate` first.

### Step 2: Read Source Files (REQUIRED)

**BEFORE drawing ANY diagram, you MUST read:**

| Section | Required YAML Reads |
|---------|---------------------|
| `all` | taxonomy.yaml + ALL node-classes + ALL arc-classes |
| `nodes` | taxonomy.yaml + node-classes/**/*.yaml |
| `arcs` | taxonomy.yaml + arc-classes/**/*.yaml |
| `seo` | node-classes/shared/knowledge/*.yaml (SEO* nodes) |
| `geo` | node-classes/shared/knowledge/*.yaml (GEO* nodes) |
| `knowledge` | node-classes/shared/knowledge/*.yaml |
| `locale` | node-classes/shared/locale/*.yaml |
| `geography` | node-classes/shared/geography/*.yaml |

**Quick commands to list files:**

```bash
# List all node kinds with realm/layer/trait
grep -r "^  trait:" packages/core/models/node-classes --include="*.yaml" | sed 's|.*/node-classes/||' | sort

# List all arc kinds
find packages/core/models/arc-classes -name "*.yaml" -not -name "_index.yaml" | sort

# Count nodes per layer
find packages/core/models/node-classes -name "*.yaml" | xargs -I{} dirname {} | sort | uniq -c
```

### Step 3: Extract Data

From YAML files, extract:
- **Node names** from `node.name`
- **Realms** from `node.realm` (shared | org)
- **Layers** from `node.layer` (10 layers: config, locale, geography, knowledge | config, foundation, structure, semantic, instruction, output)
- **Traits** from `node.trait` (defined | authored | imported | generated | retrieved)
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
| `all` or empty | Complete schema graph taxonomy (default) |
| `nodes` | Node taxonomy by realm/layer |
| `arcs` | Arc taxonomy by family/scope |
| `seo` | SEO nodes (shared/knowledge) |
| `geo` | GEO nodes (shared/knowledge) |
| `knowledge` | Knowledge layer (Sets, Atoms) (shared/knowledge) |
| `locale` | Locale layer (shared/locale) |
| `geography` | Geography layer (shared/geography) |
| `org` | Org realm (all business content) |
| `pipeline` | YAML -> TypeScript -> Neo4j sync flow |
| `visual` | Visual encoding (colors, borders, strokes) |

---

## ASCII Template

Use this structure for consistency:

```
+===============================================================================+
|                           TITLE (from YAML version)                           |
+===============================================================================+

+-------------------------------------------------------------------------------+
|  SECTION TITLE                                                    color       |
+-------------------------------------------------------------------------------+
|                                                                               |
|  LAYER NAME ---------------------------------------------- color              |
|  +-- NodeName (trait)        description                                      |
|  +-- NodeName (trait)        description                                      |
|                                                                               |
+-------------------------------------------------------------------------------+

+-------------------------------------------------------------------------------+
|  TRAIT LEGEND (from taxonomy.yaml node_traits)                                |
+-------------------------------------------------------------------------------+
|  solid     = defined         dashed   = authored                              |
|  dotted    = imported        double   = generated                             |
|  thin-dot  = retrieved                                                        |
+-------------------------------------------------------------------------------+
```

---

## Section: nodes

**Required reads:**
1. `packages/core/models/taxonomy.yaml` -> realms, layers, colors
2. `packages/core/models/node-classes/**/*.yaml` -> all node definitions

**Extract from each YAML:**
```yaml
node:
  name: SEOKeyword      # <- Node name
  realm: shared         # <- WHERE (shared | org)
  layer: knowledge      # <- WHAT layer
  trait: imported       # <- HOW (defined | authored | imported | generated | retrieved)
```

**Output format:**
```
REALM (count) -------------------------------------------- realm_color
  LAYER (count) ------------------------------------------ layer_color
    +-- NodeName (trait)    description
    +-- NodeName (trait)    description
```

---

## Section: arcs

**Required reads:**
1. `packages/core/models/taxonomy.yaml` -> arc_families, arc_scopes
2. `packages/core/models/arc-classes/**/*.yaml` -> all arc definitions

**Extract from each YAML:**
```yaml
arc:
  name: EXPRESSES       # <- Arc name (UPPER_SNAKE_CASE)
  family: semantic      # <- ownership | localization | semantic | generation | mining
  scope: cross_realm    # <- intra_realm | cross_realm
  source: SEOKeyword
  target: Entity
  cardinality: many_to_one
```

---

## Section: seo

**Required reads:**
- `packages/core/models/node-classes/shared/knowledge/*.yaml` (SEO* nodes)
- `packages/core/models/arc-classes/mining/*.yaml`

Show: SEOKeyword, SEOKeywordMetrics, SEOCluster, etc. and their relationships.

---

## Section: geo

**Required reads:**
- `packages/core/models/node-classes/shared/knowledge/*.yaml` (GEO* nodes)

Show: GEOQuery, GEOAnswer, GEOMetrics and their relationships.

---

## Section: knowledge

**Required reads:**
- `packages/core/models/node-classes/shared/knowledge/*.yaml`
- `packages/core/models/node-classes/org/semantic/*.yaml` (Entity, EntityNative)

Show: All Sets (TermSet, etc.), all Atoms (Term, etc.), Entity/EntityNative from org realm.

---

## Examples

```bash
/novanet-arch              # Full architecture (reads ALL YAML)
/novanet-arch nodes        # Node taxonomy from node-classes/**
/novanet-arch arcs         # Arc taxonomy from arc-classes/**
/novanet-arch seo          # SEO nodes from shared/knowledge/
/novanet-arch geo          # GEO nodes from shared/knowledge/
/novanet-arch knowledge    # Knowledge layer from shared/knowledge/
/novanet-arch locale       # Locale layer from shared/locale/
/novanet-arch org          # Org realm (v11.3)
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
----------
[x] Ran schema validate (passed)
[x] Read taxonomy.yaml
[x] Read relevant node-classes/**/*.yaml
[x] Read relevant arc-classes/**/*.yaml

ACCURACY
--------
[ ] Every node in diagram has a YAML file
[ ] Realm matches YAML (not assumed)
[ ] Layer matches YAML (not assumed)
[ ] Trait matches YAML (not assumed)
[ ] Colors from taxonomy.yaml
[ ] Node count matches file count

OUTPUT
------
[ ] Used standard ASCII template
[ ] Included trait legend
[ ] Included statistics
```

---

## ADR References

Include relevant ADR citations in diagrams. Use `/adr <number>` for full details.

| Section | Key ADRs | Topic |
|---------|----------|-------|
| `nodes` | ADR-012, ADR-024 | 2-Realm architecture, Trait = Data Origin |
| `arcs` | ADR-026, ADR-027 | Inverse arc policy, Generation family |
| `org` | ADR-028, ADR-029 | Page-Entity architecture, *Native pattern |
| `seo` | ADR-031, ADR-032 | Pillar/cluster, URL slugification |
| `visual` | ADR-005, ADR-013 | Trait encoding, Icons source |
| `pipeline` | ADR-003, ADR-021 | YAML-first, Query-first |

**In diagram footer, cite relevant ADRs:**
```
+-------------------------------------------------------------------------------+
|  ADRs: 012 (2-Realm), 024 (Trait=Origin) | /adr <number> for details          |
+-------------------------------------------------------------------------------+
```
