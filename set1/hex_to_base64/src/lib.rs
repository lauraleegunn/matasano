
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

fn hex_encode(input: &[u8]) -> String {
    return String::new();
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

