"""Base64 encoding and decoding functions.

Currently only supports standard base64 alphabet (RFC 4648).
"""

def encode(data: str) -> str:
    """Encodes string using base64.

    Args:
        data: The string to be encoded.

    Returns:
        Encoded base64 string.
    """
    ...

def decode(data: str) -> str:
    """Decodes base64 string.

    Args:
        data: The base64 string to be decoded.
        strict: Ensure string is correct length. Defaults to False. (optional)

    Returns:
        Decoded base64 string.

    Raises:
        ValueError: If the string is invalid base64.
    """
    ...

def encode_bytes(data: bytes) -> bytes:
    """Encode bytes using base64.

    Args:
        data: The data to be encoded.

    Returns:
        Encoded base64 bytes.
    """
    ...

def decode_bytes(data: bytes) -> bytes:
    """Decode bytes using base64.

    Args:
        data: The data to be decoded.

    Returns:
        Decoded base64 bytes.

    Raises:
        ValueError: If the bytes are invalid base32.
    """
    ...
