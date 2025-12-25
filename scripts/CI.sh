#!/usr/bin/env bash
set -euo pipefail
shopt -s globstar 

# Pytest(No no benchmark)
uv run pytest --benchmark-skip
echo "Passed pytest testing"

# Ruff formatting and linting
uv run ruff check .
uv run ruff format --diff
echo "Passed ruff formatting and linting"

# Rustfmt
rustfmt --check src/**/*.rs
echo "Passed rustfmt formatting"

# Clippy
uv run cargo clippy --all-targets --all-features -- -D warnings
echo "Passed clippy linting"