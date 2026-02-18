# Unified Kind/Instance Info Panel Design

**Date**: 2026-02-07
**Status**: Approved
**Author**: Thibaut + Claude

## Overview

Unify the middle info panel layout between Kind and Instance views in the TUI, enabling side-by-side comparison and real-time validation of Neo4j against YAML source of truth.

## Goals

1. **Visual alignment**: Same information on same lines for easy comparison
2. **Schema validation**: Show if Neo4j properties match YAML definitions
3. **Type visibility**: Display property types (`[str]`, `[bool]`, `[dt]`) in Kind view
4. **Examples**: Show YAML examples with `→` separator in Kind view

## Layout Specification

### Unified Header (Lines 1-7)

Both Kind and Instance views share identical header structure:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Kind View                         │  Instance View                         │
├─────────────────────────────────────────────────────────────────────────────┤
│  type        Node Kind             │  type        Instance                  │
│  key         Locale                │  key         af-ZA                     │
│  kind        —                     │  kind        Locale                    │
│  realm       Global                │  realm       Global                    │
│  layer       Locale Knowledge      │  layer       Locale Knowledge          │
│  trait       knowledge             │  trait       knowledge                 │
│                                    │                                        │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Properties Section (Line 8+)

Aligned format with validation status for Kind, values for Instance:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Kind (schema + validation):       │  Instance (actual values):             │
├─────────────────────────────────────────────────────────────────────────────┤
│  ✓*[str ] key        → "en-US"     │  *[str ] key        "af-ZA"            │
│  ✓*[str ] display    → "Name"      │  *[str ] display    "Afrikaans"        │
│  ⚠ [bool] is_rtl     → false       │   [bool] is_rtl     "false"            │
│  ? [str ] legacy                   │                                        │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Validation Logic

### Data Sources

| Source | Provides |
|--------|----------|
| **YAML** (source of truth) | prop name, type, required, example |
| **Neo4j** (runtime) | prop names in `properties[]`, `required_properties[]` |

### Validation States

| Status | Icon | Color | Meaning |
|--------|------|-------|---------|
| **Sync** | ✓ | Green (`STYLE_SUCCESS`) | Property exists in both YAML and Neo4j |
| **Missing** | ⚠ | Yellow (`STYLE_WARNING`) | YAML defines it, Neo4j missing |
| **Extra** | ? | Gray (`STYLE_DIM`) | Neo4j has it, not in YAML schema |

### Validation Flow

```
1. Load YAML schema    → Vec<SchemaProperty>
2. Load Neo4j props    → KindInfo.properties: Vec<String>
3. Compare & classify:
   ├── YAML ∩ Neo4j  → ✓ Sync
   ├── YAML - Neo4j  → ⚠ Missing
   └── Neo4j - YAML  → ? Extra
```

## Data Structures

### New: ValidationStatus enum

```rust
pub enum ValidationStatus {
    /// Property exists in both YAML and Neo4j
    Sync,
    /// YAML defines property, but Neo4j Kind node missing it
    Missing,
    /// Neo4j has property, but not defined in YAML schema
    Extra,
}
```

### New: ValidatedProperty struct

```rust
pub struct ValidatedProperty {
    pub name: String,
    pub prop_type: String,      // from YAML, or "?" for Extra
    pub required: bool,
    pub example: Option<String>,
    pub status: ValidationStatus,
}
```

## Files to Modify

| File | Changes |
|------|---------|
| `src/tui/schema.rs` | Add `ValidationStatus`, `ValidatedProperty`, `validate_kind_properties()` |
| `src/tui/app.rs` | Add `validated_kind_properties` field, `load_validated_kind_properties()` method |
| `src/tui/ui/mod.rs` | Update Kind view to render validated properties with ✓/⚠/? icons |

## Files NOT Changed

- **YAML schemas**: Already source of truth, no changes needed
- **Neo4j schema**: `properties[]` stays `Vec<String>`
- **Instance view**: Current format preserved (already has type badges)

## Visual Encoding

### Kind View Property Line Format

```
{status}{required}[{type}] {name:<15} → {example}
```

- `{status}`: ✓ / ⚠ / ? with appropriate color
- `{required}`: `*` if required, space otherwise
- `[{type}]`: 4-char type badge (`str `, `bool`, `dt  `, `json`, `enum`)
- `{name}`: Property name, left-aligned 15 chars
- `→`: Arrow separator (dim)
- `{example}`: Example value from YAML (muted color)

### Instance View Property Line Format (unchanged)

```
{required}[{type}] {name:<15} {value}
```

## Example Comparison

```
┌─────────────────── Kind: Locale ───────────────────┐
│ type        Node Kind                              │
│ key         Locale                                 │
│ kind        —                                      │
│ realm       Global                                 │
│ layer       Locale Knowledge                       │
│ trait       knowledge                              │
│                                                    │
│ properties  14 defined, 200 inst ━━━━━━━░░░        │
│                                                    │
│ ✓*[str ] key             → "en-US"                 │
│ ✓*[str ] display_name    → "English (US)"         │
│ ✓*[str ] language_code   → "en"                   │
│ ✓*[str ] country_code    → "US"                   │
│ ✓ [bool] is_primary      → true                   │
│ ⚠ [str ] currency_code   → "USD"                  │
└────────────────────────────────────────────────────┘

┌─────────────────── Instance: af-ZA ────────────────┐
│ type        Instance                               │
│ key         af-ZA                                  │
│ kind        Locale                                 │
│ realm       Global                                 │
│ layer       Locale Knowledge                       │
│ trait       knowledge                              │
│                                                    │
│ Properties (14/14) ━━━━━━━━━━ 100%                 │
│                                                    │
│ *[str ] key             "af-ZA"                    │
│ *[str ] display_name    "Afrikaans (South Africa)" │
│ *[str ] language_code   "af"                       │
│ *[str ] country_code    "ZA"                       │
│  [bool] is_primary      "true"                     │
│  [str ] currency_code   "ZAR"                      │
└────────────────────────────────────────────────────┘
```

## Implementation Order

1. Add `ValidationStatus` and `ValidatedProperty` to `schema.rs`
2. Add `validate_kind_properties()` function
3. Add state field to `App` in `app.rs`
4. Add `load_validated_kind_properties()` method
5. Update Kind view rendering in `ui/mod.rs`
6. Test with various Kinds (Locale, Entity, Page, etc.)
