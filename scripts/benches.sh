#!/usr/bin/env bash
set -e

# Help option
if [[ $1 == "--help" || $1 == "-h" ]]; then
    echo "benches.sh - run pytest benchmarks."
    echo ""
    echo "Usage: benches.sh <output> <iterations>"
    echo "       <output>: where the generated csv will go inside .benchmarks/."
    echo "       <iterations>: how many times to repeat benchmarks. Default is 3 times."
    exit 0
fi

# Run benchmark thrice
if [[ $2 != "" ]]; then
    echo "Running benchmarks $2 times..."
    for i in $(seq 1 $2); do
        uv run pytest --benchmark-only --benchmark-save=$1
        echo "Completed run $i/$2"
    done
else
    echo "Running benchmarks thrice..."
    for i in {1..3}; do
        uv run pytest --benchmark-only --benchmark-save=$1
        echo "Completed run $i/3"
    done
fi

# Compare(Uses globbing for max compatibility)
uv run pytest-benchmark compare '*-*-3.*-*bit/*' --csv .benchmarks/$1

# Clean up benchmark files
rm .benchmarks/**/*.json
