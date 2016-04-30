
pub fn encode(input: &[u8]) -> String {
    return String::from_utf8(input.chunks(3)
        .map(|chunk|
             if chunk.len() > 2 {
                 vec![chunk[0] >> 2, 
                  ((chunk[0] << 4) | (chunk[1] >> 4)) & ((1 << 6)-1), 
                  ((chunk[1] << 2) | (chunk[2] >> 6)) & ((1 << 6)-1), 
                  chunk[2] & ((1 << 6) -1)]
             } else if chunk.len() > 1 {
                 vec![chunk[0] >> 2, 
                  ((chunk[0] << 4) | (chunk[1] >> 4)) & ((1 << 6)-1), 
                  (chunk[1] << 2) & ((1 << 6)-1), 
                  64]
             } else {
                 vec![chunk[0] >> 2, 
                  (chunk[0] << 4) & ((1 << 6)-1), 
                  64,
                  64]
             })
        .flat_map(|e| e.into_iter())
        .map(|c| match c {
            alph @ 0 ... 25 => ('A' as u8) + alph,
            alph @ 26 ... 51 => ('a' as u8) + alph - 26,
            num  @ 52 ... 61 => ('0' as u8) + num - 52,
            62 => '+' as u8,
            63 => '/' as u8,
            _ => '=' as u8
        })
        .collect())
        .unwrap();
}

#[test]
fn base64_encode_works_correctly_on_small_input() {
    assert_eq!(encode("M".as_bytes()), "TQ==");
    assert_eq!(encode("Ma".as_bytes()), "TWE=");
}

#[test]
fn base64_encode_padding_with_equal_signs_works() {
    assert_eq!(encode("pleasure.".as_bytes()), "cGxlYXN1cmUu");
    assert_eq!(encode("leasure.".as_bytes()), "bGVhc3VyZS4=");
    assert_eq!(encode("easure.".as_bytes()), "ZWFzdXJlLg==");
    assert_eq!(encode("asure.".as_bytes()), "YXN1cmUu");
    assert_eq!(encode("sure.".as_bytes()), "c3VyZS4=");
}

pub fn decode(input: &str) -> Vec<u8> {
    let translated: Vec<u8> = input.as_bytes().iter()
        .map(|c| match *c as char {
            'A' ... 'Z' => c - ('A' as u8),
            'a' ... 'z' => c - ('a' as u8) + 26,
            '0' ... '9' => c - ('0' as u8) + 52,
            '+' => 62 as u8,
            '/' => 63 as u8,
            _ => 64 as u8
        })
        .collect();

    return translated.chunks(4)
        .map(|chunk|
             if chunk.len() < 3 || chunk[2] == 64 {
                 vec![(chunk[0] << 2) | (chunk[1] >> 4)]
             } else if chunk.len() < 4 || chunk[3] == 64 {
                 vec![(chunk[0] << 2) | (chunk[1] >> 4),
                 (chunk[1] << 4) | (chunk[2] >> 2)]
             } else {
                 vec![(chunk[0] << 2) | (chunk[1] >> 4),
                 (chunk[1] << 4) | (chunk[2] >> 2),
                 (chunk[2] << 6) | chunk[3]]
             })
        .flat_map(|e| e.into_iter())
        .collect();
}

#[test]
fn base64_decode_works_correctly_on_small_input() {
    assert_eq!(decode("TQ=="), "M".as_bytes());
    assert_eq!(decode("TWE="), "Ma".as_bytes());
}

#[test]
fn base64_decode_correctly_parses_padding() {
    assert_eq!(decode("cGxlYXN1cmUu"), "pleasure.".as_bytes());
    assert_eq!(decode("bGVhc3VyZS4="), "leasure.".as_bytes());
    assert_eq!(decode("ZWFzdXJlLg=="), "easure.".as_bytes());
    assert_eq!(decode("YXN1cmUu"), "asure.".as_bytes());
    assert_eq!(decode("c3VyZS4="), "sure.".as_bytes());
}

#[test]
fn base_64_roundtrip_conversion_works() {
    assert_eq!(decode(&encode("string".as_bytes())), "string".as_bytes());
    assert_eq!(decode(&encode("morning glory".as_bytes())), "morning glory".as_bytes());
    assert_eq!(decode(&encode("strifleness".as_bytes())), "strifleness".as_bytes());
}
