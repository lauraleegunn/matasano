extern crate repeating_key_xor;
extern crate hex_to_base64;
use hex_to_base64::base64;
use repeating_key_xor::break_cipher;
use std::env;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let decoded = base64::decode(&arg);
    let broken = break_cipher(&decoded);
    let result = String::from_utf8_lossy(&broken).into_owned();
    println!("{}", result);
}
