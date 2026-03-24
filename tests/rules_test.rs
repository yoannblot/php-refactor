use std::fs;
use std::path::Path;

const SEPARATOR: &str = "\n-----\n";

#[test]
fn test_all_rules() {
    let rules = php_refactor::rules::all_rules();

    for (rule_path, rule_fn) in rules {
        let fixture_dir = format!("tests/rules/{}", rule_path);
        let fixture_path = Path::new(&fixture_dir);

        if !fixture_path.exists() {
            panic!(
                "Fixture directory not found for rule '{}': {}",
                rule_path, fixture_dir
            );
        }

        let mut entries: Vec<_> = fs::read_dir(fixture_path)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        entries.sort(); // deterministic order

        for path in entries {
            // Skip non-.php.inc files
            if path.extension().map(|e| e.to_str()).flatten() != Some("inc") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Failed to read fixture: {}", path.display()));
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if content.contains(SEPARATOR) {
                // Transform fixture: expects Some(output)
                let parts: Vec<&str> = content.splitn(2, SEPARATOR).collect();
                let input = parts[0];
                let expected = parts[1];

                let result = rule_fn(input);
                assert!(
                    result.is_some(),
                    "Rule '{}': Expected Some for fixture '{}', got None",
                    rule_path,
                    file_name
                );
                assert_eq!(
                    result.unwrap().trim(),
                    expected.trim(),
                    "Rule '{}': Output mismatch for fixture '{}'",
                    rule_path,
                    file_name
                );
            } else {
                // No-transform fixture: expects None
                let result = rule_fn(content.trim());
                assert!(
                    result.is_none(),
                    "Rule '{}': Expected None for fixture '{}', got Some",
                    rule_path,
                    file_name
                );
            }
        }
    }
}
