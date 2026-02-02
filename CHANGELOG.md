# Changelog

All notable changes to NovaNet are documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [9.0.0] - 2026-02-02

### Breaking Changes
- **Ontology v9**: Scope → Realm, Subcategory → Layer, NodeTypeMeta → Kind
- **NodeCategory removed**: Use Layer directly for filtering
- **PascalCase → lowercase**: `'Global'` → `'global'`, `'localeKnowledge'` → `'knowledge'`
- **DataMode → NavigationMode**: 2 modes (data/schema) → 4 modes (data/meta/overlay/query)
- **@novanet/schema-tools deleted**: Absorbed into Rust binary
- **@novanet/cli deleted**: Absorbed into Rust binary
- **core/scripts/, core/src/parsers/, core/src/services/, core/src/db/ deleted**: Absorbed into Rust

### Added
- **Rust CLI binary** (`tools/novanet/`): 13 commands, 8 generators, 4 parsers, 396 tests
  - `novanet schema generate` — orchestrates 7 generators (YAML → Cypher/TS/Mermaid)
  - `novanet schema validate` — YAML coherence checks
  - `novanet data/meta/overlay/query` — 4 navigation modes
  - `novanet node create/edit/delete` — CRUD with meta-graph validation
  - `novanet relation create/delete` — relationship management
  - `novanet search --query` — fulltext + property search
  - `novanet db seed/reset` — database management
  - `novanet locale list` — locale operations
  - `novanet doc generate` — 12 view Mermaid diagrams
  - `novanet filter build` — JSON filter → Cypher (for Studio subprocess)
  - `novanet tui` — interactive terminal UI with taxonomy tree
- **TUI** (Phase 7A): Galaxy-themed terminal UI
  - Taxonomy tree (Realm > Layer > Kind), mode toggle (1/2/3/4)
  - Async Neo4j queries via mpsc channel bridge
  - Deep space color palette, status bar, detail pane
- **Advanced TUI** (Phase 7B): Mission control cockpit
  - Search + detail pane (nucleo fuzzy, edge explorer)
  - CRUD dialogs (node create/edit/delete, relation CRUD)
  - Galaxy visual theme (deep space palette)
  - Dashboard mode (Neo4j stats, realm/family charts)
  - Boot animation (6-stage: matrix rain, logo reveal, fade)
  - Command palette (fuzzy search, 11 commands, 5 categories)
  - Onboarding flow (welcome screen, 5-step guided tour)
  - Wow effects (CRT scanlines, screen shake, glitch transitions, nebula pulse)
- **3 new meta-node types**: Trait (5 values), EdgeFamily (5 families), EdgeKind (50 relationships)
- **OF_KIND instance bridge**: Replaces IN_SUBCATEGORY
- **`:Meta` double-label**: All meta-nodes discoverable via `MATCH (n:Meta)`
- **Faceted classification**: Realm × Layer × Trait × EdgeKind (4 axes)
- **Studio 4-mode navigation**: Data, Meta, Overlay, Query with FacetFilterPanel
- **NavigationModeToggle** toolbar component
- **FacetFilterPanel** sidebar component (Realm/Layer/Trait/EdgeFamily checkboxes)
- **novanetBridge.ts**: Subprocess wrapper for Rust CLI integration
- **Visual system**: `design/layerColors.ts` (9 colors), `design/traitStyles.ts` (5 border styles)
- **Auto-generated files**: `hierarchy.ts`, `layers.ts` (Rust → MiniJinja → TypeScript)
- **8 Neo4j facet property indexes** for Kind, EdgeKind, L10n quality/fingerprint

### Changed
- Migrated `config/categoryColors.ts` to `design/nodeColors.ts` — all 5 imports updated
- Studio components renamed: ScopeGroupNode → RealmGroupNode, SubcategoryGroupNode → LayerGroupNode
- Studio stores: `collapsedScopes` → `collapsedRealms`, `collapsedSubcategories` → `collapsedLayers`
- ViewCategory: `'scope'` → `'overview'`
- relations.yaml: dict → list format with `family`, `source`/`target`, `cardinality`
- organizing-principles.yaml: v9.0.0 with realms/layers/traits/edge_families
- 35 node YAMLs: added `locale_behavior`, removed `category`
- All seed files regenerated for v9 meta-graph
- ESLint migrated to flat config

### Removed
- `packages/schema-tools/` — entire package (~7000 lines TypeScript)
- `packages/cli/` — empty stub package
- `core/scripts/` — seed.ts, validate.ts, import-locale.ts
- `core/src/parsers/` — 7 markdown parsers
- `core/src/services/` — graph-traversal.ts, hybrid-retriever.ts, vector-search.ts
- `core/src/db/client.ts` — Neo4j connection
- `NodeCategory` type — replaced by Layer
- `IN_SUBCATEGORY` relationship — replaced by OF_KIND
- v8 meta-labels: Scope, Subcategory, NodeTypeMeta
- Dead `schemaLayouts/magnetic.ts` module (220 lines)
- Dead `closeDriver()` and `verifyConnectionStrict()` from neo4j.ts
- `NODE_CATEGORIES` references from validate-types.ts (replaced by Layer)

## [8.3.0] - 2026-02-01

### Added
- **Meta-graph foundation**: organizing-principles.yaml as source of truth
- **OrganizingPrinciplesGenerator**: YAML → Neo4j Cypher seed generation
- **SubcategoryGenerator**: YAML → TypeScript sync
- **Schema mode**: ELK layout engine, SchemaFilterPanel, collapsed groups
- **Magnetic layout**: d3-force simulation, attractor nodes, taxonomy dashboard
- **AI search overlay** (Cmd+J) for Studio
- **Design token system**: spacing, icons, glass patterns, golden ratio
- **Accessibility pass**: ARIA labels, grid navigation, focus rings
- **IN_SUBCATEGORY** relationship + auto-wiring script
- **v9 design plan**: 16-phase roadmap (v9 → v12)
- **Claude DX**: /novanet-arch, /novanet-sync commands, codebase-audit skill

### Changed
- Studio FilterTree upgraded to A+ architecture
- Studio components refactored with shared design system
- Performance: `useShallow` on all store subscriptions, specific CSS transitions
- 11 Ralph Wiggum audit passes executed

### Removed
- Dead code: unused exports, legacy aliases, orphan components (-223 lines)
- Dead hooks, unused store methods, stale CSS
- Unused dependencies cleaned

## [8.2.0] - 2026-01-30

### Added
- **YAML views system**: ViewSelector UI, viewStore, useUrlSync, API routes
- **NODE_ICONS** config: presentation layer separated from data types
- **Zod validation** for YAML views with security hardening

### Changed
- Studio types aligned with core v8.2.0
- Deprecated properties removed from types, seeds, YAML examples

### Removed
- `icon`, `priority`, `freshness` properties (moved to presentation layer)
- `PageMetrics` node type
- Migration 001-006 applied to clean deprecated data

## [8.1.0] - 2026-01-30

### Added
- **Turborepo generators** and pnpm catalogs
- Security hardening: API routes, credentials, CI pipeline

### Changed
- ESLint downgraded to v8 for Next.js compatibility
- Models documentation updated to v8.1.0

### Fixed
- Jest environment variables for Neo4j tests
- Path references and aliases for monorepo structure

## [8.0.0] - 2026-01-30

### Added
- **Turborepo monorepo** structure with pnpm workspaces
- Packages: @novanet/core, @novanet/db, @novanet/studio
- Professional README with badges and workspace structure

### Changed
- Migrated from git submodules to true monorepo
- Organization links updated to supernovae-st

[Unreleased]: https://github.com/supernovae-st/novanet-dev/compare/v9.0.0...HEAD
[9.0.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.3.0...v9.0.0
[8.3.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.2.0...v8.3.0
[8.2.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.1.0...v8.2.0
[8.1.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.0.0...v8.1.0
[8.0.0]: https://github.com/supernovae-st/novanet-dev/releases/tag/v8.0.0
