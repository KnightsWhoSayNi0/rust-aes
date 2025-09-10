//! Rust AES Implementation
//!
//! Trying my best to implement the AES encryption algorithm in Rust
//! NIST FIPS-197
//!
//! John Godman

fn cipher_128(input: &[u8; 16], key: [[u8; 4]; 4]) {
    // put input into the state
    let mut state = [[0u8; 4]; 4];

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

}


#[cfg(test)]
mod tests {
    use crate::cipher_128;

    #[test]
    fn cipher_test() {

    }
}