"""AES encryption and decryption. Currently only supports ECB mode. Work in progress."""

def encrypt_ecb(data: bytes, key: bytes, padding_mode: str) -> bytes:
    """Encrypt data using AES ECB mode with the inputted padding mode.

    Args:
        data: The data to be encrypted.
        key: The key to encrypt the data with. Can be 128, 196 and 256 bytes, as per AES standards.
        padding_mode: The padding to use for encryption. Can be any of the following: pkcs, PKCS, pkcs#7, PKCS#7, for PKCS#7 padding,
            and ISO, iso, ISO/IEC, iso/iec, for ISO/IEC 9797-1 Padding Method 2.

    Returns:
        The encrypted and padded data.

    Raises:
        ValueError: If the `padding_mode` argument is invalid, *or* if the key size is invalid.
    """

def decrypt_ecb(data: bytes, key: bytes, padding_mode: str) -> bytes:
    """Decrypt data using AES ECB mode with the inputted padding mode.

    Args:
        data: The data to be decrypted.
        key: The key to decrypt the data with. Can be 128, 196 and 256 bytes, as per AES standards.
        padding_mode: The padding to use for decryption. Can be any of the following: pkcs, PKCS, pkcs#7, PKCS#7, for PKCS#7 padding,
            and ISO, iso, ISO/IEC, iso/iec, for ISO/IEC 9797-1 Padding Method 2.

    Returns:
        The decrypted and unpadded data.

    Raises:
        ValueError: If the `padding_mode` argument is invalid, *or* if the key size is invalid.
    """
