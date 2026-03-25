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
