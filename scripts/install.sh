#!/usr/bin/env bash
# Requires python>=3.11 for doc dependencies if --no-docs is not passed in
set -e

# Help option
if [[ $1 == "--help" || $1 == "-h" ]]; then
    echo "install.sh - install all dependencies and editable version of CryptoPyX."
    echo ""
    echo "Usage: install.sh [OPTIONS]"
    echo "       --no-docs: do not install doc dependencies. This is used as doc dependencies require python>=3.11."
    exit 0
fi

if [[ $1 == "--no-docs" ]]; then 
    uv sync --frozen --inexact
else
    uv sync --group dev --group docs --frozen --inexact
fi

uv pip install -e .
