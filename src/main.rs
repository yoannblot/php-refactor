use php_refactor::rules;
use std::process;
use std::time::Instant;

fn main() {
    let total_start = Instant::now();

    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!(
                "[ERROR] Something went wrong: Usage: php-refactor <path/to/file.php|directory|config.toml>"
            );
            eprintln!("[INFO] No processing, exiting.");
            process::exit(1);
        }
    };

    let mut total_changed = 0;
    let mut total_analyzed = 0;

    for (_, rule_fn) in rules::all_rules() {
        let result = rule_fn(&path);
        total_changed += result.files_changed;
        total_analyzed = result.files_analyzed;
    }

    let total_elapsed = total_start.elapsed();

    if total_analyzed == 0 {
        println!("[OK] No PHP files found, nothing to do.");
    } else {
        println!(
            "[OK] {} file(s) changed, {} file(s) analyzed.",
            total_changed, total_analyzed
        );
    }

    eprintln!(
        "[INFO] {:.2}ms total duration.",
        total_elapsed.as_secs_f64() * 1000.0
    );
}
