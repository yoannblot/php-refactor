use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

// NOTE: This rule is purely syntactic. It uses naive brace counting that is not
// aware of PHP strings, heredocs, or comments. A `{` or `}` inside a string
// literal inside a readonly class body can skew the scan. Scope application
// via `config.toml` path globs to code you trust.

static HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Matches a class declaration whose modifier list contains `readonly`
    // in any order (readonly class, final readonly class, readonly final class,
    // abstract readonly class, etc.). Capture 1 is the full modifier string;
    // we post-check that it contains `readonly` to reject `final class` etc.
    Regex::new(r"(?m)^[ \t]*((?:(?:final|abstract|readonly)[ \t]+)+)class[ \t]+\w+[^{]*\{").unwrap()
});

static STRIP_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Matches visibility-prefixed declarations (property or promoted param)
    // followed by `readonly`. Capture 1 is the prefix we preserve.
    Regex::new(r"(?m)^(\s*(?:public|protected|private)(?:[ \t]+static)?)[ \t]+readonly\b").unwrap()
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
    if !source.contains("readonly") || !source.contains("class ") {
        return None;
    }

    let bytes = source.as_bytes();

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for caps in HEADER_RE.captures_iter(source) {
        let modifiers = caps.get(1).expect("group 1 is non-optional").as_str();
        if !modifiers.split_whitespace().any(|w| w == "readonly") {
            continue;
        }
        let header_match = caps.get(0).expect("group 0 is always present on a match");
        let open_brace_pos = header_match.end() - 1;
        let Some(close_brace_pos) = find_matching_brace(bytes, open_brace_pos) else {
            continue;
        };
        let body_start = open_brace_pos + 1;
        let body_end = close_brace_pos;
        if body_start < body_end {
            ranges.push((body_start, body_end));
        }
    }

    if ranges.is_empty() {
        return None;
    }

    ranges.sort_by_key(|r| std::cmp::Reverse(r.0));

    let mut result = source.to_string();
    for (start, end) in ranges {
        let replaced = STRIP_RE.replace_all(&result[start..end], "$1").into_owned();
        result.replace_range(start..end, &replaced);
    }

    if result == source { None } else { Some(result) }
}

/// Finds the byte offset of the `}` that closes the `{` at `open_pos`.
/// Returns `None` if the braces are unbalanced (malformed input).
fn find_matching_brace(bytes: &[u8], open_pos: usize) -> Option<usize> {
    let mut depth: usize = 1;
    let mut i = open_pos + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}
