extern crate hex_to_base64;

use hex_to_base64::hex;

/// # Fixed XOR
///
/// Task: 
/// >   Write a function that takes two equal-length buffers 
/// >   and produces their XOR combination.
///
/// I can't (read: don't know how to) force the two buffers
/// to be equal-length, so I omitted that. 
///
/// This function takes two &[u8] and returns their pairwise
/// XOR'ed elements as a Vec<u8>.
///
/// ## Example:
///
/// ```
/// assert_eq!(fixed_xor::fixed_xor(&[0x12, 0x34], &[0x12, 0x11]), [0x00, 0x25]);
/// ```
pub fn fixed_xor(buffer_a: &[u8], buffer_b: &[u8]) -> Vec<u8> {
    return buffer_a.iter()
        .zip(buffer_b.iter())
        .map(|vals| vals.0 ^ vals.1)
        .collect();
}

#[test]
fn fixed_xor_xors_correctly() {
    assert_eq!(fixed_xor(&[0x00], &[0x00]), [0x00]);
    assert_eq!(fixed_xor(&[0x01], &[0x02]), [0x03]);
    assert_eq!(fixed_xor(&[0x5F], &[0x5E]), [0x01]);
}

#[test]
fn fixed_xor_challenge() {
    let input_a = "1c0111001f010100061a024b53535009181c";
    let input_b = "686974207468652062756c6c277320657965";
    let result = "746865206b696420646f6e277420706c6179";

    assert_eq!(hex::encode(&fixed_xor(&hex::decode(input_a).unwrap(), &hex::decode(input_b).unwrap()), false), result);
    assert_eq!(hex::encode(&fixed_xor(&hex::decode(input_b).unwrap(), &hex::decode(input_a).unwrap()), false), result);
}
