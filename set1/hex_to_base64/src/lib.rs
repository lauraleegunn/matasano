
/// # Decoding Hex Strings
///
/// ## What could possibly go wrong? 
///
/// Well, the string could contain non-hex characters (anything outside
/// the range of [a-zA-Z0-9]. In that case, we don't return anything.
///
/// Normally, we use two hex characters to represent a byte, so the
/// input string needs to have an even number of characters. If it doesn't,
/// the function assumes a leading zero.
fn hex_decode(input: &str) -> Option<Vec<u8>> {
    let prefix = match input.len() % 2 {
        1 => "0",
        _ => ""
    };
    
    let parsed: Option<Vec<u8>> = prefix
        .chars().chain(input.chars())
            .map(|x| 
                match x {
                    num @ '0' ... '9'  => Some((num as u8) - ('0' as u8)),
                    alph @ 'a' ... 'f' => Some((alph as u8) - ('a' as u8) + 10),
                    alph @ 'A' ... 'F' => Some((alph as u8) - ('A' as u8) + 10),
                    _ => None
        }).collect();
    
    let joined: Option<Vec<u8>> = match parsed {
        None => None,
        Some(parsed) => Some(parsed.chunks(2).map(|x| (x[0] << 4) | x[1]).collect())
    };

    return joined;
}

#[test]
fn hex_decode_fails_on_invalid() {
    // make sure that hex_decode returns None when passed invalid
    // input (eg. stuff that can't be decoded).
    assert!(hex_decode("ggg") == None);
    assert!(hex_decode("gsdfkhg") == None);
    assert!(hex_decode("ffffffx") == None);
}

#[test]
fn hex_decode_works_on_valid() {
    // make sure that hex_decode works correctly for correct
    // input (both lower- and uppercase characters).
    assert!(hex_decode("0001020304").unwrap() == [0, 1, 2, 3, 4]);
    assert!(hex_decode("FF").unwrap() == [255]);
    assert!(hex_decode("90AF").unwrap() == [144, 175]);
    assert!(hex_decode("deadbeef").unwrap() == [222, 173, 190, 239]);
}

#[test]
fn hex_decode_works_on_odd() {
    // make sure that hex_decode works correctly with an odd
    // number of characters (in that case, it should prepend
    // a zero)
    assert!(hex_decode("A").unwrap() == [10]);
    assert!(hex_decode("012").unwrap() == [0, 18]);
    assert!(hex_decode("eadbeef").unwrap() == [14, 173, 190, 239]);
}

fn hex_encode(input: &[u8], uppercase: bool) -> String {
    let base_char: u8 = if uppercase {
        'A' as u8
    } else {
        'a' as u8
    };
    
    let split: Vec<[u8; 2]> = input
        .iter()
        .map(|x| [(x >> 4), x & (1 | 2 | 4 | 8)])
        .collect();

    let joint: Vec<u8> = split
        .iter()
        .flat_map(|x| x.iter())
        .map(|c|
            match *c {
                num @ 0 ... 9 => ('0' as u8) + num,
                alph @ 10 ... 15 => base_char + alph - 10,
                _ => unreachable!()
            })
        .collect();
      
    let string: String = String::from_utf8(joint).unwrap();

    return string;
}

#[test]
fn hex_encode_works_correctly_with_ascii() {
    assert_eq!(hex_encode("abcdefg".as_bytes(), true), "61626364656667");
    assert_eq!(hex_encode("abcdefg".as_bytes(), false), "61626364656667");
    assert_eq!(hex_encode("JKLMNOP".as_bytes(), false), "4a4b4c4d4e4f50");
    assert_eq!(hex_encode("JKLMNOP".as_bytes(), true), "4A4B4C4D4E4F50");
}

#[test]
fn hex_encode_works_correctly_with_binary() {
    assert_eq!(hex_encode(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], true), "1234567890ABCDEF");
    assert_eq!(hex_encode(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], false), "1234567890abcdef");
}

#[test]
fn hex_reencode_works_correctly() {
    assert_eq!(hex_decode(&hex_encode("DeadBeefCafeBabe".as_bytes(), true)).unwrap(), "DeadBeefCafeBabe".as_bytes());
    assert_eq!(hex_decode(&hex_encode("DeadBeefCafeBabe".as_bytes(), false)).unwrap(), "DeadBeefCafeBabe".as_bytes());
}

fn base64_encode(input: &[u8]) -> String {
    let input: Vec<u8> = Vec::from(input);
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
    assert_eq!(base64_encode("M".as_bytes()), "TQ==");
    assert_eq!(base64_encode("Ma".as_bytes()), "TWE=");
}

#[test]
fn base64_encode_padding_with_equal_signs_works() {
    assert_eq!(base64_encode("pleasure.".as_bytes()), "cGxlYXN1cmUu");
    assert_eq!(base64_encode("leasure.".as_bytes()), "bGVhc3VyZS4=");
    assert_eq!(base64_encode("easure.".as_bytes()), "ZWFzdXJlLg==");
    assert_eq!(base64_encode("asure.".as_bytes()), "YXN1cmUu");
    assert_eq!(base64_encode("sure.".as_bytes()), "c3VyZS4=");
}


fn base64_decode(input: &str) -> Vec<u8> {
    return Vec::new();
}
