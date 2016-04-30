// Single-byte XOR decryption tool.
extern crate hex_to_base64;
extern crate single_byte_xor;
use hex_to_base64::hex;
use std::env;


fn main() {
    if let Some(hex_string) = env::args().nth(1) {
        if let Some((data, code, sureness)) = try_decrypting(&hex_string) {
            println!("code: {}", code);
            println!("sure: {}%", (sureness*100.0)-100.0);
            println!("data: '{}'", data);
        } else {
            println!("couldn't decrypt data :(");
        }
    } else {
        help();
    }
}

fn help() {
    println!("single_byte_xor_decrypter [hex string]");
    println!(" decrypts the given hex string.");
}

fn try_decrypting(input: &str) -> Option<(String, u8, f32)> {
    let data = hex::decode(input);

    if data.is_none() {
        return None;
    }

    let data = data.unwrap();

    return Some(single_byte_xor::decrypt(&data));
}

#[test]
fn correctly_deciphers_challenge() {
    let result = try_decrypting("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    assert!(result.is_some());
    assert!(result.unwrap().1 == 88);
}
