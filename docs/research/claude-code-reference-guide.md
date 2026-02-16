# Claude Code Reference Guide for Large Projects

**Quick lookup**: Memory hierarchy, settings, hooks, context optimization

---

## 1. Memory Hierarchy Quick Reference

### What Loads When?

| Memory Type | Location | When Loaded | Amount | Scope |
|-------------|----------|-------------|--------|-------|
| **Managed Policy** | System (IT managed) | Session start | Full | All users |
| **Project CLAUDE.md** | Root + child dirs | Session start | Full (parent), on-demand (child) | Team |
| **Project rules** | `.claude/rules/` | Session start | Full, recursive | Team |
| **User CLAUDE.md** | `~/.claude/CLAUDE.md` | Session start | Full | You |
| **Local project** | `./CLAUDE.local.md` | Session start | Full | You (not committed) |
| **Auto memory** | `~/.claude/projects/<proj>/memory/MEMORY.md` | Session start | **First 200 lines only** | You |
| **Topic memory** | `~/.claude/projects/<proj>/memory/*.md` | On-demand | Full | You |

**Key insight**: Auto memory first 200 lines is the bottleneck—optimize it ruthlessly.

### Precedence Order (Highest → Lowest)

1. Managed policy (IT admin, immutable)
2. Project settings (`.claude/settings.json`, team-shared)
3. Local settings (`.claude/settings.local.json`, personal)
4. Project CLAUDE.md (team-shared rules)
5. User CLAUDE.md (`~/.claude/CLAUDE.md`, personal)
6. Auto memory (Claude's learnings)

**Rule**: More specific overrides more general. Project setting overrides user setting.

---

## 2. File Size Recommendations

### Memory Files

| File | Recommended Size | Notes |
|------|------------------|-------|
| Root `CLAUDE.md` | **140 lines max** | Use imports for detailed content |
| Path-specific rule | **80 lines** | Focused on one concern |
| Language rule (Rust/TS) | **100 lines** | Keep to essentials |
| Auto memory `MEMORY.md` | **200 lines max** | Only these load every session |
| Topic memory file | **500+ lines** | Load on-demand as needed |
| `.claude/rules/` file | Unlimited | Recursive discovery, all loaded |

### Project Settings

| File | Size | Editable |
|------|------|----------|
| `.claude/settings.json` | < 50 lines | Team (version control) |
| `.claude/settings.local.json` | < 30 lines | You (not committed) |
| `.claude/hooks.json` | < 100 lines | Inline in settings |

---

## 3. Path-Specific Rules Examples

### Glob Patterns

| Pattern | Matches |
|---------|---------|
| `src/**/*.ts` | All TypeScript files in src/ |
| `**/*.test.ts` | All test files anywhere |
| `tools/novanet/**/*.rs` | All Rust files in tools/novanet/ |
| `packages/**/*.{ts,tsx}` | TS and TSX files in packages/ |
| `{src,lib}/**/*.ts` | Files in src/ OR lib/ |
| `*.md` | Markdown files in project root |

### YAML Frontmatter

```markdown
---
paths:
  - "src/api/**/*.ts"
  - "src/**/*.test.ts"
---

# These rules apply ONLY to matching files
```

**Files without `paths:` apply to all files** (unconditional).

---

## 4. Auto Memory Strategy

### Two-Level Structure

**Level 1: Index (MEMORY.md, 200 lines, loaded every session)**
- Quick facts, versions, build commands
- Key files, current focus
- Links to topic files

**Level 2: Topics (debugging.md, patterns.md, etc., on-demand)**
- Detailed debugging patterns
- Build command combinations
- Architecture relationships
- Performance insights

### Topics to Create

| Topic | Content | Typical Size |
|-------|---------|--------------|
| `debugging.md` | Error patterns, solutions | 200-300 lines |
| `architecture.md` | Module relationships, data flow | 150-200 lines |
| `patterns.md` | Build commands, test patterns | 200+ lines |
| `performance.md` | Optimization notes, bottlenecks | 100-150 lines |
| `schema.md` | Node/arc counts, current state | 100+ lines |

**Example MEMORY.md**:
```markdown
# Index

## Quick Facts
- Monorepo: Turborepo, pnpm, Rust, TypeScript
- Build: pnpm build && cargo build
- Test: pnpm test && cargo test

## See Also
- Full architecture: See memory/architecture.md
- Debugging: See memory/debugging.md
- Performance: See memory/performance.md
```

---

## 5. Settings Configuration

### Permissions (allow/deny)

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
  }
}
```

**Precedence**: deny > allow. Denies override allows.

### Context Auto-Compaction

```bash
# Control when compaction triggers (default 95%)
export CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=50

# Increase max output tokens (default 32K)
export CLAUDE_CODE_MAX_OUTPUT_TOKENS=48000
```

**Trade-off**: Higher output tokens = less context window for input.

---

## 6. Hooks Reference

### Hook Events

| Event | When | Use Case |
|-------|------|----------|
| `FileEdit` | File changed | Auto-format, lint |
| `BashCommand` | Before/after command | Validation, logging |
| `Commit` | Before git commit | Pre-commit checks |
| `Permission` | Permission needed | Auto-approve safe actions |
| `Notification` | Claude needs input | Desktop alerts |
| `SessionStart` | Session begins | Environment setup |

### Hook Structure

```json
{
  "hooks": {
    "EventType": [
      {
        "matcher": "glob-pattern",
        "hooks": [
          {
            "type": "command",
            "command": "shell command with {path} placeholder"
          }
        ]
      }
    ]
  }
}
```

### Common Hooks

**Auto-format on edit**:
```json
{
  "FileEdit": [
    {
      "matcher": "**/*.rs",
      "hooks": [{"type": "command", "command": "cargo fmt"}]
    },
    {
      "matcher": "**/*.{ts,tsx}",
      "hooks": [{"type": "command", "command": "prettier --write {path}"}]
    }
  ]
}
```

**Pre-commit validation**:
```json
{
  "Commit": [
    {
      "hooks": [
        {
          "type": "command",
          "command": "cargo clippy -- -D warnings && cargo test"
        }
      ]
    }
  ]
}
```

**Desktop notification**:
```json
{
  "Notification": [
    {
      "matcher": "",
      "hooks": [
        {
          "type": "command",
          "command": "osascript -e 'display notification \"Claude Code needs input\"'"
        }
      ]
    }
  ]
}
```

---

## 7. Directory Structure Template

### Large Monorepo Layout

```
your-monorepo/
├── CLAUDE.md (140 lines, with @imports)
├── README.md
├── ROADMAP.md
│
├── .claude/
│   ├── CLAUDE.md (optional, for root-specific rules)
│   ├── settings.json (permissions, hooks)
│   ├── settings.local.json (your personal settings)
│   ├── rules/
│   │   ├── general.md (applies everywhere)
│   │   ├── architecture.md (system overview)
│   │   ├── decisions.md (ADRs, imported)
│   │   ├── security.md (compliance)
│   │   ├── rust/
│   │   │   ├── safety.md
│   │   │   ├── async.md
│   │   │   └── testing.md
│   │   ├── typescript/
│   │   │   ├── naming.md
│   │   │   ├── imports.md
│   │   │   └── react.md
│   │   └── paths/
│   │       ├── rust-cli.md (paths: tools/**/*)
│   │       ├── typescript-core.md (paths: packages/**/*)
│   │       └── yaml-schema.md (paths: models/**/*)
│   └── agents/
│       └── my-agent.md (custom subagent)
│
├── packages/
│   ├── core/CLAUDE.md (optional, only if package-specific)
│   ├── studio/CLAUDE.md (optional)
│   └── db/CLAUDE.md (optional)
│
├── tools/
│   └── novanet/CLAUDE.md (optional, language-specific)
│
└── .gitignore
    ├── .claude/settings.local.json
    ├── CLAUDE.local.md
    └── .claude/.DS_Store
```

---

## 8. Common Mistakes & How to Avoid

### Mistake 1: Root CLAUDE.md Too Large

❌ **Bad**: 1500+ lines of inline content
✅ **Good**: 140 lines with imports pointing to `.claude/rules/`

```markdown
# Root CLAUDE.md (140 lines)

See @.claude/rules/architecture.md for system design.
See @.claude/rules/decisions.md for architectural decisions.
```

### Mistake 2: Loading Unnecessary Rules

❌ **Bad**: All rules apply to all developers
```
.claude/rules/
├── frontend-react.md
├── backend-api.md
└── database-sql.md
(All loaded for Rust developer!)
```

✅ **Good**: Path-specific rules
```markdown
---
paths:
  - "packages/studio/**/*.tsx"
---

# Only loads for React developers
```

### Mistake 3: Auto Memory Not Optimized

❌ **Bad**: MEMORY.md is 500 lines (but only first 200 load)
✅ **Good**: 200-line index + topic files
```
MEMORY.md (200 lines) → index
debugging.md → load on-demand
patterns.md → load on-demand
```

### Mistake 4: Duplicate Information

❌ **Bad**: Same rules in root CLAUDE.md AND `.claude/rules/general.md`
✅ **Good**: Put content in one place, import from root
```
Root: See @.claude/rules/general.md
```

### Mistake 5: Forgotten .gitignore

❌ **Bad**: Personal settings committed to git
✅ **Good**: `.claude/settings.local.json` in .gitignore
```
.claude/settings.local.json
CLAUDE.local.md
```

---

## 9. Quick Commands

### File Management

```bash
# View memory files in editor
/memory

# Initialize CLAUDE.md for project
/init

# Check loaded memory files
/memory (shows loaded files + paths)
```

### Environment Variables

```bash
# Force auto memory on/off
export CLAUDE_CODE_DISABLE_AUTO_MEMORY=0    # Force on
export CLAUDE_CODE_DISABLE_AUTO_MEMORY=1    # Force off

# Control auto-compaction
export CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=50   # Compact at 50%

# Load additional directory memories
export CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1

# Override max output tokens
export CLAUDE_CODE_MAX_OUTPUT_TOKENS=48000
```

### Settings Locations

```bash
# User memory (all projects)
~/.claude/CLAUDE.md

# Project memory (team shared)
./CLAUDE.md or ./.claude/CLAUDE.md

# Project rules
./.claude/rules/*.md

# Project settings (team)
./.claude/settings.json

# Project settings (personal)
./.claude/settings.local.json

# Auto memory (per project)
~/.claude/projects/<project>/memory/MEMORY.md
```

---

## 10. Checklist for New Project Setup

### Initial Setup (1 hour)

- [ ] Create `.claude/` directory
- [ ] Create `CLAUDE.md` (140 lines with imports)
- [ ] Create `.claude/rules/` directory structure
- [ ] Create `.claude/settings.json` (permissions + hooks)
- [ ] Create `.claude/settings.local.json` (personal, not committed)
- [ ] Add to `.gitignore`: `.claude/settings.local.json`, `CLAUDE.local.md`

### Rules Organization (2 hours)

- [ ] Create `.claude/rules/general.md` (applies everywhere)
- [ ] Create `.claude/rules/architecture.md` (system overview)
- [ ] Create language-specific rules: `.claude/rules/rust/`, `.claude/rules/typescript/`
- [ ] Create path-specific rules: `.claude/rules/paths/` with glob patterns
- [ ] Test that rules load correctly (use `/memory` to verify)

### Auto Memory Setup (1 hour)

- [ ] Optimize `~/.claude/projects/<project>/memory/MEMORY.md` (200 lines max)
- [ ] Create topic files: debugging.md, architecture.md, patterns.md
- [ ] Document what's in each topic file
- [ ] Test that topic files load on-demand

### Team Sharing (30 min)

- [ ] Commit `.claude/CLAUDE.md` to git
- [ ] Commit `.claude/rules/` to git
- [ ] Commit `.claude/settings.json` to git
- [ ] Keep `.claude/settings.local.json` and `CLAUDE.local.md` out of git
- [ ] Update README with memory strategy

---

## 11. Environment Variables Quick Reference

| Variable | Values | Effect |
|----------|--------|--------|
| `CLAUDE_CODE_DISABLE_AUTO_MEMORY` | 0/1 | Enable/disable auto memory during rollout |
| `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` | 1-100 | Trigger auto-compaction at % capacity |
| `CLAUDE_CODE_MAX_OUTPUT_TOKENS` | 1K-64K | Max tokens per response (default 32K) |
| `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD` | 0/1 | Load CLAUDE.md from --add-dir directories |
| `ENABLE_TOOL_SEARCH` | auto/true/false | MCP tool search (auto at 10%) |
| `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR` | 0/1 | Reset cd after each bash command |

---

## 12. Scalability Tiers

### Small Project (< 10K files)

```
CLAUDE.md (100 lines)
  └─ .claude/rules/ (3-5 files)
     └─ Auto memory (50 lines)
```

**Complexity**: Low, single CLAUDE.md sufficient

### Medium Project (10K-100K files, 2-5 packages)

```
CLAUDE.md (140 lines with imports)
  └─ .claude/rules/ (15-20 files)
     ├─ general.md
     ├─ language-specific/ (Rust, TypeScript, etc.)
     ├─ paths/ (path-specific rules)
     └─ Auto memory (200 lines + 3-5 topic files)
```

**Complexity**: Medium, hierarchical rules + path-specific loading

### Large Project (100K+ files, 5+ packages, 3+ languages)

```
CLAUDE.md (140 lines with imports)
  └─ .claude/rules/ (30+ files, organized by domain)
     ├─ general.md
     ├─ architecture.md
     ├─ decisions.md (imported)
     ├─ security.md
     ├─ language/
     │   ├─ rust/ (6+ files)
     │   ├─ typescript/ (6+ files)
     │   └─ yaml/ (3+ files)
     ├─ paths/ (per-package rules)
     └─ Auto memory (200 lines + 5-8 topic files)
   └─ Per-package CLAUDE.md (optional, only if complex)
```

**Complexity**: High, multiple layers + selective loading

---

## 13. Links & Resources

**Official Documentation**:
- Memory: https://code.claude.com/docs/en/memory
- Settings: https://code.claude.com/docs/en/settings
- Hooks: https://code.claude.com/docs/en/hooks-guide
- Best practices: https://code.claude.com/docs/en/best-practices

**Local Docs** (on your system):
- `/Users/thibaut/.claude-code-docs/docs/claude-code__memory.md`
- `/Users/thibaut/.claude-code-docs/docs/claude-code__settings.md`
- `/Users/thibaut/.claude-code-docs/docs/claude-code__hooks-guide.md`

---

## Conclusion

**Golden rules** for large project success:

1. **Root CLAUDE.md ≤ 140 lines** (use imports)
2. **Path-specific rules** (prevent unnecessary context)
3. **Auto memory** first 200 lines (ruthless optimization)
4. **Hooks for determinism** (format, lint automatically)
5. **Settings boundaries** (permissions, not hopes)

This enables Claude Code to scale from 1K to 100K+ file codebases with clean, focused context per session.
