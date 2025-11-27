"""Vigenere shift encryption and decryption."""

def encrypt(data: str, key: str, skip_non_alpha: bool = True) -> str:
    """Encrypts string using the provided key.

    Args:
        data: The string to encrypt.
        key: The key to encrypt the data.
        skip_non_alpha: Whether to skip non-alphabetic characters
            (optional, default is True).

    Returns:
        The encrypted string.

    Raises:
        ValueError: If the key is invalid,
            **i.e**. it's empty or contains non-alphabetic characters,
            though lowercase and uppercase characters are both supported.
            *or*, if data is non-ascii.

    """
    ...

def decrypt(data: str, key: str, skip_non_alpha: bool = True) -> str:
    """Decrypts string using the provided key.

    Args:
        data: The string to decrypt.
        key: The key to decrypt the data.
        skip_non_alpha: Whether to skip non-alphabetic characters
            (optional, default is True).

    Returns:
        The decrypted string.

    Raises:
        ValueError: If the key is invalid,
            **i.e**. it's empty or contains non-alphabetic characters,
            though lowercase and uppercase characters are both supported.
            *or*, if data is non-ascii.
    """
    ...
