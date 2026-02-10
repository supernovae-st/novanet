# Nexus Visual Effects with Ratatui

## Widget Catalog for WOW Effects

### 1. Progress Bars (XP & Mission Progress)

```rust
use ratatui::widgets::{Gauge, LineGauge, Block};
use ratatui::style::{Style, Color, Modifier};

// XP Bar with gradient effect
Gauge::default()
    .block(Block::bordered().title("⚡ XP PROGRESS"))
    .gauge_style(Style::new()
        .fg(Color::Cyan)
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD))
    .percent(75)
    .use_unicode(true)  // Smooth progress with Unicode blocks
    .label("1,847 / 2,500 XP");

// Mission progress (thin line)
LineGauge::default()
    .block(Block::bordered().title("MISSION 3/6"))
    .filled_style(Style::new().fg(Color::Green).bg(Color::Black))
    .unfilled_style(Style::new().fg(Color::DarkGray))
    .filled_symbol("━")
    .unfilled_symbol("─")
    .ratio(0.5);
```

### 2. Sparklines (Activity/Streak History)

```rust
use ratatui::widgets::{Sparkline, SparklineBar};

// Streak history visualization
let streak_data: Vec<SparklineBar> = vec![
    SparklineBar::from(3).style(Some(Style::default().fg(Color::Yellow))),  // Day 1
    SparklineBar::from(5).style(Some(Style::default().fg(Color::Yellow))),  // Day 2
    SparklineBar::from(7).style(Some(Style::default().fg(Color::Red))),     // Day 3 🔥
    SparklineBar::from(10).style(Some(Style::default().fg(Color::Magenta))), // Day 4 🔥🔥
];

Sparkline::default()
    .block(Block::bordered().title("🔥 STREAK HISTORY"))
    .data(streak_data)
    .style(Style::default().fg(Color::Yellow));
```

### 3. Boot Sequence Animation

```rust
// Frame-by-frame boot animation
const BOOT_FRAMES: &[&str] = &[
    "ESTABLISHING NEURAL LINK",
    "ESTABLISHING NEURAL LINK.",
    "ESTABLISHING NEURAL LINK..",
    "ESTABLISHING NEURAL LINK...",
];

// Loading bar with custom symbols
fn render_boot_progress(progress: f64) -> String {
    let filled = (progress * 30.0) as usize;
    let empty = 30 - filled;
    format!(
        "[{}{}] {:>3.0}%",
        "█".repeat(filled),
        "░".repeat(empty),
        progress * 100.0
    )
}

// Typing reveal effect
fn reveal_text(text: &str, tick: usize) -> String {
    text.chars().take(tick).collect()
}
```

### 4. Achievement Unlock Animation

```rust
// Full-screen takeover for achievement
fn render_achievement_unlock(frame: &mut Frame, achievement: &Achievement, tick: u16) {
    let area = frame.area();

    // Pulsing border effect
    let border_color = if tick % 10 < 5 {
        Color::Yellow
    } else {
        Color::Rgb(255, 215, 0)  // Gold
    };

    // Centered achievement card
    let card = Block::bordered()
        .title("🏆 ACHIEVEMENT UNLOCKED 🏆")
        .title_style(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .border_style(Style::new().fg(border_color));

    // ASCII art badge
    let badge_art = vec![
        "    ╔═══════════╗    ",
        "   ╔╝           ╚╗   ",
        "  ╔╝  ★ ★ ★ ★ ★  ╚╗  ",
        " ╔╝               ╚╗ ",
        " ║   ARC NAVIGATOR  ║ ",
        " ╚╗               ╔╝ ",
        "  ╚╗             ╔╝  ",
        "   ╚═════════════╝   ",
    ];

    // XP gained with animation
    let xp_text = format!("+{} XP", achievement.xp());
}
```

### 5. Color Palette (Galaxy Theme)

```rust
// Nexus color constants
pub mod nexus_colors {
    use ratatui::style::Color;

    // Primary actions
    pub const SUCCESS: Color = Color::Rgb(16, 185, 129);   // #10b981 emerald
    pub const WARNING: Color = Color::Rgb(245, 158, 11);   // #f59e0b amber
    pub const ERROR: Color = Color::Rgb(239, 68, 68);      // #ef4444 red
    pub const INFO: Color = Color::Rgb(59, 130, 246);      // #3b82f6 blue

    // Ranks
    pub const CADET: Color = Color::Rgb(156, 163, 175);    // #9ca3af gray
    pub const NAVIGATOR: Color = Color::Rgb(34, 197, 94); // #22c55e green
    pub const COMMANDER: Color = Color::Rgb(59, 130, 246); // #3b82f6 blue
    pub const CAPTAIN: Color = Color::Rgb(168, 85, 247);   // #a855f7 purple
    pub const ADMIRAL: Color = Color::Rgb(236, 72, 153);   // #ec4899 pink
    pub const LEGEND: Color = Color::Rgb(251, 191, 36);    // #fbbf24 gold

    // XP bar gradient
    pub const XP_LOW: Color = Color::Rgb(34, 197, 94);     // green
    pub const XP_MID: Color = Color::Rgb(250, 204, 21);    // yellow
    pub const XP_HIGH: Color = Color::Rgb(249, 115, 22);   // orange
    pub const XP_MAX: Color = Color::Rgb(239, 68, 68);     // red (near level up)

    // Streaks
    pub const STREAK_COLD: Color = Color::Rgb(156, 163, 175);   // gray
    pub const STREAK_WARM: Color = Color::Rgb(251, 191, 36);    // yellow
    pub const STREAK_HOT: Color = Color::Rgb(249, 115, 22);     // orange
    pub const STREAK_FIRE: Color = Color::Rgb(239, 68, 68);     // red 🔥
}
```

### 6. Flash Effects (Success/Error)

```rust
// Screen flash on correct answer
pub struct FlashEffect {
    pub color: Color,
    pub frames_remaining: u8,
}

impl FlashEffect {
    pub fn success() -> Self {
        Self { color: Color::Green, frames_remaining: 3 }
    }

    pub fn error() -> Self {
        Self { color: Color::Red, frames_remaining: 3 }
    }

    pub fn achievement() -> Self {
        Self { color: Color::Yellow, frames_remaining: 5 }
    }

    pub fn tick(&mut self) -> bool {
        if self.frames_remaining > 0 {
            self.frames_remaining -= 1;
            true
        } else {
            false
        }
    }
}

// Apply flash to entire screen
fn render_with_flash(frame: &mut Frame, flash: &Option<FlashEffect>) {
    if let Some(effect) = flash {
        let overlay = Block::default()
            .style(Style::default().bg(effect.color));
        frame.render_widget(overlay, frame.area());
    }
}
```

### 7. Particle Effects (Future: firework-rs)

```rust
// Placeholder for firework-rs integration
// TODO: Add confetti on major achievements
pub struct ParticleSystem {
    particles: Vec<Particle>,
    active: bool,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    char: char,
    color: Color,
    life: u8,
}

// Simple star burst effect
fn create_starburst(center_x: u16, center_y: u16) -> Vec<Particle> {
    let chars = ['*', '✦', '✧', '·', '°'];
    let colors = [Color::Yellow, Color::Cyan, Color::Magenta, Color::White];
    // ... particle generation
    vec![]
}
```

### 8. Animated Counters

```rust
// Counting up XP animation
pub struct AnimatedCounter {
    current: u32,
    target: u32,
    step: u32,
}

impl AnimatedCounter {
    pub fn new(from: u32, to: u32, duration_frames: u32) -> Self {
        let step = (to - from) / duration_frames.max(1);
        Self { current: from, target: to, step: step.max(1) }
    }

    pub fn tick(&mut self) -> u32 {
        if self.current < self.target {
            self.current = (self.current + self.step).min(self.target);
        }
        self.current
    }

    pub fn is_done(&self) -> bool {
        self.current >= self.target
    }
}

// Usage: +847 XP counting up
let mut xp_counter = AnimatedCounter::new(0, 847, 30);
let display = format!("+{} XP", xp_counter.tick());
```

### 9. Pulsing Elements

```rust
// Pulsing effect for important elements
fn pulse_color(base: Color, tick: u16, speed: u16) -> Color {
    let intensity = ((tick % speed) as f32 / speed as f32 * std::f32::consts::PI).sin();
    let factor = 0.5 + intensity * 0.5;

    match base {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * factor) as u8,
            (g as f32 * factor) as u8,
            (b as f32 * factor) as u8,
        ),
        _ => base,
    }
}

// Blinking cursor for active input
fn blink_cursor(tick: u16) -> &'static str {
    if tick % 20 < 10 { "█" } else { " " }
}
```

### 10. Matrix Rain Effect (Boot Sequence)

```rust
// Digital rain background during boot
pub struct MatrixRain {
    columns: Vec<RainColumn>,
    width: u16,
}

struct RainColumn {
    chars: Vec<char>,
    y_offset: i16,
    speed: u8,
}

impl MatrixRain {
    pub fn new(width: u16, height: u16) -> Self {
        let katakana = "アイウエオカキクケコサシスセソタチツテト";
        // ... initialization
        Self { columns: vec![], width }
    }

    pub fn tick(&mut self) {
        for col in &mut self.columns {
            col.y_offset += col.speed as i16;
        }
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        // Render green falling characters
        for (x, col) in self.columns.iter().enumerate() {
            // ... render logic
        }
    }
}
```

## Animation Timing

| Effect | Duration | Frames | Use Case |
|--------|----------|--------|----------|
| Flash | 100ms | 3 | Correct/wrong answer |
| Achievement | 2s | 60 | Major unlock |
| XP counter | 1s | 30 | Points gained |
| Boot sequence | 3s | 90 | App startup |
| Pulse | Continuous | 20/cycle | Active elements |
| Typing reveal | Variable | 1 char/2 frames | Hacker effect |

## Implementation Priority

1. **P0**: Gauge/LineGauge for XP bars
2. **P0**: Flash effects for feedback
3. **P1**: Animated counters
4. **P1**: Pulsing borders
5. **P2**: Sparklines for history
6. **P2**: Boot sequence
7. **P3**: Matrix rain
8. **P3**: Particle effects
