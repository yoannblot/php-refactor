---
name: add-new-rule
description: |
  Step-by-step guide to implement a new PHP transformation rule in this project: scaffold the rule module, register in the module hierarchy, add test fixtures, update documentation. Use when the user asks to "add a rule", "add a new rule", "create a rule", "implement a transformation", or invokes /add-new-rule.
user-invocable: true
allowed-tools: Read, Write, Edit, Glob, Grep
---

# Adding a New Rule

Follow these four steps to implement a new PHP transformation rule.

---

## 1. Create the Rule Module

Create `src/rules/<category>/<rule_name>.rs`:

```rust
/// Applies transformation to source code. Returns None if no changes needed.
pub fn apply(source: &str) -> Option<String> {
    // Parse source, apply transformation, return Some(modified) or None
    todo!()
}
```

The rule must be a pure function: no global state, no side effects, deterministic output.

**File naming**: Use snake_case. Example: `add_final_keyword.rs`, `remove_dead_code.rs`.

---

## 2. Register in Module Hierarchy

### Create or update `src/rules/<category>/mod.rs`

If the category doesn't exist, create it:

```rust
pub mod rule_name;
```

### Update `src/rules/mod.rs`

Add your rule to the `all_rules()` registry:

```rust
pub fn all_rules() -> Vec<(&'static str, RuleFn)> {
    vec![
        ("quality/add_final_keyword", quality::add_final_keyword::apply),
        ("category/rule_name", category::rule_name::apply),  // ← new entry
    ]
}
```

**Key format**:
- Prefix: category (matches directory name)
- Suffix: rule name (matches file name)
- Use forward slashes and snake_case

---

## 3. Add Test Fixtures

Create `tests/rules/<category>/<rule_name>/` directory with `.php.inc` fixture files.

### Transform Fixture (rule applies transformation)

```php
// Input above separator
class MyClass {}
-----
// Expected output below separator
final class MyClass {}
```

Name it: `<rule_name>.php.inc` or something descriptive like `add_final_keyword.php.inc`.

### No-op Fixture (rule skips it)

```php
// Just the content, no separator
abstract class MyClass {}
```

Name it: `skip_<reason>.php.inc` (e.g., `skip_abstract_class.php.inc`).

### Run Tests

```bash
just tests              # Run all tests in Docker
cargo test             # Run locally
cargo test -- --nocapture  # Show output
```

Fixtures are discovered and tested automatically via `tests/rules_test.rs`.

---

## 4. Implementation Tips

### Use mago-syntax AST
Walk `Sequence<Statement>` to find classes, namespaces, functions, etc.

```rust
use bumpalo::Bump;
use mago_syntax::parser::Parser;

let arena = Bump::new();
let mut parser = Parser::new(&arena, source.as_bytes());
let result = parser.parse().ok()?;
let ast = result;

for statement in &ast.statements {
    // Process each statement
}
```

### Byte Offsets & Text Edits
- AST spans are byte offsets (not character offsets)
- Collect all edits, then apply back-to-front to preserve offset validity
- Return `None` if no changes; return `Some(String)` if changed

```rust
let mut edits = vec![];
// ... collect (position, "text") tuples ...
edits.sort_by(|a, b| b.0.cmp(&a.0));  // Sort descending

let mut result = source.to_string();
for (pos, text) in edits {
    result.insert_str(pos, text);
}

if result == source { None } else { Some(result) }
```

### Allocator
Use `bumpalo::Bump` arena for fast, efficient AST allocation (single-free).

### Return Semantics
- `None` = no change needed
- `Some(String)` = changed
- Never panic on malformed input — either transform or return `None`

### Idempotency (Critical)
Applying a rule twice to the same file must produce the same output as once. This is essential for rule chaining.

Test explicitly: write a fixture, run your rule on its output, verify it returns `None` (no further changes).

---

## 5. Update Documentation

After the rule passes tests, update these three documentation surfaces so the new rule is discoverable:

- **`docs/rules.md`** — user-facing reference. Add a new `## quality/<rule_name>` section with Summary, When to use, What it does, What it skips, Example (before/after PHP), and Known limitations.
- **`README.md`** — one-line row in the "Available Rules" table.
- **`.claude/rules/architecture.md`** — agent-facing implementation notes under "Current Rules". Describe the regex/AST approach, skip conditions, and known limitations.

Each doc has a distinct audience — do not collapse them into one.

---

## Example: Full Skeleton

```rust
use bumpalo::Bump;
use mago_syntax::parser::Parser;

pub fn apply(source: &str) -> Option<String> {
    let arena = Bump::new();
    let mut parser = Parser::new(&arena, source.as_bytes());
    let result = parser.parse().ok()?;
    let ast = result;

    let mut edits = vec![];

    for statement in &ast.statements {
        // Process statements, collect edits
    }

    if edits.is_empty() {
        return None;
    }

    // Apply edits back-to-front
    edits.sort_by(|a, b| b.0.cmp(&a.0));
    let mut result = source.to_string();
    for (pos, text) in edits {
        result.insert_str(pos, text);
    }

    Some(result)
}
```

---

## Checklist

- [ ] Rule module created in `src/rules/<category>/<rule_name>.rs`
- [ ] Module registered in `src/rules/<category>/mod.rs`
- [ ] Rule added to `all_rules()` in `src/rules/mod.rs`
- [ ] At least one positive fixture created (`<rule_name>.php.inc`)
- [ ] At least one no-op fixture per skip case created
- [ ] Tests pass: `just tests` or `cargo test`
- [ ] Rule is idempotent (applying twice = applying once)
- [ ] No panics on malformed input
- [ ] `docs/rules.md` updated with a user-facing section
- [ ] `README.md` rules table updated
- [ ] `.claude/rules/architecture.md` updated with implementation notes
