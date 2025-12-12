"""Base64 encoding and decoding functions.

Currently only has encode_bytes and encode function.
"""

def encode(data: str) -> str:
    """Encodes string using base64.

    Args:
        data: The string to be encoded.

    Returns:
        Encoded base64 string.
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
