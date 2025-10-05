from cryptopyx.ciphers import rot13
from utils import random_string


# Encryption tests
def test_rot13_encrypt():
    assert rot13.encrypt('ABC') == 'NOP'
    assert rot13.encrypt('哈哈') == '哈哈'
    assert (
        rot13.encrypt("Sentence with puctuation, they're really cool, right?")
        == "Fragrapr jvgu chpghngvba, gurl'er ernyyl pbby, evtug?"
    )


# Decryption tests
def test_rot13_decrypt():
    assert rot13.decrypt('NOP') == 'ABC'
    assert rot13.decrypt('哈哈') == '哈哈'
    assert (
        rot13.decrypt("Fragrapr jvgu chpghngvba, gurl'er ernyyl pbby, evtug?")
        == "Sentence with puctuation, they're really cool, right?"
    )


# Benchmarks (cause why not?)
def test_rot13_encrypt_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings

    def multi_encrypt():
        for string in strings:
            rot13.encrypt(string)

    benchmark(multi_encrypt)


def test_rot13_decrypt_benchmark(benchmark):
    strings: list[str] = random_string(
        1000000, string_num=100
    )  # 1 million chars, 100 strings

    def multi_decrypt():
        for string in strings:
            rot13.decrypt(string)

    benchmark(multi_decrypt)
