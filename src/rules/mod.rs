pub mod quality;

/// Result of applying a rule to a path.
#[derive(Debug, Clone, Copy)]
pub struct RuleResult {
    pub files_changed: usize,
    pub files_matched: usize,
    pub files_analyzed: usize,
}

/// File-aware rule: takes a list of files to process and returns stats.
pub type RuleFn = fn(&[std::path::PathBuf]) -> RuleResult;

/// Pure source transformation: takes source code, returns modified source or None.
pub type SourceTransformFn = fn(&str) -> Option<String>;

// Include auto-generated rule registry (discovered from src/rules/ at build time)
include!(concat!(env!("OUT_DIR"), "/generated_rules.rs"));
