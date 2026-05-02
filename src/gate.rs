// gate.rs — Gating & progression logic
// Rust concepts: pure functions, iterators, closures, pattern matching

use crate::curriculum::MODULE_ORDER;
use crate::models::UserProgress;

/// Checks whether a given module is unlocked for the user.
///
/// Rules:
/// - Module "01" is always unlocked.
/// - A module is unlocked if all prior modules in MODULE_ORDER have been passed.
/// - Major quizzes ("05_major", "10_major") gate everything after them.
pub fn is_module_unlocked(progress: &UserProgress, module_id: &str) -> bool {
    // First module is always available
    if module_id == "01" {
        return true;
    }

    let target_index = match MODULE_ORDER.iter().position(|&id| id == module_id) {
        Some(idx) => idx,
        None => return false, // Unknown module
    };

    // Every module before this one must be completed
    for i in 0..target_index {
        let prior_id = MODULE_ORDER[i];
        if !progress.completed_modules.contains_key(prior_id) {
            return false;
        }
    }

    true
}

/// Determines the next module the user should work on.
/// Returns `None` if all modules are completed (curriculum finished).
pub fn next_module_id(progress: &UserProgress) -> Option<&'static str> {
    MODULE_ORDER
        .iter()
        .find(|&&id| !progress.completed_modules.contains_key(id))
        .copied()
}

/// Checks whether a module ID corresponds to a major quiz milestone.
pub fn is_major_milestone(module_id: &str) -> bool {
    module_id.contains("major")
}

/// Returns a list of all module IDs that the user has unlocked.
pub fn unlocked_modules(progress: &UserProgress) -> Vec<&'static str> {
    MODULE_ORDER
        .iter()
        .filter(|&&id| is_module_unlocked(progress, id))
        .copied()
        .collect()
}

/// Returns true if every module in the curriculum has been passed.
pub fn is_curriculum_complete(progress: &UserProgress) -> bool {
    MODULE_ORDER
        .iter()
        .all(|&id| progress.completed_modules.contains_key(id))
}

/// Returns the fraction of progress as (completed, total).
pub fn progress_fraction(progress: &UserProgress) -> (usize, usize) {
    let completed = MODULE_ORDER
        .iter()
        .filter(|&&id| progress.completed_modules.contains_key(id))
        .count();
    (completed, MODULE_ORDER.len())
}

// ─── Unit Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{QuizResult, UserProgress};

    fn make_progress_with(completed: &[&str]) -> UserProgress {
        let mut p = UserProgress::new("tester");
        for &id in completed {
            let is_major = is_major_milestone(id);
            let result = QuizResult::new(id, 10, 10, is_major);
            p.record_result(&result);
        }
        p
    }

    #[test]
    fn test_first_module_always_unlocked() {
        let progress = UserProgress::new("newbie");
        assert!(is_module_unlocked(&progress, "01"));
    }

    #[test]
    fn test_second_module_locked_without_first() {
        let progress = UserProgress::new("newbie");
        assert!(!is_module_unlocked(&progress, "02"));
    }

    #[test]
    fn test_second_module_unlocked_after_first() {
        let progress = make_progress_with(&["01"]);
        assert!(is_module_unlocked(&progress, "02"));
    }

    #[test]
    fn test_major_quiz_gates_next_section() {
        // Completed 01–04 but NOT the major quiz 05
        let progress = make_progress_with(&["01", "02", "03", "04"]);
        assert!(is_module_unlocked(&progress, "05_major"));
        assert!(!is_module_unlocked(&progress, "06"));
    }

    #[test]
    fn test_major_quiz_passed_unlocks_next() {
        let progress = make_progress_with(&["01", "02", "03", "04", "05_major"]);
        assert!(is_module_unlocked(&progress, "06"));
    }

    #[test]
    fn test_next_module_id_new_user() {
        let progress = UserProgress::new("newbie");
        assert_eq!(next_module_id(&progress), Some("01"));
    }

    #[test]
    fn test_next_module_after_partial() {
        let progress = make_progress_with(&["01", "02"]);
        assert_eq!(next_module_id(&progress), Some("03"));
    }

    #[test]
    fn test_curriculum_complete() {
        let progress = make_progress_with(&[
            "01", "02", "03", "04", "05_major", "06", "07", "08", "09", "10_major",
        ]);
        assert!(is_curriculum_complete(&progress));
        assert_eq!(next_module_id(&progress), None);
    }

    #[test]
    fn test_progress_fraction() {
        let progress = make_progress_with(&["01", "02", "03"]);
        let (done, total) = progress_fraction(&progress);
        assert_eq!(done, 3);
        assert_eq!(total, 10);
    }
}
