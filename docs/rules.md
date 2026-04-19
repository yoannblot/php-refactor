# PHP Refactor Rules

Detailed documentation of all available transformation rules.

---

## `quality/add_final_keyword`

**Summary**: Adds `final` keyword to concrete classes to prevent accidental subclassing.

**When to use**: Enforce class design intent — mark classes that shouldn't be extended as `final`.

**What it does:**
- Adds `final` keyword before any non-abstract, non-final class declaration
- Supports classes inside namespaces
- Handles `readonly` classes: produces `final readonly class`
- Returns `None` (no change) if already processed

**What it skips:**
- Abstract classes (marked `abstract`)
- Classes that already have `final`
- Interfaces and traits (no `final` modifier in PHP)
- Enums (PHP enums cannot be marked `final`)

**Example:**

```php
// Before
class Service {}
readonly class Config {}
namespace App;
class Model {}

// After
final class Service {}
final readonly class Config {}
namespace App;
final class Model {}
```

---

## `quality/add_readonly_keyword`

**Summary**: Adds `readonly` keyword to bare concrete classes so all instance properties are implicitly readonly (PHP 8.2+).

**When to use**: Enforce immutability on value objects, DTOs, and other classes whose state shouldn't change after construction.

**What it does:**
- Adds `readonly` keyword before any bare `class` declaration
- Supports classes inside namespaces
- Preserves original indentation

**What it skips:**
- Classes already marked `readonly`
- `final class` and `abstract class` declarations (out of scope — bare classes only)
- Interfaces, traits, and enums (cannot be `readonly` in PHP)
- Class references in expressions (`::class`, `$class`, etc.)

**Example:**

```php
// Before
class UserDto {}
namespace App;
class Config {}

// After
readonly class UserDto {}
namespace App;
readonly class Config {}
```

**Known limitations (syntactic rule — no class body analysis):**

PHP will raise a fatal error at class-load time if a `readonly` class contains any of:
- Static properties
- Untyped properties
- Properties with default values
- `#[AllowDynamicProperties]` attribute
- A non-readonly parent class

Scope the rule via `config.toml` path globs to classes you know are readonly-compatible:

```toml
[quality]
add_readonly_keyword.paths = ["src/Dto/**/*.php", "src/ValueObject/**/*.php"]
```

---

## `quality/remove_redundant_readonly_keyword`

**Summary**: Removes redundant `readonly` from properties and constructor-promoted parameters inside classes already declared `readonly` (PHP 8.2+).

**When to use**: Clean up noise after class-level `readonly` is adopted — the class modifier already makes every instance property readonly, so per-property `readonly` is redundant.

**What it does:**
- Detects classes whose modifier list contains `readonly` in any order (`readonly class`, `final readonly class`, `readonly final class`, `abstract readonly class`)
- Inside each matching class body, strips the `readonly` keyword from lines that start with a visibility modifier (`public`, `protected`, `private`), optionally followed by `static`
- Applies to both regular property declarations and constructor-promoted parameters (they share the same syntactic shape)
- Uses brace counting to scope edits strictly to the class body

**What it skips:**
- Bare classes (`class Foo { private readonly Bar $x; }`) — here `readonly` is load-bearing and must NOT be stripped
- Classes already free of redundant `readonly` (idempotent)
- Interfaces, traits, enums (cannot be `readonly` in PHP)

**Example:**

```php
// Before
final readonly class ActivityDashboardFactory
{
    private readonly TeamRepository $teamRepository;

    public function __construct(
        private readonly PeriodFactory $periodFactory,
    ) {}
}

// After
final readonly class ActivityDashboardFactory
{
    private TeamRepository $teamRepository;

    public function __construct(
        private PeriodFactory $periodFactory,
    ) {}
}
```

**Known limitations (syntactic rule — no string/comment awareness):**

The brace counter used to scope the class body is naive: raw `{` or `}` bytes inside string literals, heredocs, or comments can skew the scan. Scope via `config.toml` path globs to code you trust.

