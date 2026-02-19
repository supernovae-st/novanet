---
name: release
description: Release workflow for supernovae-agi monorepo. Use when preparing or executing releases for NovaNet, Nika, or coordinated releases.
disable-model-invocation: false
user-invocable: true
---

# Release Skill

Orchestrates releases for the supernovae-agi monorepo (NovaNet + Nika).

## Quick Start

```bash
/release status      # Current versions and pending changes
/release preview     # Preview changelog for next release
/release novanet     # Release NovaNet
/release nika        # Release Nika
/release coordinated # Coordinated release (both projects)
```

---

## Commands

### `/release status`

Shows current state:
- Current versions (supernovae-agi, NovaNet, Nika)
- Uncommitted changes
- Unreleased commits since last tag
- VERSION_MATRIX compatibility

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  RELEASE STATUS                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  supernovae-agi: v0.14.0                                                      ║
║  ├── NovaNet:    v0.13.1  (3 unreleased commits)                             ║
║  └── Nika:       v0.2.0   (7 unreleased commits)                             ║
║                                                                               ║
║  Pending Changes:                                                             ║
║  ├── feat(mcp): add denomination_forms to novanet_generate                   ║
║  ├── fix(tui): correct panel rendering on resize                             ║
║  └── docs: update VERSION_MATRIX                                             ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### `/release preview [project]`

Preview changelog for next release using git-cliff:

```bash
/release preview           # All projects
/release preview novanet   # NovaNet only
/release preview nika      # Nika only
```

### `/release novanet [version]`

Release NovaNet:

1. Validate schema sync (`cargo run -- schema validate`)
2. Run tests (`cargo test`)
3. Generate changelog (`git-cliff`)
4. Update VERSION file
5. Create tag and push
6. GitHub Actions creates release

```bash
/release novanet           # Auto-determine version from commits
/release novanet 0.14.0    # Explicit version
/release novanet --dry-run # Preview without executing
```

### `/release nika [version]`

Release Nika:

1. Validate workflows (`cargo run -- validate examples/`)
2. Run tests (`cargo test`)
3. Check NovaNet compatibility (VERSION_MATRIX)
4. Generate changelog (`git-cliff`)
5. Update VERSION file
6. Create tag and push

```bash
/release nika              # Auto-determine version
/release nika 0.3.0        # Explicit version
/release nika --dry-run    # Preview without executing
```

### `/release coordinated`

Coordinated release when changes affect both projects:

1. Verify VERSION_MATRIX compatibility
2. Release NovaNet first (MCP server)
3. Update Nika's MCP requirements
4. Release Nika (MCP client)
5. Update supernovae-agi submodules
6. Release supernovae-agi

```bash
/release coordinated               # Auto-detect needed releases
/release coordinated --breaking    # For breaking MCP changes
```

---

## Version Policy

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  0.x VERSIONING (Pre-Production)                                              ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Commit Type        │ Version Bump │ Example                                  ║
║  ───────────────────┼──────────────┼──────────────────────────────────────── ║
║  feat:              │ MINOR        │ 0.13.0 → 0.14.0                         ║
║  fix:, perf:        │ PATCH        │ 0.13.0 → 0.13.1                         ║
║  feat!: (breaking)  │ MINOR + docs │ 0.13.0 → 0.14.0 + migration guide       ║
║  docs:, chore:      │ No bump      │ (unless explicitly requested)           ║
║                                                                               ║
║  Note: Breaking changes bump MINOR (not MAJOR) until v1.0.0                  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Pre-Release Checklist

Before any release:

- [ ] All tests pass (`pnpm test` / `cargo test`)
- [ ] No uncommitted changes (`git status`)
- [ ] CHANGELOG preview looks correct (`/release preview`)
- [ ] VERSION_MATRIX updated if compatibility changes
- [ ] MCP contracts validated (`cargo run -- mcp validate` in nika-dev)

---

## Release Flow

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  RELEASE FLOW                                                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. Developer runs /release [project]                                           │
│     │                                                                           │
│     ▼                                                                           │
│  2. Skill validates:                                                            │
│     ├── Tests pass                                                              │
│     ├── No uncommitted changes                                                  │
│     ├── Schema sync (NovaNet) / MCP contracts (Nika)                           │
│     └── VERSION_MATRIX compatibility                                            │
│     │                                                                           │
│     ▼                                                                           │
│  3. git-cliff generates changelog                                               │
│     │                                                                           │
│     ▼                                                                           │
│  4. VERSION file updated                                                        │
│     │                                                                           │
│     ▼                                                                           │
│  5. Tag created: v{version}                                                     │
│     │                                                                           │
│     ▼                                                                           │
│  6. Push triggers GitHub Actions:                                               │
│     ├── release.yml creates GitHub Release                                      │
│     ├── CHANGELOG.md auto-updated                                               │
│     └── (future) crates.io publish                                              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Troubleshooting

### "Tests failing"

```bash
# Run full test suite to identify failures
cd novanet-dev && cargo test
cd nika-dev && cargo test
```

### "Schema sync failed"

```bash
# Regenerate schema artifacts
cd novanet-dev && cargo run -- schema generate
```

### "MCP contract mismatch"

```bash
# Validate contracts against current NovaNet
cd nika-dev && cargo run -- mcp validate --contracts ../contracts/
```

### "VERSION_MATRIX outdated"

Edit `VERSION_MATRIX.md` to reflect new compatibility:
- Add new version row
- Update MCP tool versions if changed
- Document breaking changes

---

## Files

| File | Purpose |
|------|---------|
| `VERSION` | Single source of truth for monorepo version |
| `VERSION_MATRIX.md` | Compatibility tracking |
| `CHANGELOG.md` | Auto-generated changelog |
| `cliff.toml` | git-cliff configuration |
| `contracts/` | MCP tool contracts |
| `release-plz.toml` | release-plz configuration (per project) |

---

## Related Skills

- `/novanet-sync` - Schema validation/regeneration
- `/security-audit` - Pre-release security check
- `/codebase-audit` - Pre-release code health check
