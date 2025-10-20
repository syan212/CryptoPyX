from cryptopyx.encodings import base32
from utils import random_string


def test_base32_encode():
    # Basic functionality
    assert base32.encode('hello') == 'NBSWY3DP'
    assert base32.encode('hello there') == 'NBSWY3DPEB2GQZLSMU======'
    assert (
        base32.encode('The quick brown fox jumps over the lazy dog.')
        == 'KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWOLQ='
    )

def test_base32_decode():
    # Basic functionality
    assert base32.decode('NBSWY3DP') == 'hello'
    assert base32.decode('NBSWY3DPEB2GQZLSMU======') == 'hello there'
    assert (
        base32.decode('KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWOLQ=')
        == 'The quick brown fox jumps over the lazy dog.'
    )

# Benchmarks (cause why not?)
def test_base32_encode_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings

    def multi_encode():
        for string in strings:
            base32.encode(string)

    benchmark(multi_encode)
