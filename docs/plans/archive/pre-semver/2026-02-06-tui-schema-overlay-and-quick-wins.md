# TUI Schema Overlay + Quick Wins

**Date**: 2026-02-06
**Status**: Approved
**Author**: Thibaut + Claude

## Overview

Enhance the TUI middle panel to show the relationship between Meta (schema) and Data (instance) more clearly, plus 5 additional UX improvements.

## Features

### 1. Schema Overlay in Data Mode (Core)

**Problem**: When viewing an instance, only filled properties are shown. User can't see which schema fields are missing.

**Solution**: In Data mode, show ALL schema properties with status indicators:

```
┌─ af-ZA Culture Norms (Culture) ────────────────────────── [s:schema] ───┐
│ type     Instance                                                       │
│ key      af-ZA                                                          │
│ kind     Culture                                                        │
│                                                                         │
│ Properties (14/19) ━━━━━━━━━━━━━━░░░░░ 74%   ⚠ 2 required missing       │
│ ─────────────────────────────────────────────────────────────────────── │
│ *key                  "af-ZA"                                           │
│ *display_name         "af-ZA Culture Norms"                             │
│ *description          "Cultural context and norms for af-ZA"            │
│ *hemisphere           "southern"                                        │
│  business_hours       "{"end":"17:00","lunch_break":true..."            │
│  culture_summary      —  (string) e.g. "Warm, ubuntu-driven..."  (dim)  │
│  seasonal_greetings   —  (string) e.g. "Happy Heritage Day"      (dim)  │
│ *gift_taboos          ⚠  (string) e.g. "No white flowers"        (red)  │
│ *religious_refs       ⚠  (string) e.g. "Christian majority"      (red)  │
└─────────────────────────────────────────────────────────────────────────┘
```

**Encoding**:
- Bright text = value present
- Dim gray + `—  (type) e.g. "example"` = optional, empty
- Red + `⚠  (type) e.g. "example"` = required, missing

**Data sources**:
- Schema properties: from Kind's YAML `standard_properties` + `properties`
- Instance values: from Neo4j query
- Examples: from YAML `example` field

### 2. Tree Health Badges

**Problem**: No way to see data quality at a glance in the tree.

**Solution**: Add coverage % and warning count to Kind nodes:

```
Before:  Culture →200
After:   Culture →200 ━░ 87% ⚠12
```

- Mini progress bar (2-3 chars): `━━` filled, `░░` empty
- Warning count: instances with required fields missing

### 3. Truncate Intelligent

**Problem**: Long values are cut with "..." but no way to see full value.

**Solution**:
- Normal: `business_hours "{"end":"17:00","lun..."`
- When property is focused/selected: expand to show full value (wrap or scroll)

### 4. JSON Pretty-Print Toggle [j]

**Problem**: JSON values are hard to read in compact form.

**Solution**: Press `j` to toggle between:
- Compact: `"{"end":"17:00","lunch_break":true}"`
- Pretty:
  ```json
  {
    "end": "17:00",
    "lunch_break": true
  }
  ```

### 5. Yank to Clipboard [y]

**Problem**: Can't easily copy values from TUI.

**Solution**: Vim-style yank:
- Focus on a property → press `y` → value copied to clipboard
- Status message: "Yanked: af-ZA"

### 6. Audit Mode [6]

**Problem**: No global view of data quality across all Kinds.

**Solution**: New mode accessible via `6`:

```
┌─ Data Quality Audit ───────────────────────────────────────────────────┐
│                                                                        │
│ Global Coverage: 84%  ━━━━━━━━━━━━━━━░░░  Total Issues: 116            │
│                                                                        │
│ Kind            Instances   Coverage              Issues               │
│ ────────────────────────────────────────────────────────────────────── │
│ Culture         200         ━━━━━━━━━━━━━░░░ 87%  ⚠ 12 missing        │
│ Locale          200         ━━━━━━━━━━━━━━━━ 100% ✓ complete          │
│ Term            4521        ━━━━━━━━━━━━░░░░ 76%  ⚠ 89 missing        │
│ Entity          342         ━━━━━━━━━━━━━━░░ 92%  ⚠ 3 missing         │
│ Expression      1205        ━━━━━━━━━━━━━░░░ 85%  ⚠ 12 missing        │
│ ...                                                                    │
│                                                                        │
│ [Enter] Drill down  [r] Refresh  [e] Export CSV                        │
└────────────────────────────────────────────────────────────────────────┘
```

Drill-down: Select a Kind → see list of instances with issues.

## Implementation Plan

### Phase 1: Data Structures (shared by all features)

```
src/tui/
├── schema.rs     (NEW) — Schema property loader + matcher
└── audit.rs      (NEW) — Audit calculations
```

**schema.rs**:
```rust
/// Property from YAML schema
pub struct SchemaProperty {
    pub name: String,
    pub prop_type: String,      // "string", "json", "enum", "datetime"
    pub required: bool,
    pub example: Option<String>,
    pub description: Option<String>,
    pub enum_values: Option<Vec<String>>,
}

/// Matched property (schema + instance value)
pub struct MatchedProperty {
    pub schema: SchemaProperty,
    pub value: Option<String>,   // None = missing
    pub status: PropertyStatus,
}

pub enum PropertyStatus {
    Filled,           // Has value
    EmptyOptional,    // No value, not required
    MissingRequired,  // No value, required
}

/// Load schema properties from YAML
pub fn load_schema_properties(yaml_path: &str) -> Vec<SchemaProperty>;

/// Match instance properties against schema
pub fn match_properties(
    schema: &[SchemaProperty],
    instance_props: &HashMap<String, String>,
) -> Vec<MatchedProperty>;
```

**audit.rs**:
```rust
/// Audit stats for a Kind
pub struct KindAuditStats {
    pub kind_key: String,
    pub instance_count: usize,
    pub total_properties: usize,
    pub filled_properties: usize,
    pub missing_required: usize,
    pub coverage_percent: u8,
}

/// Calculate audit stats (requires Neo4j query)
pub async fn calculate_kind_stats(db: &Db, kind_key: &str) -> KindAuditStats;
```

### Phase 2: Schema Overlay (Feature 1)

**Files**: `app.rs`, `ui/mod.rs`, `data.rs`

1. **app.rs**: Add state
   ```rust
   pub schema_overlay_enabled: bool,  // Toggle with 's'
   pub matched_properties: Option<Vec<MatchedProperty>>,
   ```

2. **data.rs**: Add query for instance properties
   ```rust
   pub async fn load_instance_properties(db: &Db, key: &str) -> HashMap<String, String>;
   ```

3. **ui/mod.rs**: Modify `render_detail_panel`
   - If Data mode + Instance selected + schema loaded:
     - Render `MatchedProperty` list instead of raw properties
     - Show progress bar header
     - Color-code by status

### Phase 3: Tree Health Badges (Feature 2)

**Files**: `data.rs`, `ui/mod.rs`

1. **data.rs**: Add to `KindInfo`
   ```rust
   pub struct KindInfo {
       // existing...
       pub health_percent: Option<u8>,      // 0-100
       pub issues_count: Option<usize>,     // Required missing
   }
   ```

2. **data.rs**: Query to calculate health (batch for all Kinds)
   ```cypher
   MATCH (k:Kind:Meta)
   OPTIONAL MATCH (k)<-[:INSTANCE_OF]-(i)
   // ... calculate coverage
   RETURN k.key, health, issues
   ```

3. **ui/mod.rs**: Modify `render_tree_item` for Kind nodes
   ```
   Culture →200 ━░ 87% ⚠12
   ```

### Phase 4: Truncate Intelligent (Feature 3)

**Files**: `app.rs`, `ui/mod.rs`

1. **app.rs**: Add focused property index
   ```rust
   pub focused_property_idx: Option<usize>,
   ```

2. **ui/mod.rs**: In property rendering:
   - If `focused_property_idx == current_idx`: render full value (wrapped)
   - Else: truncate with "..."

3. **Keybindings**: `j/k` in detail panel to move focus between properties

### Phase 5: JSON Pretty-Print (Feature 4)

**Files**: `app.rs`, `ui/mod.rs`

1. **app.rs**: Add toggle state
   ```rust
   pub json_pretty: bool,  // Toggle with 'j'
   ```

2. **ui/mod.rs**: When rendering JSON values:
   ```rust
   if app.json_pretty && is_json(&value) {
       render_pretty_json(&value)
   } else {
       render_compact(&value)
   }
   ```

### Phase 6: Yank to Clipboard (Feature 5)

**Files**: `app.rs`, `Cargo.toml`

1. **Cargo.toml**: Add dependency
   ```toml
   copypasta = "0.10"  # Or cli-clipboard
   ```

2. **app.rs**: Handle `y` key
   ```rust
   KeyCode::Char('y') => {
       if let Some(value) = self.get_focused_property_value() {
           clipboard.set_contents(value);
           self.set_status("Yanked!");
       }
   }
   ```

### Phase 7: Audit Mode (Feature 6)

**Files**: `app.rs`, `data.rs`, `ui/mod.rs`, `ui/audit.rs` (NEW)

1. **app.rs**: Add NavMode variant
   ```rust
   pub enum NavMode {
       Meta,
       Data,
       Overlay,
       Query,
       Atlas,
       Audit,  // NEW
   }
   ```

2. **data.rs**: Add audit data loading
   ```rust
   pub async fn load_audit_stats(db: &Db) -> Vec<KindAuditStats>;
   ```

3. **ui/audit.rs**: New render function
   ```rust
   pub fn render_audit_panel(f: &mut Frame, app: &App, area: Rect);
   ```

4. **Keybindings**:
   - `6` → Enter Audit mode
   - `Enter` → Drill down to Kind's problematic instances
   - `r` → Refresh stats
   - `e` → Export to CSV

## File Changes Summary

| File | Changes |
|------|---------|
| `src/tui/mod.rs` | Add `mod schema; mod audit;` |
| `src/tui/schema.rs` | NEW: Schema loading + property matching |
| `src/tui/audit.rs` | NEW: Audit calculations |
| `src/tui/app.rs` | Add state: overlay, focus, json_pretty, NavMode::Audit |
| `src/tui/data.rs` | Add queries: instance props, health stats, audit stats |
| `src/tui/ui/mod.rs` | Modify detail panel, tree rendering |
| `src/tui/ui/audit.rs` | NEW: Audit mode UI |
| `Cargo.toml` | Add `copypasta` for clipboard |

## Keybindings Summary

| Key | Action | Context |
|-----|--------|---------|
| `s` | Toggle schema overlay | Data mode |
| `j` | Toggle JSON pretty-print | Detail panel |
| `y` | Yank value to clipboard | Detail panel, property focused |
| `6` | Enter Audit mode | Global |
| `Enter` | Drill down | Audit mode |
| `r` | Refresh stats | Audit mode |
| `e` | Export CSV | Audit mode |

## Testing

1. **Schema overlay**: Select instance, verify all schema fields shown
2. **Health badges**: Check Kind nodes show correct % and issue count
3. **Truncate**: Focus long value, verify expansion
4. **JSON pretty**: Press `j`, verify formatting toggle
5. **Yank**: Press `y`, paste elsewhere, verify value
6. **Audit mode**: Press `6`, verify dashboard, drill down works

## Effort Estimate

| Feature | Complexity | Lines |
|---------|------------|-------|
| 1. Schema overlay | Medium | ~300 |
| 2. Tree badges | Low | ~100 |
| 3. Truncate | Low | ~80 |
| 4. JSON pretty | Low | ~50 |
| 5. Yank clipboard | Low | ~40 |
| 6. Audit mode | High | ~400 |
| **Total** | | **~970** |
