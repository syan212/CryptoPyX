import base64
import random


# Random Strings
def random_string(length: int, string_num: int = 100) -> list[str]:
    """Random string from all letters, spaces and three punctuation marks"""
    random.seed(69420)
    letters: str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.'"
    return [
        ''.join(random.choice(letters) for _ in range(length))
        for _ in range(string_num)
    ]


# Random key for vigenere
def random_key(length: int, string_num: int = 100) -> list[str]:
    """Generate random key for vigenere"""
    random.seed(42069)
    letters: str = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
    return [
        ''.join(random.choice(letters) for _ in range(length))
        for _ in range(string_num)
    ]


# Random base32
def random_base32(length: int, string_num: int = 100) -> list[str]:
    """Generate random base32 string. For testing only"""
    random.seed(31415)
    out = []
    bytes_per_block = (length * 5) // 8  # since 8 chars = 5 bytes

    for _ in range(string_num):
        data = bytes(random.getrandbits(8) for _ in range(bytes_per_block))
        s = base64.b32encode(data).decode('ascii')
        s = s.rstrip('=')  # remove padding
        out.append(s)

    return out
