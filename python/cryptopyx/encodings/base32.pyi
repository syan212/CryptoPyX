"""Base32 encoding and decoding.

Currently only supports the standard base 32 alphabet (RFC 4648).
"""

def encode(data: str) -> str:
    """Encodes string using base32.

    Args:
        data: The string to be encoded.

    Returns:
        Encoded base32 string.
    """
    ...

def decode(data: str, strict: bool = False) -> str:
    """Decodes string using base32.

    Args:
        data: The string to be decoded.
        strict: Ensure string is correct length. Defaults to False. (optional)

    Returns:
        Decoded string.

    Raises:
        ValueError: If the string is invalid base32.
    """
    ...

def encode_bytes(data: bytes) -> bytes:
    """Encode bytes using base32.

    Args:
        data: The data to be encoded.

    Returns:
        Encoded base32 bytes.
    """
    ...

def decode_bytes(data: bytes) -> bytes:
    """Decode bytes using base32.

    Args:
        data: The data to be decoded.

    Returns:
        Decoded bytes.

    Raises:
        ValueError: If the bytes are invalid base32.
    """
    ...
