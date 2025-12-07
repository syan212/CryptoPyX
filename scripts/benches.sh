#!/usr/bin/env bash
set -e

# Run benchmark thrice
if [[ $2 != "" ]]; then
    echo "Running benchmarks $2 times..."
    for i in $(seq 1 $2); do
        pytest --benchmark-only --benchmark-save=$1
        echo "Completed run $i/$2"
    done
else
    echo "Running benchmarks thrice..."
    for i in {1..3}; do
        pytest --benchmark-only --benchmark-save=$1
        echo "Completed run $i/3"
    done
fi

# Compare(Uses globbing for max compatibility)
pytest-benchmark compare '*-*-3.*-*bit/*' --csv .benchmarks/$1

# Clean up benchmark files
rm .benchmarks/**/*.json
