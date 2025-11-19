import pytest

from cryptopyx.ciphers import vigenere


def test_vigenere_encrypt():
    # Basic functionality
    assert vigenere.encrypt('ATTACKATDAWN', 'LEMON') == 'LXFOPVEFRNHR'
    # Test with punctuation and spaces
    assert (
        vigenere.encrypt("Attack at dawn! Let's go.", 'LEMON')
        == "Lxfopv ef rnhr! Xsg'd ka."
    )
    # No non-ascii
    with pytest.raises(ValueError):
        assert vigenere.encrypt('你好', 'hello') == '你好'
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
    # Test with punctuation and spaces
    assert (
        vigenere.decrypt("Lxfopv ef rnhr! Xsg'd ka.", 'LEMON')
        == "Attack at dawn! Let's go."
    )
    # No non-ascii
    with pytest.raises(ValueError):
        assert vigenere.encrypt('你好', 'hello') == '你好'
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
