#!/usr/bin/env bash
# Requires maturin to be installed first and python>=3.11
pip uninstall cryptopyx -y
rm -r dist
set -e
maturin build --release --out dist
WHEEL_PATH=$(echo ./dist/cryptopyx-*.whl)
echo $WHEEL_PATH
pip install $WHEEL_PATH[dev, dev-docs] --force-reinstall