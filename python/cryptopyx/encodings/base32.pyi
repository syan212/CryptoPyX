"""
Base32 encoding and decoding.

Currently only supports the standard base 32 alphabet (RFC 4648).
"""

def encode(data: str) -> str:
    """
    Encodes string using base32.

    :param data: The data to be encoded.
    :type data: str
    :return: Encoded base32 string.
    :rtype: str
    """
    ...

def decode(data: str) -> str:
    """
    Decodes string using base32.

    :param data: The data to be decoded.
    :type data: str
    :return: Decoded string.
    :rtype: str

    :raises ValueError: If the string is invalid base32.
    """
    ...

def encode_bytes(data: bytes) -> bytes:
    """
    Encode bytes using base32.

    :param data: The data to be encoded.
    :type data: bytes
    :return: Encoded base32 bytes.
    :rtype: bytes
    """
    ...

def decode_bytes(data: bytes) -> bytes:
    """
    Decode bytes using base32.

    :param data: The data to be decoded.
    :type data: bytes
    :return: Decoded bytes.
    :rtype: bytes

    :raises ValueError: If the bytes are invalid base32.
    """
    ...
