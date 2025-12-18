import pytest

from cryptopyx.encodings import base64


def test_base64_encode():
    # Basic functionality
    assert base64.encode('hello') == 'aGVsbG8='
    assert base64.encode('hello there') == 'aGVsbG8gdGhlcmU='
    assert (
        base64.encode('The quick brown fox jumps over the lazy dog.')
        == 'VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4='
    )
    # Encode bytes
    assert base64.encode_bytes(b'hello') == b'aGVsbG8='


def test_base64_decode():
    # Basic functionality
    assert base64.decode('aGVsbG8=') == 'hello'
    assert base64.decode('aGVsbG8gdGhlcmU=') == 'hello there'
    assert (
        base64.decode('VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4=')
        == 'The quick brown fox jumps over the lazy dog.'
    )
    # Decode bytes
    assert base64.decode_bytes(b'aGVsbG8=') == b'hello'
    assert base64.decode_bytes(b'/w==') == b'\xff'
    # Decode errors
    with pytest.raises(ValueError):
        base64.decode('Sphinx of black quartz, judge my vow')
    with pytest.raises(ValueError):
        base64.decode('7d')
