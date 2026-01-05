##############
Simple ciphers
##############

This section introduces simple ciphers available in the CryptoPyX package, including substitution ciphers.

Caesar cipher
=============

The Caesar cipher is a substitution cipher where each letter in the plaintext is shifted a fixed number of places down or up the alphabet.

Example usage:

.. code-block:: python

   from cryptopyx.ciphers import caesar

   plaintext = "HELLO"
   shift = 3

   # Encrypting the plaintext
   ciphertext = caesar.encrypt(plaintext, shift)
   print(ciphertext)  # Output: KHOOR

   # Decrypting the ciphertext
   decrypted_text = caesar.decrypt(ciphertext, shift)
   print(decrypted_text)  # Output: HELLO

Rot13 cipher
============

The Rot13 cipher is a special case of the Caesar cipher where each letter is shifted by 13 places, hence the name.
Since the English alphabet has 26 letters, applying Rot13 twice returns the original text. Therefore, the same 
function can be used for both encryption and decryption. CryptoPyX provide 3 function that are the same: `encrypt`, `decrypt` and `rotate`.

Example usage:

.. code-block:: python

   from cryptopyx.ciphers import rot13

   plaintext = "HELLO"

   # Encrypting the plaintext
   ciphertext = rot13.encrypt(plaintext)
   print(ciphertext)  # Output: URYYB

   # Decrypting the ciphertext
   decrypted_text = rot13.decrypt(ciphertext)
   print(decrypted_text)  # Output: HELLO

Vigenere cipher
===============

The Vigenere cipher is a method of encrypting alphabetic text by using a simple form of polyalphabetic substitution. The key is a string that represents
the shifts for each letter in the plaintext.

Example usage:

.. code-block:: python

   from cryptopyx.ciphers import vigenere
   plaintext = "HELLO"
   key = "kEy" # The key is case-insensitive

   # Encrypting the plaintext
   ciphertext = vigenere.encrypt(plaintext, key)
   print(ciphertext)  # Output: RINLP

   # Decrypting the ciphertext
   decrypted_text = vigenere.decrypt(ciphertext, key)
   print(decrypted_text)  # Output: HELLO

Substitution cipher
===================

CryptoPyX also includes a substitution cipher, which allows for a more flexible form of encryption by substituting each letter in the plaintext
with another letter based on a provided mapping.

Example usage:

.. code-block:: python

   from cryptopyx.ciphers import substitution
   plaintext = "HELLO"

   # Define a substitution mapping
   mapping = {
      'H': 'P', 
      'E': 'Q', 
      'L': 'R', 
      'O': 'S'
   }

   # Create a reverse mapping for decryption
   reverse_mapping = {v: k for k, v in mapping.items()}

   # Encrypting the plaintext
   ciphertext = substitution.substitute(plaintext, mapping)
   print(ciphertext)  # Output: PQRRS

   # Decrypting the ciphertext
   decrypted_text = substitution.substitute(ciphertext, reverse_mapping)
   print(decrypted_text)  # Output: HELLO
