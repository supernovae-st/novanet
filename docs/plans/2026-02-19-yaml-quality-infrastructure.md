# YAML Quality Infrastructure Plan

**Date:** 2026-02-19
**Status:** In Progress
**Scope:** Nika + NovaNet YAML validation, linting, hooks, skills

---

## Problem Statement

### Issues Encountered Today

1. **`for_each` format mismatch**: Documentation showed nested object format, implementation expected flat format
2. **6 workflows failed validation** due to inconsistent schema/implementation
3. **No JSON Schema exists** for IDE auto-completion and validation
4. **No yamllint config** for YAML style consistency
5. **No pre-commit hooks** to catch errors before commit
6. **No Claude Code skills** for authoring perfect YAML

### Root Causes

| Issue | Root Cause | Impact |
|-------|------------|--------|
| Schema drift | Docs updated, code not synced | 6 broken workflows |
| No static validation | Errors caught at runtime only | Lost dev time |
| No IDE support | No schema.json for YAML | Manual error-prone editing |
| Tribal knowledge | YAML patterns not documented | Inconsistent examples |

---

## Solution Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  YAML QUALITY INFRASTRUCTURE                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐           │
│  │  JSON Schema    │   │   yamllint      │   │  Rust Validator │           │
│  │  (IDE support)  │   │  (style check)  │   │  (semantic)     │           │
│  └────────┬────────┘   └────────┬────────┘   └────────┬────────┘           │
│           │                     │                     │                     │
│           └──────────┬──────────┴──────────┬──────────┘                     │
│                      ▼                     ▼                                │
│           ┌─────────────────────────────────────────┐                       │
│           │           Pre-commit Hooks              │                       │
│           │  - yamllint on *.nika.yaml             │                       │
│           │  - cargo run -- validate               │                       │
│           │  - schema validation                   │                       │
│           └────────────────┬────────────────────────┘                       │
│                            ▼                                                │
│           ┌─────────────────────────────────────────┐                       │
│           │         Claude Code Skills              │                       │
│           │  - /nika-yaml (workflow authoring)     │                       │
│           │  - /novanet-yaml (schema authoring)    │                       │
│           │  - /workflow-validate (check workflow) │                       │
│           └─────────────────────────────────────────┘                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: JSON Schema for Nika Workflows

### Deliverables

1. `nika-dev/tools/nika/schemas/nika-workflow.schema.json`
2. VS Code settings for auto-completion
3. Schema validation in CI

### Schema Structure

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://nika.dev/schemas/nika-workflow.schema.json",
  "title": "Nika Workflow",
  "description": "Schema for Nika YAML workflow files",
  "type": "object",
  "required": ["schema", "tasks"],
  "properties": {
    "schema": {
      "type": "string",
      "enum": ["nika/workflow@0.1", "nika/workflow@0.2", "nika/workflow@0.3"],
      "description": "Workflow schema version"
    },
    "workflow": { "type": "string" },
    "provider": { "enum": ["claude", "openai", "mock"] },
    "mcp": { "$ref": "#/$defs/McpConfig" },
    "tasks": { "type": "array", "items": { "$ref": "#/$defs/Task" } },
    "flows": { "type": "array", "items": { "$ref": "#/$defs/Flow" } }
  }
}
```

### Task Schema with for_each (CORRECT)

```json
{
  "$defs": {
    "Task": {
      "type": "object",
      "required": ["id"],
      "properties": {
        "id": { "type": "string" },
        "for_each": {
          "oneOf": [
            { "type": "array", "items": { "type": "string" } },
            { "type": "string", "pattern": "^\\{\\{.*\\}\\}$|^\\$" }
          ],
          "description": "Array or binding expression to iterate over"
        },
        "as": { "type": "string", "default": "item" },
        "concurrency": { "type": "integer", "minimum": 1, "default": 1 },
        "fail_fast": { "type": "boolean", "default": true }
      }
    }
  }
}
```

---

## Phase 2: yamllint Configuration

### Deliverables

1. `nika-dev/tools/nika/.yamllint.yaml`
2. CI integration

### Configuration

```yaml
extends: default

rules:
  line-length:
    max: 120
    level: warning
  indentation:
    spaces: 2
    indent-sequences: consistent
  truthy:
    allowed-values: ['true', 'false']
  document-start: disable
  comments:
    require-starting-space: true
    min-spaces-from-content: 1
```

---

## Phase 3: Pre-commit Hooks

### Deliverables

1. `.claude/hooks/yaml-validate.sh`
2. Hook configuration in `.claude/settings.json`

### Hook Logic

```bash
#!/bin/bash
# Hook: yaml-validate
# Runs on: PreToolUse for Write/Edit on *.nika.yaml

FILE="$1"

# 1. yamllint check
yamllint -c .yamllint.yaml "$FILE" || exit 1

# 2. JSON Schema validation (via ajv-cli or similar)
# ajv validate -s schemas/nika-workflow.schema.json -d "$FILE" || exit 1

# 3. Rust semantic validation
cargo run --quiet -- validate "$FILE" || exit 1

echo "✓ $FILE is valid"
```

---

## Phase 4: Claude Code Skills

### 4.1 /nika-yaml Skill

**Purpose:** Guide perfect Nika workflow authoring

**Location:** `nika-dev/tools/nika/.claude/skills/nika-yaml.md`

**Content highlights:**
- Schema version selection (0.1, 0.2, 0.3)
- 5 semantic verbs (infer, exec, fetch, invoke, agent)
- for_each FLAT format (not nested!)
- Binding syntax ({{use.alias}})
- MCP configuration patterns
- Common mistakes to avoid

### 4.2 /novanet-yaml Skill

**Purpose:** Guide perfect NovaNet node/arc YAML authoring

**Location:** `novanet-dev/tools/novanet/.claude/skills/novanet-yaml.md`

**Content highlights:**
- BLOC structure (6 BLOCs)
- llm_context dual pattern
- Trait selection rules
- Icon dual format
- Property ordering
- Relation patterns (A vs B)

### 4.3 /workflow-validate Skill

**Purpose:** Quick validation command

**Location:** `nika-dev/tools/nika/.claude/skills/workflow-validate.md`

---

## Phase 5: Sync and Updates

### Documentation Updates

| File | Update Needed |
|------|---------------|
| `CLAUDE.md` (nika) | ✅ Done - for_each flat format |
| `.claude/rules/nika.md` | ✅ Done - for_each flat format |
| `examples/README.md` | Add for_each examples |
| `ROADMAP.md` | Add this plan to Phase 8 |

### Schema-Code Sync Mechanism

```
YAML Schema (schema.json)
        ↓ generates
Rust Types (ast/workflow.rs)
        ↓ validates
Examples (examples/*.nika.yaml)
        ↓ tested by
Integration Tests
```

---

## Execution Order

| Step | Task | Effort | Priority |
|------|------|--------|----------|
| 1 | Create JSON Schema | 2h | HIGH |
| 2 | Create yamllint config | 30min | HIGH |
| 3 | Create /nika-yaml skill | 1h | HIGH |
| 4 | Create /novanet-yaml skill | 1h | MEDIUM |
| 5 | Create pre-commit hook | 1h | MEDIUM |
| 6 | Update VS Code settings | 15min | HIGH |
| 7 | Add v0.4 schema support | 30min | MEDIUM |
| 8 | Create /workflow-validate skill | 30min | LOW |

---

## Success Criteria

- [ ] JSON Schema validates all 28 example workflows
- [ ] yamllint passes on all *.nika.yaml files
- [ ] Pre-commit hook catches invalid YAML before commit
- [ ] /nika-yaml skill produces valid workflows
- [ ] No more for_each format confusion
- [ ] IDE auto-completion works in VS Code

---

## Related

- Plan B: Test coverage gaps
- Plan D: TUI verification
- MVP 8: RLM enhancements
- ADR-001: 5 Semantic Verbs
- ADR-002: YAML-First
