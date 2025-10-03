#########
CryptoPyX
#########

.. image:: https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json
    :target: https://github.com/astral-sh/ruff
    :alt: Ruff

.. image:: https://img.shields.io/pypi/pyversions/cryptopyx
    :target: https://pypi.org/project/cryptopyx/
    :alt: PyPI - Python Version

.. image:: https://img.shields.io/pypi/v/cryptopyx?label=PyPI%20Version&color=blue
    :alt: PyPI - Version
    :target: https://pypi.org/project/cryptopyx/#history

.. image:: https://img.shields.io/pypi/dm/cryptopyx?label=PyPI%20Downloads&color=blue
    :alt: PyPI - Downloads
    :target: https://pypi.org/project/cryptopyx/

.. image:: https://github.com/syan212/CryptoPyX/actions/workflows/CI.yml/badge.svg
    :target: https://github.com/syan212/CryptoPyX/actions/workflows/CI.yml
    :alt: CI Status

.. image:: https://github.com/syan212/CryptoPyX/actions/workflows/dependabot/dependabot-updates/badge.svg
    :target: https://github.com/syan212/CryptoPyX/actions/workflows/dependabot/dependabot-updates
    :alt: Dependabot Status

.. image:: https://github.com/syan212/CryptoPyX/actions/workflows/pypi.yml/badge.svg?event=release
    :target: https://github.com/syan212/CryptoPyX/actions/workflows/pypi.yml
    :alt: Publish to PyPI Status

.. image:: https://img.shields.io/github/license/syan212/cryptopyx
    :target: https://github.com/syan212/CryptoPyX/blob/main/LICENSE
    :alt: License

**Notice**:

    **This is still under early development!**

A simple python cryptography package.

Installation guide
------------------

Install from PyPI

::

    pip install cryptopyx

Or get the source code from github for the latest version

::

    git clone https://github.com/syan212/CryptoPyX.git
    cd CryptoPyX
    pip install .

Example Usage
-------------

::

    from cryptopyx.ciphers import caesar
    caesar.encrypt('ABC', 3) # DEF
    caesar.decrypt('DEF', 3) # ABC


Documentation
-------------

Due to the fact that there is no documentation, please refer to the code itself for now.
