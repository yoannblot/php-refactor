---
name: project-glossary
description: |
  Definitions of key terms used in the php-refactor project (Rule, Fixture, AST, Span, Bump Arena, Idempotent, mago-syntax).
user-invocable: true
allowed-tools: Read, Grep
---

# Project Glossary

Key definitions for understanding the php-refactor codebase.

---

## **Rule**

A pure function `fn(&str) -> Option<String>` that transforms PHP code or returns `None`.

- **Input**: PHP source code as `&str`
- **Output**: `None` if no changes, `Some(modified_source)` if changed
- **Properties**: No state, no side effects, deterministic (same input = same output)
- **Example**: `add_final_keyword::apply` adds `final` keyword to classes

See `.claude/rules/rule-contract.md` for full contract.

---

## **Fixture**

A `.php.inc` test file that defines the expected behavior of a rule.

**Transform fixture** (rule applies transformation):
```php
// Input
class MyClass {}
-----
// Expected output
final class MyClass {}
```

**No-op fixture** (rule skips it):
```php
// Just content, no separator
abstract class MyClass {}
```

Fixtures are discovered and tested automatically by `tests/rules_test.rs`.

---

## **AST**

Abstract Syntax Tree — a hierarchical representation of source code structure.

For PHP, `mago-syntax` parses source into an AST with nodes like:
- `Statement` (class, function, namespace, etc.)
- `Class` (class declaration with modifiers, name, members)
- `Namespace` (wrapper for namespaced statements)

Rules walk the AST to find and transform code elements.

---

## **Span**

A byte offset range in source code. `span.start` and `span.end` mark the position of an AST node.

- **Byte offsets**: Positions in UTF-8 bytes, not characters
- **Example**: A class starting at byte 10 might have `span: {start: 10, end: 35}`
- **Why it matters**: Text edits use spans to know where to insert/delete

---

## **Bump Arena** (bumpalo)

A memory allocator optimized for allocating many short-lived objects that are freed all at once.

- **Used for**: AST allocation in mago-syntax parsing
- **Benefit**: Zero-copy traversal; single-free (arena destruction frees all nodes at once)
- **Pattern**: Create one arena per rule invocation

```rust
let arena = Bump::new();
let mut parser = Parser::new(&arena, source.as_bytes());
let ast = parser.parse().ok()?;
// arena freed when dropped
```

---

## **Idempotent**

A property where applying an operation twice yields the same result as applying it once.

For rules: **Applying a rule twice to the same file must produce the same output as applying it once.**

**Why it matters**: Rules are chained. If rule A modifies file → rule B modifies result → rule A runs again, rule A must not re-trigger on its own output.

**Test it**:
```rust
let output1 = rule::apply(input);
let output2 = rule::apply(&output1.unwrap_or(input));
assert_eq!(output2, None);  // Second run should be no-op
```

---

## **mago-syntax**

A PHP parser written in Rust. Provides:
- `Parser` struct for parsing source into AST
- AST node types (`Statement`, `Class`, `Namespace`, etc.)
- Span information for each node

**Key types**:
- `Statement` — enum of statement variants
- `Sequence<T>` — collection of AST nodes
- `Class` — class declaration with modifiers and members

See `.claude/rules/php-ast-patterns.md` for usage patterns.

---

## **Rule Registry**

The `all_rules()` function in `src/rules/mod.rs` returns a list of all registered rules.

```rust
pub fn all_rules() -> Vec<(&'static str, RuleFn)> {
    vec![
        ("quality/add_final_keyword", quality::add_final_keyword::apply),
    ]
}
```

- **Key**: Rule name (used in timing reports and test discovery)
- **Value**: Function pointer to the rule's `apply` function
- **Single source of truth**: All rules must be registered here
- **Integration tests**: Automatically discover and test all registered rules

---

## **Rule Chaining**

The process of applying multiple rules in sequence.

In `src/main.rs`:
1. Load file into `original`
2. For each rule in `all_rules()`:
   - Run rule on current content
   - If `Some(modified)`, use that as input to next rule
   - If `None`, use same content
3. Write result if changed from original

This is why **idempotency** is critical — each rule sees the output of the previous one.

---

## **Peak Memory Usage**

The maximum amount of memory consumed during rule execution, reported by `libc::getrusage()`.

- **Reported in**: Timing output line (e.g., "peak memory: 2.3 MB")
- **What it measures**: Max resident set size during rule chaining
- **Limitation**: Only available on macOS/Linux; returns 0 elsewhere
- **Note**: Includes unnecessary clones and allocations (see H3 code quality issue)

---

## **Timing Report**

Output line printed after rule execution:

```
[INFO] Refactoring took 12.34ms (peak memory: 2.3MB) | quality/add_final_keyword: 5.67ms, another_rule: 1.23ms
```

Components:
- **Total time**: Wall-clock time for all rules
- **Peak memory**: Max resident set size
- **Per-rule timing**: Wall-clock time for each rule in sequence

Reported by `src/reporter.rs`.
