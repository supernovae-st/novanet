//! Persistence for Nexus learning progress.
//!
//! Saves tutorial progress and quiz scores to ~/.novanet/
//! so users can resume their learning journey.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::tutorial::TutorialState;

// =============================================================================
// PERSISTENCE STRUCTURES
// =============================================================================

/// Persisted tutorial progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialProgress {
    /// Schema version for migration.
    pub version: String,
    /// When the tutorial was started.
    pub started_at: Option<String>,
    /// When the tutorial was last updated.
    pub updated_at: Option<String>,
    /// Per-step progress.
    pub steps: Vec<StepProgress>,
    /// Quiz high score (if any).
    pub quiz_high_score: Option<usize>,
    /// Total time spent in minutes.
    pub total_time_minutes: usize,
    // ═══════════════════════════════════════════════════════════════════════════
    // STREAK SYSTEM (v0.12.0)
    // ═══════════════════════════════════════════════════════════════════════════
    /// Current streak (consecutive days of quiz activity).
    #[serde(default)]
    pub current_streak: usize,
    /// Best streak ever achieved.
    #[serde(default)]
    pub best_streak: usize,
    /// Last date quiz was completed (YYYY-MM-DD format).
    #[serde(default)]
    pub last_quiz_date: Option<String>,
    /// Total quizzes completed.
    #[serde(default)]
    pub total_quizzes_completed: usize,
}

/// Progress for a single tutorial step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepProgress {
    /// Step ID (1-indexed).
    pub id: usize,
    /// Step title.
    pub title: String,
    /// Whether the step is complete.
    pub completed: bool,
    /// When the step was completed.
    pub completed_at: Option<String>,
    /// Task completion status.
    pub tasks: Vec<bool>,
}

impl Default for TutorialProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl TutorialProgress {
    /// Create new TutorialProgress with default values.
    pub fn new() -> Self {
        Self {
            version: "1.1".to_string(), // v1.1 adds streak system
            started_at: None,
            updated_at: None,
            steps: Vec::new(),
            quiz_high_score: None,
            total_time_minutes: 0,
            current_streak: 0,
            best_streak: 0,
            last_quiz_date: None,
            total_quizzes_completed: 0,
        }
    }

    /// Get the path to the progress file.
    pub fn path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".novanet")
            .join("tutorial_progress.json")
    }

    /// Load progress from disk.
    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    /// Save progress to disk (atomic write via temp file + rename).
    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;

        // Atomic write: write to temp file, then rename
        // This prevents data corruption if process is interrupted during write
        let temp_path = path.with_extension("json.tmp");
        std::fs::write(&temp_path, &content)?;
        std::fs::rename(&temp_path, &path)
    }

    /// Update from TutorialState.
    pub fn update_from_state(&mut self, state: &TutorialState) {
        // Initialize steps if needed
        if self.steps.is_empty() {
            self.steps = super::tutorial::STEPS
                .iter()
                .map(|step| StepProgress {
                    id: step.id,
                    title: step.title.to_string(),
                    completed: false,
                    completed_at: None,
                    tasks: vec![false; step.tasks.len()],
                })
                .collect();
        }

        // Update step progress
        for (i, tasks) in state.tasks_completed.iter().enumerate() {
            if let Some(step) = self.steps.get_mut(i) {
                step.tasks = tasks.clone();
                let was_complete = step.completed;
                step.completed = tasks.iter().all(|&t| t);

                // Set completed_at if just completed
                if step.completed && !was_complete {
                    step.completed_at = Some(chrono_now());
                }
            }
        }

        // Update timestamp
        self.updated_at = Some(chrono_now());

        // Set started_at if not set
        if self.started_at.is_none() {
            self.started_at = Some(chrono_now());
        }
    }

    /// Convert to TutorialState.
    pub fn to_state(&self) -> TutorialState {
        let mut state = TutorialState::new();

        // Find current step (first incomplete or last if all complete)
        let mut current_step = 0;
        for (i, step) in self.steps.iter().enumerate() {
            if !step.completed {
                current_step = i;
                break;
            }
            if i == self.steps.len() - 1 && step.completed {
                current_step = i;
            }
        }
        state.current_step = current_step;

        // Copy task completion status
        for (i, step) in self.steps.iter().enumerate() {
            if let Some(tasks) = state.tasks_completed.get_mut(i) {
                for (j, &completed) in step.tasks.iter().enumerate() {
                    if let Some(task) = tasks.get_mut(j) {
                        *task = completed;
                    }
                }
            }
        }

        // Check completion
        state.complete = self.steps.iter().all(|s| s.completed);

        state
    }

    /// Update quiz high score.
    pub fn update_quiz_score(&mut self, score: usize) {
        if self.quiz_high_score.map(|s| score > s).unwrap_or(true) {
            self.quiz_high_score = Some(score);
            self.updated_at = Some(chrono_now());
        }
    }

    /// Check if tutorial has been started.
    pub fn has_started(&self) -> bool {
        self.started_at.is_some()
    }

    /// Get overall completion percentage.
    pub fn completion_percent(&self) -> usize {
        if self.steps.is_empty() {
            return 0;
        }

        let total_tasks: usize = self.steps.iter().map(|s| s.tasks.len()).sum();
        let completed_tasks: usize = self
            .steps
            .iter()
            .flat_map(|s| s.tasks.iter())
            .filter(|&&t| t)
            .count();

        if total_tasks == 0 {
            0
        } else {
            (completed_tasks * 100) / total_tasks
        }
    }
}

/// Get current timestamp in ISO 8601 format.
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Simple ISO 8601 format: 2026-02-12T15:30:00Z
    // (Not using chrono crate to avoid extra dependency)
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Calculate year/month/day from days since epoch (1970-01-01)
    let (year, month, day) = days_to_ymd(days_since_epoch as i64);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

/// Convert days since epoch to year/month/day.
fn days_to_ymd(days: i64) -> (i32, u32, u32) {
    // Simplified algorithm (doesn't handle all edge cases perfectly)
    let mut remaining = days;
    let mut year = 1970;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    let leap = is_leap_year(year);
    let month_days: [i64; 12] = if leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &days_in_month in &month_days {
        if remaining < days_in_month {
            break;
        }
        remaining -= days_in_month;
        month += 1;
    }

    let day = remaining + 1;

    (year, month, day as u32)
}

/// Check if a year is a leap year.
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_progress_new() {
        let progress = TutorialProgress::new();
        assert_eq!(progress.version, "1.0");
        assert!(progress.started_at.is_none());
        assert!(progress.steps.is_empty());
        assert!(progress.quiz_high_score.is_none());
    }

    #[test]
    fn test_tutorial_progress_path() {
        let path = TutorialProgress::path();
        assert!(path.ends_with("tutorial_progress.json"));
        assert!(path.to_string_lossy().contains(".novanet"));
    }

    #[test]
    fn test_update_from_state() {
        let mut progress = TutorialProgress::new();
        let mut state = TutorialState::new();

        // Complete first task
        state.tasks_completed[0][0] = true;

        progress.update_from_state(&state);

        assert!(progress.started_at.is_some());
        assert!(progress.updated_at.is_some());
        assert!(!progress.steps.is_empty());
        assert!(progress.steps[0].tasks[0]);
    }

    #[test]
    fn test_to_state() {
        let mut progress = TutorialProgress::new();
        progress.steps = vec![
            StepProgress {
                id: 1,
                title: "Step 1".to_string(),
                completed: true,
                completed_at: Some("2026-02-12T00:00:00Z".to_string()),
                tasks: vec![true, true, true],
            },
            StepProgress {
                id: 2,
                title: "Step 2".to_string(),
                completed: false,
                completed_at: None,
                tasks: vec![true, false, false],
            },
        ];

        let state = progress.to_state();
        assert_eq!(state.current_step, 1); // First incomplete step
        assert!(state.tasks_completed[0][0]);
    }

    #[test]
    fn test_update_quiz_score() {
        let mut progress = TutorialProgress::new();
        assert!(progress.quiz_high_score.is_none());

        progress.update_quiz_score(10);
        assert_eq!(progress.quiz_high_score, Some(10));

        // Lower score should not update
        progress.update_quiz_score(5);
        assert_eq!(progress.quiz_high_score, Some(10));

        // Higher score should update
        progress.update_quiz_score(15);
        assert_eq!(progress.quiz_high_score, Some(15));
    }

    #[test]
    fn test_has_started() {
        let mut progress = TutorialProgress::new();
        assert!(!progress.has_started());

        progress.started_at = Some("2026-02-12T00:00:00Z".to_string());
        assert!(progress.has_started());
    }

    #[test]
    fn test_completion_percent() {
        let mut progress = TutorialProgress::new();
        assert_eq!(progress.completion_percent(), 0);

        progress.steps = vec![
            StepProgress {
                id: 1,
                title: "Step 1".to_string(),
                completed: true,
                completed_at: None,
                tasks: vec![true, true], // 2/2
            },
            StepProgress {
                id: 2,
                title: "Step 2".to_string(),
                completed: false,
                completed_at: None,
                tasks: vec![true, false], // 1/2
            },
        ];

        // 3 out of 4 tasks = 75%
        assert_eq!(progress.completion_percent(), 75);
    }

    #[test]
    fn test_chrono_now_format() {
        let now = chrono_now();
        // Should match ISO 8601 format
        assert!(now.contains("T"));
        assert!(now.ends_with("Z"));
        assert_eq!(now.len(), 20);
    }

    #[test]
    fn test_days_to_ymd() {
        // 1970-01-01
        assert_eq!(days_to_ymd(0), (1970, 1, 1));

        // 2000-01-01 (10957 days after epoch)
        assert_eq!(days_to_ymd(10957), (2000, 1, 1));
    }

    #[test]
    fn test_is_leap_year() {
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(2000));
    }

    #[test]
    fn test_serialization() {
        let mut progress = TutorialProgress::new();
        progress.started_at = Some("2026-02-12T00:00:00Z".to_string());
        progress.quiz_high_score = Some(15);

        let json = serde_json::to_string(&progress).unwrap();
        let deserialized: TutorialProgress = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.version, progress.version);
        assert_eq!(deserialized.started_at, progress.started_at);
        assert_eq!(deserialized.quiz_high_score, progress.quiz_high_score);
    }
}
