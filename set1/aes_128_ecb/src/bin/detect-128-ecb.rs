extern crate hex_to_base64;
use hex_to_base64::hex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;

fn main() {
    if env::args().len() < 2 {
        println!("{}", "usage:...");
    } else {
        let mut f = File::open(&env::args().nth(1).unwrap()).unwrap();
        let mut reader = BufReader::new(f);

        for (n, l) in (0..).zip(reader.lines()) {
            if let Ok(l) = l {
                let decoded = hex::decode(&l).unwrap();
                let detected = detect(&decoded);
                if detected > 0 {
                    println!("ECB detected on line #{} (with {} detections)", n, detected);
                }
            }
        }
    }
}

fn detect(data: &[u8]) -> u32 {
    (0..).zip(data.chunks(16))
        .map(|(n, left)|
            data.chunks(16)
                .skip(n+1)
                .map(|right| left == right)
                .fold(0 as u32, |a, b| if b {a+1} else {a}))
        .fold(0 as u32, |a, b| a+b)
}
