# CryptoPy

[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)

> [!CAUTION]
> This is still under early development!
> Don't use it

A simple python cryptogrpahy package.

## Installation guide

Currently, this is not yet on PyPi. Therefore, to install it, you need to first clone the repository

``` bash
git clone https://github.com/syan212/CryptoPy.git
```

Then install it using pip in the directory

``` bash
cd CryptoPy
pip install .
```

Now you should have it installed.

## Example Usage

``` python
from cryptopy.ciphers import caesar
caesar.encrypt('ABC', 3) # DEF
caesar.decrypt('DEF', 3) # ABC
```

## Documentation

It's nonexistent. Looking through the code should be the best method. :/
