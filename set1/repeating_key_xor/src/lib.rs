//! # Braking Repeating-Key XOR
//!
//! >   It is officially on, now.
//! >   
//! >   This challenge isn't conceptually hard, but it involves actual 
//! >   error-prone coding. The other challenges in this set are there 
//! >   to bring you up to speed. This one is there to qualify
//! >   you. If you can do this one, you're probably just fine up to Set 6.
//!
//! [There's a file here](http://cryptopals.com/static/challenge-data/6.txt).
//! It's been base64'd after being encrypted with repeating-key XOR.
//!
//! Decrypt it.
//!
//! Here's how:
//!
//! 1.  Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
//! 2.  Write a function to compute the edit distance/Hamming distance between two 
//!     strings. *The Hamming distance is just the number of differing bits.*
//!     The distance between: `this is a test` and `wokka wokka!!!`
//!     is 37. Make sure your code agrees before you proceed.
//! 3.  For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of
//!     bytes, and find the edit distance between them. Normalize this result by dividing by
//!     KEYSIZE.
//! 4.  The KEYSIZE with the smallest normalized edit distance is probably the key. You could
//!     proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2
//!     and average the distances.
//! 5.  Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
//! 6.  Now transpose the blocks: make a block that is the first byte of every block, and a block
//!     that is the second byte of every block, and so on.
//! 7.  Solve each block as if it was single-character XOR. You already have code to do this.
//! 8.  For each block, the single-byte XOR key that produces the best looking histogram is the
//!     repeating-key XOR key byte for that block. Put them together and you have the key.
//!
//! This code is going to turn out to be surprisingly useful later on. Breaking repeating-key XOR
//! ("Vigenere") statistically is obviously an academic exercise, a "Crypto 101" thing. But more
//! people "know how" to break it than can actually break it, and a similar technique breaks
//! something much more important.

extern crate itertools;
extern crate single_byte_xor;
pub mod hamming;
use std::cmp;
use itertools::Itertools;

/// # Apply Repeating-Key XOR
///
/// This function takes some input and a key, and XOR's the
/// input with the key, repeating it when it is shorter than
/// the input.
///
/// ## Example
///
/// ```
/// assert_eq!(repeating_key_xor::apply(&[0x12, 0x34], &[0x11]), [0x03, 0x25]);
/// ```
pub fn apply(input: &[u8], code: &[u8]) -> Vec<u8> {
    input.iter()
        .zip(code.iter().cycle())
        .map(|(i, c)| (*i) ^ (*c))
        .collect()
}

#[test]
fn repeated_key_xor_apply_works() {
    assert_eq!(apply(&[0x11, 0x11, 0x00], &[0x11, 0x00]), [0x00, 0x11, 0x11]);
}

fn apnd(v: &Vec<u8>, e: u8) -> Vec<u8> {
    let mut c = v.clone();
    c.push(e);
    c
}

fn transpose(strs: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    strs.iter()
        .fold(None as Option<Vec<Vec<u8>>>, |transposed, line| 
            match transposed {
                None => Some(line.iter().map(|x| vec![*x]).collect()),
                Some(transposed) => Some(transposed.iter().zip(line.iter()).map(|(c,e)| apnd(c, *e)).collect::<Vec<Vec<u8>>>())
    }).unwrap()
}

pub fn break_cipher(input: &[u8]) -> Vec<u8> {
    let keysizes = keysize_candidates(input, 2, 40);

    // how many candidates should we test?
    let n_candidates = 3;

    let results: Vec<Vec<u8>> = keysizes.iter()
        .take(n_candidates)
        .map(|candidate|
             transpose((0..(*candidate))
                .map(|n| 
                     input.iter()
                        .cloned()
                        .skip(n as usize)
                        .step((*candidate) as usize)
                        .collect::<Vec<u8>>())
                .map(|s| single_byte_xor::decrypt(&s))
                .map(|(result, _, _)| Vec::from(result.as_bytes()))
                .collect::<Vec<Vec<u8>>>()).iter()
                .nth(0).unwrap().clone())
        .inspect(|n| println!("{:?}", String::from_utf8(n.clone()).unwrap()))
        .collect();

    results[0].clone()
}

/// # Find Keysize Candidates
///
/// This function will attempt to find possible candidates
/// for the keysize using the edit distance between keysize
/// sized block of the input data. The list of candidates 
/// will be arbitrarily long, the only guarantee is that
/// the candidates will be between min and max, and that
/// they will be sorted according to their likelyhood (as
/// estimated by the normalized edit distance).
///
/// ## Example
///
/// ```
/// use repeating_key_xor::keysize_candidates;
///
/// assert_eq!(keysize_candidates(&[0x44], 4, 5), [4]);
/// ```
pub fn keysize_candidates(data: &[u8], min: u32, max: u32) -> Vec<u32> {
    let mut candidates: Vec<(u32, f32)> = (min..max)
        .map(|candidate|
            (candidate, data.chunks(candidate as usize)
                .zip(data.chunks(candidate as usize).skip(1))
                .map(|(a, b)| 
                     (hamming::distance(a, b) / (cmp::min(a.len(), b.len()) as u32)))
                .fold(0 as u32, |a,b| a+b) as f32 / 
                cmp::max(1, ((data.len() as u32 + candidate - 1)/candidate) - 1) as f32))
        .collect();
    
    candidates.sort_by(|a,b| (a.1).partial_cmp(&b.1).unwrap());
    candidates.iter().map(|n| n.0).collect()
}

