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

