// main.rs — Entry point & main application loop
// Rust concepts: mod, use, ownership across modules, stateful loops

mod curriculum;
mod gate;
mod models;
mod progress;
mod quiz;
mod ui;

use anyhow::Result;
use colored::*;
use std::path::PathBuf;

/// Resolves the data directory path.
/// Looks for a `data/` folder relative to the executable, then falls back
/// to a `data/` folder relative to the current working directory.
fn find_data_dir() -> Result<PathBuf> {
    // Try relative to executable
    if let Ok(exe_path) = std::env::current_exe() {
        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let candidate = exe_dir.join("data");
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    // Fall back to current working directory
    let cwd = std::env::current_dir()?;
    let candidate = cwd.join("data");
    if candidate.exists() {
        return Ok(candidate);
    }

    anyhow::bail!(
        "Could not find the 'data/' directory.\n\
         Make sure the 'data/' folder (with modules/) is in the same directory \
         as the executable or in the current working directory."
    );
}

fn main() -> Result<()> {
    // ─── Startup ───
    ui::show_welcome();
    ui::show_loading("Loading curriculum...", 600);

    let data_dir = find_data_dir()?;
    let modules = curriculum::load_curriculum(&data_dir)?;

    println!(
        "  {} {} modules loaded.\n",
        "✓".green().bold(),
        modules.len()
    );

    // ─── Load or create user progress ───
    let mut user_progress = match progress::load_progress(&data_dir)? {
        Some(p) => {
            println!(
                "  Welcome back, {}! Resuming your progress.\n",
                p.username.cyan().bold()
            );
            p
        }
        None => {
            println!("  {}\n", "Welcome, new learner!".yellow());
            let name = ui::ask_username();
            let p = models::UserProgress::new(&name);
            progress::save_progress(&data_dir, &p)?;
            println!(
                "\n  Progress file created for {}.\n",
                p.username.cyan().bold()
            );
            p
        }
    };

    // ─── Main Application Loop ───
    loop {
        match ui::show_main_menu(&user_progress) {
            // ── Next Lesson ──
            ui::MenuChoice::NextLesson => {
                if gate::is_curriculum_complete(&user_progress) {
                    ui::show_completion_screen(&user_progress);
                    continue;
                }

                let next_id = match gate::next_module_id(&user_progress) {
                    Some(id) => id.to_string(),
                    None => {
                        ui::show_completion_screen(&user_progress);
                        continue;
                    }
                };

                run_module(&next_id, &modules, &mut user_progress, &data_dir)?;
            }

            // ── View Progress ──
            ui::MenuChoice::ViewProgress => {
                ui::show_detailed_progress(&modules, &user_progress);
                ui::pause();
            }

            // ── Select Module ──
            ui::MenuChoice::SelectModule => {
                if let Some(module_id) = ui::show_module_selector(&modules, &user_progress) {
                    run_module(&module_id, &modules, &mut user_progress, &data_dir)?;
                }
            }

            // ── Reset Progress ──
            ui::MenuChoice::ResetProgress => {
                if ui::confirm_reset() {
                    progress::reset_progress(&data_dir)?;
                    let name = ui::ask_username();
                    user_progress = models::UserProgress::new(&name);
                    progress::save_progress(&data_dir, &user_progress)?;
                    println!("\n  {}\n", "Progress has been reset.".yellow());
                } else {
                    println!("\n  {}\n", "Reset cancelled.".dimmed());
                }
            }

            // ── Quit ──
            ui::MenuChoice::Quit => {
                progress::save_progress(&data_dir, &user_progress)?;
                println!(
                    "\n  {} See you next time, {}! 🦀\n",
                    "Goodbye!".cyan().bold(),
                    user_progress.username
                );
                break;
            }
        }
    }

    Ok(())
}

/// Runs a single module: show lesson → prompt quiz → run quiz → save result.
fn run_module(
    module_id: &str,
    modules: &[models::Module],
    progress: &mut models::UserProgress,
    data_dir: &PathBuf,
) -> Result<()> {
    let module = match curriculum::find_module(modules, module_id) {
        Some(m) => m,
        None => {
            println!("  {}", "Module not found.".red());
            return Ok(());
        }
    };

    // Show the lesson content (skip for major quizzes — they're pure quiz)
    if !module.is_major_quiz {
        ui::show_lesson(module);
    } else {
        println!();
        println!(
            "  {} {}",
            "⭐ MAJOR QUIZ:".yellow().bold(),
            module.title.bold()
        );
        println!(
            "  {}",
            "This quiz covers the last several modules. 90% required to pass."
                .dimmed()
        );
    }

    // Prompt to start quiz
    if !ui::prompt_start_quiz() {
        println!("  {}", "Returning to menu.".dimmed());
        return Ok(());
    }

    // Run the quiz
    let result = quiz::run_quiz(module)?;
    quiz::display_result(&result, module.is_major_quiz);

    // Record result and advance if passed
    progress.record_result(&result);

    if result.passed {
        // Advance current_module_id if this was the next expected module
        if let Some(next) = gate::next_module_id(progress) {
            progress.current_module_id = next.to_string();
        } else {
            progress.current_module_id = "complete".to_string();
        }
    }

    // Save progress to disk
    progress::save_progress(data_dir, progress)?;
    println!(
        "  {}",
        "💾 Progress saved.".dimmed()
    );

    ui::pause();
    Ok(())
}
