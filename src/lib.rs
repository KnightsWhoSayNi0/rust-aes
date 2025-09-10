//! Rust AES Implementation
//!
//! Trying my best to implement the AES encryption algorithm in Rust
//! NIST FIPS-197
//!
//! John Godman

fn cipher_128(input: &[u8; 16], key: [[u8; 4]; 4]) -> [u8; 16] {
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