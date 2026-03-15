//! Centralized color palette for the NovaNet TUI.
//!
//! All colors used across the TUI are defined here as `const` values.
//! NO inline `Color::Rgb()` should exist outside this module.

use ratatui::style::Color;

// ── Solarized Base ───────────────────────────────────────────────

pub const SOLARIZED_CYAN: Color = Color::Rgb(42, 161, 152);
pub const SOLARIZED_VIOLET: Color = Color::Rgb(108, 113, 196);
pub const SOLARIZED_GREEN: Color = Color::Rgb(133, 153, 0);
pub const SOLARIZED_ORANGE: Color = Color::Rgb(203, 75, 22);
pub const SOLARIZED_RED: Color = Color::Rgb(220, 50, 47);
pub const SOLARIZED_BLUE: Color = Color::Rgb(38, 139, 210);
pub const SOLARIZED_MAGENTA: Color = Color::Rgb(211, 54, 130);
pub const SOLARIZED_GOLD: Color = Color::Rgb(181, 137, 0);

// ── Nord Theme ───────────────────────────────────────────────────

pub const NORD_FROST: Color = Color::Rgb(136, 192, 208);
pub const NORD_AURORA_GREEN: Color = Color::Rgb(163, 190, 140);
pub const NORD_BORDER_UNFOCUSED: Color = Color::Rgb(59, 66, 82);
pub const NORD_BORDER_FOCUSED: Color = Color::Rgb(76, 86, 106);

// ── Tailwind-500 ─────────────────────────────────────────────────

pub const GREEN_500: Color = Color::Rgb(34, 197, 94);
pub const BLUE_500: Color = Color::Rgb(59, 130, 246);
pub const ORANGE_500: Color = Color::Rgb(249, 115, 22);
pub const VIOLET_500: Color = Color::Rgb(139, 92, 246);
pub const RED_500: Color = Color::Rgb(239, 68, 68);
pub const YELLOW_500: Color = Color::Rgb(234, 179, 8);
pub const CYAN_500: Color = Color::Rgb(6, 182, 212);
pub const PURPLE_500: Color = Color::Rgb(168, 85, 247);
pub const SLATE_500: Color = Color::Rgb(100, 116, 139);

// ── Grays (Background/Border scale) ─────────────────────────────

pub const BG_DARK: Color = Color::Rgb(15, 15, 20);
pub const BG_OVERLAY: Color = Color::Rgb(20, 20, 30);
pub const BG_EMPTY: Color = Color::Rgb(25, 25, 35);
pub const BG_ACTIVE: Color = Color::Rgb(25, 35, 45);
pub const BG_HIGHLIGHT: Color = Color::Rgb(30, 40, 50);
pub const BG_SEARCH: Color = Color::Rgb(30, 50, 70);
pub const BG_PROPERTY_FOCUSED: Color = Color::Rgb(30, 50, 80);
pub const BORDER_UNFOCUSED: Color = Color::Rgb(60, 60, 70);
pub const SEPARATOR: Color = Color::Rgb(70, 70, 80);
pub const HINT_TEXT: Color = Color::Rgb(80, 80, 100);
pub const DIM: Color = Color::Rgb(100, 100, 100);
pub const DIM_110: Color = Color::Rgb(100, 100, 110);
pub const MUTED: Color = Color::Rgb(100, 100, 120);
pub const MUTED_130: Color = Color::Rgb(130, 130, 140);
pub const BRIGHT_DIM: Color = Color::Rgb(140, 140, 140);
pub const DESC_TEXT: Color = Color::Rgb(150, 150, 150);
pub const COUNT_TEXT: Color = Color::Rgb(180, 180, 180);
pub const FILE_TEXT: Color = Color::Rgb(180, 180, 200);

// ── Realm colors ─────────────────────────────────────────────────

pub const REALM_SHARED: Color = SOLARIZED_CYAN;
pub const REALM_ORG: Color = SOLARIZED_VIOLET;

// ── Layer colors ─────────────────────────────────────────────────

pub const LAYER_CONFIG: Color = BLUE_500;
pub const LAYER_LOCALE: Color = Color::Rgb(236, 72, 153); // pink-500
pub const LAYER_GEOGRAPHY: Color = GREEN_500;
pub const LAYER_KNOWLEDGE: Color = VIOLET_500;
pub const LAYER_FOUNDATION: Color = PURPLE_500;
pub const LAYER_STRUCTURE: Color = BLUE_500;
pub const LAYER_SEMANTIC: Color = ORANGE_500;
pub const LAYER_INSTRUCTION: Color = SOLARIZED_GOLD;
pub const LAYER_OUTPUT: Color = GREEN_500;

// ── Arc family colors ────────────────────────────────────────────

pub const FAMILY_OWNERSHIP: Color = BLUE_500;
pub const FAMILY_SEMANTIC: Color = ORANGE_500;
pub const FAMILY_GENERATION: Color = SOLARIZED_GOLD;
pub const FAMILY_LOCALIZATION: Color = GREEN_500;
pub const FAMILY_MINING: Color = VIOLET_500;
pub const FAMILY_LABEL: Color = Color::Rgb(180, 140, 80);

// ── YAML syntax highlighting ─────────────────────────────────────

pub const YAML_KEY: Color = Color::Rgb(86, 182, 194);
pub const YAML_STRING: Color = Color::Rgb(229, 192, 123);
pub const YAML_NUMBER: Color = Color::Rgb(209, 154, 102);
pub const YAML_BOOL: Color = Color::Rgb(198, 120, 221);
pub const YAML_BRACKET: Color = Color::Rgb(97, 175, 239);
pub const YAML_SECTION_HEADER: Color = Color::Rgb(92, 99, 112);

// ── Data value type colors ───────────────────────────────────────

pub const VALUE_BOOL: Color = Color::Rgb(189, 147, 249);
pub const VALUE_NUMBER: Color = Color::Rgb(249, 226, 175);
pub const VALUE_STRING: Color = Color::Rgb(166, 227, 161);
pub const VALUE_ARRAY: Color = Color::Rgb(137, 180, 250);
pub const VALUE_OBJECT: Color = Color::Rgb(245, 194, 231);

// ── Specialty ────────────────────────────────────────────────────

pub const PROP_KEY: Color = Color::Rgb(139, 233, 253);
pub const ENTITY_SLUG: Color = Color::Rgb(148, 163, 184);
pub const EMPTY_SLOT: Color = Color::Rgb(40, 40, 50);
