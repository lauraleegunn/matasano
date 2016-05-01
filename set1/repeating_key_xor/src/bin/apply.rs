extern crate repeating_key_xor;
extern crate hex_to_base64;
use hex_to_base64::hex;
use std::env::args;

fn main() {
    let opts: Option<(bool, String, String)> = match args().count() {
        3 => Some((true, args().nth(1).unwrap(), args().nth(2).unwrap())),
        4 => match &args().nth(1).unwrap() as &str {
            "-e"|"--encrypt" => Some((true, args().nth(2).unwrap(), args().nth(3).unwrap())),
            "-d"|"--decrypt" => Some((false, args().nth(2).unwrap(), args().nth(3).unwrap())),
            _ => None
        },
        _ => None
    };

    if let Some(opts) = opts {
        println!("{}", crypt(opts.0, &opts.1, &opts.2));
    } else {
        help(&args().nth(0).unwrap());
    }
}

fn help(name: &str) {
    println!("Usage: [-ed] {} data code", name);
    println!(" -e   encrypt data (default)");
    println!(" -d   decrypt data");
}

fn crypt(encrypt: bool, input: &str, code: &str) -> String {
    let input: Vec<u8> = match encrypt {
        true => Vec::from(input),
        false => hex::decode(input).unwrap()
    };

    let code = code.as_bytes();

    let result = repeating_key_xor::apply(&input, code);

    let result = match encrypt {
        true => hex::encode(&result, false),
        false => String::from_utf8_lossy(&result).into_owned()
    };

    return result;
}

#[test]
fn opening_staza_works() {
    let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let code = "ICE";
    assert_eq!(crypt(true, text, code), hex);
    assert_eq!(crypt(false, hex, code), text);
}
