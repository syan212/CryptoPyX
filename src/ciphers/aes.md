# Implementation checklist

* AES
  * [*] KeyExpansion
  * [ ] SubBytes (S-box)
    * [x] Box
    * [ ] Implement for 128-bit block
  * [ ] ShiftRows
  * [ ] MixColumns
  * [ ] AddRoundKey
  * [ ] Modes
    * [ ] ECB
    * [ ] CBC
    * [ ] GCM
