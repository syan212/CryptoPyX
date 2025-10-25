"""
Base32 encoding. Currently only supports the standard base 32 alphabet (RFC 4648).
"""

def encode(data: str) -> str:
    """Encode string using base32."""
    ...

def decode(data: str) -> str:
    """Decode string using base32. Raises a ValueError if string is invalid."""
    ...

def encode_bytes(data: bytes) -> bytes:
    """Encode bytes using base32."""
    ...
    
def decode_bytes(data: bytes) -> bytes:
    """Decode bytes using base32."""
    ...