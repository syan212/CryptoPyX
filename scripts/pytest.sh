#!/usr/bin/env bash
pytest --benchmark-only --benchmark-save=$1
pytest --benchmark-only --benchmark-save=$1
pytest --benchmark-only --benchmark-save=$1
pytest-benchmark compare 'Linux-CPython-3.14-64bit/*'
