import pytest
from cryptopyx.ciphers import vigenere
from utils import random_key, random_string


def test_vigenere_encrypt():
    # Basic functionality
    assert vigenere.encrypt('ATTACKATDAWN', 'LEMON') == 'LXFOPVEFRNHR'
    assert vigenere.encrypt('你好', 'hello') == '你好'

    # Test with punctuation and spaces
    assert (
        vigenere.encrypt("Attack at dawn! Let's go.", 'LEMON')
        == "Lxfopv ef rnhr! Xsg'd ka."
    )
    # Make sure empty key raises error
    with pytest.raises(ValueError):
        vigenere.encrypt('ATTACKATDAWN', '')
    with pytest.raises(ValueError):
        vigenere.encrypt('你好', '')
    # Make sure key with non-alphabetic characters raises error
    with pytest.raises(ValueError):
        vigenere.encrypt('ATTACKATDAWN', 'LEMON123')
    with pytest.raises(ValueError):
        vigenere.encrypt('ATTACKATDAWN', '你好')


def test_vigenere_decrypt():
    # Basic functionality
    assert vigenere.decrypt('LXFOPVEFRNHR', 'LEMON') == 'ATTACKATDAWN'
    assert vigenere.decrypt('你好', 'hello') == '你好'

    # Test with punctuation and spaces
    assert (
        vigenere.decrypt("Lxfopv ef rnhr! Xsg'd ka.", 'LEMON')
        == "Attack at dawn! Let's go."
    )
    # Make sure empty key raises error
    with pytest.raises(ValueError):
        vigenere.decrypt('LXFOPVEFRNHR', '')
    with pytest.raises(ValueError):
        vigenere.decrypt('你好', '')
    # Make sure key with non-alphabetic characters raises error
    with pytest.raises(ValueError):
        vigenere.decrypt('LXFOPVEFRNHR', 'LEMON123')
    with pytest.raises(ValueError):
        vigenere.decrypt('LXFOPVEFRNHR', '你好')


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
