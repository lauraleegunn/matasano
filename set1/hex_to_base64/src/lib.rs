pub mod base64;
pub mod hex;


#[test]
fn hex_to_base64_challenge() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(base64::encode(&hex::decode(hex).unwrap()), base64);
    assert_eq!(hex::encode(&base64::decode(base64), false), hex);
}
