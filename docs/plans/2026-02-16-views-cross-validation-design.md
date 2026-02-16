# Views Cross-Validation Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Guarantee TUI (Rust) and Studio (TypeScript) interpret views.yaml identically.

**Architecture:** Shell script orchestrates cross-validation by comparing JSON exports from both parsers.

**Tech Stack:** Rust (novanet CLI), TypeScript (export script), Shell (orchestrator), SHA256 (cypher hashing)

---

## Context

- **Source of truth:** `packages/core/models/views.yaml` (11 views)
- **Consumers:**
  - TUI (Rust): `parsers/views.rs::load_simple_views()`
  - Studio (TS): `filters/ViewLoader.ts::loadRegistry()`
- **Problem:** No validation that both parsers produce identical interpretation
- **Risk:** Divergent behavior between TUI and Studio

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        views.yaml                                   │
│                    (source de vérité)                               │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
         ┌─────────────────┴─────────────────┐
         ▼                                   ▼
┌─────────────────┐                 ┌─────────────────┐
│  Rust Parser    │                 │   TS Parser     │
│  (novanet CLI)  │                 │ (export-views)  │
└────────┬────────┘                 └────────┬────────┘
         │                                   │
         ▼                                   ▼
    views.json                          views.json
         │                                   │
         └─────────────┬─────────────────────┘
                       ▼
            ┌─────────────────────┐
            │  validate-views.sh  │
            │   (compare JSON)    │
            └──────────┬──────────┘
                       │
         ┌─────────────┼─────────────┐
         ▼             ▼             ▼
      CI/CD          CLI           Hook
```

## Files to Create

| File | Purpose |
|------|---------|
| `tools/novanet/src/commands/views.rs` | `novanet views export --format=json` |
| `packages/core/scripts/export-views.mjs` | TS export script, same JSON format |
| `tools/scripts/validate-views.sh` | Compare JSON outputs, exit 0/1 |
| `.claude/hooks/views-sync-reminder.sh` | Trigger on views.yaml change |

## Canonical JSON Format

Both parsers must produce this exact format:

```json
{
  "version": "0.12.5",
  "count": 11,
  "views": [
    {
      "id": "data-complete",
      "name": "All Instances",
      "description": "All data instances (non-schema)",
      "category": "data",
      "icon": { "web": "database", "terminal": "●" },
      "color": "#6366f1",
      "root_type": null,
      "contextual": false,
      "applicable_types": [],
      "cypher_hash": "a1b2c3d4"
    }
  ]
}
```

**Canonicalization rules:**
- Views sorted by `id` (alphabetically)
- JSON keys sorted alphabetically
- `cypher_hash`: SHA256 first 8 chars (avoids whitespace diff)
- Explicit `null` (not omitted)
- Explicit empty arrays `[]`

## Integration Points

### 1. CLI Usage

```bash
$ novanet views validate
✓ Rust parsed 11 views
✓ TypeScript parsed 11 views
✓ All views match

$ novanet views validate --verbose
✓ data-complete: match
✓ schema-complete: match
...
```

### 2. CI/CD (GitHub Actions)

```yaml
- name: Validate views sync
  run: ./tools/scripts/validate-views.sh
```

### 3. Claude Hook

```bash
# .claude/hooks/views-sync-reminder.sh
# Trigger: packages/core/models/views.yaml modified
# Action: Reminder to run validation
```

### 4. Slash Command

`/novanet-sync validate` will include `novanet views validate`

## Validated Fields (11 per view)

| Field | Type | Purpose |
|-------|------|---------|
| `id` | string | Unique identifier |
| `name` | string | Display name |
| `description` | string | Help text |
| `category` | string | Grouping (schema/data/generation/contextual) |
| `icon.web` | string | Lucide icon name |
| `icon.terminal` | string | Unicode symbol |
| `color` | string | Hex color code |
| `root_type` | string\|null | Required root node type |
| `contextual` | boolean | Requires node selection |
| `applicable_types` | string[] | Valid node types for contextual |
| `cypher_hash` | string | SHA256 of cypher query |

## Success Criteria

- [ ] `novanet views validate` passes
- [ ] CI fails if views diverge
- [ ] Hook reminds on views.yaml changes
- [ ] Both parsers produce byte-identical JSON
