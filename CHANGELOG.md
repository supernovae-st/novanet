# Changelog

All notable changes to NovaNet are documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [11.7.0] - 2026-02-11

### Added
- **Unified Tree Architecture**: Merge 5 navigation modes into 2 (Graph/Nexus)
  - All nodes clickable: Realm, Layer, Kind, Instance, ArcFamily, ArcKind
  - Principle: "Node in Neo4j = Node everywhere"
  - **ADR-022**: Unified Tree Architecture approved
- **Neo4j Schema Migration**: New structural relationships
  - `HAS_LAYER`: Realm → Layer (10 relationships)
  - `HAS_KIND`: Layer → Kind (60 relationships)
  - `BELONGS_TO_FAMILY`: ArcKind → ArcFamily (114 relationships)
  - Performance indexes for tree navigation
- **Dual Icon System**: Consistent iconography across TUI and Studio
  - `visual-encoding.yaml`: Added `meta_types` icons (realm, layer, kind, instance, arc_family, arc_kind)
  - `_registry.yaml`: All 29 views converted to dual format (web + terminal)
  - Web: Lucide icons, Terminal: Unicode symbols
  - No emoji in codebase
- **Rust Type Definitions**: `unified_types.rs` in TUI module
  - `NodeId` enum, `UnifiedNode` struct, `LazyChildren` enum
  - `AsyncCommand`/`AsyncEvent` for tokio channels
  - Badge presets, pagination constants
- **TypeScript Type Definitions**: `unified-tree.ts` in core package
  - `DualIcon`, `UnifiedNode` discriminated union
  - Type guards: `isRealmNode`, `isLayerNode`, etc.
  - `TreeState`/`TreeActions` interfaces
- **Studio treeStore**: Zustand store for unified tree
  - Lazy loading via `/api/tree/:id/children`
  - Expand/collapse, selection, focus state
  - Pagination support with `loadMoreChildren`
- **UnifiedTreeNode component**: React component for tree rendering
  - Dynamic Lucide icon loading
  - Loading states, badge rendering
  - Recursive children rendering

### Changed
- **TUI NavMode**: 5 modes → 2 modes
  - `[1]Graph`: Unified tree (replaces Meta/Data/Overlay)
  - `[2]Nexus`: Quiz/Audit/Stats/Help hub (replaces Atlas)
  - Search via `[/]` overlay (no separate Query mode)
- **View parser**: Supports both legacy string icons and new dual icon format
- **View generator**: Outputs `ViewIcon` interface with web/terminal fields
- **Version bump**: All files updated from 11.6.0 to 11.7.0

### Migration
```bash
# Run Neo4j migration for structural relationships
cargo run -- db migrate
# Or manually execute:
# packages/db/seed/migrations/v11.7-unified-tree.cypher

# Regenerate schema artifacts
cargo run -- schema generate
```

### Statistics
- **1008 Rust tests passing** (4 new for NavMode)
- **TypeScript type-check passing**
- **12 artifacts regenerated** from YAML sources

## [11.6.0] - 2026-02-10

### Added
- **Tabbed Detail Panel**: New 4-tab interface for node details in Studio
  - **Overview Tab**: Type badge, classification grid (realm/layer/trait), description, timestamps
  - **Data Tab**: Stats bar (in/out arcs, props count), collapsible property sections, coverage progress bar
  - **Graph Tab**: View switcher (Ego/Arcs/Flow/Context), Mermaid diagram placeholder, relations list with navigation
  - **Code Tab**: Format switcher (JSON/YAML/Cypher/TypeScript) with syntax-colored preview and copy
- **CypherPill component**: Displays current Cypher query at top of graph canvas
- **MatrixRain component**: Visual effect during Neo4j queries
- **useNodeSelection hook**: Syncs React Flow selection with uiStore
- **useMatrixRain hook**: Controls matrix rain effect timing
- **Magic UI components**: border-beam, particles, shine-border for visual effects
- **Motion animations**: Spring-based tab transitions and indicator animations

### Changed
- Replaced `NodeDetailsPanel` with `TabbedDetailPanel` in page.tsx
- Added `DetailPanelTab` type to uiStore for tab state persistence

## [11.5.0] - 2026-02-10

### Breaking Changes
- **Locale moved**: Locale from shared/locale to shared/config (definitions layer pattern)
  - Locale is a DEFINITION (invariant trait), not a parameter/setting
  - Follows EntityCategory pattern: definitions go in config layer
- **SEO/GEO consolidation**: SEO/GEO nodes moved from org realm to shared/knowledge
  - SEO keywords are universal across organizations (not org-specific)
  - org/seo and org/geo layers removed

### Changed
- **Node count**: 61 → 60 nodes (consolidated)
- **Layer count**: 11 → 10 layers (4 shared + 6 org)
- **Node distribution**: 39 shared + 21 org (was 32 + 29)
- **ADR-020**: Schema Refinement documentation added

### Fixed
- Version mismatch across CLAUDE.md files (synchronized to v11.5.0)
- VERSION file created as single source of truth
- Node counts corrected in all documentation
- Arc count documentation: 116 → 114 (matches YAML reality)
- ADR version header: v11.3 → v11.5
- TUI layer mappings: Added locale 🌍, geography 🗺 for v11.3 split
- Schema stats example: 64/120 → 60/114 Kinds/ArcKinds

## [11.3.0] - 2026-02-10

### Breaking Changes
- **Layer split**: `locale-knowledge` → `locale`, `geography`, `knowledge` (3 shared layers)
  - `locale` (7 nodes): Locale, Culture, Adaptation, Market, Style, Slugification, Formatting
  - `geography` (6 nodes): Continent, GeoRegion, GeoSubRegion, EconomicRegion, IncomeGroup, LendingCategory
  - `knowledge` (19 nodes): TermSet, Term, ExpressionSet, Expression, PatternSet, Pattern, CultureSet, CultureRef, TabooSet, Taboo, AudienceSet, AudienceTrait, EntityCategory, CategorySet, CulturalRealm, CulturalSubRealm, LanguageFamily, LanguageBranch, PopulationCluster, PopulationSubCluster
- **New layer**: `geo` added to org realm for GEO intelligence
  - GEOQuery, GEOAnswer, GEOMetrics moved from seo to geo layer
- **Node merge**: Organization + Tenant → OrgConfig
  - Single org config entry point replaces redundant nodes

### Added
- **ADR-019**: Layer Reorganization documentation
- **11 layers**: 3 shared (locale, geography, knowledge) + 8 org (config, foundation, structure, semantic, instruction, seo, geo, output)

### Changed
- **Node count**: 62 → 61 nodes (merged Organization + Tenant)
- **Layer count**: 9 → 11 layers (split locale-knowledge, added geo)
- **YAML structure**: 32 shared realm files reorganized into new layer directories
- **Rust tests**: 950 tests passing
- **TypeScript tests**: 478 tests passing

### Fixed
- Studio type definitions updated for 11 layers
- FacetFilterPanel updated for new layer structure
- NodeConfig updated for OrgConfig (removed Organization, Tenant)
- ResultsOverview layer config updated
- hierarchical layout updated for new realm/layer structure

## [11.2.0] - 2026-02-10

### Breaking Changes
- **Realm renames**: `global` → `shared`, `tenant` → `org`
  - Clearer naming: `shared` describes purpose (shared resources), `org` describes ownership
  - Updated 65 YAML files, 40+ Rust files, 8+ TypeScript files
- **Trait split**: `derived` → `generated` + `aggregated`
  - `generated` (4 nodes): PageGenerated, BlockGenerated, OutputArtifact, PromptArtifact
  - `aggregated` (3 nodes): GeoAnswer, GeoMetrics, SEOKeywordMetrics
- **Job removal**: Removed `job` trait and 3 nodes
  - Deleted: GenerationJob, SEOMiningRun, EvaluationSignal
  - Deleted: 7 arc kinds referencing job nodes

### Added
- **Visibility derivation**: `internal` | `fragment` | `publishable` derived from (realm, layer, kind)
- **Status property**: `draft` | `reviewed` | `published` on EntityContent, ProjectContent
- **ADR-018**: Classification System Refinement documentation

### Changed
- **Node count**: 65 → 62 nodes (removed 3 job nodes)
- **Trait count**: 6 → 5 traits (removed job, split derived)
- **Container traits**: TermSet, ExpressionSet, etc. now `invariant` (was `knowledge`)
- **TUI Icons**: Updated defaults for generated/aggregated traits

### Fixed
- KEYBINDINGS.md deprecated trait references (gd→gg, gj removed, ga added)
- Documentation version references updated to v11.2.0

## [11.0.0] - 2026-02-09

### Breaking Changes
- **Type renames** (v10.9 naming refactor):
  - `ProjectL10n` → `ProjectContent`
  - `EntityL10n` → `EntityContent`
  - Updated all TypeScript files, tests, and documentation
- **SEO realm migration** (ADR-012):
  - Moved 9 SEO/GEO nodes from `global/seo` to `tenant/seo`
  - SEO is now tenant-scoped per YAML source of truth
  - Fixed 22 arc scopes (cross_realm → intra_realm)

### Added
- **TUI test infrastructure**: 929 tests (was 245)
  - Snapshot testing with `insta`
  - Property-based testing with `proptest`
  - Render tests for all UI modules (tree, yaml_panel, atlas, graph, overlays)
  - `TaxonomyTree::mock_for_testing()` for unit tests
- **SEO seed data**: 26 French SEO questions from ATP export
- **Knowledge Atoms**: 18 locale-knowledge nodes (6 Sets + 6 Atoms + 6 Linguistic)

### Changed
- **Architecture**: 2 realms (global: 31 nodes, tenant: 33 nodes), 9 layers total
  - Global: config (13), locale-knowledge (18)
  - Tenant: config (2), foundation (3), structure (3), semantic (4), seo (9), instruction (7), output (5)
- **Node count**: 46 → 64 nodes
- **Test count**: 1,632 tests (142 core + 561 studio + 929 Rust)

### Fixed
- EntityContent seed descriptions (9 corrections)
- LLM context corruptions and typos in seed data
- YAML consistency + orphan node cleanup

### Removed
- Legacy code and backward compatibility shims
- 3,622 low-volume SEO keywords (volume < 20)

## [9.7.0] - 2026-02-04

### Added
- **Intent Layer nodes** (project/semantic):
  - `AudiencePersona` — Target audience definition with demographics, behaviors, goals
  - `ChannelSurface` — Publication channel with format, constraints, tone requirements
  - `ContentSlot` — Placeholder for generated content within blocks
  - `SearchIntent` — User search intent classification for SEO targeting
  - `TopicCluster` — Topic hierarchy for content organization

- **Generation Domain nodes** (project/output):
  - `GenerationJob` — Async generation task with status tracking
  - `PromptArtifact` — Versioned prompt with parameters and metadata
  - `OutputArtifact` — Generated content artifact with quality scores
  - `EvaluationSignal` — Quality evaluation feedback signal

- **Unified Thing Model** (shared/geo):
  - `Thing` — Schema.org-aligned entity with Wikidata linking
  - `ThingL10n` — Localized thing representation

- **Semantic arcs**:
  - `MENTIONS` — Page/Block mentions Thing
  - `COVERS` — Concept covers Thing
  - `SPECIALIZES` — Thing specializes another Thing
  - `TARGETS_THING` — SEO keyword targets Thing
  - `SATISFIES_INTENT` — Content satisfies SearchIntent
  - `SUBTOPIC_OF` — TopicCluster hierarchy
  - `CLUSTERS_TOPIC` — Page clusters under TopicCluster

- **Visual encoding system** (v9.5.0):
  - `taxonomy.yaml` — Unified taxonomy with colors, borders, terminal palette
  - `visual-encoding.yaml` — Presentation rules for nodes and arcs
  - Icons for all 46 node types

### Changed
- **Schema count**: 44 → 46 NodeKinds, 50 → 76 ArcKinds
- **Studio**: Updated NodeConfig, nodeTypes.ts for new nodes
- **Tests**: Updated expected counts (44 → 46 nodes, 21 → 23 project realm)

## [9.0.1] - 2026-02-03

### Added
- **Auto-release workflow** (`.github/workflows/release.yml`): Push a tag, get a release
- **Dependabot** (`.github/dependabot.yml`): Automated dependency updates for npm, Cargo, Actions, Docker
- **CodeQL security scanning** (`.github/workflows/codeql.yml`): Weekly + on PR
- **CONTRIBUTING.md**: Setup guide, workflow, code style conventions
- **PR template** + **Issue templates** (bug report, feature request)
- **LICENSE**: Proprietary license file
- **SECURITY.md**: Security vulnerability reporting policy

### Changed
- **TUI v2 rebuild**: Complete rewrite for stability (-7,600 lines, +1,200 lines)
  - Consolidated 12 modules → 3 (app, data, ui)
  - Simplified architecture: tree navigation + detail panel + status bar
  - Galaxy theme features deferred to v9.5.0
- **Studio v9.0.0**: Version bump from 0.1.0 to match core
- **NavigationModeToggle**: Complete UI redesign
- **MatrixExplosionOverlay**: Enhanced animation and performance
- **README.md**: Added Rust CLI section, ASCII mockup, fixed badges

### Fixed
- CI badge URL (novanet-hq → novanet-dev)
- CLAUDE.md file structures (removed deleted directories)
- NodeCategory → Layer terminology in Studio docs

### Removed
- Premature tags (v9.5.0, v10.0.0) — work not complete
- Empty `docs/src/` folder
- Stale `.worktrees/yaml-views/`
- Standalone `EasterEggButton.tsx` (integrated elsewhere)

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
- **Rust CLI binary** (`tools/novanet/`): 13 commands, 12 generators, 4 parsers, 950 tests
  - `novanet schema generate` — orchestrates 12 generators (YAML → Cypher/TS/Mermaid/Rust)
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
- **3 new meta-node types**: Trait (5 values), ArcFamily (5 families), ArcKind (77 relationships)
- **OF_KIND instance bridge**: Replaces IN_SUBCATEGORY
- **`:Meta` double-label**: All meta-nodes discoverable via `MATCH (n:Meta)`
- **Faceted classification**: Realm × Layer × Trait × ArcKind (4 axes)
- **Studio 4-mode navigation**: Data, Meta, Overlay, Query with FacetFilterPanel
- **NavigationModeToggle** toolbar component
- **FacetFilterPanel** sidebar component (Realm/Layer/Trait/ArcFamily checkboxes)
- **novanetBridge.ts**: Subprocess wrapper for Rust CLI integration
- **Visual system**: `design/layerColors.ts` (9 colors), `design/traitStyles.ts` (5 border styles)
- **Auto-generated files**: `hierarchy.ts`, `layers.ts` (Rust → MiniJinja → TypeScript)
- **8 Neo4j facet property indexes** for Kind, ArcKind, L10n quality/fingerprint

### Changed
- Migrated `config/categoryColors.ts` to `design/nodeColors.ts` — all 5 imports updated
- Studio components renamed: ScopeGroupNode → RealmGroupNode, SubcategoryGroupNode → LayerGroupNode
- Studio stores: `collapsedScopes` → `collapsedRealms`, `collapsedSubcategories` → `collapsedLayers`
- ViewCategory: `'scope'` → `'overview'`
- relations.yaml: dict → list format with `family`, `source`/`target`, `cardinality`
- organizing-principles.yaml: v9.0.0 with realms/layers/traits/edge_families
- 44 node YAMLs: added `locale_behavior`, removed `category`
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

[Unreleased]: https://github.com/supernovae-st/novanet-dev/compare/v11.3.0...HEAD
[11.3.0]: https://github.com/supernovae-st/novanet-dev/compare/v11.2.0...v11.3.0
[11.2.0]: https://github.com/supernovae-st/novanet-dev/compare/v11.0.0...v11.2.0
[11.0.0]: https://github.com/supernovae-st/novanet-dev/compare/v9.7.0...v11.0.0
[9.7.0]: https://github.com/supernovae-st/novanet-dev/compare/v9.0.1...v9.7.0
[9.0.1]: https://github.com/supernovae-st/novanet-dev/compare/v9.0.0...v9.0.1
[9.0.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.3.0...v9.0.0
[8.3.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.2.0...v8.3.0
[8.2.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.1.0...v8.2.0
[8.1.0]: https://github.com/supernovae-st/novanet-dev/compare/v8.0.0...v8.1.0
[8.0.0]: https://github.com/supernovae-st/novanet-dev/releases/tag/v8.0.0
