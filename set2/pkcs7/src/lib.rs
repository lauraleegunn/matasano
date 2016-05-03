//! # PKCS#7 Padding Implementation
//!
//! PKCS#7 is defined in [RFC2315](https://tools.ietf.org/html/rfc2315),
//! and the padding algorithm in section 10.3:
//!
//!	>	Some content-encryption algorithms assume the
//!	>	input length is a multiple of k octets, where k > 1, and
//!	>	let the application define a method for handling inputs
//!	>	whose lengths are not a multiple of k octets. For such
//!	>	algorithms, the method shall be to pad the input at the
//!	>	trailing end with k - (l mod k) octets all having value k -
//!	>	(l mod k), where l is the length of the input. [...]
//!	>	The padding can be removed unambiguously since all input is
//!	>	padded and no padding string is a suffix of another. This
//!	>	padding method is well-defined if and only if k < 256;
//!	>	methods for larger k are an open issue for further study.

pub fn pad(input: &[u8], block_size: usize) -> Vec<u8> {
    // calculate how many bytes of padding we need (and
    // we use this value as the value for all the padding
    // bytes, as well)
	let pad_size = block_size - (input.len() % block_size);
    
    // take an iterator of out input and chain an iterator
    // that will produce exactly `pad_size` bytes of
    // `pad_size`, and voilá, you got yourself some padded
    // data!
    input.iter()
        .chain([pad_size as u8].iter().cycle().take(pad_size))
        .cloned()
        .collect::<Vec<u8>>()
}

#[test]
fn padding_works() {
    assert_eq!(pad("YELLOW SUBMARINE".as_bytes(), 20), 
               "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes());
    assert_eq!(pad("YELLOW SUBMARINE".as_bytes(), 19), 
               "YELLOW SUBMARINE\x03\x03\x03".as_bytes());
    assert_eq!(pad("YELLOW SUBMARINE".as_bytes(), 18), 
               "YELLOW SUBMARINE\x02\x02".as_bytes());
    assert_eq!(pad("YELLOW SUBMARINE".as_bytes(), 17), 
               "YELLOW SUBMARINE\x01".as_bytes());
    assert_eq!(pad("YELLOW SUBMARINE".as_bytes(), 8), 
               "YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes());
}

pub fn unpad(input: &[u8]) -> Option<Vec<u8>> {
    // read the last byte to see how much padding
    // was added
    let pad_size = input[input.len()-1] as usize;

    // error handling: check if the padding is
    // alright
    for i in 0..pad_size {
        if input[input.len()-i-1] != (pad_size as u8) {
            return None;
        }
    }

    // return the input without the padding
    Some(input.iter().take(input.len()-pad_size).cloned().collect())
}

#[test]
fn unpad_works() {
    assert_eq!(unpad("YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes()).unwrap(), 
               "YELLOW SUBMARINE".as_bytes());
    assert_eq!(unpad("YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes()).unwrap(), 
               "YELLOW SUBMARINE".as_bytes());
    assert_eq!(unpad("YELLOW SUBMARINE\x03\x03\x03".as_bytes()).unwrap(), 
               "YELLOW SUBMARINE".as_bytes());
    assert_eq!(unpad("YELLOW SUBMARINE\x02\x02".as_bytes()).unwrap(), 
               "YELLOW SUBMARINE".as_bytes());
    assert_eq!(unpad("YELLOW SUBMARINE\x01".as_bytes()).unwrap(), 
               "YELLOW SUBMARINE".as_bytes());
    assert_eq!(unpad("YELLOW SUBMARINE\x03\x03".as_bytes()), None);
}
