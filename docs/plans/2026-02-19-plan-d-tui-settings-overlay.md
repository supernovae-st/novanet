# Plan D: TUI Settings Overlay (API Keys Configuration)

**Date:** 2026-02-19
**Scope:** Nika TUI - Add settings overlay for API key configuration
**Methodology:** TDD (Red-Green-Refactor)
**Effort:** ~2-3 hours

---

## Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SETTINGS OVERLAY DESIGN                                                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════════════╗ │
│  ║  SETTINGS                                                         [s]     ║ │
│  ╠═══════════════════════════════════════════════════════════════════════════╣ │
│  ║                                                                           ║ │
│  ║  API Keys                                                                 ║ │
│  ║  ─────────────────────────────────────────────────────────────────────── ║ │
│  ║                                                                           ║ │
│  ║  Anthropic:  [sk-ant-api03-*******************] ✓ Set                    ║ │
│  ║  OpenAI:     [                                 ] ✗ Not set               ║ │
│  ║                                                                           ║ │
│  ║  ─────────────────────────────────────────────────────────────────────── ║ │
│  ║                                                                           ║ │
│  ║  Provider:   [Claude ▼]  (auto-detected from keys)                       ║ │
│  ║  Model:      [claude-sonnet-4-20250514 ▼]                                ║ │
│  ║                                                                           ║ │
│  ║  ─────────────────────────────────────────────────────────────────────── ║ │
│  ║                                                                           ║ │
│  ║  [Tab] Next field  [Enter] Edit  [Esc] Close  [Ctrl+S] Save              ║ │
│  ║                                                                           ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════╝ │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Architecture

### New Files

| File | Purpose |
|------|---------|
| `src/tui/overlays/mod.rs` | Overlay module |
| `src/tui/overlays/settings.rs` | Settings overlay widget |
| `src/config.rs` | Config file management (~/.config/nika/) |

### Modified Files

| File | Changes |
|------|---------|
| `src/tui/state.rs` | Add `TuiMode::Settings`, `SettingsState` |
| `src/tui/app.rs` | Handle 's' key, render settings overlay |
| `src/tui/mod.rs` | Export new modules |
| `src/lib.rs` | Export config module |

---

## Phase 1: Config Module (TDD)

### Step 1.1: Create config.rs with tests first

```rust
// src/config.rs

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NikaConfig {
    pub api_keys: ApiKeys,
    pub defaults: Defaults,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiKeys {
    pub anthropic: Option<String>,
    pub openai: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Defaults {
    pub provider: Option<String>,
    pub model: Option<String>,
}

impl NikaConfig {
    /// Load config from ~/.config/nika/config.toml
    pub fn load() -> Result<Self, ConfigError>;

    /// Save config to ~/.config/nika/config.toml
    pub fn save(&self) -> Result<(), ConfigError>;

    /// Get config file path
    pub fn config_path() -> PathBuf;

    /// Merge with environment variables (env takes precedence)
    pub fn with_env(self) -> Self;

    /// Get effective Anthropic key (config or env)
    pub fn anthropic_key(&self) -> Option<String>;

    /// Get effective OpenAI key (config or env)
    pub fn openai_key(&self) -> Option<String>;
}
```

### Step 1.2: Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_config_path_is_in_config_dir() {
        let path = NikaConfig::config_path();
        assert!(path.to_string_lossy().contains(".config/nika"));
    }

    #[test]
    fn test_config_save_and_load_roundtrip() {
        let config = NikaConfig {
            api_keys: ApiKeys {
                anthropic: Some("sk-ant-test".into()),
                openai: None,
            },
            ..Default::default()
        };
        // Save to temp file, load back, compare
    }

    #[test]
    fn test_env_overrides_config() {
        std::env::set_var("ANTHROPIC_API_KEY", "sk-ant-env");
        let config = NikaConfig {
            api_keys: ApiKeys {
                anthropic: Some("sk-ant-config".into()),
                ..Default::default()
            },
            ..Default::default()
        }.with_env();

        assert_eq!(config.anthropic_key(), Some("sk-ant-env".into()));
        std::env::remove_var("ANTHROPIC_API_KEY");
    }
}
```

---

## Phase 2: TUI State Updates

### Step 2.1: Add TuiMode::Settings

```rust
// src/tui/state.rs

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TuiMode {
    #[default]
    Normal,
    // ... existing modes ...
    Settings,  // NEW
}

/// Settings overlay state
#[derive(Debug, Clone, Default)]
pub struct SettingsState {
    /// Currently focused field (0=anthropic, 1=openai, 2=provider, 3=model)
    pub focus: usize,
    /// Edit mode (typing in field)
    pub editing: bool,
    /// Input buffer for current field
    pub input_buffer: String,
    /// Cursor position in input
    pub cursor: usize,
    /// Loaded config
    pub config: NikaConfig,
    /// Has unsaved changes
    pub dirty: bool,
}
```

### Step 2.2: Add to TuiState

```rust
pub struct TuiState {
    // ... existing fields ...

    /// Settings overlay state
    pub settings: SettingsState,
}
```

---

## Phase 3: Settings Overlay Widget

### Step 3.1: Create overlays module

```rust
// src/tui/overlays/mod.rs
mod settings;
pub use settings::SettingsOverlay;

// src/tui/overlays/settings.rs
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct SettingsOverlay<'a> {
    state: &'a SettingsState,
    theme: &'a Theme,
}

impl<'a> SettingsOverlay<'a> {
    pub fn new(state: &'a SettingsState, theme: &'a Theme) -> Self {
        Self { state, theme }
    }

    /// Mask API key for display (show first 10 chars + ***)
    fn mask_key(key: &str) -> String {
        if key.len() <= 10 {
            "*".repeat(key.len())
        } else {
            format!("{}***", &key[..10])
        }
    }
}

impl Widget for SettingsOverlay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 1. Clear background
        Clear.render(area, buf);

        // 2. Draw bordered box
        let block = Block::default()
            .title(" Settings [s] ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        // 3. Render fields
        // ...
    }
}
```

---

## Phase 4: Input Handling

### Step 4.1: Add key handlers in app.rs

```rust
// In handle_key()
KeyCode::Char('s') => Action::SetMode(TuiMode::Settings),

// In Settings mode
TuiMode::Settings => match code {
    KeyCode::Esc => {
        if self.state.settings.editing {
            // Cancel edit, restore original value
            Action::SettingsCancelEdit
        } else {
            Action::SetMode(TuiMode::Normal)
        }
    }
    KeyCode::Tab => Action::SettingsNextField,
    KeyCode::BackTab => Action::SettingsPrevField,
    KeyCode::Enter => Action::SettingsToggleEdit,
    KeyCode::Char(c) if self.state.settings.editing => {
        Action::SettingsInput(c)
    }
    KeyCode::Backspace if self.state.settings.editing => {
        Action::SettingsBackspace
    }
    KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
        Action::SettingsSave
    }
    _ => Action::Continue,
}
```

### Step 4.2: New Action variants

```rust
pub enum Action {
    // ... existing ...
    SettingsNextField,
    SettingsPrevField,
    SettingsToggleEdit,
    SettingsInput(char),
    SettingsBackspace,
    SettingsCancelEdit,
    SettingsSave,
}
```

---

## Phase 5: Integration

### Step 5.1: Load config on TUI start

```rust
// In App::new()
let config = NikaConfig::load().unwrap_or_default().with_env();
let settings = SettingsState {
    config,
    ..Default::default()
};
```

### Step 5.2: Render overlay in render_frame()

```rust
// In render_frame()
match &state.mode {
    TuiMode::Help => render_help_overlay(frame, theme, size),
    TuiMode::Metrics => render_metrics_overlay(frame, state, theme, size),
    TuiMode::Settings => {
        let overlay = SettingsOverlay::new(&state.settings, theme);
        let area = centered_rect(60, 50, size);
        frame.render_widget(overlay, area);
    }
    _ => {}
}
```

---

## Implementation Order

| # | Task | File | Tests |
|---|------|------|-------|
| 1 | Create config.rs with NikaConfig struct | src/config.rs | 5 tests |
| 2 | Add TuiMode::Settings to state.rs | src/tui/state.rs | 2 tests |
| 3 | Add SettingsState struct | src/tui/state.rs | 3 tests |
| 4 | Create overlays/mod.rs | src/tui/overlays/mod.rs | - |
| 5 | Create SettingsOverlay widget | src/tui/overlays/settings.rs | 2 tests |
| 6 | Add Action variants | src/tui/app.rs | 1 test |
| 7 | Add key handlers | src/tui/app.rs | 3 tests |
| 8 | Integrate in render_frame | src/tui/app.rs | 1 test |
| 9 | Load config on startup | src/tui/app.rs | 1 test |

**Total: ~18 tests, ~400 lines of code**

---

## Verification

```bash
# Run tests
cargo test config --all-features
cargo test settings --all-features

# Manual test
cargo run --features tui -- tui examples/thibaut-first-workflow.nika.yaml
# Press 's' to open settings
# Tab between fields
# Enter to edit
# Ctrl+S to save
# Esc to close
```

---

## Success Criteria

- [ ] 's' key opens settings overlay
- [ ] Tab navigates between fields
- [ ] Enter enables edit mode
- [ ] API keys are masked in display
- [ ] Ctrl+S saves to ~/.config/nika/config.toml
- [ ] Config persists across sessions
- [ ] Env vars override config values
- [ ] All 18 tests pass
