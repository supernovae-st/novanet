# Claude Code Best Practices for Large Project Maintenance

**Research Date**: February 16, 2026
**Source**: Official Claude Code Documentation (270 docs)
**Scope**: Memory management, context optimization, path-specific rules, large monorepo patterns

---

## Executive Summary

Claude Code's memory system is hierarchical and composable, designed specifically to handle large projects without overwhelming context. By strategically organizing instructions into memory tiers, using path-specific rules with YAML frontmatter, and implementing hooks for deterministic automation, teams can maintain clean, focused context that scales to 100K+ file codebases.

**Key insight**: The first 200 lines of auto memory are loaded every session—optimize what goes there ruthlessly. Everything else loads on-demand or through targeted imports.

---

## 1. Memory Hierarchy (4-Level Architecture)

### 1.1 Four-Level Memory Precedence

Claude Code uses a **hierarchical memory system** where more specific memories override broader ones:

```
Level 4 (Highest)  ← Managed Policy (IT/DevOps, organization-wide)
                     ↓
Level 3            ← Project Memory (./.claude/CLAUDE.md, ./.claude/rules/*.md)
                     ↓
Level 2            ← User Memory (~/.claude/CLAUDE.md)
                     ↓
Level 1 (Lowest)   ← Auto Memory (~/.claude/projects/<project>/memory/MEMORY.md)
```

**Behavior**:
- All CLAUDE.md files in parent directories are loaded **in full** at session launch
- Child directory CLAUDE.md files load **on demand** (only when reading those files)
- Managed policies are **immutable** by users
- More specific rules take precedence (project > user > auto)

### 1.2 Memory Types & Storage Locations

| Type | Location | Loaded | Scope | Use Case |
|------|----------|--------|-------|----------|
| **Managed Policy** | `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS)<br>`/etc/claude-code/CLAUDE.md` (Linux)<br>`C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) | Full | All users | Company standards, security, compliance |
| **Project Memory** | `./CLAUDE.md` or `./.claude/CLAUDE.md` | Full | Team (git) | Architecture, team workflows, conventions |
| **Project Rules** | `./.claude/rules/*.md` | Full | Team (git) | Language-specific, testing, API standards |
| **User Memory** | `~/.claude/CLAUDE.md` | Full | Individual | Personal preferences, all projects |
| **Local Project** | `./CLAUDE.local.md` | On-demand | You (not committed) | Sandbox URLs, test data |
| **Auto Memory** | `~/.claude/projects/<project>/memory/MEMORY.md` | First 200 lines | You (per project) | Patterns, debugging insights |

**Critical constraint**: Auto memory loads only **first 200 lines** into system prompt every session. Everything else requires on-demand reads.

### 1.3 Directory Hierarchy & Recursive Loading

```
your-project/
├── CLAUDE.md                      # Full load (parent dir)
├── .claude/
│   ├── CLAUDE.md                  # Full load (main project rules)
│   ├── rules/
│   │   ├── code-style.md          # Full load (unconditional)
│   │   ├── testing.md             # Full load (unconditional)
│   │   ├── security.md            # Full load (unconditional)
│   │   ├── api-routes/
│   │   │   └── auth.md            # Full load (recursive discovery)
│   │   └── path-specific/
│   │       └── frontend-rules.md  # Full load (with paths: frontmatter)
│   └── settings.local.json        # Your project-specific perms
│
├── packages/
│   ├── core/
│   │   └── CLAUDE.md              # On-demand (child directory)
│   └── db/
│       └── CLAUDE.md              # On-demand (child directory)
│
└── tools/novanet/
    └── CLAUDE.md                  # On-demand (child directory)
```

**Key behavior**:
- Parent directory CLAUDE.md files: **loaded in full at launch**
- Child directory CLAUDE.md files: **loaded on-demand** when reading files in that subtree
- `.claude/rules/` files: **always discovered** recursively (symlinks supported)
- Auto memory: **first 200 lines only** at launch, topic files on-demand

---

## 2. Path-Specific Rules with YAML Frontmatter

### 2.1 Conditional Rules Architecture

Rules can be scoped to specific file paths using YAML frontmatter with the `paths` field:

```markdown
---
paths:
  - "src/api/**/*.ts"
  - "src/**/*.test.ts"
---

# API Development Rules

- All endpoints must validate input
- Use standard error response format
- Include OpenAPI documentation comments
```

**Key points**:
- Rules **without** `paths` field apply to **all files** (unconditional)
- Rules **with** `paths` field apply **only** to matching files
- Glob patterns supported: `**/*.ts`, `src/**/*`, `*.md`, `{src,lib}/**/*.ts`
- Brace expansion: `src/**/*.{ts,tsx}` expands to match both `.ts` and `.tsx`
- Multiple patterns allowed (OR logic)
- Evaluated **only** outside markdown code blocks (safe in examples)

### 2.2 Example: NovaNet Path-Specific Rules

```markdown
---
paths:
  - "packages/core/src/**/*.ts"
  - "packages/studio/src/**/*.{ts,tsx}"
---

# TypeScript/React Rules

## Naming Conventions
- PascalCase for components and types
- camelCase for functions and variables
- UPPER_SNAKE_CASE for constants

## Imports
- Use absolute imports with ~/ prefix
- Group imports: external → internal → components

---
paths:
  - "tools/novanet/src/**/*.rs"
---

# Rust Conventions

## Safety
- Use `thiserror` for error types
- Prefer `?` operator over `match`
- Document safety invariants with SAFETY comments

## Testing
- Unit tests in same file (mod tests { })
- Integration tests in tests/ directory
- Property tests with proptest for complex logic
```

### 2.3 Subdirectory Organization

Rules can be organized hierarchically in `.claude/rules/`:

```
.claude/rules/
├── general.md                # Applies to all files
├── frontend/
│   ├── react.md             # Only src/**/*.{ts,tsx}
│   └── styles.md            # Only src/styles/**
├── backend/
│   ├── api.md               # Only src/api/**
│   └── database.md          # Only src/db/**
└── shared/                  # Symlink to ~/shared-claude-rules
```

All `.md` files discovered recursively, including subdirectories and symlinks.

---

## 3. Auto Memory: What Claude Writes for Itself

### 3.1 Auto Memory Purpose & Structure

Auto memory is what **Claude writes for itself** during sessions—not instructions you author. It's automatically saved to:

```
~/.claude/projects/<project>/memory/
├── MEMORY.md              # Index (200 lines loaded every session)
├── debugging.md           # Detailed debugging patterns
├── architecture.md        # Key files and module relationships
├── patterns.md            # Build commands, test conventions
└── ...                    # Topic files (on-demand)
```

**Git worktrees**: Each worktree gets **separate** auto memory (derived from git root).

### 3.2 What Claude Saves to Auto Memory

As Claude works, it might save:
- **Project patterns**: build commands, test conventions, code style preferences
- **Debugging insights**: solutions to tricky problems, common error patterns
- **Architecture notes**: key files, module relationships, important abstractions
- **Your preferences**: communication style, workflow habits, tool choices

### 3.3 Optimizing Auto Memory (200-Line Constraint)

Since only **first 200 lines** of `MEMORY.md` load into every session:

```markdown
# NovaNet Memory Index

## Quick Facts
- **Monorepo**: Turborepo with pnpm workspaces
- **Languages**: Rust (CLI/TUI), TypeScript (Studio), YAML (schema)
- **Build**: `cargo build` (Rust), `pnpm build` (TypeScript)
- **Test**: `cargo test` (1000+ tests), `pnpm test`

## Architecture Layers
- SHARED realm: 4 layers, 40 nodes (config, locale, geography, knowledge)
- ORG realm: 6 layers, 21 nodes (config, foundation, structure, semantic, instruction, output)
- Total: 61 nodes, 169 arcs, 10 layers
- Source of truth: YAML in `packages/core/models/`

## Key Commands
- Schema: `cargo run -- schema generate` (12 artifacts)
- Validate: `cargo run -- schema validate`
- TUI: `cargo run -- tui` (unified tree mode)
- Tests: `cargo test` (zero clippy warnings)

## Critical Files
- `.claude/rules/novanet-decisions.md` → 30 ADRs (architecture decisions)
- `.claude/rules/novanet-terminology.md` → canonical terms (Class, Arc, Trait)
- `packages/core/models/` → YAML source of truth
- `tools/novanet/src/` → Rust binary (980 tests)

## Current Focus Areas
- v0.13.0 Native Pattern (ADR-029/030)
- Slug Ownership architecture
- SEO Pillar/Cluster system

See detailed docs in `.claude/` for full context.
```

**Strategy**: MEMORY.md is an **index**. Keep it concise (100-200 lines). Detailed notes go in topic files that Claude reads on-demand.

### 3.4 Asking Claude to Save to Memory

Direct instruction:
```
> remember that we use pnpm, not npm
> save to memory that tests require Node 18+
> add to memory that deployments need $SECURE_KEY env var
```

---

## 4. File Organization Best Practices

### 4.1 Monorepo Memory Strategy

For large monorepos, organize memory hierarchically:

```
monorepo/
├── .claude/
│   ├── CLAUDE.md                    # Root rules (140 lines max)
│   ├── rules/
│   │   ├── general.md               # Applies everywhere
│   │   ├── architecture.md          # System overview
│   │   ├── terminology.md           # Canonical terms
│   │   ├── decisions.md             # 30+ ADRs (import from source)
│   │   ├── security.md              # Company policy
│   │   ├── typescript/
│   │   │   ├── naming.md            # TS naming conventions
│   │   │   ├── testing.md           # Test patterns
│   │   │   └── dependencies.md      # Approved libraries
│   │   ├── rust/
│   │   │   ├── safety.md            # Unsafe code standards
│   │   │   ├── error-handling.md    # thiserror patterns
│   │   │   └── performance.md       # Optimization guidelines
│   │   └── paths/
│   │       ├── frontend-react.md    # src/**/*.{ts,tsx}
│   │       ├── api-routes.md        # src/api/**
│   │       └── database-layer.md    # src/db/**
│   ├── settings.json                # Permissions, hooks, tools
│   └── agents/                      # Custom subagents (YAML)
│
├── packages/
│   ├── core/
│   │   └── CLAUDE.md                # (Optional) Package-specific rules
│   ├── db/
│   │   └── CLAUDE.md                # (Optional) Database layer rules
│   └── studio/
│       └── CLAUDE.md                # (Optional) Frontend-specific
│
└── tools/novanet/
    └── CLAUDE.md                    # Rust CLI-specific rules
```

### 4.2 Root CLAUDE.md (140 Lines Target)

Keep root `CLAUDE.md` focused and concise:

```markdown
# NovaNet Development Guide

## Quick Start
- **Monorepo**: Turborepo (pnpm workspaces)
- **Languages**: Rust, TypeScript, YAML
- **Build**: `pnpm build && cargo build`
- **Test**: `pnpm test && cargo test`

## Architecture
See @.claude/rules/architecture.md for system overview.

## Terminology
All technical terms defined in @.claude/rules/terminology.md.

## Decisions & ADRs
See @.claude/rules/decisions.md (30+ architecture decisions).

## Import Code Style Rules
See @.claude/rules/typescript/ and @.claude/rules/rust/ for language-specific conventions.

## Current Project Status
- **Version**: v0.13.0
- **Focus**: Native Pattern (ADR-029/030)
- **Tests**: 1000+ passing, zero clippy warnings
```

Use **imports** (`@path/to/file`) to keep root file small while composing detailed rules.

### 4.3 CLAUDE.md Imports

Import related files to keep main file focused:

```markdown
See @README for project overview.
See @ROADMAP.md for release planning.
See @CHANGELOG.md for version history.

# Development Workflow
See @.claude/rules/architecture.md for system design.
See @.claude/rules/decisions.md for 30+ ADRs.

# Code Standards
See @.claude/rules/typescript/ for TS/React patterns.
See @.claude/rules/rust/ for Rust conventions.
```

**Behavior**:
- Relative paths resolve **relative to importing file**
- Absolute paths supported: `@~/shared-project-rules/api.md`
- Max import depth: 5 hops (circular detection prevents infinite loops)
- First import approval: one-time dialog per project

### 4.4 CLAUDE.local.md (Personal Preferences)

For personal project-specific settings:

```markdown
# Personal Sandbox

- Local test URLs: http://localhost:3000
- Preferred test database: dev_sandbox
- Favorite breakpoint: src/tui/app.rs:142

## Personal Workflow
- Commit style: Conventional Commits (type(scope): description)
- Test command: `cargo nextest run` (parallel, faster)
- Preferred diff format: `git diff --unified=5`
```

**Key**: Automatically added to `.gitignore`—never committed.

---

## 5. Context Optimization Strategies

### 5.1 Large Project Context Management

Claude Code has **context auto-compaction** when context approaches capacity:

```
Context Usage Over Time:
Start (10K tokens)
    ↓
Working... (adding files, reasoning)
    ↓
95% full (auto-compaction triggers)
    ↓
Context compacted: saves summaries, drops old context
    ↓
Continue working with fresh capacity
```

**Control auto-compaction**:
```bash
# Trigger compaction at 50% instead of 95%
export CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=50

# Increase max output tokens (reduces effective context window)
export CLAUDE_CODE_MAX_OUTPUT_TOKENS=48000
```

### 5.2 Memory File Size Recommendations

**Best practices**:

| File Type | Recommended Max | Notes |
|-----------|-----------------|-------|
| Root `CLAUDE.md` | 140 lines | Keep concise, use imports |
| Path-specific rules | 80 lines each | Focused on one concern |
| Auto memory `MEMORY.md` | 200 lines | Only these loaded at launch |
| Topic memory files | 500+ lines | Load on-demand as needed |
| `.claude/rules/` files | Unlimited | Recursive discovery, all loaded |

**Rule of thumb**: If a rules file exceeds 150 lines, split it into focused topic files.

### 5.3 Reducing Token Usage

**Strategies**:

1. **Selective imports**: Only import files needed for task
   ```markdown
   # API Development
   See @.claude/rules/typescript/api.md for endpoint patterns.
   ```

2. **Path-specific rules**: Use `paths:` frontmatter to avoid loading unnecessary rules
   ```markdown
   ---
   paths:
     - "src/api/**/*.ts"
   ---
   # Only loaded when working with API files
   ```

3. **Auto memory on-demand**: Keep MEMORY.md index small, move details to topic files
   ```bash
   # Topic files loaded only when needed
   ~/.claude/projects/<project>/memory/
   ├── MEMORY.md        (200 lines loaded every session)
   ├── debugging.md     (loaded on demand)
   └── patterns.md      (loaded on demand)
   ```

4. **Monorepo context boundaries**: Separate CLAUDE.md files per package
   ```
   packages/core/CLAUDE.md        (loaded when working in packages/core)
   packages/studio/CLAUDE.md      (loaded when working in packages/studio)
   tools/novanet/CLAUDE.md        (loaded when working in tools/novanet)
   ```

---

## 6. Settings & Permissions for Large Projects

### 6.1 Project-Level Settings Structure

```json
{
  ".claude/settings.json": {
    "description": "Shared team settings (committed to git)",
    "scope": "all team members",

    "permissions": {
      "allow": [
        "Bash(pnpm *)",
        "Bash(cargo *)",
        "Bash(git *)"
      ],
      "deny": [
        "Bash(rm -rf /)",
        "Bash(sudo *)"
      ]
    },

    "tools": {
      "enableAllProjectMcpServers": true,
      "enabledMcpjsonServers": ["github", "memory"]
    },

    "contextWindow": {
      "autoCompactPercentage": 85
    }
  },

  ".claude/settings.local.json": {
    "description": "Personal project settings (not committed)",
    "scope": "you only",

    "permissions": {
      "allow": [
        "Bash(curl http://localhost:*)"
      ]
    }
  }
}
```

**Precedence**:
1. Local settings (highest priority)
2. Project settings
3. User settings (`~/.claude/settings.json`)
4. Managed policy (lowest priority, immutable)

### 6.2 MCP Server Configuration

For large projects with external tools:

```json
{
  ".mcp.json": {
    "mcpServers": {
      "github": {
        "command": "npx",
        "args": ["@modelcontextprotocol/server-github"],
        "env": {
          "GITHUB_PERSONAL_ACCESS_TOKEN": "ghp_..."
        }
      },
      "memory": {
        "command": "npx",
        "args": ["@modelcontextprotocol/server-memory"]
      },
      "custom-tools": {
        "command": "python3",
        "args": ["/Users/thibaut/tools/mcp-server.py"]
      }
    }
  }
}
```

**Tool search**: Set `ENABLE_TOOL_SEARCH=auto` to scale MCP at 10% context (custom threshold: `auto:5` for 5%).

---

## 7. Hooks for Deterministic Automation

### 7.1 Hook Lifecycle Events

Hooks run at key points in Claude Code's lifecycle:

| Event | Triggers When | Use Case |
|-------|---------------|----------|
| `FileEdit` | File changed | Auto-format, lint, validate |
| `BashCommand` | Before/after command | Block unsafe commands, log executions |
| `Commit` | Before git commit | Validate messages, run pre-commit |
| `Permission` | Permission prompt | Auto-approve safe actions |
| `Notification` | Claude needs input | Desktop notifications |
| `SessionStart` | Session begins | Inject environment, welcome message |

### 7.2 Example: Pre-Commit Hook

```json
{
  "hooks": {
    "Commit": [
      {
        "type": "command",
        "command": "cargo fmt && cargo clippy -- -D warnings"
      }
    ]
  }
}
```

Run before every commit to enforce code quality.

### 7.3 Example: Auto-Format Hook

```json
{
  "hooks": {
    "FileEdit": [
      {
        "matcher": "**/*.rs",
        "hooks": [
          {
            "type": "command",
            "command": "rustfmt {path}"
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
      }
    ]
  }
}
```

Auto-format TypeScript, Rust, and other files after edits.

---

## 8. Large Project Patterns

### 8.1 NovaNet Case Study

**Project structure**:
- **41 CLAUDE.md files** (root + per-package rules)
- **7 major rule documents** (decisions, terminology, security)
- **3 Rust packages** (1000+ tests)
- **2 TypeScript packages** (Studio, core)
- **1 YAML schema** (61 nodes, 169 arcs)

**Memory strategy**:
```
.claude/
├── CLAUDE.md (140 lines)
│   └── Imports: README, ROADMAP, architecture, decisions
├── rules/
│   ├── novanet-decisions.md (30+ ADRs, imported)
│   ├── novanet-terminology.md (canonical terms)
│   ├── arc-design-guide.md (arc best practices)
│   ├── security.md (compliance)
│   ├── rust/
│   │   ├── safety.md (unsafe code)
│   │   ├── async.md (tokio patterns)
│   │   └── testing.md (proptest, insta)
│   ├── typescript/
│   │   ├── naming.md (PascalCase, camelCase)
│   │   ├── imports.md (absolute paths)
│   │   └── react.md (hooks, composition)
│   └── paths/
│       ├── rust-cli.md (paths: tools/novanet/**/*.rs)
│       ├── typescript-core.md (paths: packages/core/**/*.ts)
│       └── react-studio.md (paths: packages/studio/**/*.tsx)
└── settings.json
    └── Permissions: pnpm, cargo, git only
    └── Hooks: cargo fmt, clippy, tests
```

**Auto memory strategy**:
```
~/.claude/projects/novanet-hq/memory/
├── MEMORY.md (200 lines)
│   └── Quick facts, architecture summary, key commands
├── debugging.md (on-demand)
│   └── Common error patterns (Neo4j, TypeScript)
├── architecture.md (on-demand)
│   └── Key files, module relationships
└── performance.md (on-demand)
    └── Optimization notes, bottlenecks
```

### 8.2 When to Use Sub-Project CLAUDE.md Files

Add package-specific rules only when:

✅ **DO**: Language-specific patterns
```
packages/studio/CLAUDE.md  → React hooks, TypeScript patterns
tools/novanet/CLAUDE.md    → Rust safety, async/await patterns
```

❌ **DON'T**: Duplicate team-level rules
```
# Instead of repeating in each package:
packages/core/CLAUDE.md (repetitive code style rules)

# Use path-specific rules:
.claude/rules/paths/typescript.md
---
paths:
  - "packages/**/*.ts"
---
# Code style rules
```

---

## 9. Quick Reference Checklist

### For Initial Project Setup

- [ ] Create `.claude/CLAUDE.md` (140 lines max, with imports)
- [ ] Create `.claude/rules/` directory structure (architecture, decisions, terminology)
- [ ] Create `.claude/settings.json` with team permissions
- [ ] Create `.claude/settings.local.json` with personal permissions
- [ ] Set up path-specific rules in `.claude/rules/paths/`
- [ ] Configure hooks for auto-format and testing
- [ ] Document auto memory strategy in `~/.claude/projects/<project>/memory/MEMORY.md`

### For Maintenance

- [ ] Monthly review: Keep MEMORY.md first 200 lines optimized
- [ ] Move detailed notes to topic files when MEMORY.md exceeds 200 lines
- [ ] Update rules when architecture changes
- [ ] Prune unused path-specific rules
- [ ] Review hooks quarterly (still needed?)
- [ ] Monitor context usage: adjust `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` if compacting too early

### For Team Collaboration

- [ ] Check that all rules are in `.claude/rules/` (not scattered)
- [ ] Use imports in root CLAUDE.md instead of duplicating content
- [ ] Document paths for path-specific rules (which files do they apply to?)
- [ ] Keep `.claude/settings.json` and `.claude/rules/` in version control
- [ ] Keep `.claude/settings.local.json` and `.CLAUDE.local.md` out of git (.gitignore)

---

## 10. References & Further Reading

**Official documentation**:
- [Memory Management](/en/memory) - Full memory system docs
- [Settings](/en/settings) - All settings options
- [Hooks Guide](/en/hooks-guide) - Hook events and examples
- [Common Workflows](/en/common-workflows) - Real-world patterns
- [CLI Reference](/en/cli-reference) - Terminal commands

**Claude Code surfaces**:
- Terminal: Full-featured CLI for scripting and automation
- VS Code: IDE integration with inline diffs
- Desktop: Visual diff review, side-by-side sessions
- Web: Remote execution, long-running tasks
- JetBrains: Plugin support for IntelliJ, PyCharm, WebStorm

**Related tools**:
- [MCP (Model Context Protocol)](/en/mcp) - Connect external data sources
- [Agent SDK](https://platform.claude.com/docs/agent-sdk/overview) - Build custom agents
- [Skills](/en/skills) - Custom slash commands for teams
- [Sub-agents](/en/sub-agents) - Parallel task execution

---

## Conclusion

Large project success with Claude Code depends on **strategic memory organization**:

1. **Root CLAUDE.md** stays lean (140 lines) with imports pointing to detailed rules
2. **Path-specific rules** in `.claude/rules/paths/` prevent loading unnecessary context
3. **Auto memory** optimized: 200-line index + topic files for on-demand loading
4. **Hooks** enforce deterministic automation (no guessing)
5. **Settings** create permission boundaries between local and team-level configurations

This architecture scales from 1K to 100K+ file codebases while keeping per-session context clean and focused.
