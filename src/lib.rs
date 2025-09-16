//! Rust AES Implementation
//!
//! Trying my best to implement the AES encryption algorithm in Rust
//! NIST FIPS-197
//!
//! John Godman

// constants
const WORD_SIZE: usize = 4;
const NUM_KEY_WORDS: usize = 4;
const NUM_ROUNDS: usize = 10;

// round constants
const RCON: [[u8; 4]; 10] = [
    [0x01, 00, 00, 00],
    [0x02, 00, 00, 00],
    [0x04, 00, 00, 00],
    [0x08, 00, 00, 00],
    [0x10, 00, 00, 00],
    [0x20, 00, 00, 00],
    [0x40, 00, 00, 00],
    [0x80, 00, 00, 00],
    [0x1b, 00, 00, 00],
    [0x36, 00, 00, 00]
];

fn xor_word(w1: &[u8; WORD_SIZE], w2: &[u8; WORD_SIZE]) -> [u8; WORD_SIZE] {
    let mut result: [u8; WORD_SIZE] = [0u8; WORD_SIZE];

    for i in 0..WORD_SIZE {
        result[i] = w1[i] ^ w2[i];
    }

    result
}

fn sub_word(w: &[u8; WORD_SIZE]) -> [u8; WORD_SIZE] {
    todo!()
}

fn rot_word(w: &[u8; WORD_SIZE]) -> [u8; WORD_SIZE] {
    [w[1], w[2], w[3], w[0]]
}

fn key_expansion(
    key: &[[u8; WORD_SIZE]; NUM_KEY_WORDS]
) -> [[u8; WORD_SIZE]; 4 * (NUM_ROUNDS + 1) - 1] {
    let mut w: [[u8; 4]; 4 * (NUM_ROUNDS + 1) - 1] = [[0; 4]; 4 * (NUM_ROUNDS + 1) - 1];
    let mut i: usize = 0;

    while i <= NUM_KEY_WORDS - 1 {
        w[i] = key[i * 4]; // is this right?
        i += 1;
    }

    while i <= 4 * NUM_ROUNDS + 3 {
        let mut temp: [u8; WORD_SIZE] = w[i];

        if i % NUM_KEY_WORDS == 0 {
            temp = xor_word(&sub_word(&rot_word(&temp)),&RCON[i/NUM_KEY_WORDS]);
        } else if NUM_KEY_WORDS > 6 && i % NUM_KEY_WORDS == 4 {
            temp = sub_word(&temp);
        }

        w[i] = xor_word(&w[i - NUM_KEY_WORDS], &temp);
        i += 1;
    }

    w
}

fn add_round_key(state: &[u8; 16], w: [u8; 4]) {

}

fn cipher_128(input: &[u8; 16], w: [[u8; 4]; 4]) -> [u8; 16] {
    // put input into the state
    let mut state = [[0u8; 4]; 4];

    for r in 0..4 {
        for c in 0..4 {
            state[r][c] = input[r + 4 * c];
        }
    }

    // add round key

    for round in 0..10 {
        // sub bytes
        // shift rows
        // mix columns
        // add round key
    }

    // sub bytes
    // shift rows
    // add round key

    let mut out: [u8; 16] = [0; 16];
    for r in 0..4 {
        for c in 0..4 {
            out[r + 4 * c] = state[r][c];
        }
    }

    out
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn xor_word_test() {
        let w1 = [0x0a, 0xc7, 0xe2, 0xd6];
        let w2 = [0x99, 0x70, 0x7c, 0x88];
        let expected = [0x93, 0xb7, 0x9e, 0x5e];

        assert_eq!(xor_word(&w1, &w2), expected);
    }

    #[test]
    fn rot_word_test() {
        let w = [0xaa, 0xbb, 0xcc, 0xdd];
        let expected = [0xbb, 0xcc, 0xdd, 0xaa];

        assert_eq!(rot_word(&w), expected);
    }

    #[test]
    fn cipher_test() {
        assert_eq!(
            cipher_128(
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                [[0; 4]; 4]
            ),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }
}