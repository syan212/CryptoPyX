import random

from cryptopy.ciphers import rot13


# Random Strings
def random_string(length: int, string_num: int = 100) -> list[str]:
    random.seed(69420)
    all_strings: list[str] = []
    letters: str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.'"
    for _ in range(string_num):
        all_strings.append(''.join(random.choice(letters) for _ in range(length)))
    return all_strings


# Encryption tests
def test_rot13_encrypt():
    assert rot13.encrypt('ABC') == 'NOP'
    assert (
        rot13.encrypt("Sentence with puctuation, they're really cool, right?")
        == "Fragrapr jvgu chpghngvba, gurl'er ernyyl pbby, evtug?"
    )


# Decryption tests
def test_rot13_decrypt():
    assert rot13.decrypt('NOP') == 'ABC'
    assert (
        rot13.decrypt("Fragrapr jvgu chpghngvba, gurl'er ernyyl pbby, evtug?")
        == "Sentence with puctuation, they're really cool, right?"
    )


# Benchmarks (cause why not?)
def test_rot13_encrypt_benchmark(benchmark):
    strings: list[str] = random_string(1000)

    def multi_encrypt():
        for string in strings:
            rot13.encrypt(string)

    benchmark(multi_encrypt)


def test_rot13_decrypt_benchmark(benchmark):
    strings: list[str] = random_string(1000)

    def multi_decrypt():
        for string in strings:
            rot13.decrypt(string)

    benchmark(multi_decrypt)
