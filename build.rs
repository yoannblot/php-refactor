use std::fs;
use std::path::Path;

fn main() {
    // Scan src/rules/ directory to auto-discover rule modules
    let rules_dir = "src/rules";
    let mut rules_code = String::from("// Auto-generated rule registry\n\n");

    // Track rules for all_rules() and all_source_transforms()
    let mut all_rules_entries = Vec::new();
    let mut all_transforms_entries = Vec::new();

    // Scan categories (subdirectories of src/rules/)
    if let Ok(entries) = fs::read_dir(rules_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Skip mod.rs and other files, only process directories
            if !path.is_dir() {
                continue;
            }

            let category = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            if category == "quality" {
                // Scan quality subdirectory for rule modules
                if let Ok(rule_entries) = fs::read_dir(&path) {
                    for rule_entry in rule_entries.flatten() {
                        let rule_path = rule_entry.path();

                        // Look for .rs files that are rule modules
                        if rule_path.extension().map(|e| e == "rs").unwrap_or(false) {
                            let rule_name =
                                rule_path.file_stem().and_then(|n| n.to_str()).unwrap_or("");

                            // Skip mod.rs
                            if rule_name == "mod" {
                                continue;
                            }

                            let rule_key = format!("{}/{}", category, rule_name);
                            let module_path = format!("{}::{}", category, rule_name);

                            all_rules_entries.push(format!(
                                "        (\"{}\", {}::apply),",
                                rule_key, module_path
                            ));
                            all_transforms_entries.push(format!(
                                "        (\"{}\", {}::apply_to_source),",
                                rule_key, module_path
                            ));
                        }
                    }
                }
            }
        }
    }

    // Generate all_rules() function
    rules_code.push_str("pub fn all_rules() -> Vec<(&'static str, RuleFn)> {\n");
    rules_code.push_str("    vec![\n");
    for entry in all_rules_entries {
        rules_code.push_str(&entry);
        rules_code.push('\n');
    }
    rules_code.push_str("    ]\n");
    rules_code.push_str("}\n\n");

    // Generate all_source_transforms() function
    rules_code
        .push_str("pub fn all_source_transforms() -> Vec<(&'static str, SourceTransformFn)> {\n");
    rules_code.push_str("    vec![\n");
    for entry in all_transforms_entries {
        rules_code.push_str(&entry);
        rules_code.push('\n');
    }
    rules_code.push_str("    ]\n");
    rules_code.push_str("}\n");

    // Write generated code to a file in OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_rules.rs");
    fs::write(&dest_path, rules_code).unwrap();

    println!("cargo:rerun-if-changed=src/rules/");
}
