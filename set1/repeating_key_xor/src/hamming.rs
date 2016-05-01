//! # Hamming Distance
//!
//! The hamming distance is the amount of bits that need
//! to be changed to go from one value to another. 
//! This is some code to calculate (normalized)
//! hamming distances of bytes and strings of bytes.

/// # Hamming Distance
///
/// Computes the hamming distance between two bytes.
///
/// ## Example
///
/// ```
/// use repeating_key_xor::hamming;
///
/// assert_eq!(hamming::distance(0x01, 0x10), 2);
/// ```
pub fn distance(a: u8, b: u8) -> u8 {
    return (0..8).map(|n| ((a >> n) ^ (b >> n)) & 1).fold(0, |a,b| a+b);
}

#[test]
fn hamming_distance_between_bytes_works() {
    assert_eq!(distance(0x12, 0x00), 2);
    assert_eq!(distance(0x12, 0x33), 2);
    assert_eq!(distance(0x55, 0x67), 3);
    assert_eq!(distance(0xFF, 0x00), 8);
    assert_eq!(distance(0xF0, 0x00), 4);
    assert_eq!(distance(0x0F, 0x00), 4);
}

//fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    
