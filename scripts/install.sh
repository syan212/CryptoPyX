#!/usr/bin/env bash
# Requires python>=3.11 for doc dependencies if --no-docs is not passed in
set -e

if [[ $1 == "--no-docs" || $2 == "--no-docs" ]]; then 
    uv sync --frozen --inexact
else
    uv sync --group dev --group docs --frozen --inexact
fi

uv pip install -e .
