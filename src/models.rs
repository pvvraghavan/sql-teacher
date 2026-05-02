// models.rs — All shared data types (structs & enums)
// Rust concepts: struct, enum, #[derive], serde

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Curriculum Models (loaded from .toml files) ───

/// Represents a single quiz question with multiple-choice answers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    /// The question text displayed to the user.
    pub question: String,
    /// List of answer choices (typically 4 options).
    pub options: Vec<String>,
    /// Zero-based index of the correct answer in `options`.
    pub correct: usize,
    /// Brief explanation shown after the user answers.
    pub explanation: String,
}

/// Represents a single teaching module loaded from a .toml file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Unique module identifier, e.g. "01", "02", "05_major".
    pub id: String,
    /// Display title, e.g. "SELECT Basics".
    pub title: String,
    /// The lesson content (explanatory text shown to the user).
    pub lesson: String,
    /// Whether this is a major quiz (covers multiple modules).
    pub is_major_quiz: bool,
    /// The list of quiz questions for this module.
    pub questions: Vec<Question>,
}

// ─── Quiz Result ───

/// The outcome of a quiz attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    /// Module ID this result belongs to.
    pub module_id: String,
    /// Number of questions answered correctly.
    pub correct: usize,
    /// Total number of questions in the quiz.
    pub total: usize,
    /// Calculated percentage score (0.0–100.0).
    pub score: f64,
    /// Whether the user met the passing threshold.
    pub passed: bool,
}

impl QuizResult {
    /// Creates a new QuizResult and auto-calculates score and pass/fail.
    pub fn new(module_id: &str, correct: usize, total: usize, is_major: bool) -> Self {
        let score = if total > 0 {
            (correct as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        // Major quizzes require 90%, module quizzes require 95%
        let threshold = if is_major { 90.0 } else { 95.0 };
        let passed = score >= threshold;

        QuizResult {
            module_id: module_id.to_string(),
            correct,
            total,
            score,
            passed,
        }
    }

    /// Returns the passing threshold for display purposes.
    pub fn passing_threshold(is_major: bool) -> f64 {
        if is_major {
            90.0
        } else {
            95.0
        }
    }
}

// ─── User Progress (persisted to JSON) ───

/// Tracks a user's progress across all modules, saved to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    /// The user's display name.
    pub username: String,
    /// Map of module_id → best QuizResult for that module.
    pub completed_modules: HashMap<String, QuizResult>,
    /// The ID of the module the user should work on next.
    pub current_module_id: String,
    /// Total number of quiz attempts across all modules.
    pub total_attempts: u32,
}

impl UserProgress {
    /// Creates a brand-new progress record starting at module "01".
    pub fn new(username: &str) -> Self {
        UserProgress {
            username: username.to_string(),
            completed_modules: HashMap::new(),
            current_module_id: "01".to_string(),
            total_attempts: 0,
        }
    }

    /// Records a quiz result. If the user passed, updates best score if higher.
    pub fn record_result(&mut self, result: &QuizResult) {
        self.total_attempts += 1;
        if result.passed {
            // Only update if new score is higher or module not yet completed
            let dominated = self
                .completed_modules
                .get(&result.module_id)
                .map_or(true, |prev| result.score > prev.score);
            if dominated {
                self.completed_modules
                    .insert(result.module_id.clone(), result.clone());
            }
        }
    }

    /// Returns the number of distinct modules the user has passed.
    pub fn modules_passed(&self) -> usize {
        self.completed_modules.len()
    }
}
