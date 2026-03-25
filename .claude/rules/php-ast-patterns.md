---
paths:
  - "src/rules/**"
---

# PHP AST Implementation Patterns

## Overview

Rules transform PHP code by:
1. Parsing source into an AST using `mago-syntax`
2. Allocating the AST into a `bumpalo` arena (zero-copy, single-free)
3. Walking the AST to collect byte offsets (from spans)
4. Applying text edits back-to-front (to preserve offset validity)
5. Returning `Some(modified_source)` or `None`

---

## Parsing & Allocation

```rust
use bumpalo::Bump;
use mago_syntax::parser::Parser;

pub fn apply(source: &str) -> Option<String> {
    // Create arena for zero-copy AST allocation
    let arena = Bump::new();

    // Parse into AST
    let mut parser = Parser::new(&arena, source.as_bytes());
    let result = parser.parse();

    // Handle parse errors gracefully — return None, don't panic
    let ast = result.ok()?;

    // ... rest of rule logic
}
```

---

## Walking the AST

`mago-syntax` provides AST node types like:
- `Statement` (enum of statement variants)
- `Sequence<Statement>` (collection of statements)
- `Class` (class declaration with modifiers, name, members)
- `Namespace` (namespace wrapper around statements)

Example: find all top-level and namespaced classes:

```rust
for statement in &ast.statements {
    match statement {
        Statement::Class(class) => {
            // Process top-level class
        }
        Statement::Namespace(ns) => {
            for inner_stmt in &ns.statements {
                if let Statement::Class(class) = inner_stmt {
                    // Process namespaced class
                }
            }
        }
        _ => {}
    }
}
```

---

## Collecting Byte Offsets

AST nodes have `.span` (a byte offset range). Collect all spans that need edits:

```rust
let mut edits: Vec<(usize, &'static str)> = vec![];

// For each class that needs a `final` keyword
for class in classes_needing_final {
    let position = class.span.start; // byte offset
    edits.push((position, "final "));
}
```

**Important**: Spans are byte offsets, not character offsets. UTF-8 matters.

---

## Applying Text Edits Back-to-Front

To avoid offset shifting, apply edits in reverse order (from end of file to start):

```rust
// Sort edits by position (descending)
edits.sort_by(|a, b| b.0.cmp(&a.0));

let mut result = source.to_string();

for (pos, text) in edits {
    result.insert_str(pos, text);
}

if result == source {
    None  // No changes
} else {
    Some(result)
}
```

**Why back-to-front?** Inserting at position 10 shifts all positions after 10 forward. By working backward, earlier positions remain valid.

---

## Common Patterns

### Checking Modifiers

```rust
if class.modifiers.contains_final() {
    // Already has `final`
    return None;
}

if class.modifiers.contains_abstract() {
    // Skip abstract classes
    return None;
}
```

### Handling `readonly` Classes

If inserting before the `class` keyword, check for `readonly`:

```rust
let insert_before = if class.modifiers.contains_readonly() {
    class.readonly_position
} else {
    class.class_position
};

edits.push((insert_before, "final "));
```

### Safe Byte Offset Handling

Document UTF-8 assumptions. If casting `usize`, use `usize::try_from()`:

```rust
let pos: usize = usize::try_from(span.start)
    .expect("span offset should fit in usize");
```

---

## Error Handling

- **Never panic on malformed input.** The rule runs on user code.
- **Return `None` if AST walk fails or is incomplete.** Better to skip than crash.
- **Log suspicious assumptions with comments.** Example: "assumes mago-syntax always sets span.start < span.end"

---

## Testing Your Rule

See `/testing-patterns` for fixture format.

Key test cases:
- One positive fixture (shows transformation)
- One no-op fixture per skip condition (abstract, final, readonly, interface, trait, enum)
- Edge case: mixing `readonly` with other modifiers
- Idempotency: running rule twice should be idempotent
