import sys

from . import _cryptopyx

# Rewrite sys.modules
sys.modules['cryptopyx.ciphers'] = _cryptopyx.ciphers
sys.modules['cryptopyx.ciphers.rot13'] = _cryptopyx.ciphers.rot13
sys.modules['cryptopyx.ciphers.caesar'] = _cryptopyx.ciphers.caesar
