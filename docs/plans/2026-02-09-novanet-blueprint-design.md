# NovaNet Blueprint Command Design

**Date**: 2026-02-09
**Status**: Approved
**Author**: Thibaut + Claude

## Overview

`novanet blueprint` is a comprehensive meta-graph visualization and validation command that provides deep understanding of the NovaNet knowledge graph architecture.

Unlike the basic `novanet meta` (which just lists nodes), `blueprint` is:
- **Pedagogical**: Explains concepts with examples and diagrams
- **Visual**: Rich ASCII flows and hierarchies
- **Validating**: Compares YAML, Neo4j, and Cypher for coherence
- **Multi-view**: 10 specialized views for different needs

## Command Interface

```bash
# Default: rich overview with all sections
novanet blueprint

# Specific view
novanet blueprint --view=<VIEW>

# Output formats
novanet blueprint --format=json    # Machine-readable
novanet blueprint --format=table   # Default ASCII

# Performance
novanet blueprint --no-validate    # Skip YAML↔Neo4j validation (faster)

# Filters (for some views)
novanet blueprint --view=deps --kind=Entity
novanet blueprint --view=coverage --locale=fr-FR
```

## 10 Available Views

### Understanding the Model

| View | Purpose | Key Question |
|------|---------|--------------|
| `tree` | Hierarchy Realm > Layer > Kind | "What types exist and where?" |
| `flow` | 6 data flow diagrams | "How does data move?" |
| `content` | Content Model deep-dive | "How do Entity/Page/Block connect?" |
| `arcs` | All arcs grouped by family | "What relationships exist?" |
| `cardinality` | 1:1, 1:N, N:M constraints | "What are the rules?" |
| `glossary` | Concept definitions | "What does X mean?" |

### Analyzing State

| View | Purpose | Key Question |
|------|---------|--------------|
| `audit` | Health check, drift detection | "Is everything coherent?" |
| `deps` | Dependency impact analysis | "If I delete X, what breaks?" |
| `coverage` | Locale completion status | "How complete is locale X?" |
| `stats` | Raw numbers (for CI/scripts) | "Give me the facts" |

## Default Output Structure

When running `novanet blueprint` without arguments:

```
╭──────────────────────────────────────────────────────────────────────────────╮
│  ◉ NOVANET BLUEPRINT                                          v11.0.0       │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  STATS             76 NodeKinds │ 123 ArcKinds │ 2 Realms │ 9 Layers        │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  REALMS                                                                      │
│  ◉ global    ████░░░░░░░░░░░░  12 kinds (read-only, universal knowledge)    │
│  ◎ tenant    ████████████████  64 kinds (business-specific content)         │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  LAYERS                                                                      │
│  ⚙ config           ██         4    System settings                         │
│  📚 locale-knowledge ████       8    Terms, expressions, culture             │
│  🏗 foundation       ███        6    Project, tenant base                    │
│  📄 structure        █████     12    Pages, blocks                           │
│  ◆ semantic         ███████   18    Entities, content                       │
│  📝 instruction      ██         5    Generation prompts                      │
│  🔍 seo              ████       9    Keywords, geo, queries                  │
│  ● output           ██████    14    Generated artifacts                     │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  TRAITS (how nodes behave with locales)                                      │
│  ■ invariant   Same across all locales      │ Entity, Page, Block           │
│  □ localized   Native content per locale    │ EntityContent, ProjectL10n    │
│  ◊ knowledge   Locale-specific atoms        │ Term, Expression, Taboo       │
│  ◇ derived     Generated from invariants    │ PageGenerated, BlockGenerated │
│  ○ job         Async processing tasks       │ GenerationJob                 │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  CORE FLOW                                                                   │
│                                                                              │
│  Entity ────[HAS_CONTENT]────► EntityContent ────[USES_TERM]────► Term       │
│     │                               │                                        │
│     │                               ▼                                        │
│     │                         (LLM generation)                               │
│     │                               │                                        │
│     └────[HAS_GENERATED]────► PageGenerated ◄────[GENERATED_FOR]──── Page    │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  ARC FAMILIES                                                                │
│  → ownership      43 arcs    Parent-child hierarchy (HAS_PAGE, HAS_BLOCK)   │
│  ⇢ localization   12 arcs    Invariant↔localized links (HAS_CONTENT)        │
│  ⇄ semantic       28 arcs    Meaning connections (USES_ENTITY)              │
│  ⇉ generation     15 arcs    LLM pipeline (HAS_GENERATED)                   │
│  ⇶ mining         25 arcs    Knowledge extraction (EXTRACTS_TERM)           │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  VALIDATION                                                      ✓ COHERENT │
│  YAML Schema ✓ │ Neo4j Meta ✓ │ Seed Files ✓ │ 0 issues                     │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  📖 Views: --view=tree|flow|content|arcs|cardinality|glossary|audit|deps|   │
│            coverage|stats                                                    │
╰──────────────────────────────────────────────────────────────────────────────╯
```

## View: flow

6 detailed flow diagrams:

### 1. Ownership Flow
```
Tenant
  ├──[HAS_PROJECT]──► Project ──[HAS_L10N]──► ProjectL10n
  │                      ├──[HAS_PAGE]──► Page ──[HAS_BLOCK]──► Block
  │                      └──[HAS_ENTITY]──► Entity
  └──[HAS_CONFIG]──► TenantConfig
```

### 2. Localization Flow
```
┌─────────────────────────────────────────────────────────────────────────┐
│ INVARIANT (defined 1×)              LOCALIZED (exists per locale)       │
├─────────────────────────────────────────────────────────────────────────┤
│  Entity ─────[HAS_CONTENT]─────►  EntityContent                         │
│  (key: "qr-generator")            (key: "entity:qr-generator@fr-FR")    │
│                                   (key: "entity:qr-generator@de-DE")    │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3. Knowledge Flow
```
Locale (fr-FR)
  ├──[HAS_TERMS]──► TermSet ──[CONTAINS_TERM]──► Term ◊
  ├──[HAS_EXPRESSIONS]──► ExpressionSet ──[CONTAINS_EXPRESSION]──► Expression
  ├──[HAS_PATTERNS]──► PatternSet ──[CONTAINS_PATTERN]──► Pattern
  ├──[HAS_CULTURE]──► CultureSet ──[CONTAINS_CULTURE_REF]──► CultureRef
  └──[HAS_TABOOS]──► TabooSet ──[CONTAINS_TABOO]──► Taboo
```

### 4. Generation Flow
```
    INPUTS                      PROCESS                    OUTPUT
┌─────────────────┐      ┌─────────────────┐       ┌─────────────────┐
│ EntityContent   │─────►│                 │       │ PageGenerated   │
│ Term ◊          │─────►│ GenerationJob ○ │──────►│ BlockGenerated  │
│ Instruction     │─────►│                 │       │                 │
└─────────────────┘      └─────────────────┘       └─────────────────┘
```

### 5. SEO Flow
```
Page ■
  ├──[HAS_SEO_KEYWORDS]────► SEOKeyword □
  ├──[HAS_GEO_QUERIES]─────► GeoQuery □
  ├──[HAS_SEARCH_INTENT]───► SearchIntent □
  └──[TARGETS_AUDIENCE]────► AudienceTrait ◊
```

### 6. Cross-Realm Flow
```
╔═════════════════════════════════════════════════════════════════════╗
║  GLOBAL REALM (read-only)                                           ║
║  Locale ──► LocaleVoice ──► TermSet ──► Term                        ║
╚═══════════════════════════╦═════════════════════════════════════════╝
                            ║ [USES_TERM] (cross-realm)
╔═══════════════════════════▼═════════════════════════════════════════╗
║  TENANT REALM (read-write)                                          ║
║  EntityContent ─────────────────► uses global Terms                 ║
╚═════════════════════════════════════════════════════════════════════╝
```

## View: content

The complete Content Model showing Entity/Page/Block/Content/Generated relationships.

See detailed ASCII diagram in brainstorm notes.

## View: deps

Dependency impact analysis for refactoring safety:

```
DEPENDENCY ANALYSIS: Entity

Entity ■
  ├── DEPENDED BY (if Entity deleted, these break):
  │   ├── EntityContent □ ──[HAS_CONTENT]── (200+ instances)
  │   ├── Page ■ ──[USES_ENTITY]── (150 connections)
  │   └── SEOKeyword □ ──[TARGETS_ENTITY]── (80 connections)
  │
  ├── DEPENDS ON (Entity needs these to exist):
  │   ├── Project ■ ──[HAS_ENTITY]── (owner)
  │   └── EntityType ■ ──[HAS_TYPE]── (classifier)
  │
  └── IMPACT SCORE: 🔴 HIGH (430 dependent connections)
```

## View: coverage

Locale completion dashboard:

```
Locale        Content    Generated   Knowledge   Overall
───────────────────────────────────────────────────────
en-US         ████████   ████████    ████████    100% ✓
fr-FR         ████████   ████████    ███████░    95%  ✓
de-DE         ██████░░   █████░░░    ██████░░    72%
ja-JP         ████░░░░   ███░░░░░    █████░░░    58%
sw-KE         ░░░░░░░░   ░░░░░░░░    █░░░░░░░    8%   ⚠
```

## View: cardinality

Relationship constraints:

```
1:1 (One-to-One) — Exclusive pairing
Entity ■ ════[HAS_CONTENT]════ EntityContent □ (per locale)

1:N (One-to-Many) — Parent owns children
Project ■ ────[HAS_PAGE]────► Page ■ (many)

N:M (Many-to-Many) — Flexible associations
Page ■ ◄────[USES_ENTITY]────► Entity ■
```

## Validation System

The command validates coherence across 3 sources:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    YAML     │     │    Neo4j    │     │   Cypher    │
│  (schema)   │     │   (data)    │     │  (seeds)    │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       └─────────────┬─────┴───────────────────┘
                     ▼
            ┌─────────────────┐
            │  COMPARATEUR    │
            └─────────────────┘
```

### Validation Checks

1. **YAML → Neo4j**: All YAML kinds exist as Meta nodes in Neo4j
2. **Neo4j → YAML**: No orphan Meta nodes without YAML definition
3. **Arc coherence**: Source/target types in arcs match existing kinds
4. **Path validation**: YAML file paths match realm/layer fields
5. **Seed parsing**: All .cypher files are syntactically valid

### Validation Output

```
VALIDATION                                                      ✓ COHERENT
├── All YAML kinds exist in Neo4j                                    ✓
├── All Neo4j kinds defined in YAML                                  ✓
├── Arc source/target types match                                    ✓
├── Realm/Layer paths match YAML content                             ✓
└── No orphan nodes (nodes without arcs)                             ✓
```

Or with issues:

```
VALIDATION                                                   ⚠ 3 ISSUES
├── All YAML kinds exist in Neo4j                                    ✓
├── All Neo4j kinds defined in YAML                                  ⚠
│   └── Missing in YAML: OldLegacyNode, DeprecatedThing
├── Arc source/target types match                                    ⚠
│   └── HAS_FOO: source Page not in YAML
└── No orphan nodes                                                  ⚠
     └── 2 orphans: TestNode, DebugKind

💡 Run: novanet blueprint --view=audit for details
💡 Run: novanet schema validate --fix to auto-repair
```

## Architecture

### File Structure

```
src/
  commands/
    mod.rs              # Add blueprint module
    blueprint.rs        # Command entry point
  blueprint/
    mod.rs              # Module exports
    sources.rs          # YAML loader + Neo4j queries
    validation.rs       # Coherence checker
    ascii.rs            # Box drawing utilities
    views/
      mod.rs            # View dispatcher
      tree.rs           # Hierarchy view
      flow.rs           # 6 flow diagrams
      content.rs        # Content model deep-dive
      arcs.rs           # Arc family listing
      cardinality.rs    # 1:1/1:N/N:M view
      glossary.rs       # Concept definitions
      audit.rs          # Health check
      deps.rs           # Dependency analysis
      coverage.rs       # Locale completion
      stats.rs          # Raw numbers
```

### Key Types

```rust
/// Available blueprint views
#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum BlueprintView {
    Tree,
    Flow,
    Content,
    Arcs,
    Cardinality,
    Glossary,
    Audit,
    Deps,
    Coverage,
    Stats,
}

/// Validation result
pub struct ValidationResult {
    pub yaml_kinds: usize,
    pub neo4j_kinds: usize,
    pub issues: Vec<ValidationIssue>,
}

/// Single validation issue
pub struct ValidationIssue {
    pub category: IssueCategory,
    pub severity: Severity,
    pub message: String,
    pub fix_hint: Option<String>,
}
```

## Dependencies

Uses existing crates:
- `tabled` for table formatting (already in use)
- `neo4rs` for Neo4j queries (already in use)
- `serde_yaml` for YAML parsing (already in use)

No new dependencies required.

## Future Enhancements

1. **Interactive mode**: `novanet blueprint -i` with arrow key navigation
2. **Export**: `--export=mermaid` to generate Mermaid diagrams
3. **Watch mode**: `--watch` to auto-refresh on YAML changes
4. **Diff mode**: `novanet blueprint --diff v10.9..v11.0` to compare versions

## Success Criteria

- [ ] `novanet blueprint` displays rich overview in < 2s
- [ ] All 10 views implemented and tested
- [ ] Validation catches real drift issues
- [ ] ASCII renders correctly in standard terminals (80+ cols)
- [ ] JSON output parseable by jq
- [ ] 100% test coverage on validation logic
