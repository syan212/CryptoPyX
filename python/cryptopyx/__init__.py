import sys
from types import ModuleType

from . import _cryptopyx

# Modules definition and rewrite sys.modules
# Encodings submodule
encodings: ModuleType = _cryptopyx.encodings
sys.modules['cryptopyx.encodings'] = _cryptopyx.encodings
sys.modules['cryptopyx.encodings.base32'] = _cryptopyx.encodings.base32

# Ciphers submodule
ciphers: ModuleType = _cryptopyx.ciphers
sys.modules['cryptopyx.ciphers'] = _cryptopyx.ciphers
sys.modules['cryptopyx.ciphers.rot13'] = _cryptopyx.ciphers.rot13
sys.modules['cryptopyx.ciphers.caesar'] = _cryptopyx.ciphers.caesar
sys.modules['cryptopyx.ciphers.vigenere'] = _cryptopyx.ciphers.vigenere

# All imports
__all__ = ['encodings', 'ciphers']
