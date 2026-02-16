# TUI Header Consistency Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix the TUI info panel header to show explicit `key: value` format for all classification axes (realm, layer, trait) instead of cryptic inline badges.

**Architecture:** Modify `tools/novanet/src/tui/ui/info.rs` to:
1. Rename LOCATION section → CLASSIFICATION section
2. Replace inline badges (`●ORG ◆config ■defined`) with explicit `realm: ◎ org` format
3. Ensure ALL node types (Realm, Layer, Class, Instance, ArcFamily, ArcClass) use consistent key:value format

**Tech Stack:** Rust, ratatui, NovaNet TUI

---

## Visual Reference

### Before (Current - Confusing)

```
┌─ HEADER ─────────────────────────────────────────┐
│ type     Realm                    ◉ Organization │  ← What is "Organization"?
│ category ◈ Schema                                │
│ key      org                                     │
└──────────────────────────────────────────────────┘

┌─ HEADER ─────────────────────────────────────────┐
│ ●ORG ◆config ■defined              ◎ Organization│  ← Cryptic badges
│ key      OrgConfig                 ◎ Config      │  ← What axis is "Config"?
│ display  Org Config                ■ defined ─── │  ← "defined" what?
└──────────────────────────────────────────────────┘
```

### After (Proposed - Clear)

```
┌─ HEADER ─────────────────────────────────────────┐
│  IDENTITY                     CLASSIFICATION     │
│  ─────────                    ──────────────     │
│  type:    Realm               realm:  ◎ org      │  ← Explicit key:value
│  key:     org                 layers: 6          │
│  display: Organization        classes: 21        │
└──────────────────────────────────────────────────┘

┌─ HEADER ─────────────────────────────────────────┐
│  IDENTITY                     CLASSIFICATION     │
│  ─────────                    ──────────────     │
│  type:    Class               realm:  ◎ org      │  ← Clear axes
│  key:     OrgConfig           layer:  ⚙ config   │
│  display: Org Config          trait:  ■ defined  │
└──────────────────────────────────────────────────┘
```

---

## Source of Truth: Classification Icons & Colors

From `packages/core/models/`:

### Realm Icons (from realms/*.yaml)

| Realm | Icon (terminal) | Color | Hex |
|-------|-----------------|-------|-----|
| `shared` | `◉` | teal | `#2aa198` |
| `org` | `◎` | purple | `#6c71c4` |

### Layer Icons (from layers/*.yaml)

| Layer | Icon (terminal) | Color | Hex |
|-------|-----------------|-------|-----|
| `config` | `⚙` | slate | `#64748b` |
| `locale` | `◯` | slate | `#64748b` |
| `geography` | `◊` | emerald | `#10b981` |
| `knowledge` | `◆` | purple | `#8b5cf6` |
| `foundation` | `◆` | blue | `#3b82f6` |
| `structure` | `▫` | cyan | `#06b6d4` |
| `semantic` | `◆` | orange | `#f97316` |
| `instruction` | `□` | yellow | `#eab308` |
| `output` | `◆` | green | `#22c55e` |

### Trait Icons (from traits/*.yaml - ADR-024 Data Origin)

| Trait | Icon (terminal) | Color | Hex | Border |
|-------|-----------------|-------|-----|--------|
| `defined` | `■` | blue | `#3b82f6` | solid `─` |
| `authored` | `□` | green | `#22c55e` | dashed `┄` |
| `imported` | `◇` | purple | `#8b5cf6` | dotted `┈` |
| `generated` | `★` | golden | `#b58900` | double `═` |
| `retrieved` | `▪` | indigo | `#6c71c4` | thin `┅` |

---

## Tasks

### Task 1: Update SectionContent Helper for Consistent Formatting

**Files:**
- Modify: `tools/novanet/src/tui/ui/info.rs:62-67`

**Step 1: Modify the `add_kv` method to use consistent key width**

The current `add_kv` pads to 10 chars. We need:
- IDENTITY keys: `type:`, `key:`, `display:`, `class:` → 10 chars
- CLASSIFICATION keys: `realm:`, `layer:`, `trait:` → 8 chars (shorter section)

```rust
// In SectionContent impl (line 62):
fn add_kv(&mut self, key: &str, value: Span<'a>) {
    self.lines.push(Line::from(vec![
        Span::styled(format!("{:<10}", format!("{}:", key)), STYLE_DIM),
        value,
    ]));
}

// Add new method for classification section (narrower)
fn add_classification(&mut self, key: &str, icon: &str, value: &str, color: Color) {
    self.lines.push(Line::from(vec![
        Span::styled(format!("{:<8}", format!("{}:", key)), STYLE_DIM),
        Span::styled(format!("{} ", icon), Style::default().fg(color)),
        Span::styled(value.to_string(), Style::default().fg(color)),
    ]));
}
```

**Step 2: Run compilation check**

```bash
cd tools/novanet && cargo check
```

Expected: Compilation succeeds (new method is not yet used)

**Step 3: Commit**

```bash
git add tools/novanet/src/tui/ui/info.rs
git commit -m "feat(tui): add add_classification helper for explicit key:value format"
```

---

### Task 2: Refactor build_realm_content

**Files:**
- Modify: `tools/novanet/src/tui/ui/info.rs:229-311` (build_realm_content function)

**Step 1: Update IDENTITY section**

Replace:
```rust
// OLD (lines 234-244):
content
    .identity
    .add_kv("type", Span::styled("Realm", STYLE_ACCENT));
content.identity.add_kv(
    "category",
    Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
);
content
    .identity
    .add_kv("key", Span::styled(realm.key.clone(), STYLE_PRIMARY));
```

With:
```rust
// NEW: Explicit key:value format
content
    .identity
    .add_kv("type", Span::styled("Realm", STYLE_ACCENT));
content
    .identity
    .add_kv("key", Span::styled(realm.key.clone(), STYLE_PRIMARY));
content
    .identity
    .add_kv("display", Span::styled(realm.display_name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
```

**Step 2: Update LOCATION → CLASSIFICATION section**

Replace:
```rust
// OLD (lines 246-256):
content.location.add_line(Line::from(vec![
    Span::styled(
        format!("{} ", realm.icon),
        Style::default().fg(hex_to_color(&realm.color)),
    ),
    Span::styled(
        realm.display_name.clone(),
        Style::default().fg(hex_to_color(&realm.color)),
    ),
]));
```

With:
```rust
// NEW: Explicit classification with key:value
let realm_color = hex_to_color(&realm.color);
content.location.add_classification(
    "realm",
    &realm.icon,
    &realm.key,
    realm_color,
);
```

**Step 3: Run tests**

```bash
cd tools/novanet && cargo test --lib -- build_realm
```

Expected: Tests pass (if any exist for this function)

**Step 4: Visual verification**

```bash
cd tools/novanet && cargo run -- tui
```

Navigate to a Realm node and verify:
- IDENTITY shows `type:`, `key:`, `display:`
- Right side shows `realm: ◎ org` format

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/ui/info.rs
git commit -m "refactor(tui): realm header uses explicit key:value format"
```

---

### Task 3: Refactor build_layer_content

**Files:**
- Modify: `tools/novanet/src/tui/ui/info.rs:314-408` (build_layer_content function)

**Step 1: Update IDENTITY section**

Replace:
```rust
// OLD (lines 322-332):
content
    .identity
    .add_kv("type", Span::styled("Layer", STYLE_SUCCESS));
content.identity.add_kv(
    "category",
    Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
);
content
    .identity
    .add_kv("key", Span::styled(layer.key.clone(), STYLE_PRIMARY));
```

With:
```rust
// NEW: Explicit key:value format
content
    .identity
    .add_kv("type", Span::styled("Layer", STYLE_SUCCESS));
content
    .identity
    .add_kv("key", Span::styled(layer.key.clone(), STYLE_PRIMARY));
content
    .identity
    .add_kv("display", Span::styled(layer.display_name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
```

**Step 2: Update LOCATION → CLASSIFICATION section**

Replace:
```rust
// OLD (lines 334-351):
content.location.add_line(Line::from(vec![
    Span::styled(format!("{} ", realm.icon), STYLE_DIM),
    Span::styled(
        realm.display_name.clone(),
        Style::default().fg(hex_to_color(&realm.color)),
    ),
]));
content.location.add_line(Line::from(vec![
    Span::styled(
        format!("{} ", theme.icons.layer(&layer.key)),
        Style::default().fg(hex_to_color(&layer.color)),
    ),
    Span::styled(
        layer.display_name.clone(),
        Style::default().fg(hex_to_color(&layer.color)),
    ),
]));
```

With:
```rust
// NEW: Explicit classification with key:value
let realm_color = hex_to_color(&realm.color);
let layer_color = hex_to_color(&layer.color);

content.location.add_classification(
    "realm",
    &realm.icon,
    &realm.key,
    realm_color,
);
content.location.add_classification(
    "layer",
    theme.icons.layer(&layer.key),
    &layer.key,
    layer_color,
);
```

**Step 3: Run tests**

```bash
cd tools/novanet && cargo test --lib
```

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/ui/info.rs
git commit -m "refactor(tui): layer header uses explicit key:value format"
```

---

### Task 4: Refactor build_class_content (Most Complex)

**Files:**
- Modify: `tools/novanet/src/tui/ui/info.rs:412-668` (build_class_content function)

**Step 1: Remove inline badges from IDENTITY section**

Replace:
```rust
// OLD (lines 427-442) - cryptic inline badges:
content.identity.add_line(Line::from(vec![
    Span::styled("●", Style::default().fg(realm_color).add_modifier(Modifier::BOLD)),
    Span::styled(format!("{} ", realm.key.to_uppercase()), Style::default().fg(realm_color)),
    Span::styled("◆", Style::default().fg(layer_color).add_modifier(Modifier::BOLD)),
    Span::styled(format!("{} ", layer.key), Style::default().fg(layer_color)),
    Span::styled(trait_icon(&class.trait_name), Style::default().fg(trait_color)),
    Span::styled(class.trait_name.clone(), Style::default().fg(trait_color)),
]));
content
    .identity
    .add_kv("key", Span::styled(class.key.clone(), STYLE_PRIMARY));
content.identity.add_kv(
    "display",
    Span::styled(class.display_name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
);
```

With:
```rust
// NEW: Clean explicit key:value format
content
    .identity
    .add_kv("type", Span::styled("Class", STYLE_INFO));
content
    .identity
    .add_kv("key", Span::styled(class.key.clone(), STYLE_PRIMARY));
content.identity.add_kv(
    "display",
    Span::styled(class.display_name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
);
```

**Step 2: Update LOCATION → CLASSIFICATION section**

Replace:
```rust
// OLD (lines 444-470) - shows icon + display_name without key label:
content.location.add_line(Line::from(vec![
    Span::styled(format!("{} ", realm.icon), Style::default().fg(realm_color)),
    Span::styled(
        realm.display_name.clone(),
        Style::default().fg(realm_color),
    ),
]));
// ... similar for layer and trait
```

With:
```rust
// NEW: Explicit classification with key:value
content.location.add_classification(
    "realm",
    &realm.icon,
    &realm.key,
    realm_color,
);
content.location.add_classification(
    "layer",
    theme.icons.layer(&layer.key),
    &layer.key,
    layer_color,
);
if !class.trait_name.is_empty() {
    content.location.add_classification(
        "trait",
        trait_icon(&class.trait_name),
        &class.trait_name,
        trait_color,
    );
}
```

**Step 3: Run compilation and tests**

```bash
cd tools/novanet && cargo check && cargo test --lib
```

**Step 4: Visual verification**

```bash
cd tools/novanet && cargo run -- tui
```

Navigate to a Class node (e.g., OrgConfig) and verify:
- IDENTITY: `type: Class`, `key: OrgConfig`, `display: Org Config`
- CLASSIFICATION: `realm: ◎ org`, `layer: ⚙ config`, `trait: ■ defined`

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/ui/info.rs
git commit -m "refactor(tui): class header uses explicit key:value format

BREAKING: Removes inline badges (●ORG ◆config ■defined) in favor of
explicit CLASSIFICATION section with realm:/layer:/trait: key:value pairs.

Addresses UX issue where users couldn't tell what 'Organization' or
'defined' referred to in the header."
```

---

### Task 5: Refactor build_instance_content

**Files:**
- Modify: `tools/novanet/src/tui/ui/info.rs:797-948` (build_instance_content function)

**Step 1: Remove inline badges from IDENTITY section**

Replace:
```rust
// OLD (lines 814-829):
content.identity.add_line(Line::from(vec![
    Span::styled("●", Style::default().fg(realm_color).add_modifier(Modifier::BOLD)),
    Span::styled(format!("{} ", realm.key.to_uppercase()), Style::default().fg(realm_color)),
    Span::styled("◆", Style::default().fg(layer_color).add_modifier(Modifier::BOLD)),
    Span::styled(format!("{} ", layer.key), Style::default().fg(layer_color)),
    Span::styled("◇", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    Span::styled("Instance", Style::default().fg(Color::Yellow)),
]));
content
    .identity
    .add_kv("key", Span::styled(instance.key.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
content.identity.add_kv(
    "class",
    Span::styled(class.display_name.clone(), Style::default().fg(layer_color)),
);
```

With:
```rust
// NEW: Clean explicit key:value format
content
    .identity
    .add_kv("type", Span::styled("Instance", STYLE_HIGHLIGHT));
content
    .identity
    .add_kv("key", Span::styled(instance.key.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
content.identity.add_kv(
    "class",
    Span::styled(class.display_name.clone(), Style::default().fg(layer_color)),
);
```

**Step 2: Update LOCATION → CLASSIFICATION section**

Replace the 3 separate `add_line` calls (lines 831-857) with:

```rust
// NEW: Explicit classification with key:value
content.location.add_classification(
    "realm",
    &realm.icon,
    &realm.key,
    realm_color,
);
content.location.add_classification(
    "layer",
    theme.icons.layer(&layer.key),
    &layer.key,
    layer_color,
);
if !class.trait_name.is_empty() {
    content.location.add_classification(
        "trait",
        theme.icons.trait_icon(&class.trait_name),
        &class.trait_name,
        trait_color,
    );
}
```

**Step 3: Run tests**

```bash
cd tools/novanet && cargo test --lib
```

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/ui/info.rs
git commit -m "refactor(tui): instance header uses explicit key:value format"
```

---

### Task 6: Refactor build_arc_family_content and build_arc_class_content

**Files:**
- Modify: `tools/novanet/src/tui/ui/info.rs:671-795` (both functions)

**Step 1: Update build_arc_family_content IDENTITY section**

Replace:
```rust
// OLD (lines 678-689):
content.identity.add_line(Line::from(vec![
    Span::styled("◈ ", Style::default().fg(family_color).add_modifier(Modifier::BOLD)),
    Span::styled("ArcFamily", Style::default().fg(family_color)),
]));
content.identity.add_kv(
    "category",
    Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
);
content
    .identity
    .add_kv("key", Span::styled(family.key.clone(), Style::default().fg(family_color).add_modifier(Modifier::BOLD)));
```

With:
```rust
// NEW: Explicit key:value format
content
    .identity
    .add_kv("type", Span::styled("ArcFamily", Style::default().fg(family_color)));
content
    .identity
    .add_kv("key", Span::styled(family.key.clone(), Style::default().fg(family_color).add_modifier(Modifier::BOLD)));
content
    .identity
    .add_kv("display", Span::styled(family.display_name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
```

**Step 2: Update build_arc_class_content IDENTITY section**

Replace:
```rust
// OLD (lines 724-742):
content.identity.add_line(Line::from(vec![
    Span::styled("→ ", Style::default().fg(family_color).add_modifier(Modifier::BOLD)),
    Span::styled("ArcClass", Style::default().fg(family_color)),
]));
content.identity.add_kv(
    "category",
    Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
);
content
    .identity
    .add_kv("key", Span::styled(arc_class.key.clone(), Style::default().fg(family_color).add_modifier(Modifier::BOLD)));
content.identity.add_kv(
    "family",
    Span::styled(family.display_name.clone(), Style::default().fg(family_color)),
);
```

With:
```rust
// NEW: Explicit key:value format
content
    .identity
    .add_kv("type", Span::styled("ArcClass", Style::default().fg(family_color)));
content
    .identity
    .add_kv("key", Span::styled(arc_class.key.clone(), Style::default().fg(family_color).add_modifier(Modifier::BOLD)));
content
    .identity
    .add_kv("display", Span::styled(arc_class.display_name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));

// Add family as classification
content.location.add_classification(
    "family",
    arc_family_badge_icon(&family.key),
    &family.key,
    family_color,
);
```

**Step 3: Run tests**

```bash
cd tools/novanet && cargo test --lib
```

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/ui/info.rs
git commit -m "refactor(tui): arc family/class headers use explicit key:value format"
```

---

### Task 7: Final Integration Test and Documentation

**Files:**
- Test: `tools/novanet/src/tui/ui/info.rs` (full file)

**Step 1: Run full test suite**

```bash
cd tools/novanet && cargo test
```

Expected: All 1031 tests pass

**Step 2: Run clippy**

```bash
cd tools/novanet && cargo clippy -- -D warnings
```

Expected: Zero warnings

**Step 3: Manual visual verification**

```bash
cd tools/novanet && cargo run -- tui
```

Test navigation through:
- [ ] Realm node (org, shared) → Shows `realm: ◎ org`
- [ ] Layer node (config, semantic) → Shows `realm:` + `layer:`
- [ ] Class node (OrgConfig, Entity) → Shows `realm:` + `layer:` + `trait:`
- [ ] Instance node → Shows all 3 classification axes
- [ ] ArcFamily node → Shows `family:` classification
- [ ] ArcClass node → Shows `family:` classification

**Step 4: Update KEYBINDINGS.md if needed**

No keybinding changes, but verify header layout description is accurate.

**Step 5: Final commit**

```bash
git add -A
git commit -m "docs(tui): update header consistency - all nodes use key:value format

Summary of changes:
- IDENTITY section: type:/key:/display: format
- CLASSIFICATION section: realm:/layer:/trait: format with icons
- Removed cryptic inline badges (●ORG ◆config ■defined)
- Consistent across all 6 node types

Closes: TUI header consistency issue"
```

---

## Verification Checklist

After implementation, verify these screenshots match expected:

### Realm Selected
```
┌─ HEADER ─────────────────────────────────────────┐
│  type:      Realm                realm:  ◎ org   │
│  key:       org                  layers: 6       │
│  display:   Organization         classes: 21     │
└──────────────────────────────────────────────────┘
```

### Layer Selected
```
┌─ HEADER ─────────────────────────────────────────┐
│  type:      Layer                realm:  ◎ org   │
│  key:       config               layer:  ⚙ config│
│  display:   Configuration                        │
└──────────────────────────────────────────────────┘
```

### Class Selected
```
┌─ HEADER ─────────────────────────────────────────┐
│  type:      Class                realm:  ◎ org   │
│  key:       OrgConfig            layer:  ⚙ config│
│  display:   Org Config           trait:  ■ defined│
└──────────────────────────────────────────────────┘
```

### Instance Selected
```
┌─ HEADER ─────────────────────────────────────────┐
│  type:      Instance             realm:  ◎ org   │
│  key:       SuperNovae           layer:  ⚙ config│
│  class:     OrgConfig            trait:  ■ defined│
└──────────────────────────────────────────────────┘
```

---

## Rollback Plan

If issues arise, revert with:

```bash
git revert HEAD~7..HEAD  # Revert all 7 commits
```

Or cherry-pick specific fixes if only some node types have issues.
