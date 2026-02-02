//! Visual effects engine — config, state, and post-processing.
//!
//! All effects are toggleable via `EffectsConfig`. Active effects are tracked
//! in `EffectsState` and advanced each tick. Post-processing effects (CRT,
//! glitch) modify the ratatui Buffer after normal rendering.

use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::tui::theme;

// ─── Configuration ──────────────────────────────────────────────────────

/// Configuration for all visual effects (toggleable).
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields read by trigger methods in Batch 6 wow effects
pub struct EffectsConfig {
    pub boot_animation: bool,
    pub crt_scanlines: bool,
    pub screen_shake: bool,
    pub glitch_transitions: bool,
    pub nebula_pulse: bool,
    pub typewriter: bool,
}

impl Default for EffectsConfig {
    fn default() -> Self {
        Self {
            boot_animation: true,
            crt_scanlines: false, // off by default, toggle with Ctrl+R
            screen_shake: true,
            glitch_transitions: true,
            nebula_pulse: true,
            typewriter: true,
        }
    }
}

// ─── Effects State ──────────────────────────────────────────────────────

/// Active effects state — tracks running animations.
#[derive(Debug, Clone, Default)]
pub struct EffectsState {
    pub config: EffectsConfig,
    pub crt_enabled: bool,
    pub shake: Option<ScreenShake>,
    pub glitch: Option<GlitchTransition>,
    pub pulse: Option<NebulaPulse>,
}

impl EffectsState {
    /// Returns true if any transient effect is currently playing.
    pub fn is_animating(&self) -> bool {
        self.shake.is_some() || self.glitch.is_some() || self.pulse.is_some()
    }

    /// Advance all active effects by one frame. Expired effects are cleared.
    pub fn tick(&mut self) {
        if let Some(ref mut shake) = self.shake {
            shake.frames_remaining = shake.frames_remaining.saturating_sub(1);
            if shake.frames_remaining == 0 {
                self.shake = None;
            }
        }
        if let Some(ref mut glitch) = self.glitch {
            glitch.frames_remaining = glitch.frames_remaining.saturating_sub(1);
            if glitch.frames_remaining == 0 {
                self.glitch = None;
            }
        }
        if let Some(ref mut pulse) = self.pulse {
            if pulse.frame >= NebulaPulse::MAX_FRAMES {
                self.pulse = None;
            } else {
                pulse.frame += 1;
            }
        }
    }

    /// Trigger screen shake (if enabled in config).
    pub fn trigger_shake(&mut self) {
        if self.config.screen_shake {
            self.shake = Some(ScreenShake::new());
        }
    }

    /// Trigger glitch transition (if enabled in config).
    pub fn trigger_glitch(&mut self) {
        if self.config.glitch_transitions {
            self.glitch = Some(GlitchTransition::new());
        }
    }

    /// Trigger nebula pulse on panel focus (if enabled in config).
    pub fn trigger_pulse(&mut self) {
        if self.config.nebula_pulse {
            self.pulse = Some(NebulaPulse::new());
        }
    }

    /// Toggle CRT scanline mode.
    pub fn toggle_crt(&mut self) {
        self.crt_enabled = !self.crt_enabled;
    }
}

// ─── Screen Shake ───────────────────────────────────────────────────────

/// Screen shake effect — shifts render area for 3 frames after delete.
#[derive(Debug, Clone)]
pub struct ScreenShake {
    pub frames_remaining: u8,
    pub offsets: [(i16, i16); 3],
}

impl ScreenShake {
    pub const DURATION: u8 = 3;

    pub fn new() -> Self {
        Self {
            frames_remaining: Self::DURATION,
            offsets: [(-1, 0), (1, 1), (0, -1)],
        }
    }

    /// Current (dx, dy) offset for this frame.
    pub fn current_offset(&self) -> (i16, i16) {
        let idx = (Self::DURATION - self.frames_remaining) as usize;
        if idx < self.offsets.len() {
            self.offsets[idx]
        } else {
            (0, 0)
        }
    }
}

// ─── Glitch Transition ──────────────────────────────────────────────────

/// Glitch transition effect — corrupts display on mode switch for 4 frames.
#[derive(Debug, Clone)]
pub struct GlitchTransition {
    pub frames_remaining: u8,
    pub intensities: [f32; 4],
}

impl GlitchTransition {
    pub const DURATION: u8 = 4;

    pub fn new() -> Self {
        Self {
            frames_remaining: Self::DURATION,
            intensities: [0.05, 0.30, 0.15, 0.0],
        }
    }

    /// Current corruption intensity (0.0 to 1.0).
    pub fn current_intensity(&self) -> f32 {
        let idx = (Self::DURATION - self.frames_remaining) as usize;
        if idx < self.intensities.len() {
            self.intensities[idx]
        } else {
            0.0
        }
    }
}

/// Characters used for glitch corruption.
pub const GLITCH_CHARS: &[char] = &['░', '▒', '▓', '█', '╳', '┃', '━', '╋'];

/// Apply glitch corruption to a buffer. Uses a simple deterministic pattern
/// based on tick to avoid requiring `rand` in the hot path (rand used only
/// during boot for matrix rain).
pub fn apply_glitch(buf: &mut Buffer, intensity: f32, tick: u64) {
    if intensity <= 0.0 {
        return;
    }
    let area = buf.area;
    // Deterministic pseudo-random using tick + position
    for y in 0..area.height {
        for x in 0..area.width {
            let hash = pseudo_hash(x, y, tick);
            let threshold = (intensity * 1000.0) as u64;
            if hash % 1000 < threshold {
                if let Some(cell) = buf.cell_mut((x + area.x, y + area.y)) {
                    let char_idx = (hash / 1000) as usize % GLITCH_CHARS.len();
                    cell.set_char(GLITCH_CHARS[char_idx]);
                    // Purple-shift the color
                    let r = ((hash % 120) + 80) as u8;
                    let g = (hash % 60) as u8;
                    let b = ((hash % 135) + 120) as u8;
                    cell.set_fg(Color::Rgb(r, g, b));
                }
            }
        }
    }
}

// ─── CRT Scanlines ──────────────────────────────────────────────────────

/// Apply CRT scanline effect to the buffer.
///
/// Even rows are dimmed to 85%, with subtle per-row flicker based on tick.
pub fn apply_crt_scanlines(buf: &mut Buffer, tick: u64) {
    let area = buf.area;
    for y in 0..area.height {
        let scanline = if y % 2 == 0 { 0.85_f32 } else { 1.0 };
        // Subtle phosphor flicker per row
        let flicker = 1.0 - (((tick + y as u64) % 7) as f32 * 0.01);
        let factor = scanline * flicker;

        for x in 0..area.width {
            if let Some(cell) = buf.cell_mut((x + area.x, y + area.y)) {
                if let Color::Rgb(r, g, b) = cell.fg {
                    cell.set_fg(Color::Rgb(
                        (r as f32 * factor) as u8,
                        (g as f32 * factor) as u8,
                        (b as f32 * factor) as u8,
                    ));
                }
            }
        }
    }
}

// ─── Nebula Pulse ───────────────────────────────────────────────────────

/// Nebula pulse effect — border color cycle on panel focus change.
#[derive(Debug, Clone)]
pub struct NebulaPulse {
    pub frame: u8,
}

impl NebulaPulse {
    pub const MAX_FRAMES: u8 = 5;

    pub fn new() -> Self {
        Self { frame: 0 }
    }

    /// Border color for the current pulse frame.
    pub fn border_color(&self) -> Color {
        const PULSE: [Color; 6] = [
            theme::STAR_DIM,
            theme::NEBULA_INDIGO,
            theme::NEBULA_VIOLET,
            theme::NEBULA_PURPLE,
            theme::NEBULA_BLUE,
            theme::CYBER_CYAN,
        ];
        PULSE[self.frame.min(5) as usize]
    }

    /// Whether the border should be bold (at peak intensity).
    pub fn is_bold(&self) -> bool {
        self.frame == 3
    }
}

// ─── Typewriter ─────────────────────────────────────────────────────────

/// Typewriter effect — progressive text reveal character by character.
#[derive(Debug, Clone)]
pub struct Typewriter {
    full_text: String,
    chars_visible: usize,
    frames_per_char: u8,
    frame_counter: u8,
}

impl Typewriter {
    /// Create a new typewriter with the given text and speed.
    ///
    /// `frames_per_char`: number of animation frames between each character reveal.
    /// Lower = faster. Typical values: 1 (fast), 2 (normal), 4 (slow).
    pub fn new(text: impl Into<String>, frames_per_char: u8) -> Self {
        Self {
            full_text: text.into(),
            chars_visible: 0,
            frames_per_char: frames_per_char.max(1),
            frame_counter: 0,
        }
    }

    /// Advance by one frame. Returns true if still typing.
    pub fn tick(&mut self) -> bool {
        if self.chars_visible >= self.full_text.len() {
            return false;
        }
        self.frame_counter += 1;
        if self.frame_counter >= self.frames_per_char {
            self.frame_counter = 0;
            self.chars_visible += 1;
        }
        true
    }

    /// The currently visible portion of the text.
    pub fn visible(&self) -> &str {
        // Safe: we only increment by 1 from 0..len
        let end = self.chars_visible.min(self.full_text.len());
        // Find the char boundary at `end` chars
        let byte_end = self
            .full_text
            .char_indices()
            .nth(end)
            .map(|(i, _)| i)
            .unwrap_or(self.full_text.len());
        &self.full_text[..byte_end]
    }

    /// Whether typing is complete.
    pub fn is_done(&self) -> bool {
        self.chars_visible >= self.full_text.len()
    }

    /// Show all text immediately (skip animation).
    pub fn complete(&mut self) {
        self.chars_visible = self.full_text.len();
    }

    /// The full text (for width calculations).
    #[allow(dead_code)] // Used by Batch 7 onboarding typewriter layout
    pub fn full_text(&self) -> &str {
        &self.full_text
    }

    /// Cursor character (block while typing, empty when done).
    pub fn cursor_char(&self) -> Option<char> {
        if self.is_done() { None } else { Some('█') }
    }
}

// ─── Screen Shake Post-Processing ───────────────────────────────────────

/// Apply screen shake by shifting all buffer content by (dx, dy).
///
/// Vacated cells are filled with BG_VOID. This is a post-processing effect
/// applied after normal rendering.
pub fn apply_shake(buf: &mut Buffer, dx: i16, dy: i16) {
    if dx == 0 && dy == 0 {
        return;
    }
    let area = buf.area;

    // Snapshot all cells (symbol + colors)
    let mut snapshot: Vec<(String, Color, Color)> =
        Vec::with_capacity((area.width as usize) * (area.height as usize));
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell((x, y)) {
                snapshot.push((cell.symbol().to_string(), cell.fg, cell.bg));
            } else {
                snapshot.push((" ".to_string(), theme::STAR_DIM, theme::BG_VOID));
            }
        }
    }

    // Clear all cells to void
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char(' ');
                cell.set_fg(theme::STAR_DIM);
                cell.set_bg(theme::BG_VOID);
            }
        }
    }

    // Write back shifted by (dx, dy)
    for sy in 0..area.height {
        for sx in 0..area.width {
            let dst_x = sx as i16 + dx;
            let dst_y = sy as i16 + dy;
            if dst_x >= 0 && dst_x < area.width as i16 && dst_y >= 0 && dst_y < area.height as i16 {
                let src_idx = (sy as usize) * (area.width as usize) + (sx as usize);
                let (ref sym, fg, bg) = snapshot[src_idx];
                if let Some(cell) = buf.cell_mut((dst_x as u16 + area.x, dst_y as u16 + area.y)) {
                    cell.set_symbol(sym);
                    cell.set_fg(fg);
                    cell.set_bg(bg);
                }
            }
        }
    }
}

// ─── Helpers ────────────────────────────────────────────────────────────

/// Simple deterministic hash for pseudo-random effects without requiring `rand`.
fn pseudo_hash(x: u16, y: u16, tick: u64) -> u64 {
    let mut h = (x as u64) * 2654435761 + (y as u64) * 40503 + tick * 12345;
    h ^= h >> 16;
    h = h.wrapping_mul(0x45d9f3b);
    h ^= h >> 16;
    h
}

// ─── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effects_config_defaults() {
        let cfg = EffectsConfig::default();
        assert!(cfg.boot_animation);
        assert!(!cfg.crt_scanlines);
        assert!(cfg.screen_shake);
        assert!(cfg.glitch_transitions);
        assert!(cfg.nebula_pulse);
        assert!(cfg.typewriter);
    }

    #[test]
    fn effects_state_default_not_animating() {
        let state = EffectsState::default();
        assert!(!state.is_animating());
        assert!(!state.crt_enabled);
    }

    #[test]
    fn effects_state_animating_when_shake_active() {
        let mut state = EffectsState::default();
        state.trigger_shake();
        assert!(state.is_animating());
    }

    #[test]
    fn effects_state_animating_when_glitch_active() {
        let mut state = EffectsState::default();
        state.trigger_glitch();
        assert!(state.is_animating());
    }

    #[test]
    fn effects_state_animating_when_pulse_active() {
        let mut state = EffectsState::default();
        state.trigger_pulse();
        assert!(state.is_animating());
    }

    #[test]
    fn shake_expires_after_duration() {
        let mut state = EffectsState::default();
        state.trigger_shake();
        for _ in 0..ScreenShake::DURATION {
            assert!(state.shake.is_some());
            state.tick();
        }
        // After DURATION ticks, shake should be cleared
        assert!(state.shake.is_none());
    }

    #[test]
    fn shake_current_offset() {
        let shake = ScreenShake::new();
        assert_eq!(shake.current_offset(), (-1, 0));
    }

    #[test]
    fn glitch_expires_after_duration() {
        let mut state = EffectsState::default();
        state.trigger_glitch();
        for _ in 0..GlitchTransition::DURATION {
            assert!(state.glitch.is_some());
            state.tick();
        }
        assert!(state.glitch.is_none());
    }

    #[test]
    fn glitch_intensity_curve() {
        let glitch = GlitchTransition::new();
        assert!((glitch.current_intensity() - 0.05).abs() < 0.001);
    }

    #[test]
    fn pulse_expires_after_max_frames() {
        let mut state = EffectsState::default();
        state.trigger_pulse();
        for _ in 0..=NebulaPulse::MAX_FRAMES {
            assert!(state.pulse.is_some());
            state.tick();
        }
        assert!(state.pulse.is_none());
    }

    #[test]
    fn pulse_border_color_sequence() {
        let pulse = NebulaPulse::new();
        assert_eq!(pulse.border_color(), theme::STAR_DIM);
        assert!(!pulse.is_bold());
    }

    #[test]
    fn pulse_bold_at_peak() {
        let pulse = NebulaPulse { frame: 3 };
        assert!(pulse.is_bold());
    }

    #[test]
    fn crt_toggle() {
        let mut state = EffectsState::default();
        assert!(!state.crt_enabled);
        state.toggle_crt();
        assert!(state.crt_enabled);
        state.toggle_crt();
        assert!(!state.crt_enabled);
    }

    #[test]
    fn shake_not_triggered_when_disabled() {
        let mut state = EffectsState::default();
        state.config.screen_shake = false;
        state.trigger_shake();
        assert!(state.shake.is_none());
    }

    #[test]
    fn glitch_not_triggered_when_disabled() {
        let mut state = EffectsState::default();
        state.config.glitch_transitions = false;
        state.trigger_glitch();
        assert!(state.glitch.is_none());
    }

    #[test]
    fn pulse_not_triggered_when_disabled() {
        let mut state = EffectsState::default();
        state.config.nebula_pulse = false;
        state.trigger_pulse();
        assert!(state.pulse.is_none());
    }

    // ─── Typewriter tests ───────────────────────────────────────────

    #[test]
    fn typewriter_starts_empty() {
        let tw = Typewriter::new("hello", 1);
        assert_eq!(tw.visible(), "");
        assert!(!tw.is_done());
        assert!(tw.cursor_char().is_some());
    }

    #[test]
    fn typewriter_reveals_chars() {
        let mut tw = Typewriter::new("abc", 1);
        tw.tick();
        assert_eq!(tw.visible(), "a");
        tw.tick();
        assert_eq!(tw.visible(), "ab");
        tw.tick();
        assert_eq!(tw.visible(), "abc");
        assert!(tw.is_done());
        assert!(tw.cursor_char().is_none());
    }

    #[test]
    fn typewriter_speed_control() {
        let mut tw = Typewriter::new("ab", 3);
        // Need 3 ticks per char
        assert!(tw.tick()); // frame_counter=1
        assert_eq!(tw.visible(), "");
        assert!(tw.tick()); // frame_counter=2
        assert_eq!(tw.visible(), "");
        assert!(tw.tick()); // frame_counter=0, chars_visible=1
        assert_eq!(tw.visible(), "a");
    }

    #[test]
    fn typewriter_complete_skips() {
        let mut tw = Typewriter::new("hello world", 2);
        tw.complete();
        assert!(tw.is_done());
        assert_eq!(tw.visible(), "hello world");
    }

    #[test]
    fn typewriter_full_text() {
        let tw = Typewriter::new("test", 1);
        assert_eq!(tw.full_text(), "test");
    }

    #[test]
    fn typewriter_unicode() {
        let mut tw = Typewriter::new("◉★", 1);
        tw.tick();
        assert_eq!(tw.visible(), "◉");
        tw.tick();
        assert_eq!(tw.visible(), "◉★");
    }

    // ─── Post-processing tests ──────────────────────────────────────

    #[test]
    fn glitch_zero_intensity_noop() {
        let area = ratatui::layout::Rect::new(0, 0, 10, 5);
        let mut buf = Buffer::empty(area);
        let before = buf.content().to_vec();
        apply_glitch(&mut buf, 0.0, 0);
        assert_eq!(buf.content(), &before[..]);
    }

    #[test]
    fn pseudo_hash_deterministic() {
        let h1 = pseudo_hash(5, 10, 42);
        let h2 = pseudo_hash(5, 10, 42);
        assert_eq!(h1, h2);
        // Different inputs → different output (probabilistic but very likely)
        let h3 = pseudo_hash(6, 10, 42);
        assert_ne!(h1, h3);
    }

    #[test]
    fn tick_clears_all_expired_effects() {
        let mut state = EffectsState::default();
        state.trigger_shake();
        state.trigger_glitch();
        state.trigger_pulse();

        // Tick enough to expire all
        for _ in 0..10 {
            state.tick();
        }
        assert!(!state.is_animating());
    }
}
