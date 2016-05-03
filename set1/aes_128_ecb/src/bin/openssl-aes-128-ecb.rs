extern crate openssl;
extern crate hex_to_base64;
use hex_to_base64::base64;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use openssl::crypto::symm;


fn main() {
    if env::args().len() < 3 {
        println!("usage: {} <path-to-base64-encoded-file> <16-byte-secret>", 
                 env::args().nth(0).unwrap());
    } else {
        let filename = env::args().nth(1).unwrap();
        let mut file = File::open(&filename).unwrap();

        let mut data = String::new();
        file.read_to_string(&mut data);

        let decoded = base64::decode(&data);
        let secret = env::args().nth(2).unwrap();

        let decrypted = symm::decrypt(symm::Type::AES_128_ECB, &secret.as_bytes(), &[], &decoded);

        if let Ok(s) = String::from_utf8(decrypted.clone()) {
            println!("{}", s);
        } else {
            println!("{:?}", decrypted);
        }
    }
}
