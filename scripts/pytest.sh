#!/usr/bin/env bash
pytest --benchmark-only --benchmark-save=$1
pytest --benchmark-only --benchmark-save=$1
pytest --benchmark-only --benchmark-save=$1
pytest-benchmark compare '*-*-3.*-*bit/*'
