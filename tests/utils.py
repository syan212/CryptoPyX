import random


# Random Strings
def random_string(length: int, string_num: int = 100) -> list[str]:
    random.seed(69420)
    all_strings: list[str] = []
    letters: str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.'"
    for _ in range(string_num):
        all_strings.append(''.join(random.choice(letters) for _ in range(length)))
    return all_strings


def random_key(length: int, string_num: int = 100) -> list[str]:
    random.seed(42069)
    all_keys: list[str] = []
    letters: str = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
    for _ in range(string_num):
        all_keys.append(''.join(random.choice(letters) for _ in range(length)))
    return all_keys
