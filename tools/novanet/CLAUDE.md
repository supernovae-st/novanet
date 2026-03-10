# CLAUDE.md

This file provides guidance to Claude Code when working in the `tools/novanet/` Rust project.

## Overview

`novanet` is a unified Rust CLI + TUI binary for managing the NovaNet context graph.

**Version**: v0.18.2 | **Tests**: 1210 passing | **Clippy**: zero warnings

```bash
novanet              # Launch TUI (default when no command)
novanet --help       # Show all commands
novanet <command>    # Run specific command
```

## Quick Reference

| Command | Description | Requires Neo4j |
|---------|-------------|----------------|
| `novanet` | Launch interactive TUI | Yes |
| `novanet init` | Interactive setup for first-time users | No |
| `novanet blueprint` | Schema visualization | No |
| `novanet schema generate` | YAML → Cypher/TS/Mermaid | No |
| `novanet schema validate` | Check YAML coherence | No |
| `novanet db seed` | Seed Neo4j database | Yes |
| `novanet search --query=X` | Fulltext search | Yes |
| `novanet doctor` | System health check | Optional |
| `novanet doctor --fix` | Auto-fix schema sync issues | No |
| `novanet export` | Export graph to Cypher/JSON/GraphML/CSV | Yes |
| `novanet stats` | Schema statistics from YAML | No |
| `novanet diff` | Compare YAML schema with Neo4j | Yes |

## Schema Stats

- **57 nodes** (36 shared + 21 org)
- **145 arcs** (6 families)
- **10 layers** (4 shared + 6 org)
- **5 traits** (defined/authored/imported/generated/retrieved)

> v0.18.2: Ultra-deep review complete, 57 nodes, 145 arcs, PageNative.slug per ADR-030

## Tooling

| Tool | Purpose |
|------|---------|
| `cargo-nextest` | Fast parallel tests |
| `cargo-deny` | License/security policy |
| `cargo-audit` | Vulnerability scanning |
| `insta` | Snapshot testing |
| `proptest` | Property-based testing |

## Security Workflow

### Pre-Commit

```bash
# Required before every commit touching Cargo.toml or dependencies
cargo deny check

# Expected output: advisories ok, bans ok, licenses ok, sources ok
```

### Dependency Policy

**Allowed licenses:** MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, MPL-2.0, CDLA-Permissive-2.0

**Advisory handling:**
- Direct deps: Fix immediately or document exception
- Transitive deps: Document in `[advisories.ignore]` with reason

### Exception Management

Exceptions are documented in `deny.toml`:

```toml
[advisories]
ignore = [
    "RUSTSEC-2025-0012",  # backoff (neo4rs transitive) - waiting on neo4rs 0.9.0
]
```

**Review schedule:** Quarterly (check if exceptions can be removed)

### Full Security Audit

```bash
# Complete security check
cargo deny check && cargo audit && cargo machete
```

See `/.claude/rules/security.md` for full security guidelines.

## Commands

### TUI (Default)

```bash
novanet                    # Launch TUI (default when no command)
novanet tui                # Explicit TUI launch
novanet tui --fresh        # Regenerate schema + reset DB, then launch TUI
```

### Init (First-Time Setup)

```bash
novanet init                        # Interactive setup wizard
novanet init --non-interactive      # Use defaults or provided values
novanet init --neo4j-uri=URI        # Custom Neo4j URI
novanet init --neo4j-user=USER      # Custom Neo4j user
novanet init --neo4j-password=PWD   # Set password
novanet init --force                # Overwrite existing config
novanet init --status               # Show current config status
```

Creates `~/.novanet/config.toml` with Neo4j credentials and CLI preferences.

### Schema Operations (No Neo4j Required)

```bash
novanet schema generate              # Generate all artifacts from YAML
novanet schema generate --dry-run    # Preview without writing
novanet schema validate              # Validate YAML coherence
novanet schema validate --strict     # Fail on warnings
novanet schema cypher-validate       # Validate Cypher files against YAML
novanet schema stats                 # Schema statistics (JSON/table)
novanet schema stats --format=json   # JSON output for CI
```

### Blueprint (No Neo4j Required)

```bash
novanet blueprint                    # Full schema visualization
novanet blueprint --view=tree        # Realm > Layer > Class hierarchy
novanet blueprint --view=arcs        # Arc families with relationships
novanet blueprint --view=flow        # Data flow diagrams
novanet blueprint --view=stats       # Raw counts
novanet blueprint --view=glossary    # Term definitions
novanet blueprint --view=cardinality # 1:1, 1:N, N:M constraints
novanet blueprint --no-validate      # Skip validation (faster)
```

### Database Operations (Neo4j Required)

```bash
novanet db seed            # Execute seed Cypher files
novanet db migrate         # Run migrations
novanet db reset           # Drop all + reseed
novanet db verify          # Verify YAML↔Neo4j consistency
```

### Read Operations (Neo4j Required)

```bash
novanet data                         # Data nodes only
novanet overlay                      # Data + Schema overlay
novanet query --realm=org            # Faceted query
novanet query --layer=semantic       # Filter by layer
novanet query --class=Page           # Filter by class
novanet search --query="page"        # Fulltext search
novanet search --query="qr" --class=Entity --limit=20
```

### Write Operations (Neo4j Required)

```bash
novanet node create --class=Page --key=my-page --props='{"display_name":"My Page"}'
novanet node edit --key=my-page --set='{"description":"Updated"}'
novanet node delete --key=my-page --confirm
novanet arc create --from=page1 --to=entity1 --class=USES_ENTITY
novanet arc delete --from=page1 --to=entity1 --class=USES_ENTITY
```

### Documentation (No Neo4j Required)

```bash
novanet doc generate                 # Generate all Mermaid diagrams
novanet doc generate --list          # List available views
novanet doc generate --view=X        # Generate specific view
novanet doc generate --dry-run       # Preview without writing
```

### Locale Operations

```bash
novanet locale list                  # List locales (Neo4j)
novanet locale import --file=X       # Import Cypher file (Neo4j)
novanet locale generate --csv=X --identity-dir=Y  # Generate seed file
```

### Entity Operations

```bash
novanet entity list --project=qrcode-ai       # List phases
novanet entity validate --project=qrcode-ai   # Validate data
novanet entity seed --project=qrcode-ai       # Seed all phases
novanet entity seed --project=qrcode-ai --phase=1  # Seed specific phase
```

### Knowledge Operations

```bash
novanet knowledge list               # List knowledge tiers
novanet knowledge generate --tier=all         # Generate from ATH data
novanet knowledge generate --tier=technical   # Specific tier
```

### Views Operations

```bash
novanet views export                 # Export as JSON
novanet views validate               # Validate Rust↔TS parity
novanet views validate --verbose     # Detailed output
```

### Export Operations (Neo4j Required)

```bash
novanet export                       # Export entire graph to Cypher (default)
novanet export --format=cypher       # Cypher CREATE statements
novanet export --format=json         # JSON node/relationship arrays
novanet export --format=graphml      # GraphML for Gephi/yEd
novanet export --format=csv          # CSV files (nodes.csv, relationships.csv)
novanet export --output=./backup/    # Custom output directory
novanet export --filter="realm:org"  # Export only org realm
```

### Schema Statistics (No Neo4j Required)

```bash
novanet stats                        # Schema statistics from YAML
novanet stats --format=table         # Human-readable table (default)
novanet stats --format=json          # JSON output for CI/scripts
novanet stats --verbose              # Include per-layer breakdown
```

### Schema Diff (Neo4j Required)

```bash
novanet diff                         # Compare YAML schema with Neo4j
novanet diff --verbose               # Show detailed differences
novanet diff --format=json           # JSON output for CI
novanet diff --fix                   # Generate migration Cypher (dry-run)
```

### System Utilities

```bash
novanet doctor                       # System health check
novanet doctor --skip-db             # Skip Neo4j check
novanet doctor --verbose             # Detailed output
novanet doctor --fix                 # Auto-fix schema sync issues
novanet completions bash             # Shell completions (bash/zsh/fish/powershell)
novanet filter build                 # JSON stdin → Cypher stdout (Studio)
```

### Build & Quality

```bash
cargo build                          # Debug build
cargo build --release                # Release build
cargo build --no-default-features    # CLI-only (no TUI)
cargo test                           # Run all tests
cargo nextest run                    # Fast parallel tests
cargo clippy -- -D warnings          # Lint (zero warnings policy)
cargo fmt --check                    # Format check
cargo deny check                     # Security/license audit
```

## Architecture

Module structure:

```
src/
  main.rs         Thin entry point (clap parse + dispatch)
  lib.rs          Public API (re-exports all modules)
  config.rs       Root discovery (resolve_root) + path helpers
  db.rs           Neo4j connection pool (neo4rs::Graph + Arc)
  error.rs        NovaNetError enum (thiserror) + Result type alias
  cypher.rs       CypherStatement builder (data/meta/overlay/query/search)
  facets.rs       FacetFilter (Realm/Layer/Trait/ArcFamily/Class) + JSON serde
  output.rs       OutputFormat (Table/Json/Cypher) + rendering helpers
  commands/
    mod.rs        Module registry
    read.rs       data/meta/overlay/query (CypherStatement → Neo4j → format)
    node.rs       node create/edit/delete (label validation + Cypher)
    arc.rs        arc create/delete (ArcClass validation + Cypher)
    search.rs     search --query (fulltext + property match)
    locale.rs     locale list/import
    db.rs         db seed/migrate/reset (Cypher file execution)
    schema.rs     schema generate/validate (YAML → artifacts)
    doc.rs        doc generate/list (YAML views → Mermaid)
    filter.rs     filter build (JSON stdin → Cypher stdout)
    export.rs     export (Cypher/JSON/GraphML/CSV graph export)
    stats.rs      stats (YAML schema statistics)
    diff.rs       diff (YAML vs Neo4j comparison)
  parsers/        YAML parsers (yaml_node, relations, taxonomy, organizing, views)
  generators/     Code generators (organizing, kind, arc_schema, layer, mermaid, view_mermaid, autowire, hierarchy, colors, icons, visual_encoding, views, tui_icons)
  validation/     Schema validation + auto-fix system
    mod.rs        ValidationEngine + rule registry
    autofix/      AutoFix trait + 6 fixers (FixEngine registry)
      composite_key.rs      Adds pattern to composite key fields
      denormalized_key.rs   Adds denormalized keys for composite nodes
      description.rs        Generates default descriptions
      example_data.rs       Generates type-aware example data
      property_order.rs     Reorders properties to canonical order
      timestamps.rs         Adds created_at/updated_at properties
  tui/            Terminal UI v3 — Unified Tree Architecture (feature-gated)
    mod.rs        Entry point (terminal setup + event loop)
    app.rs        State machine (NavMode: Graph/Nexus, async channels for lazy loading)
    data.rs       UnifiedTree (Realm > Layer > Class > Instance, ArcFamily > ArcClass)
    theme.rs      Visual encoding + Icons (colors from taxonomy.yaml, icons from visual-encoding.yaml)
    ui.rs         3-panel layout (Tree | Info | YAML) + search/help overlays
```

## Key Patterns

- **Error handling**: `thiserror` for `NovaNetError` enum, `color-eyre` in main.rs
- **Neo4j**: `neo4rs::Graph` wrapped in `Arc` (clone freely across tasks)
- **Root discovery**: `--root` flag > `NOVANET_ROOT` env > walk up to `pnpm-workspace.yaml`
- **YAML models**: Live in `packages/core/models/` (relative to monorepo root)
- **Feature gate**: `cargo build --no-default-features` for CLI-only (no TUI deps)
- **YAML-first architecture**: Each Class YAML has explicit `realm:` and `layer:` fields (source of truth)
  - Path validation: file must be at `models/node-classes/{realm}/{layer}/{name}.yaml`
  - Generators read realm/layer from YAML content, validate against path
  - v0.17.2: 2 realms (shared, org), 10 layers (4 shared + 6 org), 57 node types, all with standardized BLOC ordering
- **Query-First architecture (v11.6)**: Cypher is the single source of truth for graph display
  - Schema mode uses CLASSES_QUERY + ARCS_QUERY to build the schema graph
  - `cargo run -- blueprint` executes these foundational queries
  - Views defined in `packages/core/models/views/*.yaml` (no hardcoded TypeScript)
  - See ADR-021 in `.claude/rules/novanet-decisions.md`
- **Icons source of truth (v11.5)**: `visual-encoding.yaml` → `icons:` section
  - Dual format: `web` (Lucide for Studio) + `terminal` (Unicode for TUI)
  - Categories: realms, layers, traits, arc_families, states, navigation, quality, modes
  - TypeScript generated: `packages/core/src/graph/visual-encoding.ts` (ICONS export)
  - Rust compile-time: `tools/novanet/src/tui/icons.rs` (generated constants)
  - Runtime fallback: `Theme::with_root()` loads from YAML with graceful defaults

## Auto-Fix System

**Architecture**: Trait-based auto-fix system with registry pattern for automatically correcting schema validation violations.

### AutoFix Trait

```rust
pub trait AutoFix: Send + Sync {
    fn can_fix(&self, issue: &SchemaIssue) -> bool;
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction>;
    fn description(&self) -> &str;
}
```

### FixEngine Registry

Central registry that applies the first matching fixer for validation issues:

```rust
let engine = FixEngine::default();  // 6 fixers registered
let result = engine.apply_fix(&mut node, &issue)?;

match result {
    FixAction::Modified { changes } => { /* applied */ },
    FixAction::Skipped { reason } => { /* not applicable */ },
}
```

### Registered Fixers

| Fixer | Rule | Action | Example |
|-------|------|--------|---------|
| **CompositeKeyFixer** | COMPOSITE_KEY_FORMAT | Adds `pattern` property to composite key fields | `pattern: "^entity:[a-z0-9-]+@[a-z]{2}-[A-Z]{2}$"` |
| **DenormalizedKeyFixer** | DENORM_REQUIRED | Adds denormalized keys (entity_key, page_key, locale_key) | For EntityNative composite keys |
| **DescriptionFixer** | DESCRIPTION_REQUIRED | Generates contextual description | `"{name} node in the {layer} layer (realm: {realm})"` |
| **ExampleDataFixer** | EXAMPLE_DATA | Generates type-aware example data for required properties | string → "example-{prop}", integer → 1 |
| **PropertyOrderFixer** | PROP_ORDER | Reorders standard_properties to canonical order | key, display_name, description, created_at, updated_at |
| **TimestampFixer** | TIMESTAMP_REQUIRED | Adds created_at/updated_at datetime properties | Required for all nodes |

### Testing Methodology

**TDD RED-GREEN-REFACTOR** cycle:

1. **RED**: Write failing tests (expected behavior, not yet implemented)
2. **GREEN**: Implement minimal code to make tests pass
3. **REFACTOR**: Add property-based tests with `proptest` to verify invariants

**Property-based tests** verify:
- **Correctness**: Required elements always added
- **Idempotence**: Applying fix twice = applying once
- **Identity preservation**: Node metadata (name, realm, layer, trait) unchanged
- **Property preservation**: Existing properties intact

**Coverage**: 52 tests (24 unit + 24 property-based + 4 integration)

### Usage Pattern

```rust
use novanet::validation::autofix::{FixEngine, FixAction};

let mut engine = FixEngine::default();
let result = engine.apply_fix(&mut node, &issue)?;

match result {
    FixAction::Modified { changes } => {
        for change in changes {
            println!("✓ Fixed {}: {:?} → {:?}",
                change.field, change.old_value, change.new_value);
        }
    }
    FixAction::Skipped { reason } => {
        println!("⊘ Skipped: {}", reason);
    }
}
```

### Future Integration

The auto-fix system is ready for integration into `schema validate`:

```bash
cargo run -- schema validate --fix  # Apply automatic fixes (future feature)
```

## TUI Keybindings

See **[KEYBINDINGS.md](./KEYBINDINGS.md)** for complete keyboard shortcuts reference.

Quick summary:
```
Navigation:  j/k (up/down)  h/l (toggle)  d/u (page)
Modes:       [1] Graph (unified tree)  [2] Nexus (Quiz, Audit, Stats, Help)
Scroll:      ENC1 (tree)    ENC2 (yaml)
Overlays:    / (help)       f (search)
Exit:        q or Esc
```

> **Hook**: `.claude/hooks/keybindings-reminder.sh` triggers when `tui/*.rs` is edited.

## Dependencies on Monorepo

This binary reads YAML from `packages/core/models/` (node-classes, arc-classes, taxonomy, views)
and writes to `packages/db/seed/` (Cypher), `packages/core/src/` (TypeScript),
`packages/core/models/docs/` (Mermaid), and `tools/novanet/src/tui/icons.rs` (Rust).
It does NOT depend on any npm packages at build time.

**v11.0 visual encoding**: The `taxonomy.yaml` file is the source of truth for:
- Colors (realms, layers, traits, arc families)
- Border styles (traits: solid/dashed/dotted/double/none)
- Stroke styles (arc families: solid/dashed)
- Unicode characters (box drawing for TUI)

The `organizing` generator reads `taxonomy.yaml` and writes visual encoding properties
to Neo4j via `00.5-taxonomy.cypher`.

## Neo4j

Same credentials as the monorepo:
- **Bolt**: bolt://localhost:7687
- **User**: neo4j
- **Password**: novanetpassword
