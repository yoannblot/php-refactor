# php-refactor — Developer Guide

A fast PHP refactoring CLI tool written in Rust. Reads a PHP file, applies a series of AST-based transformation rules in sequence, and writes the file back in-place if changed.

**Purpose**: Automate recurring PHP code quality transformations (formatting, standardization, compliance) at scale without manual review per file.

## Quick Reference

### Build & Test

```bash
just docker-build    # Build the Docker image
just build           # Build the binary (dev mode)
just build-release   # Build optimized release binary
just tests           # Run test suite
just quality-check   # fmt check, clippy, tests
just quality-tools   # Auto-format code with rustfmt
```

### Running the Tool

```bash
cargo run -- path/to/file.php
# or
./target/debug/php-refactor path/to/file.php
```

## How to Find What You Need

- **Writing or modifying a rule?** → See `.claude/rules/rule-contract.md` (function contract, return semantics)
- **Implementing PHP AST transformations?** → See `.claude/rules/php-ast-patterns.md` (mago-syntax, bumpalo, byte-offset editing)
- **Adding a new rule?** → Invoke `/add-new-rule` skill for the full 4-step walkthrough
- **Triaging code quality issues?** → Invoke `/code-quality-issues` skill for the H/M/L backlog
- **Looking at architecture in `src/`?** → `.claude/rules/architecture.md` (loaded automatically when working in src/)
- **Understanding test fixtures in `tests/`?** → `.claude/rules/testing-patterns.md` (loaded automatically when working in tests/)
- **Need a definition?** → Invoke `/glossary` skill (Rule, Fixture, AST, Span, Bump Arena, Idempotent, mago-syntax)

## Project Structure

```
src/
  ├── main.rs          # CLI entry, rule chaining, file I/O
  ├── lib.rs           # Module re-exports
  ├── reporter.rs      # Timing and memory reporting
  └── rules/mod.rs     # Rule registry (all_rules)

tests/
  ├── rules_test.rs    # Fixture-based integration test runner
  └── rules/           # Test fixtures by rule path

.github/workflows/quality.yml   # CI: check, fmt, clippy, test
Cargo.toml / Cargo.lock         # Dependencies
justfile                         # Task automation
```
