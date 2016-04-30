//! # String Scoring
//!
//! To determine if a given decryption is correct, I need some way
//! of 'scoring' decrypted texts. To do this, I make use of the
//! english letter frequency distribution (shamelessly taken from
//! wikipedia) to calculate the average letter frequency for the
//! text (like this longer texts don't score higher, which wouldn't
//! make sense).

/// # Score English Text
///
/// This function takes a reference to a string and returns an f32,
/// which represents the average letter frequency of all the letters
/// in the text. The higher the better!
///
/// I took the scoring data from
/// [here](http://fitaly.com/board/domper3/posts/136.html).
///
/// ## Example
///
/// ```
/// use single_byte_xor::scoring::english;
///
/// assert!(english("eeeeee") > english("zzzzzzzzzzzz"));
/// ```
pub fn english(input: &str) -> f32 {
    input.chars()
        .map(|c| match c {
            ' ' => 171,
            '"' => 2,
            '$' => 1,
            '\'' => 2,
            '(' => 2,
            ')' => 2,
            '*' => 1,
            ',' => 7,
            '-' => 14,
            '.' => 15,
            '/' => 2,
            '0' => 6,
            '1' => 5,
            '2' => 3,
            '3' => 2,
            '4' => 1,
            '5' => 2,
            '6' => 1,
            '7' => 1,
            '8' => 1,
            '9' => 1,
            ':' => 4,
            ';' => 1,
            '<' => 1,
            '>' => 1,
            '?' => 1,
            'A' => 3,
            'B' => 2,
            'C' => 4,
            'D' => 3,
            'E' => 3,
            'F' => 1,
            'G' => 2,
            'H' => 2,
            'I' => 3,
            'J' => 2,
            'K' => 1,
            'L' => 2,
            'M' => 4,
            'N' => 2,
            'O' => 2,
            'P' => 3,
            'R' => 3,
            'S' => 4,
            'T' => 3,
            'U' => 1,
            'V' => 1,
            'W' => 3,
            '_' => 1,
            'a' => 52,
            'b' => 10,
            'c' => 21,
            'd' => 25,
            'e' => 86,
            'f' => 14,
            'g' => 16,
            'h' => 27,
            'i' => 49,
            'j' => 1,
            'k' => 7,
            'l' => 32,
            'm' => 16,
            'n' => 50,
            'o' => 58,
            'p' => 15,
            'q' => 1,
            'r' => 43,
            's' => 44,
            't' => 64,
            'u' => 21,
            'v' => 8,
            'w' => 13,
            'x' => 2,
            'y' => 11,
            'z' => 1,
            _ => 0
        } as u8)
        .fold(0 as f32, |sum, i| sum + (i as f32))/(input.len() as f32)
}

#[test]
fn english_vovels_score_higher_than_consonants() {
    assert!(english("aeuoi") > english("cfghkp"));
    assert!(english("EaUOuoi") > english("GxzKLPm"));
    assert!(english("aiouee") > english("KLmpmnt"));
}
