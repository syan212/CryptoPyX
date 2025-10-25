#!/usr/bin/env bash
pytest --benchmark-skip
ruff check . && ruff format --diff
rustfmt --check src/**/*.rs
cargo clippy --all-targets --all-features -- -D warnings