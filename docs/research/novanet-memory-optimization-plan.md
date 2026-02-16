# NovaNet Memory Optimization Plan

**Status**: Research & Recommendations
**Target**: Implement Claude Code best practices from Feb 2026 official docs
**Scope**: Memory hierarchy, context optimization, path-specific rules

---

## Current State Analysis

### Memory Files Inventory

```
novanet-hq/
├── CLAUDE.md (project root, 1247 lines)
│   └── Current: All content inline, no imports
│
├── .claude/
│   └── rules/
│       ├── novanet-decisions.md (930 lines, ADR-001 to ADR-032)
│       ├── novanet-terminology.md (380 lines, canonical terms)
│       ├── arc-design-guide.md (160 lines, arc design patterns)
│       └── security.md (60 lines, deny.toml info)
│
├── tools/novanet/
│   └── CLAUDE.md (220 lines, Rust-specific)
│
├── packages/core/
│   └── (no CLAUDE.md)
│
├── packages/studio/
│   └── (no CLAUDE.md)
│
└── packages/db/
    └── (no CLAUDE.md)
```

### Issues Identified

1. **Root CLAUDE.md is 1247 lines** (best practice: 140 lines max)
   - All content inline, no imports
   - Duplicates content from decision docs
   - Auto memory will load only first 200 lines of anything

2. **No path-specific rules**
   - Rust developers get TypeScript rules (wasted context)
   - Frontend developers get backend rules (wasted context)
   - All rules loaded for all files

3. **Auto memory not optimized**
   - First 200 lines of auto memory unknown
   - No topic file strategy for on-demand loading
   - No documented memory index

4. **Memory not imported**
   - `.claude/rules/` files exist but root doesn't import them
   - Information scattered across 5 files
   - No clear composition strategy

---

## Recommended Optimizations

### Phase 1: Refactor Root CLAUDE.md (Immediate)

**Target**: Reduce from 1247 → 140 lines using imports.

**New structure**:

```markdown
# NovaNet Development Guide

## Quick Start

**Monorepo**: Turborepo + pnpm + Rust + TypeScript + YAML
- **Build**: `pnpm build && cargo build`
- **Test**: `pnpm test && cargo test`
- **CLI**: `cargo run --` (schema, db, tui commands)

## Architecture & Decisions

See @.claude/rules/novanet-decisions.md (30+ ADRs: native pattern, slug ownership, SEO architecture)

## Canonical Terminology

See @.claude/rules/novanet-terminology.md (Class vs Kind, Arc vs Edge, Trait = Data Origin)

## Arc Design Standards

See @.claude/rules/arc-design-guide.md (naming, llm_context, inverse policy)

## Project Status

- **Version**: v0.13.0 (Native Pattern + ADR-029/030 + Data Origin traits)
- **Nodes**: 61 total (40 shared + 21 org)
- **Arcs**: 169 total (5 families)
- **Layers**: 10 total (4 shared + 6 org)
- **Tests**: 1000+ (Rust), 250+ (TypeScript)
- **Status**: Stable, active development

## Language-Specific Rules

- **Rust**: See `.claude/rules/rust/` (safety, async, testing)
- **TypeScript**: See `.claude/rules/typescript/` (naming, imports, React)
- **YAML**: See `.claude/rules/yaml/` (schema validation, naming)

## Repository Roots

- Monorepo root: `novanet-hq/`
- Rust CLI: `tools/novanet/` (see tools/novanet/CLAUDE.md)
- TypeScript core: `packages/core/`
- Studio frontend: `packages/studio/`
- Database infra: `packages/db/`
```

**Benefits**:
- Root file: 140 lines (down from 1247)
- Auto memory loads 140 lines every session (vs 200 from original)
- Detailed docs imported on demand
- Cleaner mental model

### Phase 2: Create Path-Specific Rules (Week 1)

**Directory structure**:

```
.claude/rules/
├── general.md                     # Applies everywhere
├── novanet-decisions.md           # Keep (imported from root)
├── novanet-terminology.md         # Keep (imported from root)
├── arc-design-guide.md            # Keep (imported from root)
├── security.md                    # Keep
├── rust/
│   ├── safety.md                  # paths: tools/novanet/**/*.rs
│   ├── async-tokio.md            # paths: tools/novanet/**/*.rs
│   ├── testing.md                # paths: tools/novanet/**/*.rs
│   └── performance.md            # paths: tools/novanet/**/*.rs
├── typescript/
│   ├── naming.md                 # paths: packages/**/*.ts
│   ├── imports.md                # paths: packages/**/*.ts
│   ├── react.md                  # paths: packages/studio/**/*.tsx
│   └── testing.md                # paths: packages/**/*.test.ts
├── yaml/
│   ├── schema.md                 # paths: packages/core/models/**/*.yaml
│   ├── naming.md                 # paths: packages/core/models/**/*.yaml
│   └── validation.md             # paths: packages/core/models/**/*.yaml
└── paths/
    ├── rust-cli.md               # For tools/novanet/**
    ├── typescript-core.md        # For packages/core/**/*.ts
    ├── typescript-studio.md      # For packages/studio/**/*.tsx
    └── yaml-models.md            # For packages/core/models/**
```

**Example: rust/safety.md**

```markdown
---
paths:
  - "tools/novanet/src/**/*.rs"
---

# Rust Safety Standards

(Content from tools/novanet/CLAUDE.md + best practices)

## Unsafe Code
- All `unsafe {}` blocks require SAFETY: comments
- Use `thiserror` for error types
- Prefer `?` operator over `match`

## Async/Await
- Use Tokio for async runtime
- Channel patterns: mpsc for tasks, broadcast for events
- Use `select!` for multiple waits

## Testing
- Unit tests in mod tests { } blocks
- Integration tests in tests/ directory
- Use proptest for property-based testing
- Use insta for snapshot testing
```

**Benefits**:
- Rust developers get Rust rules only (60 lines, not 400)
- TypeScript developers get TypeScript rules only (80 lines, not 400)
- Context savings: ~60% reduction for path-specific queries

### Phase 3: Organize Auto Memory (Week 1)

**Current auto memory**: Unknown structure

**Recommended structure**:

```
~/.claude/projects/novanet-hq/memory/
├── MEMORY.md (200 lines target)
│   ├── Quick facts (monorepo, languages, versions)
│   ├── Build/test commands (pnpm, cargo)
│   ├── Architecture summary (2 realms, 10 layers, 61 nodes)
│   ├── Key files (models/, YAML, generators)
│   └── Current focus (v0.13.0, Native Pattern, SEO architecture)
│
├── debugging.md (on-demand)
│   ├── Neo4j connection issues
│   ├── YAML validation errors
│   ├── TypeScript import resolution
│   └── Common Rust compilation errors
│
├── architecture.md (on-demand)
│   ├── Module relationships (core → studio → CLI)
│   ├── Data flow (YAML → generators → artifacts)
│   ├── Test infrastructure (1000+ tests, coverage targets)
│   └── Performance notes
│
├── patterns.md (on-demand)
│   ├── Generator architecture (12 artifacts from YAML)
│   ├── Hook patterns (.claude/hooks/)
│   ├── Test patterns (proptest, insta, snapshot)
│   └── Release procedures
│
└── schema.md (on-demand)
    ├── Current node/arc counts
    ├── Realm/layer/trait definitions
    ├── Visual encoding (colors, icons)
    └── Path validation rules
```

**MEMORY.md content** (first 200 lines, loaded every session):

```markdown
# NovaNet Memory Index

## Quick Facts

**Monorepo**: Turborepo + pnpm
- **Root**: ~/supernovae-st/novanet-hq/
- **Workspaces**: 3 packages + 1 tool (Rust CLI)
- **Languages**: Rust 1500+ LOC, TypeScript 5000+ LOC, YAML schema
- **Build**: `pnpm build && cargo build`
- **Test**: `pnpm test` + `cargo test` (1000+ tests pass)

## Current v0.13.0 Status

**Native Pattern** (ADR-029/030): EntityNative, PageNative unified suffix
**Data Origin Traits** (ADR-024): defined/authored/imported/generated/retrieved
**Slug Ownership** (ADR-030): Page owns URL, Entity owns semantics
**Brand Architecture** (ADR-028): Brand, BrandDesign, BrandPrinciples, PromptStyle

## Schema Overview

- **Total**: 61 nodes, 169 arcs, 10 layers, 5 traits
- **SHARED** (4 layers, 40 nodes): config, locale, geography, knowledge (READ-ONLY)
- **ORG** (6 layers, 21 nodes): config, foundation, structure, semantic, instruction, output
- **YAML source**: packages/core/models/ (node-classes, arc-classes, taxonomy)
- **Generators**: 12 artifacts from YAML (TS, Rust, Mermaid, Cypher)

## Build Commands

```bash
# Schema generation
cargo run -- schema generate           # From YAML
cargo run -- schema validate           # Validation

# Documentation
cargo run -- doc generate              # Mermaid diagrams
cargo run -- doc generate --list       # Available views

# Database
pnpm infra:up                          # Start Neo4j
pnpm infra:seed                        # Populate schema

# Testing
cargo test                             # 1000+ tests
cargo nextest run                      # Parallel testing
pnpm test                              # TypeScript tests
```

## Key Files

- `.claude/rules/novanet-decisions.md` → 30+ ADRs
- `.claude/rules/novanet-terminology.md` → Canonical terms
- `tools/novanet/CLAUDE.md` → Rust CLI rules
- `packages/core/models/` → YAML schema (source of truth)
- `packages/core/src/generators/` → Code generators

## See Also

See detailed docs for:
- **Architecture**: memory/architecture.md
- **Debugging**: memory/debugging.md
- **Patterns**: memory/patterns.md
- **Schema**: memory/schema.md
```

**Benefit**: Every session starts with 200 lines of essential context. Detailed notes load on-demand.

### Phase 4: Settings & Hooks (Week 2)

**Create `.claude/settings.json`**:

```json
{
  "permissions": {
    "allow": [
      "Bash(pnpm *)",
      "Bash(cargo *)",
      "Bash(git *)"
    ],
    "deny": [
      "Bash(rm -rf *)",
      "Bash(sudo *)"
    ]
  },
  "hooks": {
    "FileEdit": [
      {
        "matcher": "**/*.rs",
        "hooks": [
          {
            "type": "command",
            "command": "cargo fmt"
          }
        ]
      },
      {
        "matcher": "**/*.{ts,tsx}",
        "hooks": [
          {
            "type": "command",
            "command": "prettier --write {path}"
          }
        ]
      },
      {
        "matcher": "**/*.yaml",
        "hooks": [
          {
            "type": "command",
            "command": "yamllint {path}"
          }
        ]
      }
    ]
  },
  "tools": {
    "fileAutocomplete": "~/.claude/file-index.sh"
  }
}
```

**Create `.claude/settings.local.json`** (personal, not committed):

```json
{
  "permissions": {
    "allow": [
      "Bash(http://localhost:*)"
    ]
  }
}
```

**Benefit**: Enforces code quality (auto-format, lint), sandboxes commands.

### Phase 5: Per-Package CLAUDE.md Files (Optional)

**Only create if**:
- Package has unique setup/deployment steps
- Package uses different language/framework patterns
- Package is worked on independently

**Example: packages/studio/CLAUDE.md** (only if needed)

```markdown
# Studio (React/TypeScript Frontend)

## Setup

- **Node**: v18+
- **Package manager**: pnpm
- **Framework**: React 18+ with TypeScript
- **Styles**: Tailwind CSS

## Build

```bash
pnpm build --filter=@novanet/studio
```

## Development

```bash
pnpm dev    # Start Vite dev server + TypeScript watch
```

## Testing

```bash
pnpm test --filter=@novanet/studio
```

(Detailed rules in root .claude/rules/typescript/)
```

**Current recommendation**: Skip this—root rules + path-specific rules sufficient for now.

---

## Implementation Timeline

### Week 1 (Immediate)

**Priority 1: Root CLAUDE.md refactor**
- [ ] Create new 140-line root CLAUDE.md with imports
- [ ] Commit `.claude/rules/` organization
- [ ] Test import approval dialog
- [ ] Verify auto memory loads expected first 200 lines

**Priority 2: Path-specific rules**
- [ ] Create `.claude/rules/paths/rust-cli.md`
- [ ] Create `.claude/rules/paths/typescript-core.md`
- [ ] Create `.claude/rules/paths/typescript-studio.md`
- [ ] Add YAML frontmatter with glob patterns
- [ ] Test path-specific loading (Claude should load only matching rules)

**Priority 3: Auto memory optimization**
- [ ] Review and optimize ~/.claude/projects/novanet-hq/memory/MEMORY.md (200 lines max)
- [ ] Create memory/debugging.md (common errors)
- [ ] Create memory/architecture.md (module relationships)
- [ ] Create memory/patterns.md (build commands, test patterns)

### Week 2 (Following)

**Priority 4: Settings & hooks**
- [ ] Create `.claude/settings.json` with permissions + hooks
- [ ] Test auto-format hook (cargo fmt on Rust edits)
- [ ] Test auto-lint hook (prettier on TypeScript edits)
- [ ] Create `.claude/settings.local.json` (personal, not committed)

**Priority 5: Documentation**
- [ ] Document memory strategy in README or CLAUDE.md
- [ ] List all path-specific rules with their glob patterns
- [ ] Update onboarding: new developers should see optimized context

---

## Expected Benefits

### Immediate (Session 1)

✅ **Context savings**: Root file 140 lines (vs 1247)
✅ **Focused context**: Developers get language-specific rules only
✅ **Faster loading**: Import approval one-time only per project
✅ **Auto memory**: Clear index of available resources

### Short-term (2-4 weeks)

✅ **Better debugging**: Topic files (debugging.md) on-demand for faster issue resolution
✅ **Consistent formatting**: Hooks auto-format every edit (no manual pnpm format)
✅ **Path awareness**: Claude knows which rules apply (Rust vs TypeScript)

### Long-term (1-3 months)

✅ **Scalable context**: New developers onboard with optimized, focused rules
✅ **Auto memory growth**: Patterns documented for future reference
✅ **Reduced context churn**: Path-specific rules prevent unnecessary loads

---

## Validation Checklist

After implementation, verify:

- [ ] Root CLAUDE.md is 140 lines or less
- [ ] `@imports` work (tested with new developer onboarding)
- [ ] Path-specific rules load only for matching files
- [ ] Auto memory first 200 lines are concise & useful
- [ ] Settings.json permissions allow pnpm/cargo/git
- [ ] Hooks auto-format on every edit (Rust + TypeScript)
- [ ] Memory files organized: index (200 lines) + topics (on-demand)
- [ ] No context duplication between root and rules files

---

## References

- Official doc: `/Users/thibaut/.claude-code-docs/docs/claude-code__memory.md`
- Best practices: `/Users/thibaut/supernovae-st/novanet-hq/docs/research/claude-code-large-project-best-practices.md`
- Current state: NovaNet root CLAUDE.md (1247 lines)
- Target state: Optimized 4-level memory hierarchy per Claude Code docs

---

## Notes

**Design decision rationale**:

1. **Root CLAUDE.md imports**: Following Claude Code best practice of keeping root ≤140 lines
2. **Path-specific rules**: Prevent Rust rules from loading during TypeScript edits (context savings)
3. **Auto memory topic files**: Only first 200 lines load every session; detailed notes load on-demand
4. **Settings.json hooks**: Enforce code quality deterministically (no guessing if Claude will format)
5. **Per-package CLAUDE.md**: Deferred until packages become independently workable (not needed now)

This implementation aligns with official Claude Code best practices from Feb 2026 documentation and scales to 100K+ file monorepos without context bloat.
