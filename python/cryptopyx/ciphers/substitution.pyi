"""Simple custom substitution ciphers."""

def substitute(data: str, mapping: dict[str, str]) -> str:
    """Substitutes characters in the input string according to the provided mapping.

    Args:
        data: The string to perform substitution on.
        mapping: A dictionary defining character substitutions,
            where keys are characters to be replaced and values are their replacements.

    Returns:
        The string after applying the substitutions.

    """
    ...

def substitute_bytes(data: bytes, mapping: dict[int, int]) -> str:
    """Substitutes characters in the input bytes according to the provided mapping.

    Args:
        data: The bytes to perform substitution on.
        mapping: A dictionary defining character substitutions,
            where keys are *integers* of the charactersto be replaced and values are their replacements.

    Returns:
        The bytes after applying the substitutions.

    """
    ...

def substitute_reverse(data: str, mapping: dict[str, str]) -> str:
    """Substitutes characters in the input string according to the reverse of the provided mapping.

    Args:
        data: The string to perform substitution on.
        mapping: A dictionary defining character substitutions,
            where keys are the replacements and values are what they are to be replaced with.

    Returns:
        The string after applying the substitutions.

    """
    ...

def substitute_reverse_bytes(data: bytes, mapping: dict[int, int]) -> str:
    """Substitutes characters in the input bytes according to the reverse of the provided mapping.

    Args:
        data: The bytes to perform substitution on.
        mapping: A dictionary defining character substitutions,
            where keys are *integers* of the the replacements and values are what they are to be replaced with.

    Returns:
        The bytes after applying the substitutions.

    """
    ...
