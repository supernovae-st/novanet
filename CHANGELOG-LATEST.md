# Changelog (Latest)

For complete history, see [CHANGELOG.md](./CHANGELOG.md).

## [Unreleased]

## [0.19.0] - 2026-03-11

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧠 NOVANET v0.19.0 — TRAITS DEPRECATED                                       ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ADR-024 Deprecated  │  3→2 Classification Axes  │  Provenance Per-Instance  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

### ✨ Highlights

| Feature | Status | Impact |
|---------|--------|--------|
| **ADR-024 Deprecated** | ✅ Complete | Traits removed from schema |
| **Node Classification** | ✅ Simplified | 3 axes → 2 axes (Realm + Layer only) |
| **Provenance** | ✅ Moved | Now tracked per-instance on nodes that need it |
| **Documentation** | ✅ Updated | All CLAUDE.md and rules files updated |

### Removed

- **Traits System** — Schema-level trait classification removed
  - Deleted 5 trait YAML files from `models/traits/`
  - Removed `trait_def.rs` parser module
  - Removed TUI Nexus Traits tab and `traits.rs` module
  - Provenance now tracked per-instance on nodes that need it

### Changed

- **Node Classification** — Reduced from 3 axes to 2 axes
  - Realm (WHERE?) + Layer (WHAT?) remain
  - Trait (HOW?) axis removed - no longer a schema-level classification
- **Documentation** — Updated all CLAUDE.md files and rules
  - adr-quick-reference.md: Marked ADR-024 as deprecated
  - novanet-terminology.md: Updated to v0.19.0
  - schema-standard.md: Updated BLOC 1 to remove trait field

### Fixed

- **Nexus Tests** — Fixed 4 failing tests after trait removal
  - Updated shortcut tests (Arch 'r' → 'A')
  - Updated tip tests to use `i18n::tips()` correctly

### 📊 Statistics

```
╭─────────────────────────────────────────────────────────────────────────────────╮
│  📊 v0.17.2 METRICS                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  📦 Neo4j:      19,817 nodes, 58,607 arcs                                       │
│  🌐 Locales:    204 (incl. am-ET, or-IN)                                        │
│  📝 Expressions: 17,342 (unified @locale pattern)                               │
│  📖 Terms:      35 (unified @locale pattern)                                    │
│  📊 CSR:        99.99% (78,415/78,424)                                          │
│                                                                                 │
╰─────────────────────────────────────────────────────────────────────────────────╯
```

---

## [0.17.1] - 2026-03-07

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧠 NOVANET v0.17.1 — SCHEMA CLEANUP + TEST ALIGNMENT                         ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  🧹 YAGNI Cleanup  │  🔄 Test Sync  │  📚 DX Skills  │  ⚡ MCP Perf          ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

### ✨ Highlights

| Feature | Status | Impact |
|---------|--------|--------|
| **🧹 Schema Cleanup (YAGNI)** | ✅ Complete | Removed 6 unused nodes |
| **🔄 Test Alignment** | ✅ Complete | All tests sync with v0.17.0 schema |
| **📚 DX Skill Improvements** | ✅ Complete | 11 skills with "Use when..." pattern |
| **⚡ MCP Performance** | ✅ Complete | Phase 1-3 optimizations |

### Changed

- **Schema YAGNI Cleanup** — Removed 6 unused nodes for leaner schema
  - Removed: Market, AudiencePersona, ChannelSurface, Term, TermSet, SEOKeywordMetrics
  - Added: ProjectGEOScope
  - New counts: 57 nodes (36 shared + 21 org), 131 arc classes
- **DX: Skill Descriptions** — Added "Use when..." pattern to 11 skills
  - nika-run, workflow-validate, nika-binding, nika-debug, nika-diagnose
  - ship, nika-arch, armada, release-notes (2 files), spn
- **MCP rmcp Migration** — Migrated to rmcp 1.1 builder pattern

### Fixed

- **TypeScript Schema Alignment** — Updated NODE_TYPES and CLASS_TAXONOMY to match v0.17.0 YAML
  - Updated layer counts: locale 6→5, knowledge 24→21, foundation 6→8, semantic 4→2
- **Rust Test Alignment** — Updated all test assertions for 57 nodes, 131 arcs
  - stats.rs, node_class.rs, arc_class.rs, autowire.rs, icons.rs, layer.rs, yaml_node.rs

### 📊 Statistics

```
╭─────────────────────────────────────────────────────────────────────────────────╮
│  📊 v0.17.1 METRICS                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  📦 Schema:     57 nodes (36 shared + 21 org), 131 arcs                        │
│  🔧 MCP Tools:  14 tools                                                        │
│  🧪 Rust:       1,255 passing                                                   │
│  📊 TypeScript: 165 passing                                                     │
│  📏 Clippy:     Zero warnings                                                   │
│                                                                                 │
╰─────────────────────────────────────────────────────────────────────────────────╯
```

---

## [0.17.0] - 2026-03-05

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧠 NOVANET v0.17.0 — NEURO-SYMBOLIC VALIDATION                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  🔍 novanet_check  │  📊 novanet_audit  │  📈 CSR Metrics  │  🔒 Security    ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

### ✨ Highlights

| Feature | Status | Impact |
|---------|--------|--------|
| **🔍 novanet_check** | ✅ New | Pre-write validation with llm_context |
| **📊 novanet_audit** | ✅ New | Post-write quality audit with CSR metrics |
| **📈 CSR Metrics** | ✅ New | MMKG-RDS research-based quality scoring |
| **🔒 Cypher Parameterization** | ✅ Fixed | Security hardening for audit queries |

### 🏗️ Neuro-Symbolic Validation

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VALIDATION PHILOSOPHY                                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  novanet_check  → Pre-Write  → "Can I write this?"  → Validates before action  │
│  novanet_audit  → Post-Write → "Did I write correctly?" → Validates after action│
│                                                                                 │
│  CSR (Constraint Satisfaction Rate) = satisfied / (satisfied + violated)        │
│  ├── ≥0.95  → Healthy (green)                                                   │
│  ├── 0.85-0.95 → Warning (yellow)                                               │
│  └── <0.85  → Critical (red)                                                    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Added

- **🔍 MCP Tool: novanet_check** — Pre-write validation with intelligent feedback
  - Validates node writes against schema before execution
  - Combines symbolic rules (required props, trait permissions) with neural context (llm_context)
  - Returns `valid: bool`, `errors[]`, `warnings[]`, `suggestions[]`
  - Supports `upsert_node`, `create_arc`, `update_props` operations

- **📊 MCP Tool: novanet_audit** — Post-write quality audit with CSR metrics
  - 4 audit targets: `coverage`, `orphans`, `integrity`, `freshness`
  - CSR (Constraint Satisfaction Rate) calculation based on MMKG-RDS research
  - Returns issues by severity (critical, warning, info) with actionable messages
  - Ontology insights: most violated constraint, healthiest/weakest layers

- **📈 CSR Metrics Module** — Quality scoring based on academic research
  - `ConstraintSatisfactionRate` struct with rate, satisfied/violated counts
  - `CsrSeverity` enum: Healthy (≥0.95), Warning (0.85-0.95), Critical (<0.85)
  - `LayerMetrics` for per-layer breakdown
  - `AuditSummary` with aggregate statistics

### Fixed

- **🔒 Cypher Injection Prevention** — Parameterized audit queries
  - All audit queries now use `$param` syntax instead of string interpolation
  - Fixed `audit_coverage` locale filter (was using undefined variable)
  - Removed deprecated `sanitize_string()` function

### Changed

- MCP tool count: 12 → 14 (added novanet_write, novanet_check, novanet_audit)
- Documentation updated with neuro-symbolic validation patterns
- All clippy warnings resolved (derivable_impls, field_reassign_with_default)

### 📊 Statistics

```
╭─────────────────────────────────────────────────────────────────────────────────╮
│  📊 v0.17.0 METRICS                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  🔧 MCP Tools:  14 (query, describe, search, traverse, assemble, atoms,        │
│                     generate, introspect, batch, cache_stats, cache_invalidate,│
│                     write, check, audit)                                        │
│  🧪 Tests:      472 passing (MCP server)                                        │
│  📏 Clippy:     Zero warnings                                                   │
│  🔒 Security:   All audit queries parameterized                                 │
│                                                                                 │
╰─────────────────────────────────────────────────────────────────────────────────╯
```

---

## [0.16.2] - 2026-03-03

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧠 NOVANET v0.16.2 — CONTENT GENERATION PIPELINE                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  📋 3-Phase Pipeline  │  🔧 12 MCP Tools  │  📊 1,279 tests  │  🚀 Production ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

### ✨ Highlights

| Feature | Status | Impact |
|---------|--------|--------|
| **📋 3-Phase Pipeline** | ✅ Complete | Full content generation workflow |
| **🔧 Content Generation Skill** | ✅ New | Block generation rules (hero, features, faq, cta) |
| **📊 SESSION.md** | ✅ Complete | Ontology brainstorm D1-D8 decisions |

### 🏗️ 3-Phase Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CONTENT GENERATION PIPELINE                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Phase 1: Entity Bootstrap                                                      │
│  └── 00-entity-native-bootstrap.nika.yaml → Create EntityNative                │
│                                                                                 │
│  Phase 2: SEO Discovery                                                         │
│  └── 06-seo-discovery-modular.nika.yaml → SEO Analysis                         │
│                                                                                 │
│  Phase 3: Content Generation (NEW)                                              │
│  └── 07-content-generation.nika.yaml → Generate BlockNatives + PageNative      │
│      ├── Uses novanet_generate for context assembly                            │
│      └── Validates slug match with EntityNative.denomination_forms.url         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Added

- **📋 Phase 3: Content Generation Workflow** — Complete 3-phase Nika pipeline
  - `07-content-generation.nika.yaml` — Generate BlockNatives + PageNative
  - `skills/content-generation.md` — Block generation rules (head-seo-meta, hero, features, faq, cta)
  - Uses `novanet_generate` for context assembly
  - Validates slug match with EntityNative.denomination_forms.url

- **📝 SESSION.md Complete** — Ontology brainstorm session finalized
  - D1-D8 decisions documented
  - 3-workflow architecture validated
  - 8 workflows + 4 support files catalogued

### Changed

- **📦 Dependencies** — Batch updates from dependabot
  - autoprefixer: 10.4.24 → 10.4.27
  - postcss: 8.5.6 → 8.5.8
  - framer-motion: 12.34.3 → 12.34.5
  - eslint: 9.39.2 → 9.39.3
  - lucide-react: 0.575.0 → 0.576.0

---

## [0.16.0] - 2026-03-03

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧠 NOVANET v0.16.0 — INIT + CONFIG + DOCTOR                                  ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  🚀 novanet init  │  ⚙️ User Config  │  🔧 doctor --fix  │  💡 Error Hints   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

### ✨ Highlights

| Feature | Status | Impact |
|---------|--------|--------|
| **🚀 novanet init** | ✅ New | Interactive setup wizard for first-time users |
| **⚙️ User Config** | ✅ New | Persistent ~/.novanet/config.toml |
| **🔧 doctor --fix** | ✅ New | Automatic schema sync repair |
| **💡 Error Hints** | ✅ New | Actionable suggestions for common errors |

### Added

- **🚀 novanet init** — Interactive setup wizard for first-time users
  - Creates `~/.novanet/config.toml` with Neo4j credentials
  - Interactive mode with prompts for URI, user, password
  - Non-interactive mode with CLI flags (`--neo4j-uri`, `--neo4j-user`, `--neo4j-password`)
  - `--status` flag to show current configuration
  - `--force` flag to overwrite existing config
  - Automatic Neo4j connection test on setup

- **⚙️ User Config System** — Persistent configuration via `~/.novanet/config.toml`
  - Neo4j credentials (uri, user, password)
  - CLI preferences (default_format, verbose)
  - Environment variable fallback (NEO4J_URI, NEO4J_USER, NEO4J_PASSWORD)

- **🔧 doctor --fix** — Automatic schema sync repair
  - Detects schema sync issues
  - Runs `schema generate` automatically when `--fix` is passed
  - Re-validates after fix to confirm resolution

- **💡 Error Hints System** — Actionable suggestions for common errors
  - Connection errors: Suggests starting Neo4j or running init
  - Authentication errors: Points to password configuration
  - Schema errors: Suggests validation and regeneration commands
  - I/O errors: Identifies file/permission issues
  - `ErrorHint` trait with `format_error_with_hint()` helper

### 📊 Statistics

```
╭─────────────────────────────────────────────────────────────────────────────────╮
│  📊 v0.16.0 METRICS                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  🧪 Tests:        1,255 passing (CLI + 10 new error hint tests)                 │
│  📏 Clippy:       Zero warnings                                                 │
│  🔧 Commands:     Added init, enhanced doctor --fix                             │
│                                                                                 │
╰─────────────────────────────────────────────────────────────────────────────────╯
```

---

## [0.15.3] - 2026-03-03

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧠 NOVANET v0.15.3 — SECURITY HARDENING                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  🔒 Cypher Injection  │  🧹 Memory Leak  │  🔗 HAS_NATIVE  │  ✅ Validation   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

### ⚠️ Security

| Issue | Severity | Fix |
|-------|----------|-----|
| **Cypher Injection** | 🔴 Critical | Regex validation for class/arc names |
| **Memory Leak** | 🔴 Critical | moka::sync::Cache with TTL eviction |

### 🔒 Security Details

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CYPHER INJECTION PREVENTION                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Added: validation.rs with regex validation                                     │
│                                                                                 │
│  Node classes: ^[A-Z][A-Za-z0-9]*$           (PascalCase)                      │
│  Arc classes:  ^[A-Z][A-Z0-9_]*$             (SCREAMING_SNAKE_CASE)            │
│                                                                                 │
│  Rejects injection attempts like: "Entity}DETACH DELETE n"                      │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  MEMORY LEAK FIX                                                                │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Before: Hand-rolled SchemaCache (expired entries never evicted)                │
│  After:  moka::sync::Cache (automatic TTL-based eviction)                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Added

- **🔗 HAS_NATIVE Auto-Arc** — Automatic arc creation for `*Native` classes
  - When upserting `EntityNative`, `PageNative`, or `BlockNative` with key containing `@`
  - Automatically creates `(Entity)-[:HAS_NATIVE]->(EntityNative)` arc

- **✅ Required Property Validation** — Schema-based validation before writes
  - Checks `required_properties` from ClassMetadata
  - Returns detailed error with missing property names

### 📊 Statistics

```
╭─────────────────────────────────────────────────────────────────────────────────╮
│  📊 v0.15.3 METRICS                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  🔧 MCP Tools:    12 (query, describe, search, traverse, assemble, atoms,       │
│                      generate, introspect, batch, cache_stats,                  │
│                      cache_invalidate, write)                                   │
│  🧪 Tests:        430 passing (MCP server, was 348)                             │
│  📏 Clippy:       Zero warnings                                                 │
│                                                                                 │
╰─────────────────────────────────────────────────────────────────────────────────╯
```

---

## [0.15.2] - 2026-03-03

### Fixed

- **📝 Documentation Accuracy** — Updated test counts and version references
  - Test badge: 1226 → 1279 in README.md
  - CLI test count: 950 → 1279 in README.md
  - Version references: v0.13.0 → v0.15.1 in packages/core/CLAUDE.md
  - Arc count: 169 → 182 in packages/core/CLAUDE.md
  - MCP Server version header: v0.5.0 → v0.15.1 in CLAUDE.md

- **🧹 Debug cleanup** — Removed debug console.log from `/api/graph/query` route

- **🦀 Rust code quality** — Replaced `.unwrap()` with `.expect()` in diff.rs

### 📊 Statistics

```
🧪 Tests:    1,279 passing (39 MCP + 1,240 CLI)
📊 TS Tests: 610 passing (Studio + Core)
📏 Clippy:   Zero warnings
```

---

## [0.15.1] - 2026-03-03

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🧠 NOVANET v0.15.1 — VERSION ALIGNMENT                                       ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  📦 All Packages Aligned  │  🔧 11 MCP Tools  │  🔄 Unified Versioning        ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

### Changed

- **📦 Version Alignment** — All packages now at 0.15.1
  - TypeScript packages: root, @novanet/core, @novanet/db, @novanet/studio
  - Rust CLI: tools/novanet
  - Rust MCP: tools/novanet-mcp (reset from 0.6.0 to align with NovaNet versioning)

- **🔄 MCP Versioning Strategy** — MCP server now follows NovaNet version
  - Simplifies version tracking: "NovaNet 0.15.1" = all components at 0.15.1
  - MCP protocol compatibility tracked via rmcp crate version (0.16)

### 📊 Statistics

```
╭─────────────────────────────────────────────────────────────────────────────────╮
│  📊 v0.15.1 METRICS                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  🔧 MCP Tools:    11 (query, describe, search, traverse, assemble, atoms,       │
│                      generate, introspect, batch, cache_stats, cache_invalidate)│
│  🧪 Tests:        1,279 passing (39 MCP + 1,240 CLI)                            │
│  📦 Packages:     All aligned at 0.15.1                                         │
│                                                                                 │
╰─────────────────────────────────────────────────────────────────────────────────╯
```

---

## [0.15.0] - 2026-03-02

### Added

- **🔧 MCP Tool: novanet_batch** — Bulk operations with parallel execution support
  - Execute multiple MCP operations in a single call
  - Configurable parallelism with `max_concurrent` parameter
  - `fail_fast` option to stop on first error or continue

- **📊 MCP Tool: novanet_cache_stats** — Cache statistics and monitoring
  - Get cache hit/miss counts and hit rate percentage
  - View entry count, memory usage, and TTL settings

- **🧹 MCP Tool: novanet_cache_invalidate** — Manual cache invalidation
  - Clear all cached queries with `all=true`
  - Pattern-based invalidation

- **💡 Error Hints System** — Actionable suggestions for common errors (10 categories)

- **📤 CLI Command: novanet export** — Export graph data (Cypher, JSON, GraphML, CSV)

- **📊 CLI Command: novanet stats** — Schema statistics from YAML (offline)

- **🔍 CLI Command: novanet diff** — Compare YAML schema with Neo4j database

### 📊 Statistics

```
🔧 MCP Tools:  11 (8 → 11: +batch, +cache_stats, +cache_invalidate)
🧪 Tests:      1,279 passing (39 MCP + 1,240 CLI)
📏 Clippy:     Zero warnings
```

---

## [0.14.0] - 2026-02-19

### Added

- **🔍 MCP Tool: novanet_introspect** — 8th MCP tool for schema introspection
  - Query NodeClasses filtered by realm/layer
  - Query ArcClasses filtered by family
  - Get specific class/arc details with relationships

- **📋 context_build_log** — New debugging feature in `novanet_generate`
  - 5 phases logged: structure_phase, entities_phase, atoms_phase, anchors_phase, token_decisions

---

## [0.13.1] - 2026-02-17

### Added

- **📊 6th Arc Family: Schema** — Meta-schema relationships for graph structure
  - `OF_CLASS` — Connects instance nodes to their Class definitions
  - `FROM_CLASS` — ArcClass defines source node types
  - `TO_CLASS` — ArcClass defines target node types

### 📊 Statistics

```
📦 Nodes:  61 (40 shared + 21 org), 10 layers, 5 traits
🔗 Arcs:   182 (6 families: ownership, localization, semantic, generation, mining, schema)
```

---

## [0.13.0] - 2026-02-15

### ⚠️ Breaking Changes

- **ADR-029: *Native Pattern** — All locale-specific nodes use `*Native` suffix
  - `EntityContent` → `EntityNative` (trait: authored)
  - `ProjectContent` → `ProjectNative` (trait: authored)
  - `PageGenerated` → `PageNative` (trait: generated)
  - `BlockGenerated` → `BlockNative` (trait: generated)

- **ADR-029: Arc Merges** — Unified arc pattern for native content
  - `HAS_CONTENT` + `HAS_GENERATED` → `HAS_NATIVE`
  - `CONTENT_OF` + `GENERATED_FOR` → `NATIVE_OF`

- **ADR-030: Slug Ownership** — Slugs moved from Entity to Page

### 📊 Statistics

```
📝 Files:   51 changed in commit
🧪 Tests:   1,030 Rust tests passing
📦 Nodes:   61 (40 shared + 21 org)
🔗 Arcs:    169 (was 171, merged 4 → 2)
```

---

[Unreleased]: https://github.com/supernovae-st/novanet/compare/v0.16.2...HEAD
[0.16.2]: https://github.com/supernovae-st/novanet/compare/v0.16.0...v0.16.2
[0.16.0]: https://github.com/supernovae-st/novanet/compare/v0.15.3...v0.16.0
[0.15.3]: https://github.com/supernovae-st/novanet/compare/v0.15.2...v0.15.3
[0.15.2]: https://github.com/supernovae-st/novanet/compare/v0.15.1...v0.15.2
[0.15.1]: https://github.com/supernovae-st/novanet/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/supernovae-st/novanet/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/supernovae-st/novanet/compare/v0.13.1...v0.14.0
[0.13.1]: https://github.com/supernovae-st/novanet/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/supernovae-st/novanet/releases/tag/v0.13.0
