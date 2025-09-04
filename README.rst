#########
CryptoPyX
#########

.. image:: https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json
    :target: https://github.com/astral-sh/ruff
    :alt: Ruff

.. image:: https://img.shields.io/pypi/pyversions/cryptopyx
   :target: https://pypi.org/project/cryptopyx/
   :alt: PyPI - Python Version

.. image:: https://img.shields.io/pypi/v/cryptopyx
   :alt: PyPI - Version
   :target: https://pypi.org/project/cryptopyx/

.. image:: https://img.shields.io/github/commit-activity/m/syan212/cryptopyx
   :target: https://github.com/syan212/CryptoPyX/commits/main/
   :alt: GitHub commit activity

.. image:: https://img.shields.io/github/check-runs/syan212/cryptopyx/main
   :target: https://github.com/syan212/Army-API/actions
   :alt: GitHub branch check runs

.. image:: https://img.shields.io/github/deployments/syan212/cryptopyx/pypi
   :target: https://github.com/syan212/CryptoPyX/deployments/pypi
   :alt: GitHub deployments

.. image:: https://img.shields.io/github/license/syan212/cryptopyx
   :target: https://github.com/syan212/CryptoPyX/blob/main/LICENSE
   :alt: GitHub License

**Notice**:

    **This is still under early development!
    Don't use it!**

A simple python cryptography package.

Installation guide
------------------

Install from PyPI

::

    pip install cryptopyx

Or install from github for the latest version

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

It's nonexistent. Looking through the code should be the best method. :/
