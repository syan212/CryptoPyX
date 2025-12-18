import pytest

from cryptopyx.encodings import base32


def test_base32_encode():
    # Basic functionality
    assert base32.encode('hello') == 'NBSWY3DP'
    assert base32.encode('hello there') == 'NBSWY3DPEB2GQZLSMU======'
    assert (
        base32.encode('The quick brown fox jumps over the lazy dog.')
        == 'KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWOLQ='
    )
    # Encode bytes
    assert base32.encode_bytes(b'hello') == b'NBSWY3DP'


def test_base32_decode():
    # Basic functionality
    assert base32.decode('NBSWY3DP') == 'hello'
    assert base32.decode('NBSWY3DPEB2GQZLSMU======') == 'hello there'
    assert (
        base32.decode(
            'KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWOLQ='
        )
        == 'The quick brown fox jumps over the lazy dog.'
    )
    # Decode bytes
    assert base32.decode_bytes(b'NBSWY3DP') == b'hello'
    assert base32.decode_bytes(b'74======') == b'\xff'
    # Decode errors
    with pytest.raises(ValueError):
        base32.decode('Sphinx of black quartz, judge my vow')
    with pytest.raises(ValueError):
        base32.decode('74======')
    with pytest.raises(ValueError):
        base32.decode('NBSWY3DP=', strict=True)
