# NovaNet Roadmap

Current version: **v9.0.0-rc.1** | Target: QR Code AI (https://qrcode-ai.com)

## Milestones Overview

```
v9.0  Self-Describing Context Graph    ████████████████████  ~97%  ← YOU ARE HERE
v9.5  Advanced TUI (Galaxy)            ██████████████████░░  ~90%
v10.0 Dynamic Retrieval                ░░░░░░░░░░░░░░░░░░░░    0%
v11.0 Autonomous Learning              ░░░░░░░░░░░░░░░░░░░░    0%
v12.0 Content Pipeline                 ░░░░░░░░░░░░░░░░░░░░    0%
```

## v9.0.0 — Self-Describing Context Graph

Refactor from flat tree (Scope > Subcategory > NodeTypeMeta) to faceted classification
with 6 meta-node types, dual navigation, and Rust-first tooling.

| Phase | Description | Status | Key Deliverables |
|-------|-------------|--------|------------------|
| 0 | Preparation | DONE | `v8.3.0-stable` tag, feature branch |
| 1 | YAML Foundation | DONE | 35 node YAMLs with `locale_behavior`, relations.yaml v9 |
| 2 | Rust Generators | DONE | 8 generators, 4 parsers, 279 tests |
| 3 | TypeScript Types | DONE | KIND_META, NodeCategory removed, old TS deleted |
| 4 | Neo4j Migration | DONE | v9 seeds, constraints, 8 facet indexes |
| 5 | Studio Migration | DONE | Realm/Layer components, visual system, nodeColors.ts |
| 6 | Studio Navigation | DONE | 4-mode navigation, FacetFilterPanel, ViewPicker context-aware, T/E presets |
| 7A | Rust CLI + TUI | DONE | All commands, basic TUI with taxonomy tree |
| 8 | Final Verification | TODO | Audit, tests, lint, type-check, benchmarks |

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
- [ ] Performance benchmarks — 4 navigation modes (Phase 8)
- [ ] Code review (Phase 8)
- [ ] Create GitHub Release v9.0.0 (Phase 8)

## v9.5.0 — Advanced TUI (Galaxy Theme)

Deferred from v9.0 Phase 7B. Mission control cockpit with visual effects.

| Task | Description | Status |
|------|-------------|--------|
| 7.8d | Search + detail pane | DONE (nucleo fuzzy, edge explorer) |
| 7.8e | CRUD dialogs | DONE (node create/edit/delete, relation CRUD) |
| 7.8f | Galaxy visual theme | DONE (theme.rs, deep space palette) |
| 7.8g | Dashboard mode | DONE (Neo4j stats, realm/family charts) |
| 7.8h | Animations (boot, matrix rain) | DONE (6-stage boot: matrix rain → logo reveal → fade) |
| 7.8i | ASCII logo + branding | DONE (Saturn-graph logo, Galaxy colors) |
| 7.8j | Onboarding flow | DONE (welcome screen, guided tour, 5 steps) |
| 7.8k | Command palette + UX | DONE (fuzzy search, 11 commands, 5 categories) |
| 7.8l | Wow effects (CRT, shake, glitch, pulse) | DONE (CRT scanlines, screen shake, glitch transitions, nebula pulse) |

## v10.0.0 — Dynamic Retrieval

Prereq: v9.0.0 stable. No schema migration needed — v10 activates properties v9 already carries.

| Phase | Description | Objective |
|-------|-------------|-----------|
| 10 | Context Assembly Engine | Build engine that reads meta-graph to assemble token-aware context windows autonomously |
| 11 | Dynamic Budget System | Replace static `context_budget` with adaptive system per prompt type, locale complexity |
| 12 | Retrieval API | REST/gRPC API for external agents to query the context graph |

**Success criteria**: Given a Block + Locale, the engine produces a context window without
hardcoded traversal logic. All traversal decisions come from the meta-graph.

**Key properties activated**: `traversal_depth` (Kind), `default_traversal` (EdgeFamily),
`temperature_threshold` (EdgeKind).

## v11.0.0 — Autonomous Learning

Prereq: v10.0.0 stable.

| Phase | Description | Objective |
|-------|-------------|-----------|
| 13 | Feedback Loop | Quality scores from generated content feed back into meta-graph weights |
| 14 | Self-Tuning | Meta-graph adjusts context budgets and traversal depths from feedback |
| 15 | Discovery | New concepts and relationships suggested from generation patterns |

**Success criteria**: The meta-graph improves its own context assembly quality over time
without manual tuning.

## v12.0.0 — Content Pipeline

Prereq: v11.0.0 stable.

| Phase | Description | Objective |
|-------|-------------|-----------|
| 16 | CLI-Driven Generation | Full content generation pipeline exposed through `novanet generate` |

**Success criteria**: `novanet generate --project=qrcode-ai --locale=fr-FR` produces
production-ready localized content for all pages/blocks.

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
2. Run `tools/scripts/release-notes.sh <version>` to generate notes
3. Update `CHANGELOG.md` with generated notes
4. Create annotated git tag: `git tag -a v<version> -m "v<version>"`
5. Push tag: `git push origin v<version>`
6. Create GitHub Release with generated notes
7. Update this ROADMAP.md

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
