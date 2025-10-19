import sys

from . import _cryptopyx

# Modules
encodings = _cryptopyx.encodings
ciphers = _cryptopyx.ciphers

# Rewrite sys.modules
sys.modules['cryptopyx.ciphers'] = _cryptopyx.ciphers
sys.modules['cryptopyx.ciphers.rot13'] = _cryptopyx.ciphers.rot13
sys.modules['cryptopyx.ciphers.caesar'] = _cryptopyx.ciphers.caesar
sys.modules['cryptopyx.encodings'] = _cryptopyx.encodings

__all__ = ['encodings', 'ciphers']
