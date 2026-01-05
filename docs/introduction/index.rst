############
Introduction
############

.. toctree::
   :maxdepth: 2
   :hidden:

   simple_ciphers

.. important:: This is still a work in progress. The API and features may change in future releases.

Installation
============

Most users will install the package from PyPI using pip. 

.. code-block:: shell

   pip install cryptopyx

See :ref:`Installation<installation>` for more information if it doesn't work.

Usage
=====

Here is a simple example of how to use the package to encode and decode data using base32 encoding.

.. code-block:: python

   from cryptopyx.encodings import base64

   # Encoding bytes to base64
   encoded = base64.encode_bytes(b'Hello World')
   print(encoded)  # Output: b'SGVsbG8gV29ybGQ='

   # Decoding base64 back to bytes
   decoded = base64.decode_bytes(encoded)
   print(decoded)  # Output: b'Hello World'
