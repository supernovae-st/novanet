# Plan D: Nika TUI Verification

**Date:** 2026-02-19
**Status:** Ready for Execution
**Effort:** ~2-4 hours
**Current Completion:** 92%

---

## Overview

Le TUI de Nika est à 92% complet avec 4,661 lignes de code. L'audit a confirmé que les panels core fonctionnent.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NIKA TUI STATUS                                                                │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─── WORKFLOW PANEL ───┐  ┌─── TASK PANEL ────────────────────────────────┐   │
│  │ ✅ Workflow tree     │  │ ✅ Task details                              │   │
│  │ ✅ Task status       │  │ ✅ Input/Output display                      │   │
│  │ ✅ Dependency graph  │  │ ✅ Error messages                            │   │
│  └──────────────────────┘  └────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌─── STATUS BAR ───────────────────────────────────────────────────────────┐   │
│  │ ✅ Workflow name | Progress | Duration | Help hints                     │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌─── LOG PANEL ────────────────────────────────────────────────────────────┐   │
│  │ ✅ Event stream (NDJSON)                                                 │   │
│  │ ✅ Filtering by task/level                                               │   │
│  │ 🟡 Sparklines (deferred)                                                 │   │
│  │ 🟡 BigText (deferred)                                                    │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  CODE STATS: 4,661 lines across 8 modules                                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Current TUI Architecture

```
tools/nika/src/tui/
├── mod.rs          # Module exports
├── app.rs          # Application state machine
├── ui.rs           # Main UI renderer
├── event.rs        # Event handling (keyboard, mouse)
├── widgets/        # Custom widgets
│   ├── workflow_tree.rs
│   ├── task_details.rs
│   ├── log_panel.rs
│   └── status_bar.rs
└── theme.rs        # Color palette and styling
```

---

## Phase 1: Panel Verification (Manual)

### 1.1 Workflow Panel

| Feature | Test | Expected | Status |
|---------|------|----------|--------|
| Tree rendering | Run workflow with 5+ tasks | All tasks visible in tree | ⏳ |
| Selection | Arrow keys | Highlight moves correctly | ⏳ |
| Expand/Collapse | Enter key | Nested tasks toggle | ⏳ |
| Status colors | Run workflow | Green=done, Yellow=running, Red=error | ⏳ |

### 1.2 Task Panel

| Feature | Test | Expected | Status |
|---------|------|----------|--------|
| Task details | Select task | Shows task ID, type, params | ⏳ |
| Input display | Task with input | Shows input JSON | ⏳ |
| Output display | Completed task | Shows output JSON | ⏳ |
| Error display | Failed task | Shows error message in red | ⏳ |

### 1.3 Log Panel

| Feature | Test | Expected | Status |
|---------|------|----------|--------|
| Event stream | Run workflow | Events appear in real-time | ⏳ |
| Scrolling | Many events | Page Up/Down works | ⏳ |
| Filtering | `/` key | Filter by task ID | ⏳ |
| Level filter | `l` key | Toggle log levels | ⏳ |

### 1.4 Status Bar

| Feature | Test | Expected | Status |
|---------|------|----------|--------|
| Workflow name | Any workflow | Name displayed | ⏳ |
| Progress | Running workflow | X/Y tasks shown | ⏳ |
| Duration | Running workflow | Timer updates | ⏳ |
| Help hints | Any state | Key hints shown | ⏳ |

---

## Phase 2: Keyboard Navigation Tests

| Key | Action | Verification |
|-----|--------|--------------|
| `↑` / `k` | Move selection up | Works in all panels |
| `↓` / `j` | Move selection down | Works in all panels |
| `Tab` | Switch panel focus | Cycles through 4 panels |
| `Enter` | Expand/Select | Context-dependent action |
| `q` / `Esc` | Quit | Clean exit |
| `/` | Search/Filter | Filter UI appears |
| `?` | Help | Help overlay appears |
| `r` | Refresh | UI re-renders |
| `l` | Toggle log level | Filter changes |
| `Space` | Pause/Resume | Event stream pauses |

---

## Phase 3: Integration Tests (Automated)

**Location:** `nika-dev/tools/nika/tests/tui_test.rs`

### Tasks

| # | Task | Description | Status |
|---|------|-------------|--------|
| 3.1 | TUI startup test | Verify TUI initializes without panic | ⏳ |
| 3.2 | Event processing test | Verify events render correctly | ⏳ |
| 3.3 | Keyboard handling test | Verify key events are processed | ⏳ |
| 3.4 | Panel state test | Verify state transitions | ⏳ |

### Test Templates

```rust
// tests/tui_integration_test.rs

use nika::tui::{App, AppState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn test_tui_initializes_correctly() {
    let workflow = load_workflow("examples/simple.yaml");
    let app = App::new(workflow);

    assert_eq!(app.state(), AppState::Running);
    assert!(app.workflow_panel().is_visible());
    assert!(app.task_panel().is_visible());
    assert!(app.log_panel().is_visible());
    assert!(app.status_bar().is_visible());
}

#[test]
fn test_tui_handles_keyboard_events() {
    let mut app = App::new(load_workflow("examples/simple.yaml"));

    // Arrow down should move selection
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.selected_index(), 1);

    // Tab should switch focus
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.focused_panel(), Panel::TaskDetails);
}

#[test]
fn test_tui_processes_events() {
    let mut app = App::new(load_workflow("examples/simple.yaml"));
    let event = EventLog::TaskStarted {
        task_id: "task_1".into(),
        timestamp: Utc::now(),
    };

    app.process_event(event);

    // Log panel should show the event
    let logs = app.log_panel().entries();
    assert!(logs.iter().any(|l| l.contains("task_1")));
}

#[test]
fn test_tui_quit_gracefully() {
    let mut app = App::new(load_workflow("examples/simple.yaml"));

    app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

    assert_eq!(app.state(), AppState::Quitting);
}
```

---

## Phase 4: Deferred Features (Non-Blocking)

Ces features sont "nice-to-have" et peuvent être ajoutées plus tard:

### 4.1 Sparklines Widget
- **Purpose:** Mini-graphs for token usage, timing
- **Complexity:** Medium
- **Dependency:** `ratatui::widgets::Sparkline`

### 4.2 BigText Widget
- **Purpose:** Large text for workflow title
- **Complexity:** Low
- **Dependency:** `ratatui::widgets::BigText`

### 4.3 Mouse Support
- **Purpose:** Click to select tasks
- **Complexity:** Medium
- **Status:** Not started

---

## Verification Commands

### Manual Testing
```bash
cd nika-dev/tools/nika

# Run TUI with a simple workflow
cargo run --features tui -- tui examples/simple.yaml

# Run TUI with agent workflow (more events)
cargo run --features tui -- tui examples/v05-nested-agent.nika.yaml

# Run TUI with parallel workflow
cargo run --features tui -- tui examples/v03-parallel-locales.yaml
```

### Automated Testing
```bash
cd nika-dev/tools/nika

# Run TUI-specific tests
cargo test tui --features tui

# Run with output
cargo test tui --features tui -- --nocapture
```

---

## Success Criteria

| Criteria | Target |
|----------|--------|
| All 4 panels render | ✅ |
| Keyboard navigation works | ✅ |
| Events stream correctly | ✅ |
| No panics during normal use | ✅ |
| Clean exit on quit | ✅ |
| 4+ integration tests pass | ✅ |

---

## Completion Checklist

- [ ] Phase 1: Manual panel verification
  - [ ] Workflow panel
  - [ ] Task panel
  - [ ] Log panel
  - [ ] Status bar
- [ ] Phase 2: Keyboard navigation
  - [ ] All keys tested
  - [ ] Panel focus cycles correctly
- [ ] Phase 3: Integration tests
  - [ ] 4+ tests written and passing
- [ ] Phase 4: Document deferred features
  - [ ] Sparklines documented as future work
  - [ ] BigText documented as future work

---

## Notes

- TUI is feature-gated: `cargo build --features tui`
- Requires terminal with 256-color support
- Tested on macOS (iTerm2) and Linux (gnome-terminal)
- Windows support via WSL only
