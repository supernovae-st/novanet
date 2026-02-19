# Changelog (Latest)

For complete history, see [CHANGELOG.md](./CHANGELOG.md).

## [Unreleased]

## [0.14.0] - 2026-02-19

### Added
- **MCP: context_build_log** - New debugging feature in `novanet_generate` showing step-by-step context assembly
  - 5 phases logged: structure_phase, entities_phase, atoms_phase, anchors_phase, token_decisions
  - Helps debug and understand how context is assembled for LLM generation
  - Schema: `ContextBuildLog` struct with `JsonSchema` derive for MCP clients

### Changed
- NovaNet MCP version: 0.3.0 → 0.4.0
- NovaNet CLI version: 0.13.0 → 0.14.0

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
