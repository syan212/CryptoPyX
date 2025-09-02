# Encryption
def encrypt(plaintext: str, shift: int) -> str:
    """Encrypts plaintext using a Caesar cipher with the given shift.

    :param plaintext: The text to be encrypted.
    :param shift: The number of positions to shift each letter (0-25).

    :return: The encrypted ciphertext.
    """
    base: int
    char: str
    # Shift check
    if shift < 0 or shift >= 26:
        raise ValueError('Shift must be in the range [0, 25]')
    # Encryption processs
    ciphertext: str = ''
    for char in plaintext:
        if char.isalpha():
            base = ord('A') if char.isupper() else ord('a')
            ciphertext += chr((ord(char) - base + shift) % 26 + base)
        else:
            ciphertext += char
    return ciphertext


# Decryption
def decrypt(ciphertext: str, shift: int) -> str:
    """Decrypts ciphertext using a Caesar cipher with the given shift.

    :param plaintext: The text to be decrypted.
    :param shift: The number of positions to shift each letter (0-25).
        It should be the same as used for encryption.

    :return: The encrypted ciphertext.
    """
    base: int
    char: str
    # Shift check
    if shift < 0 or shift >= 26:
        raise ValueError('Shift must be in the range [0, 25]')
    # Decryption process
    plaintext: str = ''
    for char in ciphertext:
        if char.isalpha():
            base = ord('A') if char.isupper() else ord('a')
            plaintext += chr((ord(char) - base - shift) % 26 + base)
        else:
            plaintext += char
    return plaintext
