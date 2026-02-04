# NovaNet Roadmap

Current version: **v9.9.0** | Target: QR Code AI (https://qrcode-ai.com)

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
v9.9  Tiered Knowledge Model           ████████████████████  100%  ← CURRENT
      42 NodeKinds, 73 ArcKinds

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

## v9.9.0 — Tiered Knowledge Model ← CURRENT

Refactor locale knowledge from 14 flat nodes to 10 tiered nodes organized by retrieval purpose.

| Tier | Nodes | Purpose |
|------|-------|---------|
| **Technical** | Formatting, Slugification, Adaptation | Deterministic rules (always needed) |
| **Style** | Style | Voice/tone configuration |
| **Semantic** | TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet | Domain-specific contextual content |

**Schema counts:** 42 NodeKinds, 73 ArcKinds, 3 Realms, 9 Layers

---

## Ideas Backlog

Future ideas without timeline. Detailed specs in `docs/plans/future/`.

| Idea | Description |
|------|-------------|
| **Dynamic Retrieval** | Context Assembly Engine that reads meta-graph to assemble token-aware context windows autonomously |
| **Autonomous Learning** | Feedback loops where quality scores improve meta-graph weights over time |
| **Content Pipeline** | Full CLI-driven generation: `novanet generate --project=qrcode-ai --locale=fr-FR` |

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
