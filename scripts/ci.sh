#!/usr/bin/env bash

# Colours
RESET="\e[0m"
GREEN="\e[0;32m"

# Help option
if [[ $1 == "--help" || $1 == "-h" ]]; then
    echo "ci.sh - run tests like in CI."
    echo ""
    echo "Usage: ci.sh"
    exit 0
fi

set -euo pipefail
shopt -s globstar 

# Pytest(No no benchmark)
uv run pytest --benchmark-skip
echo -e "${GREEN}Passed pytest testing${RESET}"

# Ruff formatting and linting
uv run ruff check .
uv run ruff format --diff
echo -e "${GREEN}Passed ruff formatting and linting${RESET}"

# Rustfmt
rustfmt --check src/**/*.rs
echo -e "${GREEN}Passed rustfmt formatting${RESET}"

# Clippy
uv run cargo clippy --all-targets --all-features -- -D warnings
echo -e "${GREEN}Passed clippy linting${RESET}"
