import numpy as np


# Encryption
def encrypt(plaintext: str, key: str) -> str:
    """ "Encrypts plaintext using a Vigenere cipher with the given key.

    :param plaintext: The text to be encrypted.
    :param key: The keyword used for encryption. Non-alphabetic characters are ignored.
        If no alphabetic characters are present, a ValueError is raised.

    :return: The encrypted ciphertext.
    """
    A: int = ord('A')
    a: int = ord('a')

    # numeric shifts for key (uppercase only)
    key_nums: np.ndarray = np.array([ord(c.upper()) - A for c in key if c.isalpha()])
    key_len: int = len(key_nums)
    if key_len == 0:
        raise ValueError('Key must contain at least one alphabetic character')

    result: list = []
    j: int = 0  # key index
    shift: int
    for c in plaintext:
        if c.isupper():
            shift = key_nums[j % key_len]
            result.append(chr((ord(c) - A + shift) % 26 + A))
            j += 1
        elif c.islower():
            shift = key_nums[j % key_len]
            result.append(chr((ord(c) - a + shift) % 26 + a))
            j += 1
        else:
            result.append(c)
    return ''.join(result)


# Decryption
def decrypt(ciphertext: str, key: str) -> str:
    """Decrypts ciphertext using a Vigenere cipher with the given key.

    :param ciphertext: The text to be decrypted.
    :param key: The keyword used for decryption. Non-alphabetic characters are ignored.
        If no alphabetic characters are present, a ValueError is raised.

    :return: The decrypted plaintext.
    """
    A: int = ord('A')
    a: int = ord('a')

    key_nums: np.ndarray = np.array([ord(c.upper()) - A for c in key if c.isalpha()])
    key_len: int = len(key_nums)
    if key_len == 0:
        raise ValueError('Key must contain at least one alphabetic character')

    result: list = []
    j: int = 0
    shift: int
    for c in ciphertext:
        if c.isupper():
            shift = key_nums[j % key_len]
            result.append(chr((ord(c) - A - shift) % 26 + A))
            j += 1
        elif c.islower():
            shift = key_nums[j % key_len]
            result.append(chr((ord(c) - a - shift) % 26 + a))
            j += 1
        else:
            result.append(c)
    return ''.join(result)
