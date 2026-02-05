# TUI Mode-Aware Right Panel Design

**Date:** 2026-02-05
**Status:** Approved
**Version:** v10.6.1

## Overview

The right panel content adapts based on selection type:
- **Kind** (Meta or Data mode) → YAML schema
- **Instance** (Data mode) → JSON data from Neo4j with syntax highlighting

## Design Decisions

### 1. Content by Selection Type

| Mode | Selection | Right Panel |
|------|-----------|-------------|
| Meta | Kind | YAML schema |
| Meta | Realm/Layer | YAML meta file |
| Data | Kind | YAML schema |
| Data | Instance | **JSON data (NEW)** |

### 2. JSON Formatting

Indented JSON with syntax highlighting:

```json
{
  "key": "ar-LY",                    // cyan (keys)
  "display_name": "Arabic (Libya)",  // green (strings)
  "is_primary": false,               // yellow (booleans)
  "language_code": "ar",             // green (strings)
  "created_at": "2024-01-15T..."     // magenta (dates)
}
```

Color mapping:
- Keys: Cyan (`Color::Cyan`)
- Strings: Green (`Color::Green`)
- Numbers: Yellow (`Color::Yellow`)
- Booleans: Yellow (`Color::Yellow`)
- Null: DarkGray (`Color::DarkGray`)
- Dates: Magenta (`Color::Magenta`)
- Brackets/Punctuation: White (`Color::White`)

### 3. Visual Indicator

Border color indicates content type:
- YAML → Gray border (current behavior)
- JSON → Cyan border

### 4. Mode Switch Behavior

When switching from Data (Instance) to Meta:
- Cursor moves to the **Kind** of the current instance
- Allows quick comparison: JSON data ↔ YAML schema

Example:
```
[2] Data on ar-LY → [1] Meta → Cursor on "Locale" Kind
```

## Implementation

### Files to Modify

1. **ui.rs** - `render_yaml_panel()`
   - Add branch for Instance in Data mode
   - Render JSON instead of YAML
   - Change border color to cyan

2. **app.rs** - Mode switch handlers
   - When switching Meta→Data or Data→Meta on Instance
   - Preserve Kind context for cursor positioning

3. **data.rs** - Add `Instance::to_colored_json()`
   - Serialize properties to formatted JSON
   - Return Vec<Line> with syntax highlighting

### New Function: `render_json_panel()`

```rust
fn render_json_panel(f: &mut Frame, area: Rect, instance: &InstanceData, app: &App) {
    let border_color = Color::Cyan; // JSON indicator

    let block = Block::default()
        .title(format!(" {} ", instance.key))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let json_lines = instance.to_colored_json();
    let paragraph = Paragraph::new(json_lines)
        .scroll((app.yaml_scroll as u16, 0));

    f.render_widget(block, area);
    f.render_widget(paragraph, block.inner(area));
}
```

### JSON Colorization Logic

```rust
impl InstanceData {
    pub fn to_colored_json(&self) -> Vec<Line<'static>> {
        let mut lines = vec![Line::from(Span::styled("{", Style::default().fg(Color::White)))];

        for (i, (key, value)) in self.properties.iter().enumerate() {
            let comma = if i < self.properties.len() - 1 { "," } else { "" };

            let value_span = match value {
                Value::String(s) => Span::styled(format!("\"{}\"", s), Style::default().fg(Color::Green)),
                Value::Bool(b) => Span::styled(b.to_string(), Style::default().fg(Color::Yellow)),
                Value::Number(n) => Span::styled(n.to_string(), Style::default().fg(Color::Yellow)),
                Value::Null => Span::styled("null", Style::default().fg(Color::DarkGray)),
                // ... handle dates, arrays, objects
            };

            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("\"{}\"", key), Style::default().fg(Color::Cyan)),
                Span::styled(": ", Style::default().fg(Color::White)),
                value_span,
                Span::styled(comma, Style::default().fg(Color::White)),
            ]));
        }

        lines.push(Line::from(Span::styled("}", Style::default().fg(Color::White))));
        lines
    }
}
```

## Testing

1. Navigate to Instance in Data mode → verify JSON displayed
2. Switch to Meta mode → verify YAML displayed, cursor on Kind
3. Scroll JSON with `[` and `]` keys
4. Verify all property types colorized correctly
