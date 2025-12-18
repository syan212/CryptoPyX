#!/usr/bin/env bash
set -euo pipefail
shopt -s globstar 

# Pytest(No no benchmark)
uv run pytest --benchmark-skip

# Ruff formatting and linting
uv run ruff check .
uv run ruff format --diff

# Rustfmt
rustfmt --check src/**/*.rs

# Clippy
uv run cargo clippy --all-targets --all-features -- -D warnings