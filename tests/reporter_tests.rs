use php_refactor::reporter::{format_memory, format_timing_line};
use std::time::Duration;

// format_memory tests
#[test]
fn test_format_memory_bytes_under_mb() {
    assert_eq!(format_memory(0), "0KB peak memory");
    assert_eq!(format_memory(512), "512KB peak memory");
    assert_eq!(format_memory(1023), "1023KB peak memory");
}

#[test]
fn test_format_memory_exact_mb() {
    assert_eq!(format_memory(1024), "1.0MB peak memory");
    assert_eq!(format_memory(2048), "2.0MB peak memory");
}

#[test]
fn test_format_memory_fractional_mb() {
    assert_eq!(format_memory(1536), "1.5MB peak memory");
    assert_eq!(format_memory(10240), "10.0MB peak memory");
}

// format_timing_line tests
#[test]
fn test_format_timing_line_no_rules() {
    let line = format_timing_line(&[], Duration::from_millis(100), 512);
    assert_eq!(line, "100.00ms total duration, 512KB peak memory.");
}

#[test]
fn test_format_timing_line_with_rules() {
    let timings = vec![("quality/add_final_keyword", Duration::from_millis(50))];
    let line = format_timing_line(&timings, Duration::from_millis(100), 2048);
    assert_eq!(
        line,
        "100.00ms total duration, 2.0MB peak memory. quality/add_final_keyword: 50.00ms"
    );
}

#[test]
fn test_format_timing_line_multiple_rules() {
    let timings = vec![
        ("dir1/rule_a", Duration::from_millis(20)),
        ("dir2/rule_b", Duration::from_millis(30)),
    ];
    let line = format_timing_line(&timings, Duration::from_millis(100), 1024);
    assert_eq!(
        line,
        "100.00ms total duration, 1.0MB peak memory. dir1/rule_a: 20.00ms, dir2/rule_b: 30.00ms"
    );
}
