# Encryption
def encrypt(plaintext: str) -> str:
    """Encrypts plaintext using the ROT13 cipher.

    :param plaintext: The text to be encrypted.

    :return: The encrypted ciphertext.
    """
    base: int
    char: str
    # Encryption processs
    ciphertext: str = ''
    for char in plaintext:
        if char.isalpha():
            base = ord('A') if char.isupper() else ord('a')
            ciphertext += chr((ord(char) - base + 13) % 26 + base)
        else:
            ciphertext += char
    return ciphertext


# Decryption
def decrypt(ciphertext: str) -> str:
    """Decrypts ciphertext using the ROT13 cipher.

    :param plaintext: The text to be decrypted.

    :return: The encrypted ciphertext.
    """
    base: int
    char: str
    # Decryption process
    plaintext: str = ''
    for char in ciphertext:
        if char.isalpha():
            base = ord('A') if char.isupper() else ord('a')
            plaintext += chr((ord(char) - base - 13) % 26 + base)
        else:
            plaintext += char
    return plaintext
