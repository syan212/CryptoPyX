#!/usr/bin/env bash
deactivate
echo "Removing directories"
rm -rf .venv
rm -r .pytest_cache
rm -r .ruff_cache
rm -r dist
echo "Creating virtual enviroment"
python3.14 -m venv .venv
echo "Activating virtual enviroment"
source .venv/bin/activate
pip install -r requirements-dev.txt