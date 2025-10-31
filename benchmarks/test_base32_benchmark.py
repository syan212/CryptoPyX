from cryptopyx.encodings import base32

from utils.base32 import random_base32


# Benchmarks (cause why not?)
def test_base32_encode_benchmark(benchmark):
    strings: list[str] = random_base32(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    bytes = [string.encode() for string in strings]

    def multi_encode():
        for byte in bytes:
            base32.encode_bytes(byte)

    benchmark(multi_encode)


def test_base32_decode_benchmark(benchmark):
    strings: list[str] = random_base32(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    bytes = [string.encode() for string in strings]

    def multi_decode():
        for byte in bytes:
            base32.decode_bytes(byte)

    benchmark(multi_decode)