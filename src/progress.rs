// progress.rs — Loads & saves UserProgress to JSON on disk
// Rust concepts: serde_json, fs::read/write, ? operator, Path

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::models::UserProgress;

/// Default filename for the progress file.
const PROGRESS_FILE: &str = "progress.json";

/// Returns the full path to the progress file inside the data directory.
fn progress_path(data_dir: &Path) -> std::path::PathBuf {
    data_dir.join(PROGRESS_FILE)
}

/// Attempts to load an existing user progress file from disk.
/// Returns `None` if the file does not exist (first-time user).
pub fn load_progress(data_dir: &Path) -> Result<Option<UserProgress>> {
    let path = progress_path(data_dir);

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read progress file: {}", path.display()))?;

    let progress: UserProgress = serde_json::from_str(&content)
        .with_context(|| "Failed to parse progress JSON — file may be corrupted")?;

    Ok(Some(progress))
}

/// Saves the user's progress to the JSON file on disk.
/// Creates the file if it doesn't exist; overwrites if it does.
pub fn save_progress(data_dir: &Path, progress: &UserProgress) -> Result<()> {
    let path = progress_path(data_dir);

    // Ensure the data directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create data directory: {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(progress)
        .context("Failed to serialize progress to JSON")?;

    fs::write(&path, json)
        .with_context(|| format!("Failed to write progress file: {}", path.display()))?;

    Ok(())
}

/// Deletes the progress file to start fresh. Returns Ok even if file doesn't exist.
pub fn reset_progress(data_dir: &Path) -> Result<()> {
    let path = progress_path(data_dir);
    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to delete progress file: {}", path.display()))?;
    }
    Ok(())
}
