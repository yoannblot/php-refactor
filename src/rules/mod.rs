pub mod quality;

/// Result of applying a rule to a path.
#[derive(Debug, Clone, Copy)]
pub struct RuleResult {
    pub files_changed: usize,
    pub files_analyzed: usize,
}

/// File-aware rule: takes a path (file, config, or directory) and returns stats.
pub type RuleFn = fn(&str) -> RuleResult;

/// Pure source transformation: takes source code, returns modified source or None.
pub type SourceTransformFn = fn(&str) -> Option<String>;

/// Rules registered for execution via main.rs
pub fn all_rules() -> Vec<(&'static str, RuleFn)> {
    vec![(
        "quality/add_final_keyword",
        quality::add_final_keyword::apply,
    )]
}

/// Pure source transforms used by tests
pub fn all_source_transforms() -> Vec<(&'static str, SourceTransformFn)> {
    vec![(
        "quality/add_final_keyword",
        quality::add_final_keyword::apply_to_source,
    )]
}
