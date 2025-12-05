"""Rot13 shift encryption and decryption. Ignores all non alphabetical characters."""

def encrypt(data: str) -> str:
    """ "Encrypt" a string using rot13.

    Not really encryption as rot13 encryption is the exact same as rot13 decryption

    Args:
        data: The string to be encrypted.

    Returns:
        The "encrypted" string
    """
    ...

def decrypt(data: str) -> str:
    """ "Decrypt" a string using rot13.

    Not really decryption as rot13 decryption is the exact same as rot13 encryption

    Args:
        data: The string to be decrypted.

    Returns:
        The "decrypted" string
    """
    ...

def rotate(data: str) -> str:
    """Rotates a string using rot13.

    Same as `decrypt()` and `encrypt()`.

    Args:
        data: The string to be rotated.

    Returns:
        The rotated string
    """
    ...
