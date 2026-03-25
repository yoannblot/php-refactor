use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
