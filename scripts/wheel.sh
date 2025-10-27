#!/usr/bin/env bash
pip uninstall cryptopyx
rm -r dist
maturin build --release --out dist
pip install dist/* --force-reinstall