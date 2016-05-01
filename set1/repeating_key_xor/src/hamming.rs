//! # Hamming Distance
//!
//! The hamming distance is the amount of bits that need
//! to be changed to go from one value to another. 
//! This is some code to calculate (normalized)
//! hamming distances of bytes and strings of bytes.

/// # Hamming Distance between two bytes
///
/// Computes the hamming distance between two bytes.
///
/// ## Example
///
/// ```
/// use repeating_key_xor::hamming;
///
/// assert_eq!(hamming::distance_byte(0x01, 0x10), 2);
/// ```
pub fn distance_byte(a: u8, b: u8) -> u8 {
    return (0..8).map(|n| ((a >> n) ^ (b >> n)) & 1).fold(0, |a,b| a+b);
}

#[test]
fn hamming_distance_between_bytes_works() {
    assert_eq!(distance_byte(0x12, 0x00), 2);
    assert_eq!(distance_byte(0x12, 0x33), 2);
    assert_eq!(distance_byte(0x55, 0x67), 3);
    assert_eq!(distance_byte(0xFF, 0x00), 8);
    assert_eq!(distance_byte(0xF0, 0x00), 4);
    assert_eq!(distance_byte(0x0F, 0x00), 4);
}

/// # Hamming Distance between two strings of bytes
///
/// Computes the hamming distance between two strings
/// of bytes of equal length (!).
///
/// ## Example
///
/// ```
/// use repeating_key_xor::hamming;
///
/// assert_eq!(hamming::distance(&[0x01, 0x10], &[0x10, 0x01]), 4);
/// ```
pub fn distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| distance_byte(*a, *b))
        .fold(0 as u32, |a,b| a + (b as u32))
}

#[test]
fn hamming_distance_between_strings_of_bytes_works() {
    assert_eq!(distance(&[0x12, 0x34, 0x56], &[0x04, 0x54, 0x68]), 10);
    assert_eq!(distance(&[0x00, 0xFF, 0x00], &[0xFF, 0x00, 0x00]), 16);
    assert_eq!(distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()), 37);
}

//fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    
