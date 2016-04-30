// Single-byte XOR decryption tool.
extern crate hex_to_base64;
extern crate single_byte_xor;
use hex_to_base64::hex;
use std::env;

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    match &env::args().nth(1).unwrap_or(String::new()) as &str {
        "--hex" => decrypt_hex(&env::args().nth(2).unwrap_or(String::new())),
        "--file" => decrypt_file(&env::args().nth(2).unwrap_or(String::new())),
        _ => help()
    }
}

fn decrypt_hex(hex_string: &str) {
    if let Some((data, code, sureness)) = try_decrypting(hex_string) {
        println!("code: {}", code);
        println!("sure: {}%", (sureness*100.0)-100.0);
        println!("data: '{}'", data);
    } else {
        println!("couldn't decrypt data :(");
    }
}

fn decrypt_file(file_name: &str) {
    let mut file = BufReader::new(File::open(file_name).unwrap());
    let lines: Vec<(u32, String, String, u8, f32)> = (1..).zip(file.lines())
        .map(|(n, line)| (n, line.unwrap()))
        .map(|(n, line)| (n, line.clone(), single_byte_xor::decrypt(&hex::decode(&line).unwrap())))
        .map(|(n, line, result)| (n, line, result.0, result.1, result.2))
        .collect();

    for (n, line, decrypted, code, sureness) in lines {
        println!("line #{} was decrypted with code {} and sureness {}%:", n, code, (sureness*100.0)-100.0);
        println!(" '{}' => '{}'", line, decrypted);
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
