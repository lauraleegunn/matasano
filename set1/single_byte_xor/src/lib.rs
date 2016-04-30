//! # Single-Byte XOR
//!
//! The hex encoded string 
//! `1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736`
//! .. has been XOR'd against a single character. Find the key, decrypt the message.
//!
//! You can do this by hand. But don't: write code to do it for you.
//!
//! How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a
//! good metric. Evaluate each output and choose the one with the best score.

pub mod scoring;

/// # Apply Single-Byte XOR
///
/// This function takes a &[u8] and a key, and returns a Vec<u8>
/// representing all the bytes XOR'ed with the key. Apply once to
/// encrypt, and then once again with the same key to decrypt.
///
/// ## Example
///
/// ```
/// use single_byte_xor::apply;
///
/// assert_eq!(apply(&[0x01, 0x10], 0xFF), [0xFE, 0xEF]);
/// ```
pub fn apply(input: &[u8], code: u8) -> Vec<u8> {
    input.iter().map(|c| c ^ code).collect()
}

#[test]
fn apply_single_byte_xor_works() {
    assert_eq!(apply(&[0x12, 0x34], 0x22), [0x30, 0x16]);
}

#[test]
fn apply_single_byte_xor_reencryption_works() {
    assert_eq!(apply(&apply(&[123, 232, 43, 23, 91], 0x3a), 0x3a), [123, 232, 43, 23, 91]);
}

/// # Decrypt Single-Byte XOR
///
/// This function attempts to decrypt a single-byte
/// XOR encrypted string using english letter frequency
/// scoring.
///
/// Error handling is nonexistend/broken, so don't rely
/// on this thing at all.
///
/// The function takes a &[u8] containing the encrypted
/// data, and returns a 3-tuple consisting of the
/// decrypted string, the encryption byte used, and a
/// score representing how 'sure' the algorithm is
/// of this decryption. Higher score is better!
///
/// ## Example
///
/// ```
/// use single_byte_xor::decrypt;
/// ```
pub fn decrypt(input: &[u8]) -> (String, u8, f32) {
    let candidates: Vec<(u8, Vec<u8>, f32)> = (0x00..0xFF)
        .map(|x| (x, apply(input, x)))
        .map(|(x, applied)| (x, applied.clone(), String::from_utf8(applied)))
        .map(|(x, applied, utf8decoded)| 
             if utf8decoded.is_ok() {
                 (x, applied, 2.0 * scoring::english(&utf8decoded.unwrap()))
             } else {
                 (x, applied.clone(), scoring::english(&String::from_utf8_lossy(&applied)))
             })
        .collect();

    let (winner, runnerup) = candidates.iter()
        .fold((&candidates[0], &candidates[0]), |a,b| if (a.0).2 > b.2 {a} else {(b, a.0)});
    
    return (String::from_utf8_lossy(&winner.1).into_owned(), winner.0, winner.2/runnerup.2);
}

#[test]
fn decrypt_works_for_unencrypted_strings() {
    assert_eq!(decrypt(&[101, 110, 103, 108, 105, 115, 104, 32, 115, 116, 114, 105, 110, 103]).0, String::from("english string"));
    assert_eq!(decrypt(&[101, 110, 103, 108, 105, 115, 104, 32, 115, 116, 114, 105, 110, 103]).1, 0x00);
}

#[test]
fn decrypt_works_for_encrypted_strings() {
    assert_eq!(decrypt(&apply("some english text here.".as_bytes(), 0xb8)).0, String::from("some english text here."));
    assert_eq!(decrypt(&apply("some english text here.".as_bytes(), 0xb8)).1, 0xb8);
}
