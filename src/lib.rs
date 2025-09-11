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

fn key_expansion(key: &[[u8; WORD_SIZE]; NUM_KEY_WORDS]) -> [[u8; WORD_SIZE]; 4 * (NUM_ROUNDS + 1)] {
    let mut w: [[u8; 4]; 4 * (NUM_ROUNDS + 1)] = [[0; 4]; 4 * (NUM_ROUNDS + 1)];

    for i in 0..NUM_KEY_WORDS - 1 {
        w[i] = key[i];
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
    use crate::cipher_128;

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