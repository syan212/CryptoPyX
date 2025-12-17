#!/usr/bin/env bash
# Requires python>=3.11 for doc dependencies if --no-docs is not passed in
set -e

if [[ $1 == "--no-docs" || $2 == "--no-docs" ]]; then 
    uv sync --no-install-project --frozen --inexact
else
    uv sync --no-install-project --group dev --group docs --frozen --inexact
fi

if [[ $1 == "--debug" || $2 == "--debug" ]]; then
    uv run maturin build --out dist
else
    uv run maturin build --release --out dist
fi

uv pip install dist/cryptopyx-*.whl --force-reinstall
