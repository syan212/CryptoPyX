from cryptopyx.encodings import base64
from utils.encodings import random_base64


# Benchmarks (cause why not?)
def bench_base64_encode(benchmark):
    strings: list[str] = random_base64(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    bytes = [string.encode() for string in strings]

    def multi_encode():
        for byte in bytes:
            base64.encode_bytes(byte)

    benchmark(multi_encode)


def bench_base64_decode(benchmark):
    strings: list[str] = random_base64(
        1000000, string_num=100
    )  # 1 million chars, 100 strings
    bytes = [string.encode() for string in strings]

    def multi_decode():
        for byte in bytes:
            base64.decode_bytes(byte)

    benchmark(multi_decode)
