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

**Single file mode** — refactor one PHP file:
```bash
cargo run -- app.php
./target/debug/php-refactor app.php
```

**Directory mode** — refactor all PHP files in a directory (recursive):
```bash
cargo run -- src/
./target/debug/php-refactor tests/
```

**Config mode** — refactor multiple directories defined in a TOML config file:
```bash
cargo run -- config.toml
./target/debug/php-refactor target/config.toml
```

### Config File Format

Create a `config.toml` file to process multiple directories at once:

```toml
[source]
paths = ["src", "tests", "app"]
```

Paths can be:
- Relative to the project root (where you run the command): `"src"`, `"./app"`
- Absolute: `"/home/user/project/src"`
- Nested: `"src/components/php"`

The tool will recursively walk each path and apply all rules to every `.php` file found.

## How Rules Work

Rules receive a **path** (file, directory, or config file) and are responsible for:
1. Expanding it to actual PHP files (single file, directory walk, config parsing)
2. Reading each file, applying transformations, writing back if changed
3. Returning stats: how many files changed and how many were analyzed

This design keeps `main.rs` simple — it just passes the input path to each rule in sequence. Each rule is autonomous and can process different subsets of files if needed.

## How to Find What You Need

- **Writing or modifying a rule?** → See `.claude/rules/rule-contract.md` (function contract, return semantics)
- **Implementing PHP AST transformations?** → See `.claude/rules/php-ast-patterns.md` (mago-syntax, bumpalo, byte-offset editing)
- **Adding a new rule?** → Invoke `/add-new-rule` skill for the full 4-step walkthrough
- **Triaging code quality issues?** → Invoke `/code-quality-issues` skill for the H/M/L backlog
- **Looking at architecture in `src/`?** → `.claude/rules/architecture.md` (loaded automatically when working in src/)
- **Understanding test fixtures in `tests/`?** → `.claude/rules/testing-patterns.md` (loaded automatically when working in tests/)
- **Need a definition?** → Invoke `/glossary` skill (Rule, Fixture, AST, Span, Bump Arena, Idempotent, mago-syntax)
- **Understanding file discovery?** → See `src/resolver.rs` (handles single files, directories, and TOML config expansion)

## Project Structure

```
src/
  ├── main.rs          # CLI entry: get path, loop rules, report results
  ├── lib.rs           # Module re-exports
  └── rules/
      ├── mod.rs       # Rule registry (all_rules, all_source_transforms)
      └── quality/     # Quality rules by category
          ├── mod.rs
          ├── add_final_keyword.rs      # Adds `final` to concrete classes
          └── add_readonly_keyword.rs   # Adds `readonly` to bare classes (PHP 8.2+)

tests/
  ├── *_test.rs           # Individual unit tests
  ├── rules_test.rs       # Rule integration tests (uses source fixtures)
  └── fixtures/           # Versioned fixtures for test data

Cargo.toml / Cargo.lock   # Dependencies
justfile                  # Task automation
```
