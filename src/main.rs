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

    // Load config once instead of per-rule
    let config = if path.ends_with(".toml") {
        php_refactor::config::load(&path).ok()
    } else {
        None
    };

    for (rule_key, rule_fn) in rules::all_rules() {
        let collect_start = Instant::now();
        let files =
            php_refactor::resolver::resolve_for_rule_with_config(&path, rule_key, config.as_ref());
        let collect_ms = collect_start.elapsed().as_secs_f64() * 1000.0;

        let process_start = Instant::now();
        let result = rule_fn(&files);
        let process_ms = process_start.elapsed().as_secs_f64() * 1000.0;

        eprintln!(
            "[INFO] {}: collected {} in {:.2}ms → matched {}, changed {}, processed in {:.2}ms",
            rule_key,
            files.len(),
            collect_ms,
            result.files_matched,
            result.files_changed,
            process_ms
        );

        total_changed += result.files_changed;
        total_analyzed += result.files_analyzed;
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
