// quiz.rs — Quiz engine: runs interactive quizzes, calculates scores
// Rust concepts: loops, match, mutable state, arithmetic, borrowing

use anyhow::Result;
use colored::*;
use inquire::Select;

use crate::models::{Module, QuizResult};

/// Runs an interactive quiz for the given module.
/// Presents each question, collects answers, and returns a QuizResult.
pub fn run_quiz(module: &Module) -> Result<QuizResult> {
    let total = module.questions.len();
    let mut correct_count: usize = 0;

    println!();
    let quiz_label = if module.is_major_quiz {
        "⭐ MAJOR QUIZ".yellow().bold()
    } else {
        "📝 MODULE QUIZ".cyan().bold()
    };
    println!("{}: {}", quiz_label, module.title.bold());

    let threshold = QuizResult::passing_threshold(module.is_major_quiz);
    println!(
        "{}",
        format!("   Passing score: {:.0}% | {} questions", threshold, total).dimmed()
    );
    println!("{}", "─".repeat(60).dimmed());

    for (i, question) in module.questions.iter().enumerate() {
        println!();
        println!(
            "{}",
            format!("  Question {}/{}", i + 1, total).white().bold()
        );
        println!("  {}", question.question);

        // Build labeled options for the select menu
        let labeled: Vec<String> = question
            .options
            .iter()
            .enumerate()
            .map(|(j, opt)| format!("{}. {}", (b'A' + j as u8) as char, opt))
            .collect();

        let selection = Select::new("Your answer:", labeled.clone())
            .prompt()
            .unwrap_or_else(|_| labeled[0].clone());

        // Find which index was selected
        let selected_index = labeled
            .iter()
            .position(|item| *item == selection)
            .unwrap_or(0);

        if selected_index == question.correct {
            correct_count += 1;
            println!("  {} Correct!", "✓".green().bold());
        } else {
            let correct_letter = (b'A' + question.correct as u8) as char;
            println!(
                "  {} Incorrect. The answer was {}. {}",
                "✗".red().bold(),
                format!("{}", correct_letter).yellow(),
                &question.options[question.correct].yellow()
            );
        }
        println!("  💡 {}", question.explanation.dimmed());
    }

    let result = QuizResult::new(&module.id, correct_count, total, module.is_major_quiz);
    Ok(result)
}

/// Displays the quiz result summary with pass/fail status.
pub fn display_result(result: &QuizResult, is_major: bool) {
    println!();
    println!("{}", "═".repeat(60).dimmed());

    let score_text = format!("{:.1}%", result.score);
    let count_text = format!("{}/{} correct", result.correct, result.total);

    if result.passed {
        println!(
            "  {} {} — {}",
            "🏆 PASSED!".green().bold(),
            score_text.green().bold(),
            count_text
        );
        if is_major {
            println!(
                "  {}",
                "   Major checkpoint cleared! Next modules unlocked."
                    .green()
            );
        } else {
            println!(
                "  {}",
                "   Great work! Next module unlocked.".green()
            );
        }
    } else {
        let threshold = QuizResult::passing_threshold(is_major);
        println!(
            "  {} {} — {}",
            "❌ NOT PASSED".red().bold(),
            score_text.red().bold(),
            count_text
        );
        println!(
            "  {}",
            format!(
                "   You need {:.0}% to pass. Review the lesson and try again!",
                threshold
            )
            .yellow()
        );
    }

    println!("{}", "═".repeat(60).dimmed());
    println!();
}
