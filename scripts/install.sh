#!/usr/bin/env bash
# Requires python>=3.11 for doc dependencies if --no-docs is not passed in
set -e

if [[ $1 == "--no-docs" || $2 == "--no-docs" ]]; then 
    pip install --group dev
else
    pip install --group dev --group docs
fi

if [[ $1 == "--debug" || $2 == "--debug" ]]; then
    maturin build --out dist
else
    maturin build --release --out dist
fi

pip install dist/cryptopyx-*.whl --force-reinstall
