use php_refactor::reporter;
use php_refactor::rules;
use std::fs;
use std::process;
use std::time::Instant;

fn main() {
    let total_start = Instant::now();

    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            let elapsed = total_start.elapsed();
            eprintln!("[ERROR] Something went wrong: Usage: php-refactor <path/to/file.php>");
            reporter::print_timing(&[], elapsed);
            process::exit(1);
        }
    };

    let original = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            let elapsed = total_start.elapsed();
            eprintln!("[ERROR] Something went wrong: {}", e);
            reporter::print_timing(&[], elapsed);
            process::exit(1);
        }
    };

    let mut rule_timings: Vec<(&str, std::time::Duration)> = Vec::new();
    let mut content = original.clone();

    for (rule_name, rule_fn) in rules::all_rules() {
        let rule_start = Instant::now();
        let result = rule_fn(&content);
        rule_timings.push((rule_name, rule_start.elapsed()));

        if let Some(new_content) = result {
            content = new_content;
        }
    }

    let total_elapsed = total_start.elapsed();

    // Separate "needs update?" step — rule logic stays pure
    let needs_update = content != original;

    if needs_update {
        if let Err(e) = fs::write(&path, &content) {
            eprintln!("[ERROR] Something went wrong: {}", e);
            reporter::print_timing(&rule_timings, total_elapsed);
            process::exit(1);
        }
        println!("[OK] 1 file has been changed, 1 file analyzed.");
    } else {
        println!("[OK] 1 file analyzed, nothing to do.");
    }

    reporter::print_timing(&rule_timings, total_elapsed);
}
