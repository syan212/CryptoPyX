from cryptopyx.ciphers import rot13
from utils.strings import random_string


# Benchmarks (cause why not?)
def test_rot13_rotate_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings

    def multi_encrypt():
        for string in strings:
            rot13.encrypt(string)

    benchmark(multi_encrypt)
