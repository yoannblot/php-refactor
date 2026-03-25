---
name: code-quality-issues
description: |
  Known code quality issues (High, Medium, Low priority) and recommended fixes. Use when triaging quality debt or planning refactoring.
user-invocable: true
allowed-tools: Read, Grep, Glob
---

# Known Code Quality Issues

Identified issues blocking release or requiring refactoring, categorized by priority.

---

## High Priority (Blocking per rust-reviewer)

### H1 — `reporter.rs` unsafe block missing `// SAFETY:` comment

- **Location**: `src/reporter.rs:6-12`
- **Issue**: `libc::getrusage()` return value is ignored; if it fails, the function returns 0 KB silently
- **Risk**: Silent failure mode; no visibility into allocation failures
- **Fix**:
  - Check return value of `getrusage()`
  - Document safety invariant with `// SAFETY:` comment
  - Log or handle failure gracefully instead of silently returning 0

---

### H2 — `.unwrap()` in test fixtures lose error context

- **Location**: `tests/rules_test.rs:22-24, 35`
- **Issue**: `e.unwrap()`, `path.file_name().unwrap()`, `.to_str().unwrap()` panic without useful diagnostics
- **Risk**: Test failures are cryptic; hard to debug fixture problems
- **Fix**: Replace with `.expect("descriptive message")` for each:
  - `e.unwrap()` → `.expect("failed to read fixture file")`
  - `path.file_name().unwrap()` → `.expect("fixture path must have filename")`
  - `.to_str().unwrap()` → `.expect("fixture path must be valid UTF-8")`

---

### H3 — Unnecessary `String` clone in `main.rs`

- **Location**: `src/main.rs:31`
- **Issue**: `let mut content = original.clone()` allocates unnecessarily; use `Cow<'_, str>` or just reference `original`
- **Risk**: Wasted allocation; complicates memory profiling (peak_memory_kb includes unnecessary clone)
- **Fix**:
  - Avoid clone; compare as `&str`
  - Consider using `String` only for mutations, `&str` for reads
  - Or use `Cow<'_, str>` to defer allocation until first mutation

---

## Medium Priority

### M1 — `unreachable!()` encodes mago library invariant

- **Location**: `src/rules/quality/add_final_keyword.rs:43`
- **Issue**: Assumes `contains_readonly() && get_readonly() == Readonly` are always consistent; panics if not
- **Risk**: Production panic if mago-syntax behavior changes or invariant doesn't hold
- **Fix**: Fall back to safe default instead of `unreachable!()`
  ```rust
  let insert_before = if class.modifiers.contains_readonly() {
      class.readonly_position.unwrap_or(class.class_position)
  } else {
      class.class_position
  };
  ```

---

### M2-M3 — Unchecked byte-offset assumptions and casts

- **Location**: `src/rules/quality/add_final_keyword.rs`
- **Issue**: Byte offset casts assume no overflow; UTF-8 boundary assumptions undocumented
- **Risk**: Subtle bugs if casts overflow or UTF-8 assumptions break
- **Mitigation**:
  - Add doc comment: "All spans are byte offsets valid within UTF-8 source"
  - Use `usize::try_from(span.offset)` instead of unchecked casts
  - Add test for multi-byte UTF-8 in class names (if applicable)

---

### M4 — Misleading error messages

- **Location**: `src/main.rs:14, 24, 50`
- **Issue**: Generic error messages don't distinguish between usage (wrong args) vs runtime (I/O, parse failure)
- **Risk**: Users confused about what went wrong
- **Fix**: Use distinct messages:
  - "Usage: php-refactor <file.php>" for argument errors
  - "Failed to read file: {path}" for I/O errors
  - "Failed to parse or transform file" for rule errors

---

### M5 — `all_rules()` allocates `Vec` on every call

- **Location**: `src/rules/mod.rs`
- **Issue**: `Vec::new()` + `vec![]` macro calls heap allocator every time `all_rules()` runs
- **Risk**: Unnecessary allocations on every file processed
- **Fix**: Return `&'static [(&'static str, RuleFn)]` or a static slice instead
  ```rust
  pub fn all_rules() -> &'static [(&'static str, RuleFn)] {
      &[
          ("quality/add_final_keyword", quality::add_final_keyword::apply),
      ]
  }
  ```

---

### M6 — Inefficient string building in `format_timing_line`

- **Location**: `src/reporter.rs` (not specified, but likely in timing output)
- **Issue**: Uses `collect().join()` instead of single `fold`
- **Risk**: Extra allocations for intermediate Vec
- **Fix**: Use single `fold` with a `String`:
  ```rust
  let rule_times = rules.iter().fold(String::new(), |mut acc, (name, ms)| {
      acc.push_str(&format!("{}: {}ms, ", name, ms));
      acc
  });
  ```

---

### M7 — `peak_memory_kb` is public but undocumented

- **Location**: `src/reporter.rs`
- **Issue**: Public field has no doc comment; return type and units unclear
- **Risk**: API misuse; unclear what value represents
- **Fix**:
  - Add doc comment: `/// Peak memory usage in kilobytes (KB), as reported by getrusage()`
  - Write unit test verifying function returns a valid usize

---

## Low Priority

### L1-L6 — Minor Inefficiencies

- **L1**: Test patterns could use more specific assertions
- **L2**: Missing edge-case fixtures (e.g., empty class, class with only comments)
- **L3**: Direct `libc` dependency; could use safer wrapper
- **L4**: No benchmarking harness for rule performance
- **L5**: Reporter output format not configurable (JSON, CSV options)
- **L6**: No progress indicator for batch processing

---

## Resolution Strategy

1. **Start with H1–H3**: These block release. Estimate 2–4 hours.
2. **Address M1–M3**: Production safety. Estimate 3–5 hours.
3. **M4–M7**: Quality of life. Can batch into one session, ~2 hours.
4. **L1–L6**: Deferred; revisit after core stability achieved.

Each fix should include a test demonstrating the issue before the fix and success after.
