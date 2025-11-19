#!/usr/bin/env bash
# Requires python>=3.11 for doc dependencies
pip install --group dev --group docs
maturin build --release --out dist
pip install dist/cryptopyx-*.whl --force-reinstall
