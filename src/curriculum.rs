// curriculum.rs — Loads & manages module .toml files from disk
// Rust concepts: File I/O, serde, Vec<T>, Result<T>, iterators

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::models::Module;

/// The ordered list of all module IDs in the curriculum.
/// Major quizzes are included as their own "module" entries.
pub const MODULE_ORDER: &[&str] = &[
    "01", "02", "03", "04", "05_major", "06", "07", "08", "09", "10_major",
];

/// Loads all curriculum modules from .toml files in the given directory.
/// Returns them sorted by their position in MODULE_ORDER.
pub fn load_curriculum(data_dir: &Path) -> Result<Vec<Module>> {
    let modules_dir = data_dir.join("modules");
    let mut modules = Vec::new();

    for module_id in MODULE_ORDER {
        let filename = format!("{}.toml", module_id);
        let filepath = modules_dir.join(&filename);

        let content = fs::read_to_string(&filepath)
            .with_context(|| format!("Failed to read module file: {}", filepath.display()))?;

        let module: Module = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML in: {}", filepath.display()))?;

        modules.push(module);
    }

    Ok(modules)
}

/// Finds a module by its ID from a loaded curriculum.
pub fn find_module<'a>(modules: &'a [Module], module_id: &str) -> Option<&'a Module> {
    modules.iter().find(|m| m.id == module_id)
}

/// Returns the index of a module ID in the curriculum order.
pub fn module_index(module_id: &str) -> Option<usize> {
    MODULE_ORDER.iter().position(|&id| id == module_id)
}

/// Returns the total number of modules in the curriculum.
pub fn total_modules() -> usize {
    MODULE_ORDER.len()
}
