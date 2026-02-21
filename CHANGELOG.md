# Changelog

## v0.3.0

Not yet released

### New features

- Added custom substitution cipher

## v0.2.0

### New features

- Extended CLI for all current available methods, that is:
    - Caesar cipher
    - ROT13 cipher
    - Vigenere cipher
    - Base64 encoding
    - Base32 encoding

## v0.1.3

### New features

- CLI, currently only for base32
- Base64 encoding and decoding, with the same API as base32
- Added `rotate()` alias for rot13

### Minor dev changes

- Changed logic for rot13, caesar and vigenere ciphers
- Reduce sdist size

## v0.1.2

### New features

- New `encodings` submodule containing:
    - Base32 decoding and encoding, for both strings and bytes

## v0.1.1

### New features

- Vigenere cipher under `ciphers` submodule

## v0.1.0

First full release
-
### Breaking changes

- Removed vigenere cipher (will be added soon)

### Fixes

- Fixed [Issue 1](https://github.com/syan212/CryptoPyX/issues/1)

### Major dev changes

- Changed all main code to rust

## v0.0.2

Second test release

### New features

- ROT13 encryption and decryption under the `ciphers` submodule

### Minor dev changes

- Updated README to reStructuredText
- Added CHANGELOG.md

## v0.0.1

First test release

### New features

- `ciphers` submodule containing:
    - Caesar Shift encryption and decryption
    - Vigenere Cipher encryption and decryption
