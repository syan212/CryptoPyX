import sys
from types import ModuleType

from . import _cryptopyx

# Modules definition and rewrite sys.modules
# Encodings submodule
encodings: ModuleType = _cryptopyx.encodings
sys.modules['cryptopyx.encodings'] = encodings
sys.modules['cryptopyx.encodings.base32'] = encodings.base32

# Ciphers submodule
ciphers: ModuleType = _cryptopyx.ciphers
sys.modules['cryptopyx.ciphers'] = ciphers
sys.modules['cryptopyx.ciphers.rot13'] = ciphers.rot13
sys.modules['cryptopyx.ciphers.caesar'] = ciphers.caesar
sys.modules['cryptopyx.ciphers.vigenere'] = ciphers.vigenere

# All imports
__all__ = ['encodings', 'ciphers']
