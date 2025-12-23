# Changelog

**v0.1.3 (Next release)**
Features
- Added CLI, currently only for base32
- Added base64 encoding and decoding, with the same API as base32
- Changed logic for rot13, caesar and vigenere ciphers
- Added `rotate()` alias for rot13
- Reduce sdist size

Dev dependencies
- [cargo]: Bump pyo3 to 0.27.2
- [pip]: Bump ruff to 0.14.7
- [pip]: Bump maturin to 1.10.2
- [pip]: Bump pytest-benchmark to 5.2.3
- [pip]: Bump pytest to 9.0.0

**v0.1.2: Base32**
Features
- Added the base32 decoding and encoding, for both strings and bytes

**v0.1.1: One more feature**
Features
- Added the vigenere ciphers as planned

**v0.1.0: First full release**
Features
- Changed all main code to rust
- Removed vigenere cipher (will be added soon)
- Fixed [Issue 1](https://github.com/syan212/CryptoPyX/issues/1)

**v0.0.2: Second test release**
Features
- Updated README to reStructuredText
- Added CHANGELOG.md
- ROT13 added

**v0.0.1: First test release**
Features
- Caesar Shift encryption and decryption using `cryptopyx.ciphers.caesar.encrypt()` and `cryptopyx.ciphers.caesar.decrypt()` respectively
- Vigenere Cipher encryption and decryption using `cryptopyx.ciphers.viginere.encrypt()` and `cryptopyx.ciphers.viginere.decrypt()` respectively
