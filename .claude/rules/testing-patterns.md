---
paths:
  - "tests/**"
---

# Testing Patterns

## Test Pattern

Integration tests in `tests/rules_test.rs`:
- `test_all_rules()` iterates `all_rules()` and runs each rule against its fixtures
- For each fixture file in `tests/rules/<rule_path>/`:
  - If contains `\n-----\n`: split on separator, run rule on input, assert output matches expected
  - If no separator: run rule, assert returns `None` (no-op)

## Fixture Format

### Transform Fixture (rule applies transformation)

```php
// Input above separator
class MyClass {}
-----
// Expected output below separator
final class MyClass {}
```

### No-op Fixture (rule skips it)

```php
// Just the content, no separator
abstract class MyClass {}
```

## Running Tests

```bash
just tests              # Run all tests in Docker
cargo test             # Run locally (requires Rust toolchain)
cargo test -- --nocapture  # Show test output
```

## Test Coverage Requirements

All rules registered in `all_rules()` are automatically tested. Each rule should have at least:
- One positive fixture (shows the transformation)
- One negative fixture per skip case (e.g., `skip_abstract_class.php.inc`)

For example, `add_final_keyword` has:
- `add_final_keyword.php.inc` — transforms regular class
- `add_final_keyword_in_namespace.php.inc` — transforms class in namespace
- `add_final_keyword_on_readonly_class.php.inc` — transforms readonly class
- `skip_abstract_class.php.inc` — no-op for abstract
- `skip_class_with_final_keyword.php.inc` — no-op for already-final
- `skip_interface.php.inc` — no-op for interfaces
- `skip_trait.php.inc` — no-op for traits
