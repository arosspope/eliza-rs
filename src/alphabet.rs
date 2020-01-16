//! Contains helpful constants and functions.
//!
const ALPHABET_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ALPHABET_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const NUMERIC: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub const STANDARD: Standard = Standard;
pub const ALPHANUMERIC: Alphanumeric = Alphanumeric;

pub trait Alphabet {
    /// Attempts to find the position of the character in the alphabet.
    ///
    fn find_position(&self, c: char) -> Option<usize>;

    /// Returns a letter from within the alphabet at a specific index.
    ///
    /// Will return None if the index is out of bounds
    fn get_letter(&self, index: usize, is_uppercase: bool) -> Option<char>;

    /// Performs a modulo on an index so that its value references a position within the alphabet.
    /// This function handles negative wrap around modulo as rust does not natievly support it.
    ///
    fn modulo(&self, i: isize) -> usize {
        (((i % self.length() as isize) + self.length() as isize) % self.length() as isize) as usize
    }

    /// Will check if the text contains valid alphabetic symbols only.
    ///
    fn is_valid(&self, text: &str) -> bool;

    /// Will scrub non-alphabetic characters from the text and return the scrubed version.
    ///
    fn scrub(&self, text: &str) -> String {
        text.chars()
            .into_iter()
            .filter(|&c| self.find_position(c).is_some())
            .collect()
    }

    /// Returns the length of the alphabet
    ///
    fn length(&self) -> usize;
}

pub struct Standard;
impl Alphabet for Standard {
    fn find_position(&self, c: char) -> Option<usize> {
        ALPHABET_LOWER
            .iter()
            .position(|&a| a == c)
            .or(ALPHABET_UPPER.iter().position(|&a| a == c))
    }

    fn get_letter(&self, index: usize, is_uppercase: bool) -> Option<char> {
        if index > self.length() {
            return None;
        }

        match is_uppercase {
            true => Some(ALPHABET_UPPER[index]),
            false => Some(ALPHABET_LOWER[index]),
        }
    }

    fn is_valid(&self, text: &str) -> bool {
        for c in text.chars() {
            if self.find_position(c).is_none() {
                return false;
            }
        }

        true
    }

    fn length(&self) -> usize {
        26
    }
}

pub struct Alphanumeric;
impl Alphabet for Alphanumeric {
    fn find_position(&self, c: char) -> Option<usize> {
        if let Some(pos) = STANDARD.find_position(c) {
            return Some(pos);
        }

        if let Some(pos) = NUMERIC.iter().position(|&n| n == c) {
            return Some(pos + 26);
        }

        None
    }

    fn get_letter(&self, index: usize, is_uppercase: bool) -> Option<char> {
        if index > self.length() {
            return None;
        }

        if index > 25 {
            return Some(NUMERIC[index - 26]);
        }

        match is_uppercase {
            true => Some(ALPHABET_UPPER[index]),
            false => Some(ALPHABET_LOWER[index]),
        }
    }

    fn is_valid(&self, text: &str) -> bool {
        for c in text.chars() {
            if self.find_position(c).is_none() {
                return false;
            }
        }

        true
    }

    fn length(&self) -> usize {
        36
    }
}
