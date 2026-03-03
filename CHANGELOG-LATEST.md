# Changelog (Latest)

For complete history, see [CHANGELOG.md](./CHANGELOG.md).

## [Unreleased]

## [0.15.2] - 2026-03-03

### Fixed
- **Documentation Accuracy** - Updated test counts and version references across all CLAUDE.md files
  - Test badge: 1226 → 1279 in README.md
  - CLI test count: 950 → 1279 in README.md
  - Version references: v0.13.0 → v0.15.1 in packages/core/CLAUDE.md
  - Arc count: 169 → 182 in packages/core/CLAUDE.md
  - MCP Server version header: v0.5.0 → v0.15.1 in CLAUDE.md
- **Debug console.log removed** - Removed debug logging from `/api/graph/query` route (production cleanup)
- **Rust code quality** - Replaced `.unwrap()` with `.expect()` in diff.rs for better error messages
  - Lines 170-177: node class intersection lookup
  - Lines 269-276: arc class intersection lookup

### Statistics
- **1279 tests passing** (39 MCP + 1240 CLI)
- **610 TypeScript tests passing** (Studio + Core)
- **Zero clippy warnings**

## [0.15.1] - 2026-03-03

### Changed
- **Version Alignment** - All packages now at 0.15.1
  - TypeScript packages: root, @novanet/core, @novanet/db, @novanet/studio
  - Rust CLI: tools/novanet
  - Rust MCP: tools/novanet-mcp (reset from 0.6.0 to align with NovaNet versioning)
- **MCP Versioning Strategy** - MCP server now follows NovaNet version (was independent 0.x.x)
  - Simplifies version tracking: "NovaNet 0.15.1" = all components at 0.15.1
  - MCP protocol compatibility tracked via rmcp crate version (0.16)

### Statistics
- **11 MCP tools**: query, describe, search, traverse, assemble, atoms, generate, introspect, batch, cache_stats, cache_invalidate
- **1279 tests passing** (39 MCP + 1240 CLI)
- **All packages aligned** at 0.15.1

## [0.15.0] - 2026-03-02

### Added
- **MCP Tool: novanet_batch** - Bulk operations with parallel execution support
  - Execute multiple MCP operations in a single call
  - Configurable parallelism with `max_concurrent` parameter
  - `fail_fast` option to stop on first error or continue
  - Returns detailed results for each operation
- **MCP Tool: novanet_cache_stats** - Cache statistics and monitoring
  - Get cache hit/miss counts and hit rate percentage
  - View entry count, memory usage, and TTL settings
  - Useful for debugging and performance optimization
- **MCP Tool: novanet_cache_invalidate** - Manual cache invalidation
  - Clear all cached queries with `all=true`
  - Pattern-based invalidation (returns error with guidance)
  - Reports invalidation count and remaining entries
- **Error Hints System** - Actionable suggestions for common errors
  - Pattern-based error detection (10 categories)
  - Hints for Cypher syntax, Neo4j connection, auth, timeouts, etc.
  - `with_hint()` method on Error type for enhanced messages
- **CLI Command: novanet export** - Export graph data to multiple formats
  - Formats: Cypher, JSON, GraphML, CSV
  - Filter by labels (`--labels`) and relationship types (`--relationships`)
  - Custom Cypher query support (`--query`)
  - Schema export option (`--include-schema`)
- **CLI Command: novanet stats** - Schema statistics from YAML (offline)
  - Node classes by realm, layer, and trait
  - Arc classes by family and scope
  - Output formats: text (default), JSON, YAML
  - `--detailed` and `--include-arcs` flags
- **CLI Command: novanet diff** - Compare YAML schema with Neo4j database
  - Detect added/removed/modified node and arc classes
  - Human-readable and JSON output formats
  - `--exit-code` for CI integration (exits 1 if drift detected)
  - `--nodes-only` and `--arcs-only` filters

### Changed
- MCP tool count: 8 → 11 (added batch, cache_stats, cache_invalidate)
- NovaNet MCP version: 0.5.0 → 0.6.0
- CLI command count updated with 3 new commands
- Documentation updated with 11 tools references

### Statistics
- **11 MCP tools**: query, describe, search, traverse, assemble, atoms, generate, introspect, batch, cache_stats, cache_invalidate
- **1279 tests passing** (39 MCP + 1240 CLI)
- **Type-check clean** (all packages)
- **Lint clean** (0 errors, 0 clippy warnings)

## [0.14.1] - 2026-03-02

### Fixed
- **Zod 4 Migration** - Fixed `z.record()` API breaking changes in `shared.schema.ts`
  - Zod 4 requires explicit key schema: `z.record(z.string(), valueSchema)`
  - Updated 14 instances across LocaleVoice, LocaleCulture, Adaptation schemas
- **dagre Compatibility** - Reverted `@dagrejs/dagre` from 2.0.4 to 1.1.8
  - dagre 2.0 breaking API changes caused test failures in `schemaLayoutELK.test.ts`
  - Migration to dagre 2.0 documented as future work in `docs/plans/`

### Changed
- **Batch Dependency Updates** - Updated 13 dependencies via PR #55
  - TypeScript ecosystem: `@types/node`, `@types/react`, `eslint-config-next`, `next`
  - React ecosystem: `@testing-library/react`, `lucide-react`
  - Runtime: `framer-motion`, `neo4j-driver`, `sonner`, `zustand`
  - Testing: `@playwright/test`, `@types/jest`, `ts-jest`
- **Zod 4** - Upgraded from Zod 3.24 to 4.3 with breaking API migration

### Statistics
- **610 tests passing** (Studio + Core)
- **Type-check clean** (3/3 packages)
- **Lint clean** (0 errors)

## [0.14.0] - 2026-02-19

### Added
- **MCP Tool: novanet_introspect** - 8th MCP tool for schema introspection
  - Query NodeClasses filtered by realm/layer
  - Query ArcClasses filtered by family
  - Get specific class/arc details with relationships
  - Schema: IntrospectParams, IntrospectResult with JsonSchema derives
- **MCP: context_build_log** - New debugging feature in `novanet_generate`
  - 5 phases logged: structure_phase, entities_phase, atoms_phase, anchors_phase, token_decisions
  - Helps debug and understand how context is assembled for LLM generation
  - Schema: `ContextBuildLog` struct with `JsonSchema` derive for MCP clients

### Changed
- MCP tool count: 7 → 8 (added novanet_introspect)
- NovaNet MCP version: 0.4.0 → 0.5.0
- Documentation updated with 8 tools references

### Statistics
- **8 MCP tools**: query, describe, search, traverse, assemble, atoms, generate, introspect

## [0.13.1] - 2026-02-17

### Added
- **6th Arc Family: Schema** - Meta-schema relationships for graph structure
  - `OF_CLASS` - Connects instance nodes to their Class definitions
  - `FROM_CLASS` - ArcClass defines source node types
  - `TO_CLASS` - ArcClass defines target node types
  - Family properties: indigo color (#6366f1), dotted arrow style
  - Terminal palette mappings: 256-color (99), 16-color (4 blue)

### Changed
- Arc count: 169 → 182 (+13 new arcs: 3 schema meta-arcs + 10 existing knowledge atom arcs now counted)
- Arc family count: 5 → 6 (added schema family)
- All documentation updated to reflect 182 arcs and 6 arc families
- Test suite: 1082 tests passing (6 tests updated for new counts)

### Statistics
- **61 nodes** (40 shared + 21 org), **10 layers**, **5 traits**
- **182 arcs** (6 families: ownership, localization, semantic, generation, mining, schema)
- Schema validation: 0 errors, 0 warnings
- Database reset: 22,189+ nodes seeded successfully

## [0.13.0] - 2026-02-15

### Breaking Changes
- **ADR-029: *Native Pattern** - All locale-specific nodes use `*Native` suffix
  - `EntityContent` → `EntityNative` (trait: authored)
  - `ProjectContent` → `ProjectNative` (trait: authored)
  - `PageGenerated` → `PageNative` (trait: generated)
  - `BlockGenerated` → `BlockNative` (trait: generated)
- **ADR-029: Arc Merges** - Unified arc pattern for native content
  - `HAS_CONTENT` + `HAS_GENERATED` → `HAS_NATIVE`
  - `CONTENT_OF` + `GENERATED_FOR` → `NATIVE_OF`
  - Arc count: 171 → 169 (merged 4 arcs into 2)
- **ADR-030: Slug Ownership** - Slugs moved from Entity to Page

### Statistics
- **51 files changed** in commit
- **1030 Rust tests passing**
- **61 nodes** (40 shared + 21 org)
- **169 arcs** (was 171, merged 4 → 2)
