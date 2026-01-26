# Implementation checklist

* AES
  * [*] KeyExpansion
  * [x] SubBytes (S-box)
    * [x] Box
    * [x] Implement for 128-bit block
  * [ ] One of these is not working, I don't know which
  * [ ] ShiftRows
  * [ ] MixColumns
  * [ ] AddRoundKey
  * [ ] Modes
    * [ ] ECB
    * [ ] CBC
    * [ ] GCM
