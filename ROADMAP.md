# NovaNet Roadmap

Current version: **v0.15.1** | Last release: v0.15.1 | Target: QR Code AI (https://qrcode-ai.com)

> **Versioning Note**: Starting with v0.12.0, NovaNet follows strict SemVer. Versions v8.x-v11.x were internal milestones.

## Milestones Overview

```
═══════════════════════════════════════════════════════════════════════════════
COMPLETED
═══════════════════════════════════════════════════════════════════════════════
v9.0  Self-Describing Context Graph    ████████████████████  100%  RELEASED
v9.1  TUI v2 Stability                 ████████████████████  100%  RELEASED
v9.5  Nomenclature (Arc, Realm/Layer)  ████████████████████  100%  RELEASED
v9.6  Generation Domain Nodes          ████████████████████  100%  RELEASED
v9.7  Intent Layer + Thing Model       ████████████████████  100%  RELEASED
v9.8  Polish + Cleanup                 ████████████████████  100%  RELEASED
v9.9  Tiered Knowledge Model           ████████████████████  100%  RELEASED
v10.4 Entity-Centric Architecture      ████████████████████  100%  RELEASED
v10.5 3-Realm Architecture             ████████████████████  100%  RELEASED
v10.6 2-Realm Architecture             ████████████████████  100%  RELEASED
      2 Realms (GLOBAL/TENANT), 9 Layers, simplified tenant isolation
v10.7 Geographic Taxonomy              ████████████████████  100%  RELEASED
      Population clusters, economic regions, cultural realms
v10.8 Icons Source of Truth            ████████████████████  100%  RELEASED
      visual-encoding.yaml icons (web+terminal), ADR-013
v10.9 Naming Convention Refactor       ████████████████████  100%  RELEASED
      ADR-014 (L10n→Content/Generated), ADR-015 (unidirectional arcs), ADR-016 (CONTAINS→6 typed)
v11.0 SEO Tenant + Docs Refactor       ████████████████████  100%  RELEASED
      SEO→tenant (ADR-012 fix), arc coherence, 30-sniper doc audit
v11.5 Schema Refinement                ████████████████████  100%  RELEASED
      Locale→config, SEO/GEO→shared/knowledge (ADR-020), 10 layers
v11.6 Query-First Architecture         ████████████████████  100%  RELEASED
      Tabbed detail panel, CypherPill, ADR-021 Query-First
v11.7 Unified Tree Architecture        ████████████████████  100%  RELEASED
      5 modes→2 (Graph/Nexus), ADR-022, dual icons, lazy loading
v11.8 Class Act (legacy)               ████████████████████  100%  RELEASED
      Trait renames (ADR-024): defined/authored/imported/generated/retrieved
v0.12 SemVer Transition                ████████████████████  100%  RELEASED
      Proper semantic versioning (0.x = pre-production)
v0.13 *Native Pattern                  ████████████████████  100%  RELEASED
      ADR-029 unified *Native suffix, ADR-030 Slug Ownership
v0.14 MCP Introspect                   ████████████████████  100%  RELEASED
      8 MCP tools, novanet_introspect, context_build_log (ADR-033)
v0.14.1 Maintenance                    ████████████████████  100%  RELEASED
      Zod 4 migration, dagre revert, dependency batch update
v0.15 MCP + CLI Improvements           ████████████████████  100%  RELEASED
      11 MCP tools (+batch, cache_stats, cache_invalidate), error hints
      CLI: export, stats, diff commands, 1279 tests
v0.15.1 Version Alignment              ████████████████████  100%  <- CURRENT
      All packages aligned to 0.15.1 (TS + Rust CLI + MCP)
      MCP versioning strategy: follows NovaNet version

═══════════════════════════════════════════════════════════════════════════════
IDEAS (no timeline)
═══════════════════════════════════════════════════════════════════════════════
• Dynamic Retrieval — Context Assembly Engine, token-aware traversal
• Autonomous Learning — Feedback loops, meta-graph self-improvement
• Content Pipeline — CLI-driven generation, orchestrator integration
• See: docs/plans/future/ for detailed specs
```

## v9.0.0 — Self-Describing Context Graph

Refactor from flat tree (Scope > Subcategory > NodeTypeMeta) to faceted classification
with 6 meta-node types, dual navigation, and Rust-first tooling.

| Phase | Description | Status | Key Deliverables |
|-------|-------------|--------|------------------|
| 0 | Preparation | DONE | `v8.3.0-stable` tag, feature branch |
| 1 | YAML Foundation | DONE | 44 node YAMLs with `locale_behavior`, relations.yaml v9 |
| 2 | Rust Generators | DONE | 8 generators, 4 parsers, 279 tests |
| 3 | TypeScript Types | DONE | KIND_META, NodeCategory removed, old TS deleted |
| 4 | Neo4j Migration | DONE | v9 seeds, constraints, 8 facet indexes |
| 5 | Studio Migration | DONE | Realm/Layer components, visual system, nodeColors.ts |
| 6 | Studio Navigation | DONE | 4-mode navigation, FacetFilterPanel, ViewPicker context-aware, T/E presets |
| 7A | Rust CLI + TUI | DONE | All commands, basic TUI with taxonomy tree |
| 8 | Final Verification | DONE | Audit, 955 tests, lint, type-check, benchmarks, code review |

### Remaining for v9.0.0 release

- [x] Migrate `categoryColors` imports to `design/nodeColors.ts` (Phase 5)
- [x] Verify ViewPicker context-awareness (Phase 6) — 13 views with `modes` field, filtering works
- [x] Verify keyboard presets T/E (Phase 6) — implemented + in help modal
- [x] Commit TUI work in progress (Phase 7B Batches 1-7 complete)
- [x] Run `/codebase-audit` — Ralph Wiggum #8 (Phase 8) — 6 findings fixed
- [x] `pnpm test` — 559 tests pass, 34 suites (Phase 8)
- [x] `pnpm type-check` — zero errors, 3/3 packages (Phase 8)
- [x] `pnpm lint` — zero errors (Phase 8)
- [x] `cargo test` — 396 tests pass, zero clippy warnings (Phase 8)
- [x] Performance benchmarks — data 1056ms, meta 159ms, overlay 646ms, query 125ms (Phase 8)
- [x] Code review — PRODUCTION READY, 0 critical/high issues (Phase 8)
- [x] Create GitHub Release v9.0.0 (Phase 8)

## v9.1.0 — TUI v2 Stability (shipped in v9.0.1)

Complete TUI rebuild focusing on stability and simplicity.

| Task | Description | Status |
|------|-------------|--------|
| Rebuild | Consolidate 12 modules → 3 (app, data, ui) | DONE |
| Tree nav | Realm > Layer > Kind hierarchy | DONE |
| Detail | YAML preview + edge explorer | DONE |
| Modes | 4 navigation modes [1-4] | DONE |

**Stats:** -7,600 lines deleted, +1,200 lines added. 179 tests pass.

## v9.5.0 — Advanced TUI (Galaxy Theme)

**STATUS: DEFERRED** — Galaxy features removed during v9.1 stability rebuild.
Will be reimplemented on stable v2 foundation when prioritized.

| Task | Description | Status |
|------|-------------|--------|
| 7.8d | Search + detail pane | DEFERRED |
| 7.8e | CRUD dialogs | DEFERRED |
| 7.8f | Galaxy visual theme | DEFERRED |
| 7.8g | Dashboard mode | DEFERRED |
| 7.8h | Animations (boot, matrix rain) | DEFERRED |
| 7.8i | ASCII logo + branding | DEFERRED |
| 7.8j | Onboarding flow | DEFERRED |
| 7.8k | Command palette + UX | DEFERRED |
| 7.8l | Wow effects (CRT, shake, glitch, pulse) | DEFERRED |

## v0.12.0 — SemVer Transition ← CURRENT

Adopted proper semantic versioning. v0.x.y indicates pre-production status.

| Change | Description |
|--------|-------------|
| **SemVer adoption** | 0.x.y versioning indicates pre-1.0 status |
| **ADR-024 traits** | defined/authored/imported/generated/retrieved |
| **Unified packages** | All packages now at 0.12.0 |

---

## v11.8.0 — Class Act (Legacy)

Trait rename migration per ADR-024 "Data Origin" for clearer semantics.

| Old Trait | New Trait | Description |
|-----------|-----------|-------------|
| `invariant` | `defined` | Structurally fixed, version-controlled definitions |
| `localized` | `authored` | Human-authored locale-specific content |
| `knowledge` | `imported` | External data imported from authoritative sources |
| `generated` | `generated` | LLM-generated output (unchanged) |
| `aggregated` | `retrieved` | Computed/aggregated from external APIs |

**Files updated:**
- 60+ YAML node definitions (realm: shared/org)
- Rust TUI (traits.rs, theme.rs)
- TypeScript Studio (15+ components, design system, tests)
- ADR-024 added to novanet-decisions.md

**Schema counts:** 61 NodeClasses, 182 ArcClasses, 2 Realms, 10 Layers (4 shared + 6 org), 6 ArcFamilies

---

## v11.0.0 — SEO Tenant + Docs Refactor

Major architectural cleanup with 30-sniper comprehensive audit.

| Change | Description |
|--------|-------------|
| **SEO → Tenant** | Moved 9 SEO/GEO nodes from `global/seo` to `tenant/seo` |
| **ADR-012 Fix** | Eliminated all global→tenant arc violations |
| **Arc Coherence** | Fixed 22 arc scopes (cross_realm → intra_realm) |
| **Docs Refactor** | 30-sniper audit of CLAUDE.md, DX, skills, agents, roadmap |

**Schema counts:** 64 NodeClasses, 123 ArcClasses, 2 Realms, 9 Layers (2 global + 7 tenant)

---

## v10.9.0 — Naming Convention Refactor

Semantic renaming for clearer architecture. Three ADRs implemented:

| ADR | Change | Details |
|-----|--------|---------|
| **ADR-014** | L10n → Content/Generated | `EntityL10n` → `EntityContent`, `PageL10n` → `PageGenerated`, `BlockL10n` → `BlockGenerated` |
| **ADR-015** | Unidirectional ownership | All `IS_*` arcs reversed to `HAS_*` (parent → child) |
| **ADR-016** | CONTAINS → 6 typed arcs | `CONTAINS_TERM`, `CONTAINS_EXPRESSION`, `CONTAINS_PATTERN`, `CONTAINS_CULTURE_REF`, `CONTAINS_TABOO`, `CONTAINS_AUDIENCE_TRAIT` |

**Schema counts:** 64 NodeClasses, 123 ArcClasses, 2 Realms (global/tenant), 9 Layers

---

## v9.9.0 — Tiered Knowledge Model

Refactor locale knowledge from 14 flat nodes to 10 tiered nodes organized by retrieval purpose.

| Tier | Nodes | Purpose |
|------|-------|---------|
| **Technical** | Formatting, Slugification, Adaptation | Deterministic rules (always needed) |
| **Style** | Style | Voice/tone configuration |
| **Semantic** | TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet | Domain-specific contextual content |

---

## Ideas Backlog

Future ideas without timeline. Detailed specs in `docs/plans/future/`.

| Idea | Description |
|------|-------------|
| **Dynamic Retrieval** | Context Assembly Engine that reads meta-graph to assemble token-aware context windows autonomously |
| **Autonomous Learning** | Feedback loops where quality scores improve meta-graph weights over time |
| **Content Pipeline** | Full CLI-driven generation: `novanet generate --project=qrcode-ai --locale=fr-FR` |
| **GEO Intelligence** | Geographic localization layer: Thing (invariant), ThingL10n, GEOSeed, GEOSeedMetrics, GEOMiningRun. Deferred from v10.1 — add when needed for location-based content. |

## Versioning Strategy

### Semantic Versioning (SemVer)

```
MAJOR.MINOR.PATCH[-prerelease]

MAJOR  = Breaking changes (ontology restructure, API changes)
MINOR  = New features (commands, UI components, generators)
PATCH  = Bug fixes, documentation, refactoring
```

### Pre-release tags

```
v9.0.0-rc.1    Release candidate (feature-complete, needs verification)
v9.0.0-rc.2    Second release candidate (after fixes)
v9.0.0         Stable release (Phase 8 passed)
```

### Release process

1. Ensure all tests, lint, type-check pass
2. Update `CHANGELOG.md` with release notes
3. Create annotated git tag: `git tag -a v<version> -m "v<version>"`
4. Push tag: `git push origin v<version>`
5. **GitHub Release is created automatically** via `.github/workflows/release.yml`
6. Update this ROADMAP.md

### Branch strategy

```
main            Stable releases only
feat/*          Feature branches (merged via PR)
fix/*           Bug fix branches
release/v*      Release preparation (optional, for complex releases)
```

## History

| Version | Date | Highlights |
|---------|------|------------|
| v8.0.0 | 2026-01-30 | Turborepo monorepo |
| v8.1.0 | 2026-01-30 | Security hardening, generators |
| v8.2.0 | 2026-01-30 | YAML views, deprecated props removed |
| v8.3.0 | 2026-02-01 | Meta-graph, magnetic layout, v9 design |
| v9.0.0-rc.1 | 2026-02-02 | Ontology v9, Rust CLI, 4-mode navigation |
| v9.0.0 | 2026-02-02 | Phase 8 complete: 955 tests, audit clean, benchmarked |
| v9.0.1 | 2026-02-03 | TUI v2 rebuild, DX improvements, auto-release workflow |
| v9.7.1 | 2026-02-04 | Intent layer, generation domain, Thing model |
| v10.6.0 | 2026-02-05 | 2-Realm architecture (ADR-012), tenant isolation |
| v10.7.0 | 2026-02-06 | Geographic taxonomy, population clusters |
| v10.8.0 | 2026-02-07 | Icons source of truth (ADR-013), visual-encoding.yaml |
| v10.9.0 | 2026-02-08 | Naming refactor (ADR-014/015/016), 64 Classes, 120 ArcClasses |
| v11.0.0 | 2026-02-08 | SEO → tenant migration, arc coherence (22 fixes), docs refactor |
| v11.5.0 | 2026-02-10 | Schema refinement (ADR-020), Locale→config, 10 layers |
| v11.6.0 | 2026-02-10 | Query-First Architecture (ADR-021), tabbed detail panel |
| v11.7.0 | 2026-02-11 | Unified Tree Architecture (ADR-022), 5→2 modes, dual icons |
| v11.8.0 | 2026-02-13 | Class Act (ADR-024), trait renames for data origin semantics |
| v0.12.0 | 2026-02-13 | SemVer Transition — proper versioning (0.x = pre-production) |
| v0.12.5 | 2026-02-14 | Brand Architecture (ADR-028), 61 nodes, 169 arcs |
| v0.13.0 | 2026-02-15 | *Native Pattern (ADR-029/030): EntityNative, ProjectNative, PageNative, BlockNative, HAS_NATIVE |
| v0.13.1 | 2026-02-17 | 6th Arc Family: Schema (OF_CLASS, FROM_CLASS, TO_CLASS), 182 arcs |
| v0.14.0 | 2026-02-19 | MCP Introspect, context_build_log (ADR-033), 8 MCP tools |
| v0.14.1 | 2026-03-02 | Zod 4 migration, dagre revert, batch dependency updates |
| v0.15.0 | 2026-03-02 | MCP batch/cache tools, CLI export/stats/diff, 11 MCP tools, 1279 tests |
| v0.15.1 | 2026-03-03 | Version alignment — all packages at 0.15.1, MCP versioning strategy |
