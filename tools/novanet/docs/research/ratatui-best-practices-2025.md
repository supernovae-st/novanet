# Ratatui Best Practices Research (2025-2026)

**Research Date:** 2025-03-15
**Current Project:** NovaNet TUI (ratatui 0.30, ~30k lines, 5-panel layout)
**Sources:** ratatui 0.30 changelog, gitui, bottom, kmon, ratatui templates, official docs

---

## Executive Summary

Your NovaNet TUI is already using many best practices. Key opportunities:

1. **Adopt ratatui::run()** - Simplifies terminal setup/teardown
2. **Consider component trait pattern** - gitui/ratatui-templates approach
3. **Use TestBackend for testing** - Buffer assertions are the standard
4. **Leverage new v0.30 features** - Border merging, Rect::layout(), improved serde

---

## 1. Ratatui 0.30 Changes (Breaking + Features)

### Breaking Changes to Adopt

```rust
// BEFORE (0.29)
Bar::default().label("foo".into());

// AFTER (0.30) - Into<> inference fixed
Bar::default().label("foo");
```

**Key breaking changes:**
- `Marker` is now `#[non_exhaustive]` - add wildcard match arms
- `TestBackend` uses `core::convert::Infallible` for errors (affects `no_std`)
- `List::highlight_symbol` accepts `Into<Line>` instead of `&str`
- New `Flex::SpaceEvenly` (old `SpaceAround` renamed)

### New Features Worth Adopting

```rust
// NEW: Simplified app entry point
fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("Hello", frame.area()))?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}

// NEW: Ergonomic layout splitting
let [top, main, bottom] = area.layout(&Layout::vertical([
    Constraint::Length(1),
    Constraint::Fill(1),
    Constraint::Length(1),
]));

// NEW: Border merging (automatic when blocks overlap)
// Overlapping borders now merge cleanly - no extra code needed

// NEW: Rect centering
let centered = outer_rect.centered(Size::new(40, 10));

// NEW: Rect::outer() for padding
let with_margin = inner_rect.outer(Margin::new(2, 1));
```

### Modular Architecture (v0.30)

```
ratatui (convenience re-exports)
    ratatui-core (Widget traits, Buffer, Layout, Style)
    ratatui-widgets (Block, List, Paragraph, etc.)
    ratatui-crossterm / ratatui-termion / ratatui-termwiz
    ratatui-macros
```

**For widget library authors:** Depend on `ratatui-core` only for stability.
**For apps:** Continue using `ratatui` crate - no changes needed.

---

## 2. State Management Patterns

### Pattern A: Flat Struct (Current NovaNet Approach)

```rust
// Your current approach - good for medium complexity
pub struct App {
    pub mode: NavMode,
    pub focus: Focus,
    pub tree_cursor: usize,
    pub search: SearchState,
    pub overlays: OverlayState,
    // ... sub-structs for grouping
}
```

**Pros:** Simple, fast, no indirection
**Cons:** Can grow large (your 55 fields managed with sub-structs)
**Verdict:** Your approach with sub-structs is correct

### Pattern B: Component Trait (gitui/ratatui-templates)

```rust
// From ratatui-templates
pub trait Component {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()>;
    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>>;
    fn update(&mut self, action: Action) -> Result<Option<Action>>;
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}

// gitui variant
pub trait Component {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking;
    fn event(&mut self, ev: &Event) -> Result<EventState>;
    fn focused(&self) -> bool;
    fn is_visible(&self) -> bool;
}
```

**Pros:**
- Composable widgets
- Clear responsibility separation
- Easy to test individual components

**Cons:**
- More boilerplate
- Action/message passing overhead
- Not always needed for tightly-coupled panels

**When to use:** 10+ independent UI components, popup system, reusable widgets

### Pattern C: Redux-like (serpl example)

```rust
// serpl uses redux-rs
use redux_rs::{Store, StoreApi};

let store = Store::new_with_state(reducer, state)
    .wrap(ThunkMiddleware)
    .await;
```

**Pros:** Predictable state updates, time-travel debugging possible
**Cons:** Overkill for most TUIs, async complexity
**When to use:** Complex undo/redo, state persistence, debugging needs

### Recommendation for NovaNet

Your current approach (flat struct + sub-state grouping) is appropriate. Consider Component trait only if:
- Adding more independent panels (7+)
- Building a reusable widget library
- Need to unit test UI components in isolation

---

## 3. Performance Optimization

### Frame Budgets

```rust
// Your current approach is correct
const EVENT_TIMEOUT_MS: u64 = 100;  // 10 FPS animation tick

// Common frame targets
// 60 FPS = 16ms budget (games, smooth animation)
// 30 FPS = 33ms budget (most TUIs)
// 10 FPS = 100ms budget (data-heavy, your approach)
```

### Dirty Tracking Pattern

```rust
// Mark specific regions dirty
pub struct App {
    dirty_tree: bool,
    dirty_info: bool,
    dirty_yaml: bool,
}

impl App {
    fn draw(&mut self, frame: &mut Frame) {
        // Only rebuild expensive widgets when dirty
        if self.dirty_tree {
            self.rebuild_tree_cache();
            self.dirty_tree = false;
        }
        // Render from cache
    }
}
```

**Your current approach:** You use `RenderCache` and check `has_pending_load()` - good!

### Diff-Based Rendering (Built-in)

Ratatui already does buffer diffing internally:
```rust
// Terminal::flush() only writes changed cells
// You get this for free
```

### Production Optimizations

| Technique | Benefit | Implementation |
|-----------|---------|----------------|
| `FxHashMap` | Faster hashing | Already using `rustc-hash` |
| `SmallVec` | Stack allocation | Already using for small collections |
| Lazy loading | Reduced memory | Your `pending.instance` pattern |
| Widget caching | Reduced computation | Your `RenderCache` |

---

## 4. Async Integration Patterns

### Pattern A: Channel-Based (Your Approach)

```rust
// Your current approach - excellent
async fn run_app(...) {
    let mut event_stream = EventStream::new();

    loop {
        let event = tokio::time::timeout(
            Duration::from_millis(100),
            event_stream.next()
        ).await;

        match event {
            Ok(Some(Ok(Event::Key(key)))) => {
                app.handle_key(key);
                // Spawn async work
                if let Some(key) = app.take_pending_instance_load() {
                    let result = TaxonomyTree::load_instances_fast(db, &key).await;
                    // Check generation for staleness
                    if app.navigation_generation == nav_gen {
                        app.tree.set_instances(&key, result);
                    }
                }
            }
            Err(_) => {
                // Timeout - handle animations, background work
                app.tick = app.tick.wrapping_add(1);
            }
        }
    }
}
```

**Your navigation_generation pattern is excellent** - prevents stale async results.

### Pattern B: Action/Message Queue (gitui)

```rust
// gitui uses crossbeam channels
pub struct App {
    sender_git: Sender<AsyncGitNotification>,
    sender_app: Sender<AsyncAppNotification>,
    queue: Queue,  // Internal action queue
}

// Worker threads send notifications
sender_git.send(AsyncGitNotification::Status)?;

// Main loop checks queue
while let Ok(action) = queue.try_recv() {
    self.handle_action(action);
}
```

### Pattern C: Component + mpsc (ratatui-templates)

```rust
// Each component can send actions
pub struct App {
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

// Components send actions
fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    if key.code == KeyCode::Enter {
        Ok(Some(Action::Submit))
    } else {
        Ok(None)
    }
}

// Main loop handles
fn handle_actions(&mut self) {
    while let Ok(action) = self.action_rx.try_recv() {
        match action {
            Action::Tick => self.on_tick(),
            Action::Render => self.render(),
            Action::Quit => self.should_quit = true,
        }
    }
}
```

### Recommendation

Your current approach is production-ready. Key patterns you're using correctly:
- `tokio::time::timeout` for non-blocking event polling
- Generation counter for stale detection
- Two-phase loading (fast UI update, then background enrichment)

---

## 5. Testing Strategies

### TestBackend Buffer Assertions (Standard)

```rust
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use ratatui::buffer::Buffer;

#[test]
fn test_widget_renders() {
    let backend = TestBackend::new(10, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| {
        frame.render_widget(my_widget, frame.area());
    }).unwrap();

    // Assert against expected buffer
    let expected = Buffer::with_lines([
        "Header    ",
        "Item 1    ",
        "Item 2    ",
        "Item 3    ",
        "Footer    ",
    ]);
    terminal.backend().assert_buffer(&expected);
}
```

### Snapshot Testing (Your Approach)

```rust
// You're already using insta - excellent
use insta::assert_snapshot;

#[test]
fn test_tree_render() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| render_tree(frame, &app)).unwrap();

    // Snapshot the buffer as string
    let output = terminal.backend().buffer().to_string();
    assert_snapshot!(output);
}
```

### Component Unit Testing

```rust
// Test state transitions without rendering
#[test]
fn test_navigation() {
    let tree = TaxonomyTree::mock();
    let mut app = App::new(tree, "/tmp".into());

    assert_eq!(app.tree_cursor, 0);
    app.handle_key(KeyCode::Down.into());
    assert_eq!(app.tree_cursor, 1);
}

// Test async handlers with mock DB
#[tokio::test]
async fn test_instance_loading() {
    let db = MockDb::new();
    let mut app = App::new(tree, "/tmp".into());

    app.pending.instance = Some("Entity".into());
    // ... test loading logic
}
```

### Property-Based Testing (Your Approach)

```rust
// You're already using proptest - excellent
use proptest::prelude::*;

proptest! {
    #[test]
    fn cursor_never_overflows(cursor in 0usize..1000) {
        let mut app = App::new(tree, "/tmp".into());
        app.tree_cursor = cursor;
        app.ensure_cursor_visible();
        assert!(app.tree_cursor < app.current_item_count().max(1));
    }
}
```

---

## 6. Popular Ratatui Apps to Study

### Tier 1: Production-Quality Architecture

| App | Stars | Key Patterns | Study For |
|-----|-------|--------------|-----------|
| **gitui** | 18k+ | Component trait, async git, popups | Complex state, focus management |
| **bottom** | 10k+ | Multi-platform, charts, real-time data | Performance, cross-platform |
| **helix** | 35k+ | Modal editor, LSP integration | Complex keybindings, modes |

### Tier 2: Good Reference Implementations

| App | Key Patterns | Study For |
|-----|--------------|-----------|
| **kmon** | Simple state, Linux-specific | Clean architecture |
| **gpg-tui** | Modular tabs, crypto UI | Tab navigation |
| **zellij** | Plugin system, panes | Multi-pane layouts |

### Tier 3: Specific Patterns

| App | Key Patterns | Study For |
|-----|--------------|-----------|
| **serpl** | Redux-like state | State management |
| **spotify-tui** | Async API, lists | Async data loading |

### Key Files to Study

```bash
# gitui - Component pattern
src/components/mod.rs      # Component + DrawableComponent traits
src/app.rs                 # Main app orchestration
src/popups/                # Popup management

# bottom - Performance
src/app/data_harvester/    # Async data collection
src/canvas/                # Custom widget rendering
```

---

## 7. Widget Composition Patterns

### Pattern A: Nested Widgets

```rust
// Compose widgets by nesting
let inner = Paragraph::new("content");
let outer = Block::bordered().title("Title");

// Render with inner_area
let inner_area = outer.inner(area);
frame.render_widget(outer, area);
frame.render_widget(inner, inner_area);
```

### Pattern B: Custom StatefulWidget

```rust
// Your approach for complex widgets
pub struct TreeWidget;
pub struct TreeState {
    cursor: usize,
    scroll: usize,
    selected: Option<String>,
}

impl StatefulWidget for TreeWidget {
    type State = TreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Access and mutate state during render
        let visible_items = &self.items[state.scroll..];
        // ...
    }
}

// Usage
frame.render_stateful_widget(TreeWidget::new(&items), area, &mut tree_state);
```

### Pattern C: Builder Pattern (Preferred for Configuration)

```rust
// Clean widget configuration
let list = List::new(items)
    .block(Block::bordered().title("Items"))
    .highlight_style(Style::default().fg(Color::Yellow))
    .highlight_symbol(">> ")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom);
```

### Pattern D: Widget Wrapper for Styling

```rust
// Wrap existing widgets with consistent styling
pub struct ThemedList<'a> {
    inner: List<'a>,
    theme: &'a Theme,
}

impl<'a> ThemedList<'a> {
    pub fn new(items: Vec<ListItem<'a>>, theme: &'a Theme) -> Self {
        let inner = List::new(items)
            .highlight_style(theme.highlight_style())
            .block(Block::bordered().border_style(theme.border_style()));
        Self { inner, theme }
    }
}

impl Widget for ThemedList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.inner.render(area, buf);
    }
}
```

---

## 8. Color/Theme Systems

### Pattern A: Centralized Theme Struct (Your Approach)

```rust
// Your approach is solid
pub struct Theme {
    pub realm_colors: HashMap<String, Color>,
    pub layer_colors: HashMap<String, Color>,
    pub border_style: Style,
    pub highlight_style: Style,
    // ...
}

impl Theme {
    pub fn with_root(root: &str) -> Self {
        // Load from YAML/TOML
        let encoding = load_visual_encoding(root);
        Self::from_encoding(encoding)
    }
}
```

### Pattern B: Style Presets (gitui)

```rust
// gitui loads themes from TOML
pub struct Theme {
    selected_tab: Style,
    command_fg: Color,
    selection_bg: Color,
    // ... many style fields
}

// Used directly in rendering
fn render(&self, theme: &SharedTheme) {
    let style = if focused {
        theme.title(true)
    } else {
        theme.title(false)
    };
}
```

### Pattern C: Palette + Semantic Styles

```rust
// Define palette, then semantic styles
pub struct Palette {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub surface: Color,
    pub text: Color,
}

pub struct Theme {
    palette: Palette,
}

impl Theme {
    // Semantic styles derived from palette
    pub fn header_style(&self) -> Style {
        Style::default().fg(self.palette.primary).bold()
    }

    pub fn selected_style(&self) -> Style {
        Style::default().bg(self.palette.surface).fg(self.palette.text)
    }
}
```

### Dark/Light Mode Detection

```rust
// Detect terminal color mode
pub fn detect_color_mode() -> ColorMode {
    // Check COLORFGBG environment variable
    if let Ok(val) = std::env::var("COLORFGBG") {
        if val.ends_with("15") || val.ends_with("7") {
            return ColorMode::Light;
        }
    }
    // Check terminal-specific vars
    if std::env::var("TERM_PROGRAM").ok().as_deref() == Some("Apple_Terminal") {
        // Apple Terminal defaults to light
    }
    ColorMode::Dark // Default assumption
}
```

---

## Actionable Recommendations for NovaNet

### Quick Wins (Low Effort, High Value)

1. **Adopt `Rect::layout()`** - Replace manual constraint splitting
   ```rust
   // BEFORE
   let chunks = Layout::vertical([...]).split(area);
   let (tree_area, info_area) = (chunks[0], chunks[1]);

   // AFTER
   let [tree_area, info_area] = area.layout(&Layout::vertical([...]));
   ```

2. **Use `Rect::centered()`** - Simplify overlay positioning
   ```rust
   let popup_area = frame.area().centered(Size::new(60, 20));
   ```

3. **Add more TestBackend tests** - Buffer assertion for key widgets

### Medium-Term Improvements

4. **Consider extracting reusable widgets** - Your tree, info panel, etc. could be separate `StatefulWidget` implementations

5. **Add dirty tracking** - Mark panels dirty on state changes, skip rebuild when clean

### Architectural Considerations (If Scaling)

6. **Component trait** - Only if adding 5+ more independent panels or building widget library

7. **Action queue** - Only if debugging complex state transitions becomes difficult

---

## Resources

- ratatui 0.30 changelog: https://github.com/ratatui/ratatui/releases/tag/v0.30.0
- ratatui architecture: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md
- ratatui templates: https://github.com/ratatui/templates
- gitui source: https://github.com/extrawurst/gitui
- bottom source: https://github.com/ClementTsang/bottom
