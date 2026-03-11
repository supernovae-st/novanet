//! Persistence for Nexus learning progress.
//!
//! Saves tutorial progress and quiz scores to ~/.novanet/
//! so users can resume their learning journey.
//!
//! v0.12.0: Added streak tracking and achievement system.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

use super::tutorial::TutorialState;

// =============================================================================
// ACHIEVEMENT DEFINITIONS (v0.12.0)
// =============================================================================

/// Achievement types that can be unlocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Achievement {
    /// First quiz completed.
    FirstQuiz,
    /// Perfect score on a quiz (100%).
    PerfectScore,
    /// Scored 90%+ on a quiz.
    Expert,
    /// 7-day streak.
    WeekStreak,
    /// 30-day streak.
    MonthStreak,
    /// Completed 10 quizzes.
    Dedicated,
    /// Completed 50 quizzes.
    Committed,
    /// Completed 100 quizzes.
    Master,
    /// Perfect score in all categories.
    CategoryMaster,
}

impl Achievement {
    /// Get achievement icon.
    pub fn icon(&self) -> &'static str {
        match self {
            Achievement::FirstQuiz => "🎯",
            Achievement::PerfectScore => "💯",
            Achievement::Expert => "⭐",
            Achievement::WeekStreak => "🔥",
            Achievement::MonthStreak => "🏆",
            Achievement::Dedicated => "📚",
            Achievement::Committed => "💪",
            Achievement::Master => "🎓",
            Achievement::CategoryMaster => "👑",
        }
    }

    /// Get achievement name.
    pub fn name(&self) -> &'static str {
        match self {
            Achievement::FirstQuiz => "First Steps",
            Achievement::PerfectScore => "Perfection",
            Achievement::Expert => "Expert",
            Achievement::WeekStreak => "On Fire",
            Achievement::MonthStreak => "Unstoppable",
            Achievement::Dedicated => "Dedicated",
            Achievement::Committed => "Committed",
            Achievement::Master => "Master",
            Achievement::CategoryMaster => "Category King",
        }
    }

    /// Get achievement description.
    pub fn description(&self) -> &'static str {
        match self {
            Achievement::FirstQuiz => "Complete your first quiz",
            Achievement::PerfectScore => "Score 100% on a quiz",
            Achievement::Expert => "Score 90%+ on a quiz",
            Achievement::WeekStreak => "7-day streak",
            Achievement::MonthStreak => "30-day streak",
            Achievement::Dedicated => "Complete 10 quizzes",
            Achievement::Committed => "Complete 50 quizzes",
            Achievement::Master => "Complete 100 quizzes",
            Achievement::CategoryMaster => "100% in all categories",
        }
    }

    /// All achievements in display order.
    pub fn all() -> &'static [Achievement] {
        &[
            Achievement::FirstQuiz,
            Achievement::PerfectScore,
            Achievement::Expert,
            Achievement::WeekStreak,
            Achievement::MonthStreak,
            Achievement::Dedicated,
            Achievement::Committed,
            Achievement::Master,
            Achievement::CategoryMaster,
        ]
    }
}

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
    // ═══════════════════════════════════════════════════════════════════════════
    // ACHIEVEMENT SYSTEM (v0.12.0)
    // ═══════════════════════════════════════════════════════════════════════════
    /// Unlocked achievements.
    #[serde(default)]
    pub achievements: HashSet<Achievement>,
    /// Newly unlocked achievements (shown once, then cleared).
    #[serde(skip)]
    pub new_achievements: Vec<Achievement>,
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
            version: "1.2".to_string(), // v1.2 adds achievement system
            started_at: None,
            updated_at: None,
            steps: Vec::new(),
            quiz_high_score: None,
            total_time_minutes: 0,
            current_streak: 0,
            best_streak: 0,
            last_quiz_date: None,
            total_quizzes_completed: 0,
            achievements: HashSet::new(),
            new_achievements: Vec::new(),
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

    /// Update streak when quiz is completed (v0.12.0).
    /// Call this when a quiz is finished (win or lose, streak counts participation).
    pub fn update_streak(&mut self) {
        let today = today_date();
        self.total_quizzes_completed += 1;

        if let Some(ref last_date) = self.last_quiz_date {
            let days_diff = days_between(last_date, &today);

            match days_diff {
                0 => {
                    // Same day - streak doesn't change (already played today)
                },
                1 => {
                    // Consecutive day - increment streak!
                    self.current_streak += 1;
                    if self.current_streak > self.best_streak {
                        self.best_streak = self.current_streak;
                    }
                },
                _ => {
                    // Streak broken - reset to 1 (today counts)
                    self.current_streak = 1;
                },
            }
        } else {
            // First quiz ever - start streak at 1
            self.current_streak = 1;
            if self.current_streak > self.best_streak {
                self.best_streak = self.current_streak;
            }
        }

        self.last_quiz_date = Some(today);
        self.updated_at = Some(chrono_now());
    }

    /// Get streak status message for display.
    pub fn streak_message(&self) -> String {
        match self.current_streak {
            0 => "Start your streak!".to_string(),
            1 => "🔥 1 day streak!".to_string(),
            n if n >= 7 => format!("🔥🔥 {} day streak! Amazing!", n),
            n if n >= 3 => format!("🔥 {} day streak! Keep going!", n),
            n => format!("🔥 {} day streak!", n),
        }
    }

    /// Check if streak is at risk (hasn't played today).
    pub fn streak_at_risk(&self) -> bool {
        if let Some(ref last_date) = self.last_quiz_date {
            let today = today_date();
            days_between(last_date, &today) >= 1
        } else {
            false // No streak to lose
        }
    }

    /// Check and unlock achievements based on current state (v0.12.0).
    /// Call after updating score, streak, etc.
    /// Returns list of newly unlocked achievements.
    pub fn check_achievements(
        &mut self,
        score: usize,
        total: usize,
        all_categories_perfect: bool,
    ) -> Vec<Achievement> {
        self.new_achievements.clear();

        let pct = if total > 0 {
            (score as f64 / total as f64 * 100.0) as u8
        } else {
            0
        };

        // First quiz
        if self.total_quizzes_completed >= 1 {
            self.try_unlock(Achievement::FirstQuiz);
        }

        // Perfect score
        if pct == 100 {
            self.try_unlock(Achievement::PerfectScore);
        }

        // Expert (90%+)
        if pct >= 90 {
            self.try_unlock(Achievement::Expert);
        }

        // Streak achievements
        if self.current_streak >= 7 {
            self.try_unlock(Achievement::WeekStreak);
        }
        if self.current_streak >= 30 {
            self.try_unlock(Achievement::MonthStreak);
        }

        // Quiz count achievements
        if self.total_quizzes_completed >= 10 {
            self.try_unlock(Achievement::Dedicated);
        }
        if self.total_quizzes_completed >= 50 {
            self.try_unlock(Achievement::Committed);
        }
        if self.total_quizzes_completed >= 100 {
            self.try_unlock(Achievement::Master);
        }

        // Category master (100% in all categories)
        if all_categories_perfect {
            self.try_unlock(Achievement::CategoryMaster);
        }

        self.new_achievements.clone()
    }

    /// Try to unlock an achievement. Returns true if newly unlocked.
    fn try_unlock(&mut self, achievement: Achievement) -> bool {
        if self.achievements.insert(achievement) {
            self.new_achievements.push(achievement);
            true
        } else {
            false
        }
    }

    /// Get count of unlocked achievements.
    pub fn achievement_count(&self) -> usize {
        self.achievements.len()
    }

    /// Check if an achievement is unlocked.
    pub fn has_achievement(&self, achievement: Achievement) -> bool {
        self.achievements.contains(&achievement)
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

/// Get today's date in YYYY-MM-DD format.
fn today_date() -> String {
    let now = chrono_now();
    // Extract date part (first 10 chars: YYYY-MM-DD)
    now.chars().take(10).collect()
}

/// Calculate days between two dates in YYYY-MM-DD format.
/// Returns 0 if same day, 1 if consecutive, etc.
fn days_between(date1: &str, date2: &str) -> i64 {
    // Parse dates
    let d1 = parse_date(date1).unwrap_or(0);
    let d2 = parse_date(date2).unwrap_or(0);

    // Return absolute difference
    (d2 - d1).abs()
}

/// Parse YYYY-MM-DD to days since epoch.
fn parse_date(date_str: &str) -> Option<i64> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    let year: i32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;

    Some(ymd_to_days(year, month, day))
}

/// Convert year/month/day to days since epoch.
fn ymd_to_days(year: i32, month: u32, day: u32) -> i64 {
    let mut days: i64 = 0;

    // Add days for years since 1970
    for y in 1970..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }

    // Add days for months
    let leap = is_leap_year(year);
    let month_days: [i64; 12] = if leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    for &day_count in month_days.iter().take(month.saturating_sub(1) as usize) {
        days += day_count;
    }

    // Add days
    days += (day.saturating_sub(1)) as i64;

    days
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
        assert_eq!(progress.version, "1.2"); // v1.2 adds achievement system
        assert!(progress.started_at.is_none());
        assert!(progress.steps.is_empty());
        assert!(progress.quiz_high_score.is_none());
        assert_eq!(progress.current_streak, 0);
        assert_eq!(progress.best_streak, 0);
        assert!(progress.last_quiz_date.is_none());
        assert_eq!(progress.total_quizzes_completed, 0);
        assert!(progress.achievements.is_empty());
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

    // ═══════════════════════════════════════════════════════════════════════════
    // STREAK TESTS (v0.12.0)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_streak_first_quiz() {
        let mut progress = TutorialProgress::new();
        assert_eq!(progress.current_streak, 0);
        assert_eq!(progress.total_quizzes_completed, 0);

        progress.update_streak();

        assert_eq!(progress.current_streak, 1);
        assert_eq!(progress.best_streak, 1);
        assert_eq!(progress.total_quizzes_completed, 1);
        assert!(progress.last_quiz_date.is_some());
    }

    #[test]
    fn test_streak_same_day() {
        let mut progress = TutorialProgress::new();
        let today = today_date();

        // First quiz
        progress.update_streak();
        assert_eq!(progress.current_streak, 1);

        // Second quiz same day
        progress.last_quiz_date = Some(today);
        progress.update_streak();

        // Streak doesn't change, but total increases
        assert_eq!(progress.current_streak, 1);
        assert_eq!(progress.total_quizzes_completed, 2);
    }

    #[test]
    fn test_streak_broken() {
        let mut progress = TutorialProgress::new();
        progress.current_streak = 5;
        progress.best_streak = 5;
        progress.last_quiz_date = Some("2026-02-10".to_string()); // 3 days ago

        progress.update_streak();

        // Streak resets to 1
        assert_eq!(progress.current_streak, 1);
        // Best streak preserved
        assert_eq!(progress.best_streak, 5);
    }

    #[test]
    fn test_streak_message() {
        let mut progress = TutorialProgress::new();

        assert_eq!(progress.streak_message(), "Start your streak!");

        progress.current_streak = 1;
        assert!(progress.streak_message().contains("1 day"));

        progress.current_streak = 5;
        assert!(progress.streak_message().contains("5 day"));

        progress.current_streak = 10;
        assert!(progress.streak_message().contains("Amazing"));
    }

    #[test]
    fn test_days_between() {
        assert_eq!(days_between("2026-02-12", "2026-02-12"), 0);
        assert_eq!(days_between("2026-02-12", "2026-02-13"), 1);
        assert_eq!(days_between("2026-02-10", "2026-02-13"), 3);
    }

    #[test]
    fn test_today_date_format() {
        let today = today_date();
        assert_eq!(today.len(), 10);
        assert!(today.contains('-'));
    }

    #[test]
    fn test_streak_serialization() {
        let mut progress = TutorialProgress::new();
        progress.current_streak = 7;
        progress.best_streak = 10;
        progress.last_quiz_date = Some("2026-02-12".to_string());
        progress.total_quizzes_completed = 25;

        let json = serde_json::to_string(&progress).unwrap();
        let deserialized: TutorialProgress = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.current_streak, 7);
        assert_eq!(deserialized.best_streak, 10);
        assert_eq!(deserialized.last_quiz_date, Some("2026-02-12".to_string()));
        assert_eq!(deserialized.total_quizzes_completed, 25);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ACHIEVEMENT TESTS (v0.12.0)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_achievement_first_quiz() {
        let mut progress = TutorialProgress::new();
        progress.total_quizzes_completed = 1;

        let new_achievements = progress.check_achievements(5, 10, false);

        assert!(progress.has_achievement(Achievement::FirstQuiz));
        assert!(new_achievements.contains(&Achievement::FirstQuiz));
    }

    #[test]
    fn test_achievement_perfect_score() {
        let mut progress = TutorialProgress::new();
        progress.total_quizzes_completed = 1;

        let new_achievements = progress.check_achievements(10, 10, false);

        assert!(progress.has_achievement(Achievement::PerfectScore));
        assert!(progress.has_achievement(Achievement::Expert)); // 100% is also expert
        assert!(new_achievements.contains(&Achievement::PerfectScore));
    }

    #[test]
    fn test_achievement_expert() {
        let mut progress = TutorialProgress::new();
        progress.total_quizzes_completed = 1;

        let new_achievements = progress.check_achievements(9, 10, false); // 90%

        assert!(progress.has_achievement(Achievement::Expert));
        assert!(!progress.has_achievement(Achievement::PerfectScore)); // Not perfect
        assert!(new_achievements.contains(&Achievement::Expert));
    }

    #[test]
    fn test_achievement_week_streak() {
        let mut progress = TutorialProgress::new();
        progress.current_streak = 7;
        progress.total_quizzes_completed = 7;

        let new_achievements = progress.check_achievements(5, 10, false);

        assert!(progress.has_achievement(Achievement::WeekStreak));
        assert!(new_achievements.contains(&Achievement::WeekStreak));
    }

    #[test]
    fn test_achievement_not_duplicated() {
        let mut progress = TutorialProgress::new();
        progress.total_quizzes_completed = 1;

        // First check
        let first_new = progress.check_achievements(10, 10, false);
        assert!(!first_new.is_empty());

        // Second check - should not duplicate
        let second_new = progress.check_achievements(10, 10, false);
        assert!(second_new.is_empty()); // Already unlocked
    }

    #[test]
    fn test_achievement_serialization() {
        let mut progress = TutorialProgress::new();
        progress.achievements.insert(Achievement::FirstQuiz);
        progress.achievements.insert(Achievement::Expert);

        let json = serde_json::to_string(&progress).unwrap();
        let deserialized: TutorialProgress = serde_json::from_str(&json).unwrap();

        assert!(deserialized.has_achievement(Achievement::FirstQuiz));
        assert!(deserialized.has_achievement(Achievement::Expert));
        assert_eq!(deserialized.achievement_count(), 2);
    }

    #[test]
    fn test_achievement_all_count() {
        assert_eq!(Achievement::all().len(), 9);
    }

    #[test]
    fn test_achievement_category_master() {
        let mut progress = TutorialProgress::new();
        progress.total_quizzes_completed = 1;

        let new_achievements = progress.check_achievements(10, 10, true);

        assert!(progress.has_achievement(Achievement::CategoryMaster));
        assert!(new_achievements.contains(&Achievement::CategoryMaster));
    }
}
