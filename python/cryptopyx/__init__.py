import importlib
import sys
from types import ModuleType

from . import _cryptopyx
from .stubs_processing import (
    apply_docs_and_signatures,
    find_pyi_files,
    parse_docstrings,
    parse_signatures,
)

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

# CLI
cli: ModuleType = _cryptopyx.cli  # type: ignore
sys.modules['cryptopyx.cli'] = cli

# Apply docstrings from .pyi files
for module_name, pyi_path in find_pyi_files('cryptopyx'):
    docs = parse_docstrings(pyi_path)
    signatures = parse_signatures(pyi_path)
    module = importlib.import_module(module_name)
    apply_docs_and_signatures(module, docs, signatures)

# All imports
__all__ = ['ciphers', 'cli', 'encodings']
