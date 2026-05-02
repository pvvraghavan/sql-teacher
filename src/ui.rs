// ui.rs — All terminal rendering: menus, lessons, screens
// Rust concepts: colored, inquire, indicatif crates, string formatting

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, Select};
use std::thread;
use std::time::Duration;

use crate::curriculum;
use crate::gate;
use crate::models::{Module, UserProgress};

// ─── Welcome Screen ───

/// Displays the splash/welcome screen when the app starts.
pub fn show_welcome() {
    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "║                                                          ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "║        🦀  S Q L   T E A C H E R   v1.0  🦀            ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "║        Learn SQL through structured lessons              ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "║                                                          ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝"
            .cyan()
            .bold()
    );
    println!();
}

/// Shows a loading animation (spinner) for a short duration.
pub fn show_loading(message: &str, duration_ms: u64) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    thread::sleep(Duration::from_millis(duration_ms));
    pb.finish_and_clear();
}

// ─── Progress Display ───

/// Shows a visual progress bar of curriculum completion.
pub fn show_progress_bar(progress: &UserProgress) {
    let (done, total) = gate::progress_fraction(progress);
    let bar_width = 30;
    let filled = if total > 0 {
        (done * bar_width) / total
    } else {
        0
    };
    let empty = bar_width - filled;

    let bar = format!(
        "[{}{}] {}/{}",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed(),
        done,
        total
    );

    println!("  {} {}", "Progress:".bold(), bar);
    println!(
        "  {} {}",
        "Attempts:".bold(),
        progress.total_attempts
    );
    println!();
}

// ─── Main Menu ───

/// The possible actions from the main menu.
pub enum MenuChoice {
    NextLesson,
    ViewProgress,
    SelectModule,
    ResetProgress,
    Quit,
}

/// Displays the main menu and returns the user's choice.
pub fn show_main_menu(progress: &UserProgress) -> MenuChoice {
    show_progress_bar(progress);

    // Build menu options based on state
    let mut options = vec![
        "📖  Next Lesson",
        "📊  View Progress",
        "📂  Select Module",
        "🔄  Reset Progress",
        "🚪  Quit",
    ];

    // If curriculum is complete, change the first option label
    if gate::is_curriculum_complete(progress) {
        options[0] = "🎓  Review Lessons (Complete!)";
    }

    let selection = Select::new("What would you like to do?", options)
        .prompt()
        .unwrap_or("🚪  Quit");

    match selection {
        s if s.contains("Next Lesson") || s.contains("Review Lessons") => MenuChoice::NextLesson,
        s if s.contains("View Progress") => MenuChoice::ViewProgress,
        s if s.contains("Select Module") => MenuChoice::SelectModule,
        s if s.contains("Reset") => MenuChoice::ResetProgress,
        _ => MenuChoice::Quit,
    }
}

// ─── Lesson Display ───

/// Displays the lesson content for a module.
pub fn show_lesson(module: &Module) {
    println!();
    println!("{}", "─".repeat(60).dimmed());

    let label = if module.is_major_quiz {
        format!("⭐ {}", module.title).yellow().bold().to_string()
    } else {
        format!("📖 Module {} — {}", module.id, module.title)
            .cyan()
            .bold()
            .to_string()
    };

    println!("  {}", label);
    println!("{}", "─".repeat(60).dimmed());
    println!();

    // Display lesson text with some formatting
    for line in module.lesson.lines() {
        if line.trim().is_empty() {
            println!();
        } else if line.starts_with('#') {
            println!("  {}", line.trim_start_matches('#').trim().bold());
        } else if line.starts_with("- ") || line.starts_with("• ") {
            println!("    {}", line);
        } else {
            println!("  {}", line);
        }
    }

    println!();
    println!("{}", "─".repeat(60).dimmed());
}

/// Prompts the user to start the quiz after reading the lesson.
pub fn prompt_start_quiz() -> bool {
    Confirm::new("Ready to take the quiz?")
        .with_default(true)
        .prompt()
        .unwrap_or(false)
}

// ─── Module Selector ───

/// Lets the user pick from all unlocked modules.
/// Returns the selected module ID, or None if cancelled.
pub fn show_module_selector(
    modules: &[Module],
    progress: &UserProgress,
) -> Option<String> {
    let unlocked = gate::unlocked_modules(progress);

    if unlocked.is_empty() {
        println!("  {}", "No modules available.".red());
        return None;
    }

    let options: Vec<String> = unlocked
        .iter()
        .map(|&id| {
            let module = modules.iter().find(|m| m.id == id);
            let title = module.map_or("Unknown".to_string(), |m| m.title.clone());
            let status = if progress.completed_modules.contains_key(id) {
                "✅"
            } else {
                "🔓"
            };
            format!("{} [{}] {}", status, id, title)
        })
        .collect();

    let mut opts_with_cancel = options.clone();
    opts_with_cancel.push("← Back to menu".to_string());

    let selection = Select::new("Select a module:", opts_with_cancel)
        .prompt()
        .unwrap_or_else(|_| "← Back to menu".to_string());

    if selection.contains("Back to menu") {
        return None;
    }

    // Extract the module ID from the selection string "[XX]"
    let id = unlocked
        .iter()
        .zip(options.iter())
        .find(|(_, opt)| **opt == selection)
        .map(|(&id, _)| id.to_string());

    id
}

// ─── Progress View ───

/// Shows a detailed view of the user's progress across all modules.
pub fn show_detailed_progress(modules: &[Module], progress: &UserProgress) {
    println!();
    println!(
        "  {} {}",
        "📊 Progress for:".bold(),
        progress.username.cyan().bold()
    );
    println!("{}", "─".repeat(60).dimmed());

    for &module_id in curriculum::MODULE_ORDER {
        let module = modules.iter().find(|m| m.id == module_id);
        let title = module.map_or("Unknown", |m| m.title.as_str());

        if let Some(result) = progress.completed_modules.get(module_id) {
            let score_text = format!("{:.1}%", result.score);
            println!(
                "  ✅  [{}] {} — {} ({}/{})",
                module_id, title, score_text.green(), result.correct, result.total
            );
        } else if gate::is_module_unlocked(progress, module_id) {
            println!(
                "  🔓  [{}] {} — {}",
                module_id,
                title,
                "Available".yellow()
            );
        } else {
            println!(
                "  🔒  [{}] {} — {}",
                module_id,
                title,
                "Locked".dimmed()
            );
        }
    }

    let (done, total) = gate::progress_fraction(progress);
    println!();
    println!(
        "  {}: {}/{} modules | {} total quiz attempts",
        "Summary".bold(),
        done,
        total,
        progress.total_attempts
    );
    println!();
}

// ─── Confirmations ───

/// Asks the user to confirm a progress reset.
pub fn confirm_reset() -> bool {
    println!();
    Confirm::new("⚠️  This will delete ALL your progress. Are you sure?")
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}

/// Prompts for a username (new user setup).
pub fn ask_username() -> String {
    println!();
    let name = inquire::Text::new("What's your name?")
        .with_default("Learner")
        .prompt()
        .unwrap_or_else(|_| "Learner".to_string());
    name.trim().to_string()
}

/// Shows the curriculum-complete celebration screen.
pub fn show_completion_screen(progress: &UserProgress) {
    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗"
            .green()
            .bold()
    );
    println!(
        "{}",
        "║                                                          ║"
            .green()
            .bold()
    );
    println!(
        "{}",
        "║    🎓🎉  CONGRATULATIONS! CURRICULUM COMPLETE!  🎉🎓     ║"
            .green()
            .bold()
    );
    println!(
        "{}",
        "║                                                          ║"
            .green()
            .bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝"
            .green()
            .bold()
    );
    println!();
    println!(
        "  {} completed all {} modules in {} attempts!",
        progress.username.cyan().bold(),
        progress.modules_passed(),
        progress.total_attempts
    );
    println!(
        "  {}",
        "You can now review any module from the module selector.".dimmed()
    );
    println!();
}

/// Pauses and waits for user to press Enter.
pub fn pause() {
    println!("{}", "  Press Enter to continue...".dimmed());
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
}
