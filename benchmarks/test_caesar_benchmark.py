import random

from cryptopyx.ciphers import caesar

from utils.strings import random_string


# Benchmarks (cause why not?)
def test_caesar_encrypt_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    shifts: list[int] = [random.randint(-25, 25) for _ in range(100)]

    def multi_encrypt():
        for string, shift in zip(strings, shifts, strict=False):
            caesar.encrypt(string, shift)

    benchmark(multi_encrypt)


def test_caesar_decrypt_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    shifts: list[int] = [random.randint(-25, 25) for _ in range(100)]

    def multi_decrypt():
        for string, shift in zip(strings, shifts, strict=False):
            caesar.decrypt(string, shift)

    benchmark(multi_decrypt)
