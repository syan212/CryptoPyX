#!/usr/bin/env bash
# Pytest(No no benchmark)
pytest --benchmark-skip
# Ruff formatting and linting
ruff check . && ruff format --diff
# Rustfmt 
rustfmt --check src/**/*.rs
# Clippy
cargo clippy --all-targets --all-features -- -D warnings