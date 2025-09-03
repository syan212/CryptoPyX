# CryptoPyX

[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/syan212/CryptoPyX)
![GitHub branch check runs](https://img.shields.io/github/check-runs/syan212/CryptoPyX/main)

> [!CAUTION]
> This is still under early development!
> Don't use it

A simple python cryptogrpahy package.

## Installation guide

Currently, this is not yet on PyPi. Therefore, to install it, you need to first clone the repository

``` bash
git clone https://github.com/syan212/CryptoPyX.git
```

Then install it using pip in the directory

``` bash
cd CryptoPyX
pip install .
```

Now you should have it installed.

## Example Usage

``` python
from cryptopyx.ciphers import caesar
caesar.encrypt('ABC', 3) # DEF
caesar.decrypt('DEF', 3) # ABC
```

## Documentation

It's nonexistent. Looking through the code should be the best method. :/
