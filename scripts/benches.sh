#!/usr/bin/env bash
# Run benchmark thrice
pytest --benchmark-only --benchmark-save=$1
pytest --benchmark-only --benchmark-save=$1
pytest --benchmark-only --benchmark-save=$1
# Compare(Uses globbing for max compatibility)
pytest-benchmark compare '*-*-3.*-*bit/*'
