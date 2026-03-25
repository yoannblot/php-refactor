fn fixture(name: &str) -> String {
    format!("tests/fixtures/config/{}", name)
}

#[test]
fn test_load_valid_config_single_path() {
    let config = php_refactor::config::load(&fixture("valid_single_path.toml")).unwrap();
    assert_eq!(config.source.paths.len(), 1);
    assert_eq!(config.source.paths[0], "src");
}

#[test]
fn test_load_valid_config_multiple_paths() {
    let config = php_refactor::config::load(&fixture("valid_multiple_paths.toml")).unwrap();
    assert_eq!(config.source.paths.len(), 3);
    assert_eq!(config.source.paths[0], "src");
    assert_eq!(config.source.paths[1], "tests");
    assert_eq!(config.source.paths[2], "app");
}

#[test]
fn test_load_valid_config_absolute_paths() {
    let config = php_refactor::config::load(&fixture("valid_absolute_paths.toml")).unwrap();
    assert_eq!(config.source.paths.len(), 2);
    assert_eq!(config.source.paths[0], "/home/user/src");
    assert_eq!(config.source.paths[1], "/var/app/tests");
}

#[test]
fn test_load_valid_config_empty_paths() {
    let config = php_refactor::config::load(&fixture("valid_empty_paths.toml")).unwrap();
    assert_eq!(config.source.paths.len(), 0);
}

#[test]
fn test_load_nonexistent_file() {
    let result = php_refactor::config::load("/nonexistent/path/config.toml");
    assert!(result.is_err());
}

#[test]
fn test_load_invalid_toml_syntax() {
    let result = php_refactor::config::load(&fixture("invalid_syntax.toml"));
    assert!(result.is_err());
}

#[test]
fn test_load_missing_source_section() {
    let result = php_refactor::config::load(&fixture("missing_source_section.toml"));
    assert!(result.is_err());
}

#[test]
fn test_load_missing_paths_field() {
    let result = php_refactor::config::load(&fixture("missing_paths_field.toml"));
    assert!(result.is_err());
}

#[test]
fn test_load_config_with_special_characters_in_paths() {
    let config = php_refactor::config::load(&fixture("special_characters.toml")).unwrap();
    assert_eq!(config.source.paths.len(), 3);
    assert_eq!(config.source.paths[0], "src/php-app");
    assert_eq!(config.source.paths[1], "./tests/unit");
    assert_eq!(config.source.paths[2], "../shared");
}

#[test]
fn test_load_config_with_extra_whitespace() {
    let config = php_refactor::config::load(&fixture("extra_whitespace.toml")).unwrap();
    assert_eq!(config.source.paths.len(), 2);
    assert_eq!(config.source.paths[0], "src");
    assert_eq!(config.source.paths[1], "tests");
}
