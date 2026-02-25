# CryptoPyX

[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
[![uv](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/uv/main/assets/badge/v0.json)](https://github.com/astral-sh/uv)
[![PyPI - Version](https://img.shields.io/pypi/v/cryptopyx?label=PyPI%20Version&color=blue)](https://pypi.org/project/cryptopyx)
[![CI Status](https://github.com/syan212/CryptoPyX/actions/workflows/CI.yml/badge.svg)](https://github.com/syan212/CryptoPyX/actions/workflows/CI.yml)
[![Dependabot Status](https://github.com/syan212/CryptoPyX/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/syan212/CryptoPyX/actions/workflows/dependabot/dependabot-updates)
[![Publish to PyPI Status](https://github.com/syan212/CryptoPyX/actions/workflows/pypi.yml/badge.svg?event=release)](https://github.com/syan212/CryptoPyX/actions/workflows/pypi.yml)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/cryptopyx)
[![Licence](https://img.shields.io/github/license/syan212/cryptopyx)](https://github.com/syan212/CryptoPyX/blob/main/LICENSE)

A python cryptography package written in Rust made for speed.

## Example Library Usage

``` python
from cryptopyx.encodings import base32
base32.encode_bytes(b'Hello World')  # b'JBSWY3DPEBLW64TMMQ======'
base32.decode_bytes(b'JBSWY3DPEBLW64TMMQ======')  # b'Hello World'
```

## Example CLI Usage

![image](images/cli.png)

## Installation guide

### Install from PyPI

This package supports Python 3.10 and above

``` shell
pip install cryptopyx
```

See `Building From Source` below for more information.

## Documentation

The documentation is still work in progress, but you can see it here: [Documentation website](https://cryptopyx.readthedocs.io).

## Building From source

To build from source without Rust after cloning the repository, simply run

``` shell
pip install .
```

## Licence

This is licensed under MIT licence. See `LICENSE` for full information.
