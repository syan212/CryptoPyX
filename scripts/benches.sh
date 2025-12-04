#!/usr/bin/env bash
set -e

# Run benchmark thrice
if [[ $2 != "" ]]; then
    echo "Running benchmarks $2 times..."
    for _ in {1..$2}; do
        pytest --benchmark-only --benchmark-save=$1
    done
else
    echo "Running benchmarks thrice..."
    for _ in {1..3}; do
        pytest --benchmark-only --benchmark-save=$1
    done
fi

# Compare(Uses globbing for max compatibility)
pytest-benchmark compare '*-*-3.*-*bit/*' --csv .benchmarks/$1
