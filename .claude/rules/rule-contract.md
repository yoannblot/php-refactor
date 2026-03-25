---
paths:
  - "src/rules/**"
---

# Rule Contract

## Function Signature

Every rule is a pure function:

```rust
type RuleFn = fn(&str) -> Option<String>
```

**Input**: PHP source code as `&str`
**Output**:
- `None` if no changes needed
- `Some(modified_source)` if the rule transformed the code

## Constraints

### Pure Function
- No global state
- No side effects
- Deterministic output — same input always produces same output
- No panic on malformed input — either transform or return `None`

### Return Semantics
- Return `None` = "no changes needed, skip to next rule"
- Return `Some(String)` = "I changed the code, pass this to next rule"
- Never return an unchanged copy of the input; return `None` instead

### Idempotency (Critical)
**Applying a rule twice to the same file must produce the same output as applying it once.**

This is essential for rule chaining. If rule A produces output, then rule B runs on it, then rule A runs again, the result must be identical to the first time rule A ran.

Test this explicitly: if you write a fixture file, verify that running your rule twice produces the same result.

### Registration

Rules are registered in `src/rules/mod.rs` via the `all_rules()` function:

```rust
pub fn all_rules() -> Vec<(&'static str, RuleFn)> {
    vec![
        ("category/rule_name", category::rule_name::apply),
    ]
}
```

The key (first element) is used in timing reports and test discovery. Use forward slashes and snake_case.
