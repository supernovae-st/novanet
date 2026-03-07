# TUI Entity/EntityNative Redesign Plan

**Version:** v0.17.3
**Status:** Ready for Implementation
**Files:** `src/tui/data.rs`, `src/tui/ui/tree.rs`

## Design Summary

```
Semantic
├── Entity (21)                                      ← Groupé par EntityCategory
│   ├── Product Types (8)
│   │   ├─○ QR Code              ▰▰▰▰▰▰▰▰▱▱        /qr-code
│   │   │   ├── 🇫🇷 fr-FR → Créer un QR Code         /creer-un-qr-code
│   │   │   ├── 🇪🇸 es-ES → Crear código QR          /crear-codigo-qr
│   │   │   └── 🇯🇵 ja-JP → QRコードを作成            /qr-code-sakusei
│   │   ├─● Barcode              ▰▰▰▰▰▰▱▱▱▱        /barcode
│   │   └─● Dynamic QR Code      ▰▰▰▱▱▱▱▱▱▱        /dynamic-qr-code
│   └── Features (6)
│       └─● Smart Link           ▰▰▱▱▱▱▱▱▱▱        /smart-link
│
├── EntityNative (21)                                ← Groupé par Locale, trié A-Z
│   ├─○ 🇫🇷 fr-FR (12)                               Français (France)
│   │   ├── Barcode        → Code-barres             /code-barres
│   │   ├── Dynamic QR Code→ QR Code dynamique       /qr-code-dynamique
│   │   └── QR Code        → Créer un QR Code        /creer-un-qr-code
│   ├─● 🇩🇪 de-DE (3)                                Deutsch
│   ├─● 🇪🇸 es-ES (8)                                Español (España)
│   └─● 🇯🇵 ja-JP (4)                                日本語
```

## Tasks

### Phase 1: Data Layer (`data.rs`)

#### Task 1.1: Add EntityNative locale grouping structure
**File:** `src/tui/data.rs`
**Location:** After `EntityCategory` struct (~line 710)

```rust
/// LocaleGroup for grouping EntityNative instances by locale.
/// Used in Data mode to show EntityNative instances organized by locale.
#[derive(Debug, Clone)]
pub struct LocaleGroup {
    /// Locale code (e.g., "fr-FR")
    pub locale_code: String,
    /// Locale display name (e.g., "Français (France)")
    pub locale_name: String,
    /// Flag emoji
    pub flag: String,
    /// Number of EntityNative instances for this locale
    pub instance_count: i64,
}
```

#### Task 1.2: Add locale_groups field to TaxonomyTree
**File:** `src/tui/data.rs`
**Location:** In `TaxonomyTree` struct (~line 826)

Add fields:
```rust
/// EntityNative instances grouped by locale (key = locale code like "fr-FR").
pub entity_native_by_locale: FxHashMap<String, Vec<EntityNativeInfo>>,
/// Locale groups with counts for EntityNative display.
pub locale_groups: Vec<LocaleGroup>,
```

#### Task 1.3: Add EntityNativeInfo struct for richer data
**File:** `src/tui/data.rs`
**Location:** After `LocaleGroup` struct

```rust
/// EntityNative info with parent Entity reference.
#[derive(Debug, Clone)]
pub struct EntityNativeInfo {
    /// EntityNative key (e.g., "qr-code@fr-FR")
    pub key: String,
    /// Native display name (e.g., "Créer un QR Code")
    pub display_name: String,
    /// Parent Entity key (e.g., "qr-code")
    pub entity_key: String,
    /// Parent Entity display name (invariant, e.g., "QR Code")
    pub entity_display_name: String,
    /// URL slug from denomination_forms
    pub slug: Option<String>,
}
```

#### Task 1.4: Add load_entity_natives_by_locale query
**File:** `src/tui/data.rs`
**Location:** After `load_entities_by_category` function (~line 2000)

```rust
/// Load all EntityNative instances grouped by locale.
/// Returns (locale_groups, natives_by_locale).
pub async fn load_entity_natives_by_locale(
    db: &Db,
) -> crate::Result<(Vec<LocaleGroup>, FxHashMap<String, Vec<EntityNativeInfo>>)> {
    let cypher = r#"
MATCH (en:EntityNative)-[:FOR_LOCALE]->(l:Locale)
MATCH (e:Entity)-[:HAS_NATIVE]->(en)
WITH l.key AS locale_code,
     l.display_name AS locale_name,
     en, e
ORDER BY e.display_name, e.key
WITH locale_code, locale_name,
     collect({
         key: en.key,
         display_name: en.display_name,
         entity_key: e.key,
         entity_display_name: e.display_name,
         slug: CASE
             WHEN en.denomination_forms IS NOT NULL
             THEN [form IN en.denomination_forms WHERE form.type = 'url' | form.value][0]
             ELSE null
         END
     }) AS natives
RETURN locale_code, locale_name, natives, size(natives) AS count
ORDER BY locale_code
"#;
    // ... parse results
}
```

#### Task 1.5: Add Entity slug extraction to load_entities_by_category
**File:** `src/tui/data.rs`
**Location:** In `load_entities_by_category` query

Add to Entity properties extraction:
```rust
entity_slug: CASE
    WHEN e.denomination_forms IS NOT NULL
    THEN [form IN e.denomination_forms WHERE form.type = 'url' | form.value][0]
    ELSE null
END
```

#### Task 1.6: Add relationship_power field to InstanceInfo
**File:** `src/tui/data.rs`
**Location:** In `InstanceInfo` struct (~line 3330)

```rust
/// Relationship power score (0-100) based on arc count.
/// Used for power bar visualization in Entity display.
pub relationship_power: u8,
/// Entity slug from denomination_forms.
pub entity_slug: Option<String>,
```

### Phase 2: Tree Rendering (`tree.rs`)

#### Task 2.1: Add power bar rendering function
**File:** `src/tui/ui/tree.rs`
**Location:** After constants section (~line 93)

```rust
/// Render power bar with color based on percentage.
/// ▰▰▰▰▰▰▰▰▱▱ (80% = green, 50-79% = orange, <50% = red)
fn render_power_bar(power: u8) -> (String, Color) {
    const BAR_WIDTH: usize = 10;
    let filled = (power as usize * BAR_WIDTH / 100).min(BAR_WIDTH);
    let empty = BAR_WIDTH - filled;

    let bar = format!(
        "{}{}",
        "▰".repeat(filled),
        "▱".repeat(empty)
    );

    let color = if power >= 80 {
        Color::Rgb(34, 197, 94)   // green-500
    } else if power >= 50 {
        Color::Rgb(249, 115, 22)  // orange-500
    } else {
        Color::Rgb(239, 68, 68)   // red-500
    };

    (bar, color)
}
```

#### Task 2.2: Change Entity/EntityNative text color to white
**File:** `src/tui/ui/tree.rs`
**Location:** In Entity instance rendering (~line 1110-1160)

Change:
```rust
// OLD: Style::default().fg(COLOR_INSTANCE)  // yellow
// NEW:
Style::default().fg(Color::White)
```

#### Task 2.3: Add Entity power bar + slug rendering
**File:** `src/tui/ui/tree.rs`
**Location:** In Entity instance rendering (~line 1127-1160)

Update format to:
```rust
format!(
    "{}{}{} {}{}{}        {}",
    cursor_char,
    tree_prefix,
    expand_icon,
    instance.display_name,
    power_bar,          // NEW
    slug_display,       // NEW (right-aligned)
)
```

#### Task 2.4: Add EntityNative locale grouping rendering
**File:** `src/tui/ui/tree.rs`
**Location:** Add new branch for EntityNative class (~line 1255)

When class is "EntityNative":
1. Check if `app.tree.locale_groups` is populated
2. Render locale headers: `🇫🇷 fr-FR (12)                 Français (France)`
3. When expanded, render natives sorted A-Z:
   `Invariant → Native Name     /slug`

#### Task 2.5: Update TreeItem enum for locale groups
**File:** `src/tui/data.rs`
**Location:** In `TreeItem` enum (~line 3246)

Add variant:
```rust
LocaleGroup(
    &'a RealmInfo,
    &'a LayerInfo,
    &'a ClassInfo,
    &'a LocaleGroup,
),
```

### Phase 3: Integration

#### Task 3.1: Wire up data loading
**File:** `src/tui/app/state.rs` or `mod.rs`

On EntityNative class expand:
- Call `load_entity_natives_by_locale()`
- Store in `tree.locale_groups` and `tree.entity_native_by_locale`

#### Task 3.2: Wire up Entity category data loading (already exists)
Verify `load_entity_categories()` and `load_entities_by_category()` are called.

#### Task 3.3: Calculate relationship_power for Entity instances
In `load_entities_by_category()`:
```rust
// Calculate power based on HAS_NATIVE arc count
let native_count = outgoing_arcs.iter()
    .filter(|a| a.arc_type == "HAS_NATIVE")
    .count();
let max_natives = 10; // Expected max locales
let power = ((native_count * 100) / max_natives).min(100) as u8;
```

### Phase 4: Cleanup

#### Task 4.1: Remove duplicate EntityNative flat list logic
Remove old flat instance rendering for EntityNative class.

#### Task 4.2: Update tests
Add tests for:
- `load_entity_natives_by_locale()`
- `render_power_bar()`
- Locale grouping cursor navigation

## Color Constants

```rust
// Entity/EntityNative text (was yellow, now white)
const COLOR_ENTITY_TEXT: Color = Color::White;

// Power bar colors
const COLOR_POWER_HIGH: Color = Color::Rgb(34, 197, 94);   // green-500 (≥80%)
const COLOR_POWER_MED: Color = Color::Rgb(249, 115, 22);   // orange-500 (50-79%)
const COLOR_POWER_LOW: Color = Color::Rgb(239, 68, 68);    // red-500 (<50%)

// Power bar characters
const POWER_FILLED: char = '▰';
const POWER_EMPTY: char = '▱';
```

## Verification Checklist

- [ ] Entity grouped by EntityCategory
- [ ] Entity shows power bar (▰▱) with colors
- [ ] Entity shows slug right-aligned
- [ ] Entity expandable to show natives by locale
- [ ] EntityNative grouped by Locale
- [ ] EntityNative locale header shows locale name on right
- [ ] EntityNative items sorted A-Z by invariant
- [ ] EntityNative format: `Invariant → Native Name /slug`
- [ ] All text white (not yellow)
- [ ] Cursor navigation works through all levels
- [ ] `cargo test` passes
- [ ] `cargo clippy` zero warnings
