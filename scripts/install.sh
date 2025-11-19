#!/usr/bin/env bash
# Requires python>=3.11 for doc dependencies if --no-docs is not passed in
if [[ $1 == "--no-docs" ]]; then 
    pip install --group dev
else
    pip install --group dev --group docs
fi
maturin build --release --out dist
pip install dist/cryptopyx-*.whl --force-reinstall
