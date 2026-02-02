//! Boot sequence: animated logo reveal with matrix rain.
//!
//! Six stages over ~90 frames (3 seconds at 30fps):
//! 1. MatrixRain — digital rain fills the screen
//! 2. RainClear — center clears in expanding zone
//! 3. LogoReveal — logo lines appear one by one
//! 4. BrandingType — subtitle types in via typewriter
//! 5. HoldLogo — static hold for a beat
//! 6. FadeOut — transition to dashboard

use crate::tui::effects::Typewriter;

// ─── Boot Stage State Machine ───────────────────────────────────────────

/// Boot animation stage.
#[derive(Debug, Clone)]
pub enum BootStage {
    /// Frames 0-30: Matrix rain fills the screen.
    MatrixRain,
    /// Frames 30-45: Center clears, rain slows.
    RainClear,
    /// Frames 45-60: Logo lines appear one by one.
    LogoReveal { lines_visible: usize },
    /// Frames 60-75: Subtitle types in character by character.
    BrandingType { typewriter: Typewriter },
    /// Frames 75-85: Hold the full logo for a beat.
    HoldLogo { frames_held: u8 },
    /// Frames 85-90: Fade out transition.
    FadeOut { frame: u8 },
    /// Boot complete — ready to transition to Ready state.
    Complete,
}

/// Frame thresholds for stage transitions.
const RAIN_END: u64 = 30;
const CLEAR_END: u64 = 45;
const REVEAL_END: u64 = 60;
const BRANDING_END: u64 = 75;
const HOLD_END: u64 = 85;
const FADE_END: u64 = 90;

/// Full boot state including rain columns and current stage.
#[derive(Debug, Clone)]
pub struct BootState {
    pub stage: BootStage,
    pub rain: MatrixRain,
    pub tick: u64,
    #[allow(dead_code)] // Used by Phase 7C onboarding layout
    pub width: u16,
    #[allow(dead_code)] // Used by Phase 7C onboarding layout
    pub height: u16,
}

impl BootState {
    /// Create a new boot state for the given terminal dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            stage: BootStage::MatrixRain,
            rain: MatrixRain::new(width, height),
            tick: 0,
            width,
            height,
        }
    }

    /// Advance boot animation by one frame.
    pub fn advance(&mut self) {
        self.tick += 1;
        self.rain.tick();

        match &mut self.stage {
            BootStage::MatrixRain => {
                if self.tick >= RAIN_END {
                    self.stage = BootStage::RainClear;
                }
            }
            BootStage::RainClear => {
                if self.tick >= CLEAR_END {
                    self.stage = BootStage::LogoReveal { lines_visible: 0 };
                }
            }
            BootStage::LogoReveal { lines_visible } => {
                // Reveal ~1 line per frame over 15 frames for 14-line logo
                let progress = self.tick - CLEAR_END;
                *lines_visible = (progress as usize).min(crate::tui::logo::FULL_LOGO.len());
                if self.tick >= REVEAL_END {
                    self.stage = BootStage::BrandingType {
                        typewriter: Typewriter::new("context graph engine v9.0", 2),
                    };
                }
            }
            BootStage::BrandingType { typewriter } => {
                typewriter.tick();
                if self.tick >= BRANDING_END {
                    typewriter.complete();
                    self.stage = BootStage::HoldLogo { frames_held: 0 };
                }
            }
            BootStage::HoldLogo { frames_held } => {
                *frames_held += 1;
                if self.tick >= HOLD_END {
                    self.stage = BootStage::FadeOut { frame: 0 };
                }
            }
            BootStage::FadeOut { frame } => {
                *frame += 1;
                if self.tick >= FADE_END {
                    self.stage = BootStage::Complete;
                }
            }
            BootStage::Complete => {}
        }
    }

    /// Whether the boot sequence is finished.
    pub fn is_complete(&self) -> bool {
        matches!(self.stage, BootStage::Complete)
    }

    /// Skip the boot animation entirely.
    pub fn skip(&mut self) {
        self.stage = BootStage::Complete;
    }

    /// How far the center clear has expanded (0.0 to 1.0).
    pub fn clear_progress(&self) -> f32 {
        if self.tick < RAIN_END {
            return 0.0;
        }
        if self.tick >= CLEAR_END {
            return 1.0;
        }
        (self.tick - RAIN_END) as f32 / (CLEAR_END - RAIN_END) as f32
    }

    /// How dim the fade-out is (0.0 = normal, 1.0 = fully faded).
    pub fn fade_progress(&self) -> f32 {
        match &self.stage {
            BootStage::FadeOut { frame } => *frame as f32 / (FADE_END - HOLD_END) as f32,
            BootStage::Complete => 1.0,
            _ => 0.0,
        }
    }
}

// ─── Matrix Rain ────────────────────────────────────────────────────────

/// Characters used for matrix rain — katakana subset + NovaNet glyphs + digits.
const RAIN_CHARS: &[char] = &[
    // Katakana subset
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
    'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', // NovaNet glyphs
    '◉', '◈', '⊕', '⊗', '▣', '⬡', // Digits
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

/// Matrix rain animation — vertical columns of falling characters.
#[derive(Debug, Clone)]
pub struct MatrixRain {
    pub columns: Vec<RainColumn>,
    #[allow(dead_code)] // Retained for resize support in Phase 7C
    width: u16,
    height: u16,
}

/// A single column of rain drops.
#[derive(Debug, Clone)]
pub struct RainColumn {
    pub x: u16,
    pub drops: Vec<RainDrop>,
}

/// A single falling character in the rain.
#[derive(Debug, Clone)]
pub struct RainDrop {
    pub y: f32,
    pub speed: f32,
    pub char_idx: usize,
    pub brightness: u8,
}

impl MatrixRain {
    /// Create rain for the given terminal dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        let mut columns = Vec::new();
        // Create a column every 2 cells (katakana are double-width)
        let mut x = 0u16;
        let mut seed: u64 = 42;
        while x < width {
            seed = simple_rng(seed);
            let speed_base = 0.5 + (seed % 150) as f32 / 100.0; // 0.5-2.0
            seed = simple_rng(seed);
            let start_y = -((seed % (height as u64 * 2)) as f32); // stagger start
            seed = simple_rng(seed);
            let char_idx = (seed % RAIN_CHARS.len() as u64) as usize;

            columns.push(RainColumn {
                x,
                drops: vec![RainDrop {
                    y: start_y,
                    speed: speed_base,
                    char_idx,
                    brightness: 255,
                }],
            });
            x += 2; // katakana width
        }
        Self {
            columns,
            width,
            height,
        }
    }

    /// Advance rain by one frame — drops fall, new drops spawn.
    pub fn tick(&mut self) {
        let height = self.height as f32;
        let mut seed = self.columns.len() as u64 * 997;

        for col in &mut self.columns {
            // Move existing drops
            for drop in &mut col.drops {
                drop.y += drop.speed;
                // Fade trail (brightness decreases behind head)
                if drop.brightness > 8 {
                    drop.brightness = drop.brightness.saturating_sub(6);
                }
            }

            // Remove drops that have fallen off screen (with trail)
            col.drops.retain(|d| d.y < height + 10.0);

            // Probabilistically spawn new drops at top
            seed = simple_rng(seed + col.x as u64);
            if col.drops.is_empty() || (seed % 20 == 0 && col.drops.len() < 5) {
                seed = simple_rng(seed);
                let speed = 0.5 + (seed % 150) as f32 / 100.0;
                seed = simple_rng(seed);
                let char_idx = (seed % RAIN_CHARS.len() as u64) as usize;
                col.drops.push(RainDrop {
                    y: -1.0,
                    speed,
                    char_idx,
                    brightness: 255,
                });
            }
        }
    }

    /// Get the rain character at (x, y) if one exists there.
    /// Returns (char, brightness) or None.
    pub fn char_at(&self, x: u16, y: u16) -> Option<(char, u8)> {
        for col in &self.columns {
            if col.x == x {
                for drop in &col.drops {
                    let drop_y = drop.y as i32;
                    if drop_y == y as i32 {
                        return Some((RAIN_CHARS[drop.char_idx], drop.brightness));
                    }
                    // Trail: dims behind the head
                    let dist = drop_y - y as i32;
                    if dist > 0 && dist < 8 {
                        let trail_brightness = drop.brightness.saturating_sub(dist as u8 * 30);
                        if trail_brightness > 10 {
                            return Some((
                                RAIN_CHARS[(drop.char_idx + dist as usize) % RAIN_CHARS.len()],
                                trail_brightness,
                            ));
                        }
                    }
                }
            }
        }
        None
    }

    /// Width used during creation.
    #[allow(dead_code)] // Retained for resize support in Phase 7C
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Height used during creation.
    #[allow(dead_code)] // Retained for resize support in Phase 7C
    pub fn height(&self) -> u16 {
        self.height
    }
}

// ─── Simple RNG ─────────────────────────────────────────────────────────

/// Simple deterministic PRNG (xorshift) — avoids requiring `rand` crate
/// for boot animation. Good enough for visual effects, not cryptographic.
fn simple_rng(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

// ─── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_state_new() {
        let boot = BootState::new(80, 24);
        assert!(matches!(boot.stage, BootStage::MatrixRain));
        assert_eq!(boot.tick, 0);
        assert!(!boot.is_complete());
    }

    #[test]
    fn boot_advances_through_stages() {
        let mut boot = BootState::new(80, 24);

        // Advance through MatrixRain
        for _ in 0..RAIN_END {
            boot.advance();
        }
        assert!(matches!(boot.stage, BootStage::RainClear));

        // Advance through RainClear
        for _ in RAIN_END..CLEAR_END {
            boot.advance();
        }
        assert!(matches!(boot.stage, BootStage::LogoReveal { .. }));

        // Advance through LogoReveal
        for _ in CLEAR_END..REVEAL_END {
            boot.advance();
        }
        assert!(matches!(boot.stage, BootStage::BrandingType { .. }));

        // Advance through BrandingType
        for _ in REVEAL_END..BRANDING_END {
            boot.advance();
        }
        assert!(matches!(boot.stage, BootStage::HoldLogo { .. }));

        // Advance through HoldLogo
        for _ in BRANDING_END..HOLD_END {
            boot.advance();
        }
        assert!(matches!(boot.stage, BootStage::FadeOut { .. }));

        // Advance through FadeOut
        for _ in HOLD_END..FADE_END {
            boot.advance();
        }
        assert!(boot.is_complete());
    }

    #[test]
    fn boot_skip() {
        let mut boot = BootState::new(80, 24);
        boot.skip();
        assert!(boot.is_complete());
    }

    #[test]
    fn boot_total_frames() {
        let mut boot = BootState::new(80, 24);
        let mut frames = 0;
        while !boot.is_complete() {
            boot.advance();
            frames += 1;
        }
        assert_eq!(frames, FADE_END);
    }

    #[test]
    fn clear_progress_bounds() {
        let mut boot = BootState::new(80, 24);
        assert_eq!(boot.clear_progress(), 0.0);

        // Advance to middle of clear
        for _ in 0..(RAIN_END + (CLEAR_END - RAIN_END) / 2) {
            boot.advance();
        }
        let p = boot.clear_progress();
        assert!(p > 0.0 && p < 1.0, "clear_progress={p}");

        // Advance past clear
        for _ in 0..50 {
            boot.advance();
        }
        assert_eq!(boot.clear_progress(), 1.0);
    }

    #[test]
    fn fade_progress_bounds() {
        let boot = BootState::new(80, 24);
        assert_eq!(boot.fade_progress(), 0.0);

        let complete_boot = BootState {
            stage: BootStage::Complete,
            rain: MatrixRain::new(80, 24),
            tick: 100,
            width: 80,
            height: 24,
        };
        assert_eq!(complete_boot.fade_progress(), 1.0);
    }

    #[test]
    fn matrix_rain_creates_columns() {
        let rain = MatrixRain::new(80, 24);
        assert!(!rain.columns.is_empty());
        // Columns every 2 cells → ~40 columns for width 80
        assert!(rain.columns.len() >= 30);
    }

    #[test]
    fn matrix_rain_tick_advances_drops() {
        let mut rain = MatrixRain::new(80, 24);
        let initial_y: Vec<f32> = rain.columns[0].drops.iter().map(|d| d.y).collect();
        rain.tick();
        let after_y: Vec<f32> = rain.columns[0].drops.iter().map(|d| d.y).collect();
        // First drop should have moved
        if !initial_y.is_empty() && !after_y.is_empty() {
            assert!(after_y[0] > initial_y[0] || after_y.len() != initial_y.len());
        }
    }

    #[test]
    fn rain_chars_not_empty() {
        assert!(!RAIN_CHARS.is_empty());
    }

    #[test]
    fn simple_rng_deterministic() {
        let a = simple_rng(42);
        let b = simple_rng(42);
        assert_eq!(a, b);
    }

    #[test]
    fn simple_rng_different_seeds() {
        let a = simple_rng(42);
        let b = simple_rng(43);
        assert_ne!(a, b);
    }

    #[test]
    fn logo_reveal_progresses() {
        let mut boot = BootState::new(80, 24);
        // Advance to logo reveal
        for _ in 0..CLEAR_END {
            boot.advance();
        }
        if let BootStage::LogoReveal { lines_visible } = &boot.stage {
            assert_eq!(*lines_visible, 0);
        } else {
            panic!("Expected LogoReveal stage");
        }
        // Advance a few frames
        for _ in 0..5 {
            boot.advance();
        }
        if let BootStage::LogoReveal { lines_visible } = &boot.stage {
            assert!(*lines_visible > 0);
        }
    }

    #[test]
    fn branding_typewriter_completes() {
        let mut boot = BootState::new(80, 24);
        // Advance to branding
        for _ in 0..REVEAL_END {
            boot.advance();
        }
        assert!(matches!(boot.stage, BootStage::BrandingType { .. }));
        // Advance through branding — typewriter should be completed by end
        for _ in REVEAL_END..BRANDING_END {
            boot.advance();
        }
        // Should have moved to HoldLogo
        assert!(matches!(boot.stage, BootStage::HoldLogo { .. }));
    }
}
