# NovaNet DX Strategy: Public Repo Preparation

**Date**: 2026-02-27
**Status**: IN PROGRESS (Security fixes applied ✅)
**Goal**: Match Nika's DX patterns (ARMADA, CI gates) while managing private/public separation

## Quick Status

| Task | Status |
|------|--------|
| Remove .claude from NovaNet git tracking | ✅ Done (69 files) |
| Remove .claude from Nika git tracking | ✅ Done (29 files) |
| Migrate Nika to symlinks pattern | ✅ Done |
| Security rules documentation | ✅ Created (dx/.claude/rules/security-private-content.md) |
| ARMADA workflow for NovaNet | ⏳ Pending |
| release-plz + git-cliff | ⏳ Pending |

---

## Executive Summary

NovaNet needs to prepare for a public repo release while keeping sensitive content (models, schema, cypher) private. This document proposes a DX strategy inspired by Nika's ARMADA 10-station quality system.

---

## 1. Current State Analysis

### 1.1 NovaNet Architecture

```
novanet/
├── .claude/                    # Symlinks → dx/.claude/novanet/
├── packages/
│   ├── core/models/           # Symlink → brain/models (PRIVATE)
│   └── db/seed/               # Symlink → brain/seed (PRIVATE)
├── tools/
│   ├── novanet/               # CLI + TUI (PUBLIC)
│   └── novanet-mcp/           # MCP Server (PUBLIC)
├── apps/studio/               # Web visualization (PUBLIC)
└── brain/                     # Separate git repo (PRIVATE)
```

**Key insight**: `brain/` is already isolated as a separate repo.

### 1.2 Current CI (6 workflows)

| Workflow | Stations | Missing |
|----------|----------|---------|
| `ci.yml` | Security, Schema-sync, Rust (fmt/clippy/test), Build | Coverage, Docs, AI review |
| `rust-ci.yml` | Standalone Rust checks | Duplicated with ci.yml |
| `sast.yml` | Semgrep static analysis | Good |
| `codeql.yml` | GitHub code scanning | Good |
| `design-system-validation.yml` | Design token validation | Good |
| `release.yml` | Release automation | Missing git-cliff |

### 1.3 Nika Reference (ARMADA 10 Stations)

```
Station 1:  Format         cargo fmt --check
Station 2:  Lint           cargo clippy -- -D warnings
Station 3:  Tests          cargo nextest run
Station 4:  Coverage       cargo llvm-cov (>80%)
Station 5:  Docs           cargo doc --no-deps
Station 6:  Security       cargo audit + cargo deny
Station 7:  CodeRabbit     AI review (general)
Station 8:  Claude AI      AI review (project-specific)
Station 9:  Conventional   commitlint validation
Station 10: Version Lock   0.x.x enforcement
```

---

## 2. Private vs Public Strategy

### 2.1 What Stays Private (brain/ repo)

| Content | Reason |
|---------|--------|
| `brain/models/` | Proprietary schema design |
| `brain/seed/` | Business-specific data |
| `brain/data/` | Production data templates |
| ADRs (some) | Internal architecture decisions |
| Business logic | QR Code AI specific |

### 2.2 What Goes Public (novanet/ repo)

| Content | Reason |
|---------|--------|
| `tools/novanet/` | CLI + TUI - showcases Rust patterns |
| `tools/novanet-mcp/` | MCP Server - reference implementation |
| `apps/studio/` | Web visualization - demo capabilities |
| `packages/core/src/` | TypeScript types (without private models) |
| `.claude/` (sanitized) | DX best practices showcase |

### 2.3 Git Structure for Public Release

```
Option A: Submodule (Current)
├── novanet/                    # Public repo
│   └── brain/                  # Private submodule (gitignored in public)

Option B: Separate Repos (Recommended)
├── novanet-public/             # Open source release
│   ├── packages/core/          # Types only, no models symlink
│   ├── tools/novanet/          # CLI + TUI
│   └── apps/studio/            # Web demo
└── novanet-private/            # Internal development
    ├── brain/                  # Models, schema, seed
    └── ...                     # Full development setup
```

---

## 3. Proposed ARMADA-NovaNet (10 Stations)

### 3.1 Station Mapping

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ARMADA-NOVANET — 10 QUALITY STATIONS                                         ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   Station 1: 🔧 Format         pnpm fmt + cargo fmt --check                   ║
║   Station 2: 📎 Lint           pnpm lint + cargo clippy -- -D warnings        ║
║   Station 3: 🧪 Tests          pnpm test + cargo nextest run                  ║
║   Station 4: 📊 Coverage       pnpm test:cov + cargo llvm-cov (>80%)          ║
║   Station 5: 📖 Docs           pnpm typedoc + cargo doc --no-deps             ║
║   Station 6: 🔒 Security       pnpm audit + cargo deny + TruffleHog           ║
║   Station 7: 🤖 CodeRabbit     AI review (general patterns)                   ║
║   Station 8: 🧠 Claude AI      AI review (NovaNet-specific)                   ║
║   Station 9: 📝 Conventional   commitlint validation                          ║
║   Station 10: 📐 Schema Sync   cargo run -- schema validate                   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

**Difference from Nika**: Station 10 is Schema Sync (not Version Lock) because NovaNet follows standard SemVer.

### 3.2 New Workflows Needed

| Workflow | Purpose | Priority |
|----------|---------|----------|
| `armada-checkpoints.yml` | 10-station execution | HIGH |
| `release-plz.yml` | Automated release PR | HIGH |
| `comprehensive-tests.yml` | Extended test suite | MEDIUM |
| `validate-workflows.yml` | CI self-validation | LOW |

### 3.3 Proposed `armada-checkpoints.yml`

```yaml
name: ARMADA Checkpoints

on:
  pull_request:
    branches: [main]

jobs:
  armada:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        station:
          - { name: "Format", script: "pnpm fmt:check && cargo fmt --check" }
          - { name: "Lint", script: "pnpm lint && cargo clippy -- -D warnings" }
          - { name: "Tests", script: "pnpm test && cargo nextest run" }
          - { name: "Coverage", script: "cargo llvm-cov --fail-under 80" }
          - { name: "Docs", script: "cargo doc --no-deps" }
          - { name: "Security", script: "cargo deny check && cargo audit" }
          - { name: "Schema", script: "cargo run -- schema validate" }
    steps:
      - uses: actions/checkout@v4
      - name: "Station: ${{ matrix.station.name }}"
        run: ${{ matrix.station.script }}
```

---

## 4. DX Files Strategy

### 4.1 Symlink Strategy (CORRECT PATTERN)

**Current**: NovaNet `.claude/` → symlinks to `dx/.claude/novanet/` ✅ CORRECT

**Pattern**:
- `dx/` = PRIVATE repo containing all DX (rules, ADRs, skills, hooks)
- `.claude/` in projects = symlinks to dx/ (breaks in public = intended)

| Aspect | Private Dev | Public Release |
|--------|-------------|----------------|
| dx/ repo | Available | NOT included |
| .claude/ symlinks | Work | Broken (intended) |
| Public .claude/ | N/A | Minimal sanitized version |

**Nika mistake**: Copied dx/.claude/nika inline → exposes private DX

### 4.2 Public .claude/ (Minimal, Sanitized)

For public release, create a MINIMAL `.claude/` without symlinks:

```
novanet-public/.claude/
├── settings.json              # Basic permissions only, no hooks
├── README.md                  # "Full DX available for contributors"
└── rules/
    ├── rust.md                # Generic Rust conventions
    └── typescript.md          # Generic TS conventions
```

**Why minimal?**
- Full DX is PRIVATE (in dx/ repo)
- Public contributors don't need internal workflows
- Prevents accidental exposure of ADRs, terminology, etc.

### 4.3 Private DX (stays in dx/)

All of this stays in `dx/.claude/novanet/`:

```
dx/.claude/novanet/           # PRIVATE - not in public repo
├── settings.json             # Full hooks, permissions
├── agents/                   # Internal agents
├── commands/                 # All commands including /adr
├── skills/                   # All skills
├── hooks/                    # Workflow hooks
├── rules/
│   ├── novanet-decisions.md  # ADRs
│   ├── novanet-terminology.md # Domain vocabulary
│   ├── schema-standard.md    # Full schema rules
│   └── ...
└── guides/                   # Internal guides
```

---

## 5. Hooks Strategy

### 5.1 Current NovaNet Hooks

| Hook | Purpose | Public? |
|------|---------|---------|
| `session-start.sh` | Status display | YES (sanitize) |
| `post-edit-format.sh` | Auto-format | YES |
| `keybindings-reminder.sh` | TUI reminders | YES |
| `semantic-check.sh` | Semantic validation | YES |
| `yaml-sync-reminder.sh` | YAML sync | NO (private models) |
| `views-sync-reminder.sh` | Views sync | NO (private views) |
| `doc-sync-reminder.sh` | Doc sync | YES |
| `adr-context.sh` | ADR loading | NO (private ADRs) |

### 5.2 New Hooks (Inspired by Nika)

| Hook | Purpose | Example |
|------|---------|---------|
| `check-git-commit.sh` | Validate before commit | Check tests pass |
| `version-lock-check.sh` | Not needed | NovaNet uses SemVer |
| `verify-alignment.sh` | Spec alignment | Check implementation matches spec |

---

## 6. Release Automation

### 6.1 Current State

- Manual tagging with `git tag -a`
- Manual CHANGELOG updates
- Release workflow creates GitHub Release

### 6.2 Proposed (Match Nika)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  RELEASE PIPELINE                                                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Push to main → release-plz creates Release PR → Merge → Auto-tag            ║
║                                                                               ║
║  Files:                                                                       ║
║  ├── release-plz.toml         Configuration                                  ║
║  ├── cliff.toml               Changelog generation                           ║
║  └── .github/workflows/release-plz.yml                                       ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### 6.3 git-cliff Configuration

```toml
# cliff.toml
[changelog]
header = "# Changelog\n\n"
body = """
## [{{ version }}] - {{ timestamp | date(format="%Y-%m-%d") }}
{% for group, commits in commits | group_by(attribute="group") %}
### {{ group }}
{% for commit in commits %}
- {{ commit.message | upper_first }}
{% endfor %}
{% endfor %}
"""
```

---

## 7. Implementation Plan

### Phase 1: CI Enhancement (Week 1)

- [ ] Create `armada-checkpoints.yml`
- [ ] Add coverage station (`cargo llvm-cov`)
- [ ] Add docs station (`cargo doc`)
- [ ] Integrate CodeRabbit

### Phase 2: Release Automation (Week 2)

- [ ] Install release-plz
- [ ] Configure git-cliff
- [ ] Create `release-plz.yml` workflow
- [ ] Test release pipeline

### Phase 3: Fix Nika DX Pattern (Week 3)

- [ ] Sync Nika inline .claude/ back to dx/.claude/nika/
- [ ] Replace Nika inline with symlinks → dx/.claude/nika/
- [ ] Verify both projects use same pattern
- [ ] Document symlink setup in CONTRIBUTING.md

### Phase 4: Public Repo Prep (Week 4)

- [ ] Create minimal public `.claude/` (no symlinks, sanitized)
- [ ] Write public README.md (no private refs)
- [ ] Add CONTRIBUTING.md (explain dx/ is private)
- [ ] Add LICENSE
- [ ] Gitignore: brain/, dx/, .claude/ symlinks
- [ ] Test: clone works without dx/ and brain/

---

## 8. Decision Matrix

| Decision | Option A | Option B | Recommendation |
|----------|----------|----------|----------------|
| DX location (dev) | Inline | Symlink (dx/) | **Symlink** (current = correct) |
| DX location (public) | None | Minimal .claude/ | **Minimal** sanitized |
| Version strategy | 0.x.x forever | SemVer | **SemVer** (different from Nika) |
| Station 10 | Version Lock | Schema Sync | **Schema Sync** |
| Release | Manual | release-plz | **release-plz** |
| AI review | Claude only | CodeRabbit + Claude | **Both** |
| Private content | Submodule | Separate repo | **Separate repo** (dx/ + brain/) |

---

## 9. Summary

### Key Differences from Nika

| Aspect | Nika | NovaNet |
|--------|------|---------|
| DX location | Inline (MISTAKE) | Symlinks → dx/ (CORRECT) |
| Versioning | 0.x.x forever | Standard SemVer |
| Station 10 | Version Lock | Schema Sync |
| Private content | Exposed in .claude/ | brain/ + dx/ repos (private) |
| Language | Pure Rust | Rust + TypeScript |

### Action Items (Priority Order)

1. **HIGH**: Create `armada-checkpoints.yml` with 10 stations
2. **HIGH**: Install release-plz + git-cliff
3. **HIGH**: Fix Nika to use symlinks like NovaNet (sync dx/.claude/nika)
4. **MEDIUM**: Create minimal public `.claude/` for open source release
5. **LOW**: Create public repo structure

---

## 10. Questions for Brainstorming

1. **Public scope**: How much of NovaNet Studio should be public?
2. **Demo data**: Should we create sample brain/ data for public demo?
3. **MCP Server**: Make novanet-mcp fully public or partial?
4. **CodeRabbit config**: Which rules apply to NovaNet?
5. **Commit hooks**: Pre-commit or CI-only validation?

---

*Document created by Claude Code during DX analysis session.*
