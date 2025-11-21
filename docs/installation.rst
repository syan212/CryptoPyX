.. _installation:

############
Installation
############

Install from PyPI
=================

This package supports Python 3.10 and above

.. code-block:: console
    
   pip install cryptopyx

| See below for platform compability and more information
| Or get the source code from github and build from source.
| See :ref:`building-from-source` below for more information.

Wheel Compatibility
=================

The package provides pre-compiled wheels for the following platforms tags:

- Windows 
   - win_amd64
   - win32
- macOS 
   - 10_12_x86_64
   - 11_0_arm64
- Linux 
   - manylinux_2_34_x86_64
   - manylinux_2_34_armv7l
   - manylinux_2_34_aarch64
   - manylinux_2_34_i686

| And supports python 3.10 above and free-threaded versions
| See above for download information if your platform is supported.
| However, if your platform is not supported, you will have to download the sdist and build from source.
| It should work, however it's recommended to have Rust installed to avoid any issues.

.. _building-from-source:

Building From source
====================

It is possible to install the package without Rust, as maturin can install a temporary Rust toolchain.

To build from source without Rust after cloning the repository, run

.. code-block:: console

   pip install maturin
   maturin build --release --out dist
   pip install dist/cryptopyx-*.whl # You might have to use `pip3` instead of `pip`
