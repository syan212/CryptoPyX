from cryptopyx.ciphers import rot13


# Rotate tests
def test_rot13_rotate():
    # Alphabetical only
    assert rot13.decrypt('NOP') == 'ABC'
    assert rot13.encrypt('ABC') == 'NOP'
    assert rot13.rotate('ABC') == 'NOP'
    # Non ascii
    assert rot13.decrypt('哈哈') == '哈哈'
    assert rot13.encrypt('哈哈') == '哈哈'
    assert rot13.rotate('哈哈') == '哈哈'
    # Mixed
    assert (
        rot13.decrypt("Fragrapr jvgu chpghngvba, gurl'er ernyyl pbby, evtug?")
        == "Sentence with puctuation, they're really cool, right?"
    )
    assert (
        rot13.encrypt("Sentence with puctuation, they're really cool, right?")
        == "Fragrapr jvgu chpghngvba, gurl'er ernyyl pbby, evtug?"
    )
    assert (
        rot13.rotate("Sentence with puctuation, they're really cool, right?")
        == "Fragrapr jvgu chpghngvba, gurl'er ernyyl pbby, evtug?"
    )
