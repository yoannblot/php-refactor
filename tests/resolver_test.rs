fn fixture(name: &str) -> String {
    format!("tests/fixtures/resolver/{}", name)
}

#[test]
fn test_single_php_file() {
    let result = php_refactor::resolver::collect_php_files(&fixture("single.php")).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].ends_with("single.php"));
}

#[test]
fn test_single_non_php_file() {
    let result = php_refactor::resolver::collect_php_files(&fixture("single.txt")).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_directory_with_php_files() {
    let result = php_refactor::resolver::collect_php_files(&fixture("dir_with_php")).unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_directory_with_nested_php_files() {
    let result = php_refactor::resolver::collect_php_files(&fixture("nested")).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|p| p.ends_with("root.php")));
    assert!(result.iter().any(|p| p.ends_with("nested.php")));
}

#[test]
fn test_empty_directory() {
    let result = php_refactor::resolver::collect_php_files(&fixture("empty_dir")).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_directory_with_only_non_php_files() {
    let result = php_refactor::resolver::collect_php_files(&fixture("only_non_php")).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_nonexistent_php_file() {
    let result = php_refactor::resolver::collect_php_files("/nonexistent/path/file.php").unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_config_file() {
    let result = php_refactor::resolver::collect_php_files(&fixture("config/config.toml")).unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_glob_pattern_without_path_separator() {
    // Pattern without `/` should be treated as recursive: `*.php` → `**/*.php`
    let patterns = vec!["tests/fixtures/resolver/*.php".to_string()];
    let result = php_refactor::resolver::collect_php_files_from_globs(&patterns);
    // Should find nested.php, root.php, single.php, a.php, b.php, app.php, unit.php
    assert!(!result.is_empty(), "Should match PHP files recursively");
    assert!(result.iter().any(|p| p.ends_with("single.php")));
}

#[test]
fn test_glob_pattern_with_path_separator() {
    // Pattern with `/` should be used as-is
    let patterns = vec!["tests/fixtures/resolver/dir_with_php/*.php".to_string()];
    let result = php_refactor::resolver::collect_php_files_from_globs(&patterns);
    assert_eq!(result.len(), 2, "Should match files in specific directory");
    assert!(result.iter().any(|p| p.ends_with("a.php")));
    assert!(result.iter().any(|p| p.ends_with("b.php")));
}

#[test]
fn test_glob_pattern_auto_prepend_recursive() {
    // Pattern like `*Command.php` should auto-prepend `**/`
    let patterns = vec!["*.php".to_string()];
    let result = php_refactor::resolver::collect_php_files_from_globs(&patterns);
    // Should match PHP files at any depth in current directory
    assert!(!result.is_empty(), "Pattern without / should be recursive");
}

#[test]
fn test_glob_pattern_explicit_recursive() {
    // Explicit `**/pattern.php` should work
    let patterns = vec!["**/single.php".to_string()];
    let result = php_refactor::resolver::collect_php_files_from_globs(&patterns);
    assert!(!result.is_empty(), "Explicit ** pattern should match");
    assert!(result.iter().any(|p| p.ends_with("single.php")));
}
