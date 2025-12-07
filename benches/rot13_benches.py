from cryptopyx.ciphers import rot13
from utils.strings import random_string


# Benchmarks (cause why not?)
def bench_rot13_rotate(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings

    def multi_rotate():
        for string in strings:
            rot13.rotate(string)

    benchmark(multi_rotate)
