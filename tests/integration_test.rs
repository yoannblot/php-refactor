use std::process::Command;

#[test]
fn test_main_output() {
    let output = Command::new("target/debug/php-refactor")
        .output()
        .expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("PHP Refactor")
    );
}
