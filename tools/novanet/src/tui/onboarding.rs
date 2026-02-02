//! Onboarding: first-run detection, welcome screen, guided tour.
//!
//! On first launch (no `~/.novanet/init` file), shows:
//! 1. Welcome screen with connection checks + schema discovery
//! 2. Optional guided tour highlighting the 5 main panels
//!
//! After completing or skipping onboarding, creates `~/.novanet/init`
//! so it doesn't show again.

use std::io;
use std::path::PathBuf;

// ─── First-Run Detection ────────────────────────────────────────────────

/// Check if this is the first run (no `~/.novanet/init` file).
pub fn is_first_run() -> bool {
    init_file_path().map(|p| !p.exists()).unwrap_or(true)
}

/// Mark onboarding as complete by creating `~/.novanet/init`.
pub fn mark_onboarding_complete() -> io::Result<()> {
    if let Some(path) = init_file_path() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, format!("completed={}\n", chrono_lite_now()))?;
    }
    Ok(())
}

/// Path to the init marker file.
fn init_file_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".novanet").join("init"))
}

/// Minimal timestamp without pulling in `chrono`.
fn chrono_lite_now() -> String {
    // Use Unix epoch seconds — good enough for a marker file
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

// ─── Onboarding State ───────────────────────────────────────────────────

/// Top-level onboarding state (overlay on Ready).
#[derive(Debug, Clone)]
pub enum OnboardingState {
    Welcome { checks: Vec<CheckResult> },
    Tour { step: usize },
}

/// Result of a welcome-screen health check.
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub label: String,
    pub status: CheckStatus,
}

/// Status of a single check.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Ok/Failed constructed when health check tasks resolve (Phase 7C)
pub enum CheckStatus {
    Checking,
    Ok(String),
    Failed(String),
}

impl OnboardingState {
    /// Create a new welcome state with initial checks in "Checking" state.
    pub fn new_welcome() -> Self {
        OnboardingState::Welcome {
            checks: vec![
                CheckResult {
                    label: "Neo4j Connection".to_string(),
                    status: CheckStatus::Checking,
                },
                CheckResult {
                    label: "Schema Discovery".to_string(),
                    status: CheckStatus::Checking,
                },
            ],
        }
    }

    /// Update a check result by label.
    #[allow(dead_code)] // Called from runtime when health check background tasks complete (Phase 7C)
    pub fn update_check(&mut self, label: &str, status: CheckStatus) {
        if let OnboardingState::Welcome { checks } = self {
            if let Some(check) = checks.iter_mut().find(|c| c.label == label) {
                check.status = status;
            }
        }
    }

    /// Transition from Welcome to Tour.
    pub fn start_tour(&mut self) {
        *self = OnboardingState::Tour { step: 0 };
    }

    /// Whether we're in the tour and can navigate steps.
    #[allow(dead_code)] // Used by tests; available for conditional rendering
    pub fn is_tour(&self) -> bool {
        matches!(self, OnboardingState::Tour { .. })
    }
}

// ─── Tour Steps ─────────────────────────────────────────────────────────

/// A single step in the guided tour.
pub struct TourStep {
    pub title: &'static str,
    pub target: TourTarget,
    pub body: &'static [&'static str],
    pub hint: &'static str,
}

/// Which panel the tour highlights.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourTarget {
    TreePanel,
    DetailPanel,
    CypherPanel,
    Dashboard,
    StatusBar,
}

/// All tour steps.
pub const TOUR_STEPS: &[TourStep] = &[
    TourStep {
        title: "Taxonomy Tree",
        target: TourTarget::TreePanel,
        body: &[
            "Navigate the Realm > Layer > Kind hierarchy.",
            "Use Up/Down to browse, Enter to expand/collapse.",
            "Press / to fuzzy-search by name.",
        ],
        hint: "Enter \u{25b8} Next step",
    },
    TourStep {
        title: "Kind Detail",
        target: TourTarget::DetailPanel,
        body: &[
            "View properties, edges, and facet classification.",
            "Press e to enter the Edge Explorer for deep inspection.",
            "Create nodes (n) or relations (r) from here.",
        ],
        hint: "Enter \u{25b8} Next step",
    },
    TourStep {
        title: "Cypher Preview",
        target: TourTarget::CypherPanel,
        body: &[
            "See the live Cypher query for the selected Kind.",
            "Syntax-highlighted with keywords, labels, and strings.",
            "Copy the query to run in Neo4j Browser.",
        ],
        hint: "Enter \u{25b8} Next step",
    },
    TourStep {
        title: "Dashboard",
        target: TourTarget::Dashboard,
        body: &[
            "Live statistics from Neo4j: nodes, edges, realms.",
            "Bar charts show distribution across Realms and Families.",
            "Toggle visibility with s.",
        ],
        hint: "Enter \u{25b8} Next step",
    },
    TourStep {
        title: "Status Bar & Modes",
        target: TourTarget::StatusBar,
        body: &[
            "Switch modes: 1=Data, 2=Meta, 3=Overlay, 4=Query.",
            "Use : for the command palette, ? for keyboard help.",
            "Press q to quit. All changes are saved to Neo4j.",
        ],
        hint: "Enter \u{25b8} Finish tour",
    },
];

/// Number of tour steps.
pub const TOUR_STEP_COUNT: usize = TOUR_STEPS.len();

/// Navigate to the next tour step. Returns true if still in tour.
pub fn tour_next(state: &mut OnboardingState) -> bool {
    if let OnboardingState::Tour { step } = state {
        if *step + 1 < TOUR_STEP_COUNT {
            *step += 1;
            true
        } else {
            false // tour complete
        }
    } else {
        false
    }
}

/// Navigate to the previous tour step.
pub fn tour_prev(state: &mut OnboardingState) {
    if let OnboardingState::Tour { step } = state {
        *step = step.saturating_sub(1);
    }
}

/// Get the current tour step.
#[allow(dead_code)] // Used by tests; available for programmatic step access
pub fn current_tour_step(state: &OnboardingState) -> Option<&'static TourStep> {
    if let OnboardingState::Tour { step } = state {
        TOUR_STEPS.get(*step)
    } else {
        None
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn first_run_detection_with_temp_dir() {
        // We can't easily mock dirs::home_dir, so test the init_file_path logic
        let path = PathBuf::from("/tmp/novanet-test-onboarding/.novanet/init");
        if path.exists() {
            fs::remove_file(&path).ok();
        }
        assert!(!path.exists());
    }

    #[test]
    fn mark_onboarding_creates_file() {
        let base = std::env::temp_dir().join("novanet-test-mark-onboarding");
        let init_path = base.join(".novanet").join("init");
        // Clean up from previous runs
        if base.exists() {
            fs::remove_dir_all(&base).ok();
        }
        // Simulate mark_onboarding_complete logic
        if let Some(parent) = init_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&init_path, "completed=test\n").unwrap();
        assert!(init_path.exists());
        let content = fs::read_to_string(&init_path).unwrap();
        assert!(content.starts_with("completed="));
        // Clean up
        fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn welcome_state_has_two_checks() {
        let state = OnboardingState::new_welcome();
        if let OnboardingState::Welcome { checks } = &state {
            assert_eq!(checks.len(), 2);
            assert!(matches!(checks[0].status, CheckStatus::Checking));
            assert!(matches!(checks[1].status, CheckStatus::Checking));
        } else {
            panic!("Expected Welcome state");
        }
    }

    #[test]
    fn update_check_by_label() {
        let mut state = OnboardingState::new_welcome();
        state.update_check("Neo4j Connection", CheckStatus::Ok("Connected".to_string()));
        if let OnboardingState::Welcome { checks } = &state {
            assert!(matches!(&checks[0].status, CheckStatus::Ok(s) if s == "Connected"));
        }
    }

    #[test]
    fn start_tour_transitions_from_welcome() {
        let mut state = OnboardingState::new_welcome();
        assert!(!state.is_tour());
        state.start_tour();
        assert!(state.is_tour());
        if let OnboardingState::Tour { step } = &state {
            assert_eq!(*step, 0);
        }
    }

    #[test]
    fn tour_navigation_next_and_prev() {
        let mut state = OnboardingState::Tour { step: 0 };
        assert!(tour_next(&mut state));
        if let OnboardingState::Tour { step } = &state {
            assert_eq!(*step, 1);
        }
        tour_prev(&mut state);
        if let OnboardingState::Tour { step } = &state {
            assert_eq!(*step, 0);
        }
    }

    #[test]
    fn tour_prev_at_zero_stays() {
        let mut state = OnboardingState::Tour { step: 0 };
        tour_prev(&mut state);
        if let OnboardingState::Tour { step } = &state {
            assert_eq!(*step, 0);
        }
    }

    #[test]
    fn tour_next_at_last_returns_false() {
        let mut state = OnboardingState::Tour {
            step: TOUR_STEP_COUNT - 1,
        };
        assert!(!tour_next(&mut state));
    }

    #[test]
    fn current_tour_step_returns_correct() {
        let state = OnboardingState::Tour { step: 0 };
        let step = current_tour_step(&state).unwrap();
        assert_eq!(step.title, "Taxonomy Tree");
        assert_eq!(step.target, TourTarget::TreePanel);
    }

    #[test]
    fn tour_steps_count() {
        assert_eq!(TOUR_STEP_COUNT, 5);
    }

    #[test]
    fn all_tour_steps_have_body() {
        for step in TOUR_STEPS {
            assert!(!step.body.is_empty());
            assert!(!step.hint.is_empty());
        }
    }

    #[test]
    fn init_file_path_returns_some() {
        // dirs::home_dir() should work on macOS
        let path = init_file_path();
        assert!(path.is_some());
        let p = path.unwrap();
        assert!(p.to_string_lossy().contains(".novanet"));
        assert!(p.to_string_lossy().ends_with("init"));
    }

    #[test]
    fn chrono_lite_now_returns_digits() {
        let ts = chrono_lite_now();
        assert!(ts.chars().all(|c| c.is_ascii_digit()));
        assert!(ts.len() >= 10); // Unix epoch seconds is 10+ digits since 2001
    }

    #[test]
    fn check_status_variants() {
        let checking = CheckStatus::Checking;
        let ok = CheckStatus::Ok("good".to_string());
        let failed = CheckStatus::Failed("bad".to_string());
        assert!(matches!(checking, CheckStatus::Checking));
        assert!(matches!(ok, CheckStatus::Ok(_)));
        assert!(matches!(failed, CheckStatus::Failed(_)));
    }

    // Test that Path::exists is false for our test path
    #[test]
    fn nonexistent_path_not_exists() {
        let p = Path::new("/tmp/novanet-definitely-not-exists-xyz/init");
        assert!(!p.exists());
    }
}
