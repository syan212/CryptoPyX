"""
Caesar shift encryption and decryption. Ignores all non alphabetical characters.
"""

def encrypt(data: str, shift: int) -> str:
    """
    Encrypt a string using a caesar shift.

    Equivalent to using decrypt with the sign of the shift flipped. \n
    e.g. caesar.encrypt('Hello', 5) is the same as caesar.decrypt('Hello', -5)

    :param data: The string to be encrypted.
    :type data: str
    :param shift: How much to shift each letter.
    :type shift: int
    :return: The encrypted string.
    :rtype: str

    :raises ValueError: If the shift is not in the range -25 to 25 (inclusive).
    """
    ...

def decrypt(data: str, shift: int) -> str:
    """
    Decrypt a string using a caesar shift.

    Equivalent to using encrypt with the sign of the shift flipped. \n
    e.g. caesar.encrypt('Hello', 5) is the same as caesar.decrypt('Hello', -5)

    :param data: The string to be decrypted.
    :type data: str
    :param shift: How much to shift each letter.
                  Should be the same as the one used to encrypt the data.
    :type shift: int
    :return: The decrypted string.
    :rtype: str

    :raises ValueError: If the shift is not in the range -25 to 25 (inclusive).
    """
    ...
