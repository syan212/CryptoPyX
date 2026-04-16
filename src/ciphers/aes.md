# Implementation checklist

* AES
  * [*] KeyExpansion
  * [x] SubBytes (S-box)
    * [x] Box
    * [x] Implement for 128-bit block
  * [x] ShiftRows
  * [x] MixColumns
  * [x] AddRoundKey
  * [ ] Modes
    * [x] ECB (sorta)
    * [ ] CBC
    * [ ] GCM
