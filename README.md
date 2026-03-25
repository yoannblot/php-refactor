# PHP Refactor

A fast, AST-based PHP code refactoring CLI tool written in Rust. Automatically applies transformations like adding `final` keywords, enforcing code standards, and more — with zero manual review needed.

## Why PHP Refactor?

- **AST-powered**: Parses PHP into an abstract syntax tree for safe, accurate transformations (not regex-based)
- **Fast**: Single-threaded, compiled Rust binary — processes files in milliseconds
- **Extensible**: Add new rules as pure functions; all rules are automatically tested against fixtures
- **Zero false positives**: Each rule includes fixtures for what it transforms and what it skips
- **Composable**: Rules chain together; you can apply multiple transformations in one pass

## Installation

### From Source

```bash
git clone https://github.com/yoannblot/php-refactor.git
cd php-refactor
cargo build --release
```

Binary will be at `target/release/php-refactor`.

### With Docker

```bash
docker build -t php-refactor .
docker run --rm -v $(pwd):/workspace php-refactor \
  /app/target/release/php-refactor /workspace/MyClass.php
```

## Usage

```bash
php-refactor path/to/file.php
```

**Example:**

```bash
$ cat MyClass.php
<?php
class MyClass {
    public function hello() {}
}

$ php-refactor MyClass.php

$ cat MyClass.php
<?php
final class MyClass {
    public function hello() {}
}
```

The tool modifies the file in-place. If no rules apply, the file is left unchanged.

### Output

The tool prints a summary:

```
[INFO] Refactoring took 12.34ms (peak memory: 2.1MB) | quality/add_final_keyword: 5.67ms
```

- **Timing**: Wall-clock time for the entire operation and per-rule
- **Memory**: Peak RSS memory usage (macOS/Linux only; 0 on other platforms)

## Available Rules

| Rule | What It Does |
|------|-------------|
| `quality/add_final_keyword` | Adds `final` keyword to concrete classes |

See [docs/rules.md](docs/rules.md) for detailed rule documentation.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add rules or fix bugs (see [docs/rules.md](docs/rules.md) for the implementation guide)
4. Run `just quality-check` to ensure all checks pass
5. Open a pull request

## License

See `LICENSE` file in the repository.
