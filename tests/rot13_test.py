from cryptopyx.ciphers import rot13


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
