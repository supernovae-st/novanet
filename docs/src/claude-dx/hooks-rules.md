# Hooks & Rules Reference

NovaNet uses hooks for automation and rules for path-specific guidance.

## Hooks

Hooks are shell scripts that run at specific events.

### Directory Structure

```
.claude/hooks/
├── session-start.sh       # Runs on session start
├── post-edit-format.sh    # Runs after file edits
└── keybindings-reminder.sh # TUI file changes
```

### session-start.sh

**Event**: SessionStart

**Purpose**: Display project status on conversation start.

**Output**:
```
NovaNet v0.12.0 | Branch: main | Uncommitted: 3
```

### post-edit-format.sh

**Event**: PostToolUse (after Edit/Write)

**Purpose**: Auto-format edited files.

**Actions**:
- Rust files: Run `rustfmt`
- TypeScript files: Run `prettier`

### keybindings-reminder.sh

**Event**: PostToolUse (after editing TUI files)

**Purpose**: Remind to update KEYBINDINGS.md when modifying TUI keybindings.

**Trigger**: Files matching `tools/novanet/src/tui/*.rs`

## Rules

Rules provide path-specific instructions.

### Directory Structure

```
.claude/rules/
├── rust.md                # Rust-specific rules
├── typescript.md          # TypeScript-specific rules
├── cypher.md              # Cypher-specific rules
├── novanet-decisions.md   # ADR documentation
└── novanet-terminology.md # Terminology reference
```

### Path Scoping

Rules apply only to matching file paths:

```yaml
---
paths:
  - "tools/novanet/**/*.rs"
---
# Rules apply when working in these paths
```

### rust.md

**Scope**: `tools/novanet/**/*.rs`

**Key rules**:
- Edition 2024
- Zero clippy warnings
- Error handling with thiserror
- Documentation for public items

### typescript.md

**Scope**: `packages/**/*.ts`, `apps/**/*.ts`

**Key rules**:
- 2 spaces, 100 chars
- Single quotes, semicolons
- Zod for validation
- Explicit return types

### cypher.md

**Scope**: `packages/db/seed/**/*.cypher`

**Key rules**:
- UPPER_CASE keywords
- Consistent node aliases
- Use parameters, not string interpolation
- Comment complex queries

### novanet-decisions.md

**Scope**: Global

**Purpose**: Architecture Decision Records (ADRs).

**Content**:
- ADR-001 through ADR-022
- Decision rationale
- Historical context

### novanet-terminology.md

**Scope**: Global

**Purpose**: Canonical terminology reference.

**Content**:
- Graph vocabulary (Arc, not Edge)
- Node classification axes
- Deprecated terms

## Configuration

Hooks are configured in `.claude/settings.json`:

```json
{
  "hooks": {
    "sessionStart": {
      "command": "./hooks/session-start.sh"
    },
    "postToolUse": {
      "command": "./hooks/post-edit-format.sh",
      "tools": ["Edit", "Write"]
    }
  }
}
```

## Creating New Hooks

1. Create shell script in `.claude/hooks/`
2. Make executable: `chmod +x hooks/my-hook.sh`
3. Configure in `settings.json`
4. Test manually

## Creating New Rules

1. Create `.md` file in `.claude/rules/`
2. Add YAML frontmatter with paths
3. Write rules in markdown
4. Test by working in matching paths

## Best Practices

### Hooks

- Keep hooks fast (< 1 second)
- Log errors to stderr
- Return 0 on success
- Use non-blocking operations

### Rules

- Be specific about scope
- Prioritize actionable guidance
- Include examples
- Update when patterns change
