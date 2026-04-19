use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

// NOTE: This rule is purely syntactic. It does not analyze class bodies.
// Adding `readonly` to a class with static properties, untyped properties,
// properties with default values, a non-readonly parent, or `#[AllowDynamicProperties]`
// will produce a PHP fatal error at runtime. Scope application via `config.toml`
// path globs to classes you know are readonly-compatible.

static RE: LazyLock<Regex> = LazyLock::new(|| {
    // Matches a bare "class" keyword at the start of a line (after optional whitespace).
    // Captures: (1) leading whitespace.
    // Naturally excludes: readonly/final/abstract class, interface, trait, enum, ::class, $class, etc.
    Regex::new(r"(?m)^(\s*)class\s").unwrap()
});

/// File-aware entry point: applies the rule to the given set of files in parallel.
pub fn apply(files: &[PathBuf]) -> crate::rules::RuleResult {
    let files_matched = AtomicUsize::new(0);
    let files_changed = AtomicUsize::new(0);

    files.par_iter().for_each(|file_path| {
        let Ok(original) = fs::read_to_string(file_path) else {
            return;
        };

        if let Some(modified) = apply_to_source(&original) {
            files_matched.fetch_add(1, Ordering::Relaxed);
            if fs::write(file_path, &modified).is_ok() {
                files_changed.fetch_add(1, Ordering::Relaxed);
            }
        }
    });

    crate::rules::RuleResult {
        files_changed: files_changed.load(Ordering::Relaxed),
        files_matched: files_matched.load(Ordering::Relaxed),
        files_analyzed: files.len(),
    }
}

/// Pure source transformation: used by tests.
pub fn apply_to_source(source: &str) -> Option<String> {
    // Early exit: skip expensive regex if source has no class declaration
    if !source.contains("class ") {
        return None;
    }

    // Check if pattern exists before attempting replacement
    if !RE.is_match(source) {
        return None;
    }

    let result = RE.replace_all(source, |caps: &regex::Captures| {
        let indent = &caps[1];
        format!("{}readonly class ", indent)
    });

    if result == source {
        None
    } else {
        Some(result.into_owned())
    }
}
