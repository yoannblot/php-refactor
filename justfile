# php-refactor task runner

# List available commands
default:
    @just --list

# Build Docker image
docker-build:
    docker compose build

# Build debug binary (inside Docker, fast iteration)
build:
    docker compose run --rm app cargo build

# Build release binary (optimized)
build-release:
    docker compose run --rm app cargo build --release

# Auto-fix code: format + run tests
quality-tools:
    docker compose run --rm app cargo fmt

# Quality check for CI: verify format + lint + compile + tests (fails if any issue)
quality-check:
    docker compose run --rm app cargo check
    docker compose run --rm app cargo fmt --check
    docker compose run --rm app cargo clippy -- -D warnings

# Test suite
tests:
    docker compose run --rm app cargo test
