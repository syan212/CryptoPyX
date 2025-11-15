import importlib
import sys
from types import ModuleType

from . import _cryptopyx
from .stubs import apply_docs_to_module, find_pyi_files, parse_pyi_docstrings

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

# Apply docstrings from .pyi files
for module_name, pyi_path in find_pyi_files('cryptopyx'):
    docs = parse_pyi_docstrings(pyi_path)
    module = importlib.import_module(module_name)
    apply_docs_to_module(module, docs)

# All imports
__all__ = ['encodings', 'ciphers']
