use globset::{Glob, GlobSetBuilder};
use ignore::{WalkBuilder, WalkState};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use walkdir::WalkDir;

/// Resolve the effective files for a rule with an already-loaded config.
/// Avoids re-loading and re-parsing the config per rule.
pub fn resolve_for_rule_with_config(
    path: &str,
    rule_key: &str,
    config: Option<&crate::config::Config>,
) -> Vec<PathBuf> {
    if let Some(cfg) = config {
        // Split rule_key (e.g., "quality/add_final_keyword") into category and name
        let parts: Vec<&str> = rule_key.split('/').collect();
        if parts.len() == 2 {
            let category = parts[0];
            let name = parts[1];

            // Check if this rule has a specific path configuration
            if let Some(rule_config) = cfg.rules.get(category).and_then(|c| c.get(name)) {
                return collect_php_files_from_globs(&rule_config.paths);
            }
        }
    }

    // Fall back to standard path expansion
    collect_php_files(path).unwrap_or_default()
}

/// Resolve the effective files for a rule based on CLI path and rule key.
///
/// If path is a .toml config file and the rule has per-rule paths configured,
/// use those paths (with glob expansion). Otherwise, use standard path expansion.
///
/// This function loads the config on each call. For better performance with multiple rules,
/// use `resolve_for_rule_with_config` and load the config once.
pub fn resolve_for_rule(path: &str, rule_key: &str) -> Vec<PathBuf> {
    let config = if path.ends_with(".toml") {
        crate::config::load(path).ok()
    } else {
        None
    };

    resolve_for_rule_with_config(path, rule_key, config.as_ref())
}

/// Expand a path argument into a list of PHP files to process.
/// - `.toml` file → load config, walk configured paths
/// - directory → walk and collect .php files
/// - anything else → treat as a single file (return as-is)
pub fn collect_php_files(path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let p = Path::new(path);

    if path.ends_with(".toml") {
        collect_from_config(path)
    } else if p.is_dir() {
        Ok(collect_from_directory(path))
    } else {
        // Single file: return as-is if .php, empty if not
        if path.ends_with(".php") {
            Ok(vec![p.to_path_buf()])
        } else {
            Ok(vec![])
        }
    }
}

/// Collect PHP files matching glob patterns.
/// Uses parallel directory walking (ignore::WalkParallel) anchored to pattern base directories,
/// combined with glob::Pattern matching. WalkParallel automatically respects .gitignore,
/// dramatically reducing traversal of vendor/ and other excluded directories.
///
/// Patterns without `/` are treated as recursive (auto-prepends `**/`).
/// Returns deduplicated, sorted list of matching files, ignoring inaccessible paths.
pub fn collect_php_files_from_globs(patterns: &[String]) -> Vec<PathBuf> {
    // 1. Compile all patterns into one GlobSet (fast DFA matching, all patterns in one pass).
    //    Deduplicate base directories to avoid walking the same tree multiple times.
    let mut gsbuilder = GlobSetBuilder::new();
    let mut bases: HashSet<PathBuf> = HashSet::new();

    for p in patterns {
        let effective = if p.contains('/') {
            p.clone()
        } else {
            format!("**/{}", p)
        };
        bases.insert(extract_base_dir(&effective));
        if let Ok(g) = Glob::new(&effective) {
            gsbuilder.add(g);
        }
    }

    let Ok(globset) = gsbuilder.build() else {
        return vec![];
    };

    // 2. One WalkBuilder with all base directories as roots.
    let mut bases_iter = bases.into_iter();
    let Some(first) = bases_iter.next() else {
        return vec![];
    };

    let mut builder = WalkBuilder::new(&first);
    for base in bases_iter {
        builder.add(&base);
    }

    // 3. One WalkParallel run: all roots walked in parallel, globset matching all patterns.
    let (tx, rx) = channel();
    builder.standard_filters(true).build_parallel().run(|| {
        let gs = globset.clone();
        let tx = tx.clone();
        Box::new(move |result| {
            if let Ok(entry) = result {
                let path = entry.path();
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                    && path.extension().and_then(|s| s.to_str()) == Some("php")
                    && gs.is_match(path)
                {
                    let _ = tx.send(path.to_path_buf());
                }
            }
            WalkState::Continue
        })
    });
    drop(tx);

    let mut result: Vec<PathBuf> = rx.into_iter().collect();
    result.sort();
    result
}

/// Load a TOML config and collect all .php files from configured paths.
fn collect_from_config(config_path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let cfg = crate::config::load(config_path)?;
    let mut files = Vec::new();

    for path in &cfg.source.paths {
        files.extend(collect_from_directory(path));
    }

    Ok(files)
}

/// Walk a directory recursively and collect all .php files.
fn collect_from_directory(dir_path: &str) -> Vec<PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("php"))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Extract the longest non-wildcard prefix directory from a glob pattern.
/// This anchors walkdir to the smallest useful subtree, avoiding unnecessary traversal
/// of unrelated directories (e.g., vendor/ when pattern is src/**/*.php).
///
/// Examples:
/// - "src/Interface/**/*Request.php" → PathBuf("src/Interface")
/// - "src/**/*.php"                  → PathBuf("src")
/// - "**/*.php"                      → PathBuf(".")
fn extract_base_dir(pattern: &str) -> PathBuf {
    let parts: Vec<&str> = pattern.split('/').collect();
    let base_parts: Vec<&str> = parts
        .iter()
        .take_while(|p| !p.contains('*') && !p.contains('?') && !p.contains('{'))
        .copied()
        .collect();

    if base_parts.is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(base_parts.join("/"))
    }
}
