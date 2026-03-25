#!/bin/bash
VERSION=$(grep '^version' Cargo.toml | grep -o '"[^"]*"' | head -1 | tr -d '"')
mv target/release/php-refactor target/release/php-refactor-${VERSION}
echo "Binary available at: $(pwd)/target/release/php-refactor-${VERSION}"
