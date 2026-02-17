---
paths:
  - "packages/**/*.ts"
  - "packages/**/*.yaml"
  - "apps/**/*.ts"
  - "apps/**/*.tsx"
  - "tools/novanet/**/*.rs"
  - "packages/db/**/*.cypher"
  - "docs/plans/**/*.md"
description: ADR index - load when editing NovaNet code or planning
---

# NovaNet Architecture Decisions (v0.13.1)

This file is the **index** for NovaNet's Architecture Decision Records.
Full ADR content is in `.claude/rules/adr/{domain}/adr-XXX-*.md`.

> **Version**: v0.13.1 "Auto-Fix System" (ADR-033) + *Native Pattern (ADR-029 + ADR-030)

---

## Quick Reference (TL;DR)

### Core Principles

| ADR | Principle | Rule |
|-----|-----------|------|
| **007** | Generation NOT Translation | `Entity → Generate natively → EntityNative` (not translate) |
| **003** | YAML-First | YAML = source of truth → generators → TS/Cypher/Mermaid |
| **001** | Arc Terminology | Use "Arc" (not Edge/Relation) for directed links |
| **021** | Query-First | Cypher = source of truth for graph visualization |

### Current Architecture (v0.13.0)

```
SHARED (4 layers, 40 nodes): config, locale, geography, knowledge — READ-ONLY
ORG (6 layers, 21 nodes): config, foundation, structure, semantic, instruction, output
Total: 61 nodes, 169 arcs, 10 layers, 5 arc families, 5 traits, 33 ADRs
```

### v0.13.1 Key Changes

| ADR | Change | Summary |
|-----|--------|---------|
| **033** | Auto-Fix System | Trait-based FixEngine with 6 fixers for schema validation (52 tests) |

### v0.13.0 Key Changes

| ADR | Change | Summary |
|-----|--------|---------|
| **029** | *Native Pattern | EntityContent→EntityNative, PageGenerated→PageNative, unified HAS_NATIVE |
| **030** | Slug Ownership | Page owns URL (slug, full_path), Entity owns semantics (key) |
| **028** | Brand Architecture | Brand, BrandDesign, BrandPrinciples, PromptStyle nodes |
| **024** | Trait = Data Origin | `defined`, `authored`, `imported`, `generated`, `retrieved` |

### Trait Quick Reference (ADR-024)

| Trait | Origin | Examples |
|-------|--------|----------|
| `defined` | Human creates ONCE | Page, Block, Entity, Locale, OrgConfig |
| `authored` | Human writes PER locale | EntityNative, ProjectNative |
| `imported` | External data brought in | Term, Expression, SEOKeyword, GEOQuery |
| `generated` | Our LLM produces | PageNative, BlockNative |
| `retrieved` | Fetched from external APIs | GEOAnswer, SEOKeywordMetrics |

### UX Architecture

| ADR | Feature | Rule |
|-----|---------|------|
| **022** | Unified Tree | 2 modes: `[1]Graph` + `[2]Nexus`. "If it's a node in Neo4j, it's a node everywhere" |
| **021** | Query-First | Cypher = source of truth. YAML views only (no TS hardcoding) |
| **013** | Icons | `visual-encoding.yaml` → `{ web: "lucide", terminal: "unicode" }` — NO emoji |

---

## ADR Index by Domain

### Core Principles (5 ADRs)

Foundation philosophy and methodology.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 001 | Arc Terminology | v9.5 | stable | [adr-001](adr/core-principles/adr-001-arc-terminology.md) |
| 003 | YAML-First Architecture | v9.0 | stable | [adr-003](adr/core-principles/adr-003-yaml-first.md) |
| 007 | Generation, Not Translation | core | stable | [adr-007](adr/core-principles/adr-007-generation-not-translation.md) |
| 010 | Skill-First DX | v9.5 | stable | [adr-010](adr/core-principles/adr-010-skill-first-dx.md) |
| 021 | Query-First Architecture | v11.6 | active | [adr-021](adr/core-principles/adr-021-query-first.md) |

### Schema Architecture (6 ADRs)

Realm, layer, and node organization.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 006 | Realm Differentiates Scope | v9.0 | stable | [adr-006](adr/schema-architecture/adr-006-realm-scope.md) |
| 012 | 2-Realm Architecture | v10.6 | stable | [adr-012](adr/schema-architecture/adr-012-two-realm.md) |
| 017 | EntityCategory Classification | v11.1 | stable | [adr-017](adr/schema-architecture/adr-017-entity-category.md) |
| 028 | Page-Entity Architecture | v0.12.3 | active | [adr-028](adr/schema-architecture/adr-028-page-entity.md) |
| 029 | *Native Pattern | v0.12.5 | active | [adr-029](adr/schema-architecture/adr-029-native-pattern.md) |
| 030 | Slug Ownership | v0.12.5 | active | [adr-030](adr/schema-architecture/adr-030-slug-ownership.md) |

### Node Classification (4 ADRs)

Node naming, traits, and classification axes.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 002 | Symmetric Taxonomy | v9.5 | stable | [adr-002](adr/node-classification/adr-002-symmetric-taxonomy.md) |
| 023 | Class/Instance Terminology | v0.12.0 | active | [adr-023](adr/node-classification/adr-023-class-instance.md) |
| 024 | Trait = Data Origin | v0.12.0 | active | [adr-024](adr/node-classification/adr-024-trait-data-origin.md) |
| 025 | Instruction Layer Renaming | v0.12.0 | active | [adr-025](adr/node-classification/adr-025-instruction-layer.md) |

### Arc Design (4 ADRs)

Arc families, inverses, and relationship patterns.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 015 | Unidirectional Ownership Arcs | v10.9 | stable | [adr-015](adr/arc-design/adr-015-unidirectional-ownership.md) |
| 016 | Type-Constrained Container Arcs | v10.9 | stable | [adr-016](adr/arc-design/adr-016-type-constrained-containers.md) |
| 026 | Inverse Arc Policy | v0.12.1 | active | [adr-026](adr/arc-design/adr-026-inverse-arc-policy.md) |
| 027 | Generation Family Arc Semantics | v0.12.1 | active | [adr-027](adr/arc-design/adr-027-generation-family.md) |

### Visual Encoding (4 ADRs)

Colors, icons, and visual representation.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 004 | No Color Duplication | v9.5 | stable | [adr-004](adr/visual-encoding/adr-004-no-color-duplication.md) |
| 005 | Trait-Based Visual Encoding | v9.0 | stable | [adr-005](adr/visual-encoding/adr-005-trait-visual-encoding.md) |
| 009 | Terminal Color Graceful Degradation | v9.5 | stable | [adr-009](adr/visual-encoding/adr-009-terminal-colors.md) |
| 013 | Icons Source of Truth | v10.6 | stable | [adr-013](adr/visual-encoding/adr-013-icons-source.md) |

### UX Architecture (2 ADRs)

TUI/Studio navigation and user experience.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 008 | Invariant Structure, Localized Content | v9.0 | stable | [adr-008](adr/ux-architecture/adr-008-invariant-structure.md) |
| 022 | Unified Tree Architecture | v11.7 | active | [adr-022](adr/ux-architecture/adr-022-unified-tree.md) |

### SEO/GEO (2 ADRs)

SEO pillar/cluster and URL architecture.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 031 | SEO Pillar/Cluster Architecture | v0.12.5 | active | [adr-031](adr/seo-geo/adr-031-seo-pillar-cluster.md) |
| 032 | URL Slugification Architecture | v0.12.5 | active | [adr-032](adr/seo-geo/adr-032-url-slugification.md) |

### Development Tools (1 ADR)

Tooling and automation for development workflows.

| ADR | Title | Version | Status | File |
|-----|-------|---------|--------|------|
| 033 | Auto-Fix System for Schema Validation | v0.13.1 | active | [adr-033](adr/development-tools/adr-033-autofix-system.md) |

### Deprecated (5 ADRs)

Superseded or historical-only ADRs.

| ADR | Title | Version | Status | Superseded By | File |
|-----|-------|---------|--------|---------------|------|
| 011 | Company Project Pattern | v10.5 | superseded | ADR-012 | [adr-011](adr/deprecated/adr-011-company-project.md) |
| 014 | L10n to Content/Generated | v10.9 | superseded | ADR-029 | [adr-014](adr/deprecated/adr-014-l10n-to-content.md) |
| 018 | Classification System Refinement | v11.2 | historical | — | [adr-018](adr/deprecated/adr-018-classification-refinement.md) |
| 019 | Layer Reorganization | v11.3 | historical | — | [adr-019](adr/deprecated/adr-019-layer-reorganization.md) |
| 020 | Schema Refinement | v11.5 | historical | — | [adr-020](adr/deprecated/adr-020-schema-refinement.md) |

---

## Decision Log (Chronological)

| ADR | Version | Summary |
|-----|---------|---------|
| 001 | v9.5 | Arc terminology |
| 002 | v9.5 | Symmetric taxonomy (prefixed types) |
| 003 | v9.0 | YAML-first architecture |
| 004 | v9.5 | No color duplication |
| 005 | v9.0 | Trait-based visual encoding |
| 006 | v9.0 | Realm differentiates scope |
| 007 | core | Generation, not translation |
| 008 | v9.0 | Invariant structure, localized content |
| 009 | v9.5 | Terminal color graceful degradation |
| 010 | v9.5 | Skill-first DX |
| 011 | v10.5 | Company project pattern (superseded by 012) |
| 012 | v10.6 | 2-Realm Architecture |
| 013 | v10.6 | Icons source of truth |
| 014 | v10.9 | Naming convention refactor (superseded by 029) |
| 015 | v10.9 | Unidirectional ownership arcs |
| 016 | v10.9 | Type-constrained container arcs |
| 017 | v11.1 | EntityCategory classification |
| 018 | v11.2 | Classification system refinement |
| 019 | v11.3 | Layer reorganization |
| 020 | v11.5 | Schema refinement |
| 021 | v11.6 | Query-First Architecture |
| 022 | v11.7 | Unified Tree Architecture |
| 023 | v0.12.0 | Class/Instance Terminology |
| 024 | v0.12.0 | Trait Redefinition as "Data Origin" |
| 025 | v0.12.0 | Instruction Layer Renaming |
| 026 | v0.12.1 | Inverse Arc Policy |
| 027 | v0.12.1 | Generation Family Arc Semantics |
| 028 | v0.12.3 | Page-Entity Architecture |
| 029 | v0.12.5 | *Native Pattern |
| 030 | v0.12.5 | Slug Ownership |
| 031 | v0.12.5 | SEO Pillar/Cluster Architecture |
| 032 | v0.12.5 | URL Slugification Architecture |

---

## Quick Lookup

### By Status

- **Stable** (15): 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 012, 013, 015, 016, 017
- **Active** (12): 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032
- **Superseded** (2): 011, 014
- **Historical** (3): 018, 019, 020

### Must-Know for v0.13.0

These 6 ADRs are essential for daily development:

1. **ADR-029** (*Native Pattern) — Unified suffix for locale-specific nodes
2. **ADR-030** (Slug Ownership) — Page owns URL, Entity owns semantics
3. **ADR-024** (Trait = Data Origin) — 5 traits: defined/authored/imported/generated/retrieved
4. **ADR-025** (Instruction Layer) — PageStructure, PageInstruction naming
5. **ADR-021** (Query-First) — Cypher is the source of truth
6. **ADR-022** (Unified Tree) — 2 modes: Graph + Nexus

### Machine-Readable Index

See [adr/_index.yaml](adr/_index.yaml) for programmatic access to all ADR metadata.

---

## Related Documentation

- **Terminology**: [novanet-terminology.md](novanet-terminology.md) — Current terms and deprecated mappings
- **Arc Design Guide**: [arc-design-guide.md](arc-design-guide.md) — Best practices for arc creation
- **Security**: [security.md](security.md) — Security toolchain and policies

## References

- Design plans: `docs/plans/`
- YAML models: `packages/core/models/`
- TUI implementation: `tools/novanet/src/tui/`
