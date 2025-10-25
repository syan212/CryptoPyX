import random


# Random Strings
def random_string(length: int, string_num: int = 100) -> list[str]:
    """Random string from all letters, spaces and three punctuation marks"""
    random.seed(69420)
    all_strings: list[str] = []
    letters: str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.'"
    for _ in range(string_num):
        all_strings.append(''.join(random.choice(letters) for _ in range(length)))
    return all_strings

# Random key for vigenere
def random_key(length: int, string_num: int = 100) -> list[str]:
    """Generate random key for vigenere"""
    random.seed(42069)
    all_keys: list[str] = []
    letters: str = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
    for _ in range(string_num):
        all_keys.append(''.join(random.choice(letters) for _ in range(length)))
    return all_keys

# Random base32
def random_base32(length: int, string_num: int = 100) -> list[str]:
    """Generate random base32 string"""
    random.seed(31415)
    all_base32: list[str] = []
    letters: str = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567'
    for _ in range(string_num):
        all_base32.append(''.join(random.choice(letters) for _ in range(length)))
    return all_base32
