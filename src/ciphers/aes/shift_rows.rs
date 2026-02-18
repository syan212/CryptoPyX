/// Perform ShiftRows on block
pub fn shift_rows(block: Vec<u8>) -> Vec<u8> {
    vec![
        block[0], block[5], block[10], block[15], block[4], block[9], block[14], block[3],
        block[8], block[13], block[2], block[7], block[12], block[1], block[6], block[11],
    ]
}
