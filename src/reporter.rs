use std::time::Duration;

pub fn peak_memory_kb() -> u64 {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    unsafe {
        let mut rusage: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut rusage);
        #[cfg(target_os = "macos")]
        return (rusage.ru_maxrss / 1024) as u64;
        #[cfg(target_os = "linux")]
        return rusage.ru_maxrss as u64;
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    0
}

pub fn format_memory(kb: u64) -> String {
    if kb >= 1024 {
        format!("{:.1}MB peak memory", kb as f64 / 1024.0)
    } else {
        format!("{}KB peak memory", kb)
    }
}

pub fn format_timing_line(
    rule_timings: &[(&str, Duration)],
    total: Duration,
    memory_kb: u64,
) -> String {
    let memory_str = format_memory(memory_kb);
    let rules_str: String = rule_timings
        .iter()
        .map(|(name, d)| format!("{}: {:.2?}", name, d))
        .collect::<Vec<_>>()
        .join(", ");

    if rules_str.is_empty() {
        format!("{:.2?} total duration, {}.", total, memory_str)
    } else {
        format!(
            "{:.2?} total duration, {}. {}",
            total, memory_str, rules_str
        )
    }
}

pub fn print_timing(rule_timings: &[(&str, Duration)], total: Duration) {
    println!(
        "{}",
        format_timing_line(rule_timings, total, peak_memory_kb())
    );
}
