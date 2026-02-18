/// Multiplies column with AES fixed matrix.
fn multiply_matrix(matrix: Vec<u8>) -> Vec<u8> {
    // `a` is the input multiplied by 2 in GF(2^8)
    let mut a = [0, 0, 0, 0];
    // `h` is flag for whether modulo is needed
    let mut h: u8;
    for i in 0..4 {
        h = matrix[i] >> 7;
        a[i] = matrix[i] << 1;
        a[i] ^= h * 0x1b;
    }
    vec![
        a[0] ^ matrix[3] ^ matrix[2] ^ a[1] ^ matrix[1],
        a[1] ^ matrix[0] ^ matrix[3] ^ a[2] ^ matrix[2],
        a[2] ^ matrix[1] ^ matrix[0] ^ a[3] ^ matrix[3],
        a[3] ^ matrix[2] ^ matrix[1] ^ a[0] ^ matrix[0],
    ]
}

/// Perform MixColumns operation on block
pub fn mix_columns(block: Vec<u8>) -> Vec<u8> {
    let col1 = multiply_matrix(block[0..4].to_vec());
    let col2 = multiply_matrix(block[4..8].to_vec());
    let col3 = multiply_matrix(block[8..12].to_vec());
    let col4 = multiply_matrix(block[12..16].to_vec());
    vec![
        col1[0], col1[1], col1[2], col1[3], col2[0], col2[1], col2[2], col2[3], col3[0], col3[1],
        col3[2], col3[3], col4[0], col4[1], col4[2], col4[3],
    ]
}
