from cryptopyx.ciphers import vigenere
from utils.strings import random_key, random_string


# Benchmarks (cause why not?)
def test_vigenere_encrypt_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    keys: list[str] = random_key(101, string_num=100)

    def multi_encrypt():
        for string, key in zip(strings, keys, strict=False):
            vigenere.encrypt(string, key)

    benchmark(multi_encrypt)


def test_vigenere_decrypt_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    keys: list[str] = random_key(101, string_num=100)

    def multi_decrypt():
        for string, key in zip(strings, keys, strict=False):
            vigenere.decrypt(string, key)

    benchmark(multi_decrypt)
