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
    :alt: Licence

A simple python cryptography package written in Rust.

Installation guide
==================

Install from PyPI
-----------------

This package suports Python 3.10 and above

::
    
   pip install cryptopyx

See below for platform compability and more information
Or get the source code from github and build from source.
See Building From Source below for more information.

Wheel Compability
-----------------

The package provides pre-compiled wheels for the following platforms tags:

* Windows 
    * win_amd64
    * win32
* macOS 
    * 10_12_x86_64 
    * 11_0_arm64
* Linux 
    * manylinux_2_34_x86_64
    * manylinux_2_34_armv7l
    * manylinux_2_34_aarch64
    * manylinux_2_34_i686

See below  for download information if your platform is supported.
However, if your platform is not supported, you will have to download the sdist and build from source.
It should work, however it's recommended to have Rust installed to avoid any issues.


Example Usage
=============

::

   from cryptopyx.ciphers import caesar
   caesar.encrypt('ABC', 3) # DEF
   caesar.decrypt('DEF', 3) # ABC


Documentation
=============

Due to the fact that there is no documentation, please refer to the code itself for now.

Building From source
====================

Simple
------

It is possible to install the package without Rust, as maturin can install a temporary Rust toolchain.

To build from source without Rust after cloning the repository, run:: 

   pip install maturin
   maturin build --release --out dist
   pip install dist/cryptopyx-*.whl # You might have to use `pip3` instead of `pip`

Licence
-------

This is licensed under MIT.