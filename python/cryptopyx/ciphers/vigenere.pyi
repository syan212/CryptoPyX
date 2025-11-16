"""
Vigenere shift encryption and decryption.
"""

def encrypt(data: str, key: str, skip_non_alpha: bool = True) -> str:
    """
    Encrypts string using the provided key.

    :param data: The string to encrypt.
    :type data: str
    :param key: The key to encrypt the data.
    :type key: str
    :param skip_non_alpha: Whether to skip non-alphabetic characters
        (optional, default is True).
    :type skip_non_alpha: bool
    :return: The encrypted string.
    :rtype: str

    :raises ValueError: If the key is invalid,
        **i.e**. it's empty or contains non-alphabetic characters,
        though lowercase and uppercase characters are both supported.
        *or*, if data is non-ascii.

    """
    ...

def decrypt(data: str, key: str, skip_non_alpha: bool = True) -> str:
    """
    Decrypts string using the provided key.

    :param data: The string to decrypt.
    :type data: str
    :param key: The key to decrypt the data.
    :type key: str
    :param skip_non_alpha: Whether to skip non-alphabetic characters
        (optional, default is True).
    :type skip_non_alpha: bool
    :return: The decrypted string.
    :rtype: str

    :raises ValueError: If the key is invalid,
        **i.e**. it's empty or contains non-alphabetic characters,
        though lowercase and uppercase characters are both supported.
        *or*, if data is non-ascii.
    """
    ...
