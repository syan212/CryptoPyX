#!/usr/bin/env bash
pip uninstall cryptopyx -y
maturin build --release --out dist
pip install dist/*