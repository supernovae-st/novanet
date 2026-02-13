# Skills Reference

NovaNet skills provide automatic context injection for Claude Code.

## Overview

Skills are stored in `.claude/skills/` and activate based on conversation context.

## Available Skills

### novanet-architecture

**Purpose**: Display schema architecture diagrams in ASCII.

**When activated**: User asks about architecture, system overview, component connections, meta-graph structure.

**Invocation**: `/novanet-arch`

**Output**: ASCII diagrams showing:
- Node taxonomy (Realm > Layer > Kind)
- Arc taxonomy (Family > ArcKind)
- Layer structure
- Realm organization

### novanet-sync

**Purpose**: Validate and regenerate artifacts from YAML sources.

**When activated**: YAML models change, sync validation fails, user asks about schema synchronization.

**Invocation**: `/novanet-sync`

**Actions**:
1. Validate YAML coherence
2. Check path/content consistency
3. Regenerate TypeScript, Cypher, Mermaid
4. Report changes

### novanet-tui

**Purpose**: Launch or explain the interactive TUI.

**When activated**: User wants to explore graph visually, asks about TUI keybindings, wants to navigate Realm/Layer/Kind hierarchy.

**Output**: TUI launch instructions and keybinding reference.

### codebase-audit

**Purpose**: Comprehensive codebase audit ("Ralph Wiggum Loop").

**When activated**: Before releases, after refactoring, periodic maintenance.

**Invocation**: `/codebase-audit`

**Actions**:
1. Dead code detection
2. Inconsistency scanning
3. Legacy pattern identification
4. Iterative fixing until clean

### token-audit

**Purpose**: Audit design token adoption across codebase.

**When activated**: Verify spacing tokens, find non-tokenized patterns, ensure design system consistency.

## Skill Structure

Each skill has a directory structure:

```
.claude/skills/novanet-architecture/
├── skill.md           # Main skill instructions
├── assets/            # Supporting files
│   └── diagrams/
└── examples/          # Example outputs
```

## Creating New Skills

1. Create directory in `.claude/skills/`
2. Add `skill.md` with:
   - Trigger conditions
   - Step-by-step instructions
   - Example outputs
3. Test with various prompts

## Skill Activation

Skills activate automatically when:
- User mentions related keywords
- File patterns match skill scope
- Explicit invocation via `/skill-name`

## Best Practices

- Keep skills focused on one task
- Include example outputs
- Document trigger conditions clearly
- Test activation with varied prompts
