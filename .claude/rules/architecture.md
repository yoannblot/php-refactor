---
paths:
  - "src/**"
---

# Architecture

## Core Components

**CLI Entry Point** (`src/main.rs`)
- Reads a single PHP file path from `argv[1]`
- Loads file contents into memory
- Iterates over all registered rules from `all_rules()`, chaining each rule's output into the next
- If final content differs from original, writes the file back
- Prints timing report per rule and peak memory usage

**Library Root** (`src/lib.rs`)
- Trivial re-export of public modules (`reporter`, `rules`)

**Rule System** (`src/rules/mod.rs`)
- Defines the rule contract: `type RuleFn = fn(&str) -> Option<String>`
  - Input: PHP source code as `&str`
  - Output: `None` if no changes needed, `Some(modified_source)` if changed
  - Rules are pure functions — no side effects, no state
- `pub fn all_rules() -> Vec<(&'static str, RuleFn)>` — the rule registry
  - Single place to register new rules
  - Integration tests automatically discover and run all registered rules

**Observability** (`src/reporter.rs`)
- Measures wall-clock time per rule (via `std::time::Instant`)
- Reports peak memory usage via `libc::getrusage()` (macOS/Linux only; returns 0 elsewhere)
- Formats and prints timing line to stdout: `[INFO] Refactoring took X.XXms (peak memory: X.XMB) | rule1: Xms, rule2: Xms, ...`

**PHP AST Parsing**
- Uses `mago-syntax` (PHP ecosystem) to parse source into an AST
- Allocates AST into `bumpalo` arena for zero-copy traversal
- Rules walk the AST, collect byte offsets (from spans), then apply text edits back-to-front (to preserve offset validity)

---

## Current Rules

### `src/rules/quality/add_final_keyword.rs`

**What it does**: Adds `final` keyword to non-abstract, non-final PHP classes (prevents accidental subclassing)

**Behavior**:
- Parses PHP source into AST
- Walks all class declarations and classes within namespaces
- For each class: if it has no `final` or `abstract` modifiers, inserts `final ` before the `class` keyword
- If class is `readonly`, inserts before `readonly` to produce `final readonly class Foo {}`
- Applies insertions back-to-front to avoid offset shifting
- Returns `None` if no changes; returns `Some(modified_source)` if changed

**Skips**:
- Abstract classes
- Classes that already have `final`
- Interfaces and traits (no `final` modifier)
- Enums (PHP enums cannot be marked `final`)

---

## Quality Issues (High Priority)

See `/code-quality-issues` skill for the full H1–H3 blockers, M1–M7 medium-priority, and L1–L6 low-priority list.

Key high-priority items to address:
- **H1**: `reporter.rs` unsafe block missing `// SAFETY:` comment
- **H2**: `.unwrap()` in test fixtures lose error context
- **H3**: Unnecessary `String` clone in `main.rs`
