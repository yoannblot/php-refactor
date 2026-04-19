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

