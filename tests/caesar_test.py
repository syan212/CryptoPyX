import random

import pytest
from cryptopyx.ciphers import caesar
from utils import random_string


# Encryption tests
def test_caesar_encrypt():
    # Basic functionality
    assert caesar.encrypt('ABC', 3) == 'DEF'
    assert caesar.encrypt('哈哈', 3) == '哈哈'
    assert caesar.encrypt('ABC', -3) == 'XYZ'
    assert caesar.encrypt('哈哈', -3) == '哈哈'
    # Test with punctuation and spaces
    assert (
        caesar.encrypt("Sentence with puctuation, they're really cool, right?", 25)
        == "Rdmsdmbd vhsg otbstzshnm, sgdx'qd qdzkkx bnnk, qhfgs?"
    )
    # Make sure shift out of bounds raises error
    with pytest.raises(ValueError):
        caesar.encrypt('ABC', -26)
    with pytest.raises(ValueError):
        caesar.encrypt('ABC', 26)


# Decryption tests
def test_caesar_decrypt():
    # Basic functionality
    assert caesar.decrypt('DEF', 3) == 'ABC'
    assert caesar.decrypt('哈哈', 3) == '哈哈'
    assert caesar.decrypt('DEF', -3) == 'GHI'
    assert caesar.decrypt('哈哈', -3) == '哈哈'
    # Test with punctuation and spaces
    assert (
        caesar.decrypt("Rdmsdmbd vhsg otbstzshnm, sgdx'qd qdzkkx bnnk, qhfgs?", 25)
        == "Sentence with puctuation, they're really cool, right?"
    )
    # Make sure shift out of bounds raises error
    with pytest.raises(ValueError):
        caesar.decrypt('DEF', -26)
    with pytest.raises(ValueError):
        caesar.decrypt('DEF', 26)


# Benchmarks (cause why not?)
def test_caesar_encrypt_benchmark(benchmark):
    strings: list[str] = random_string(1000, string_num=100)
    shifts: list[int] = [random.randint(-25, 25) for _ in range(100)]

    def multi_encrypt():
        for string, shift in zip(strings, shifts):
            caesar.encrypt(string, shift)

    benchmark(multi_encrypt)


def test_caesar_decrypt_benchmark(benchmark):
    strings: list[str] = random_string(1000, string_num=100)
    shifts: list[int] = [random.randint(-25, 25) for _ in range(100)]

    def multi_decrypt():
        for string, shift in zip(strings, shifts):
            caesar.decrypt(string, shift)

    benchmark(multi_decrypt)
