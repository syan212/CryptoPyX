#!/usr/bin/env bash
# Requires python>=3.11 for doc dependencies if --no-docs is not passed in
set -e

if [[ $1 == "--no-docs" || $2 == "--no-docs" ]]; then 
    uv pip install --group dev
else
    uv pip install --group dev --group docs
fi

if [[ $1 == "--debug" || $2 == "--debug" ]]; then
    uv run maturin build --out dist
else
    uv run maturin build --release --out dist
fi

uv pip install dist/cryptopyx-*.whl --force-reinstall
