import random

import pytest
from cryptopy.ciphers import vigenere


# Random Strings
def random_string(length: int, string_num: int = 100) -> list[str]:
    random.seed(69420)
    all_strings: list[str] = []
    letters: str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.'"
    for _ in range(string_num):
        all_strings.append(''.join(random.choice(letters) for _ in range(length)))
    return all_strings


# Encryption tests
def test_vigenere_encrypt():
    assert vigenere.encrypt('ATTACKATDAWN', 'LEMON') == 'LXFOPVEFRNHR'
    assert (
        vigenere.encrypt('Spaces and punctuation, right?', 'LEMON')
        == 'Dtmqrd ezr cfrohhlxuca, cmsvg?'
    )
    # Make sure empty key raises error
    with pytest.raises(ValueError):
        vigenere.encrypt('ABC', '')
        vigenere.encrypt('ABC', '1234!@#$')


# Decryption tests
def test_vigenere_decrypt():
    assert vigenere.decrypt('LXFOPVEFRNHR', 'LEMON') == 'ATTACKATDAWN'
    assert (
        vigenere.decrypt('Dtmqrd ezr cfrohhlxuca, cmsvg?', 'LEMON')
        == 'Spaces and punctuation, right?'
    )
    # Make sure empty key raises error
    with pytest.raises(ValueError):
        vigenere.decrypt('DEF', '')
        vigenere.decrypt('DEF', '1234!@#$')


# Benchmarks (cause why not?)
def test_vigenere_encrypt_benchmark(benchmark):
    plaintexts: list[str] = random_string(1000)
    keys: list[str] = random_string(10)

    def multi_encrypt():
        for plaintext, key in zip(plaintexts, keys, strict=True):
            vigenere.encrypt(plaintext, key)

    benchmark(multi_encrypt)


def test_vigenere_decrypt_benchmark(benchmark):
    ciphertexts: list[str] = random_string(1000)
    keys: list[str] = random_string(10)

    def multi_decrypt():
        for ciphertext, key in zip(ciphertexts, keys, strict=True):
            vigenere.decrypt(ciphertext, key)

    benchmark(multi_decrypt)
