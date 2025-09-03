# CryptoPyX

[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/syan212/CryptoPyX)
![GitHub branch check runs](https://img.shields.io/github/check-runs/syan212/CryptoPyX/main)

> [!CAUTION]
> This is still under early development!
> Don't use it

A simple python cryptogrpahy package.

## Installation guide

Install from PyPI

```bash
pip install cryptopyx
```

Or install from github for the latest version

``` bash
git clone https://github.com/syan212/CryptoPyX.git
cd CryptoPyX
pip install .
```

## Example Usage

``` python
from cryptopyx.ciphers import caesar
caesar.encrypt('ABC', 3) # DEF
caesar.decrypt('DEF', 3) # ABC
```

## Documentation

It's nonexistent. Looking through the code should be the best method. :/
