//! Module containing utilities for converting the array of morse code
//! characters into a usable latin alphabet character.

use crate::MorseCharacters;

/// Constant array of all possible morse code characters.
///
/// Each character is stored as a tuple with values (morsecode, value).
///
/// All values:
/// - `'a'`: `.-`
/// - `'b'`: `-...`
/// - `'c'`: `-.-.`
/// - `'d'`: `-..`
/// - `'e'`: `.`
/// - `'f'`: `..-.`
/// - `'g'`: `--.`
/// - `'h'`: `....`
/// - `'i'`: `..`
/// - `'j'`: `.---`
/// - `'k'`: `-.-`
/// - `'l'`: `.-..`
/// - `'m'`: `--`
/// - `'n'`: `-.`
/// - `'o'`: `---`
/// - `'p'`: `.--.`
/// - `'q'`: `--.-`
/// - `'r'`: `.-.`
/// - `'s'`: `...`
/// - `'t'`: `-`
/// - `'u'`: `..-`
/// - `'v'`: `...-`
/// - `'w'`: `.--`
/// - `'x'`: `-..-`
/// - `'y'`: `-.--`
/// - `'z'`: `--..`
/// - `'0'`: `-----`
/// - `'1'`: `.----`
/// - `'2'`: `..---`
/// - `'3'`: `...--`
/// - `'4'`: `....-`
/// - `'5'`: `.....`
/// - `'6'`: `-....`
/// - `'7'`: `--...`
/// - `'8'`: `---..`
/// - `'9'`: `----.`
pub const CHARACTERS: [([Option<MorseCharacters>; 8], char); 36] = [
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        'a',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
        ],
        'b',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
        ],
        'c',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
            None,
        ],
        'd',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        'e',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
        ],
        'f',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
            None,
        ],
        'g',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
        ],
        'h',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        'i',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
        ],
        'j',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
            None,
        ],
        'k',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
        ],
        'l',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        'm',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        'n',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
            None,
        ],
        'o',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
        ],
        'p',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
        ],
        'q',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
            None,
        ],
        'r',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
            None,
        ],
        's',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        't',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
            None,
        ],
        'u',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
        ],
        'v',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
            None,
        ],
        'w',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
        ],
        'x',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
            None,
        ],
        'y',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
            None,
        ],
        'z',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
        ],
        '0',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
        ],
        '1',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
        ],
        '2',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
        ],
        '3',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dash),
            None,
            None,
            None,
        ],
        '4',
    ),
    (
        [
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
        ],
        '5',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
        ],
        '6',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
        ],
        '7',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
        ],
        '8',
    ),
    (
        [
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dash),
            Some(MorseCharacters::Dot),
            None,
            None,
            None,
        ],
        '9',
    ),
];

/// Checks if a given value is capable of becoming any output in morse code.
pub fn check_possibility(morse: &[Option<MorseCharacters>; 8]) -> bool {
    for (code, _ch) in CHARACTERS {
        let mut i: usize = 0;
        let mut matches = true;

        while i < 8 {
            if morse[i] == None {
                break;
            }
            if code[i] == None {
                // This is only reached if val is longer than character,
                // which makes any matches impossible.
                matches = false;
                break;
            }

            matches = matches && (code[i] == morse[i]);
            i += 1;
        }

        if matches {
            return true;
        }
    }
    return false;
}

pub fn get_character(morse: &[Option<MorseCharacters>; 8]) -> Option<char> {
    for (code, ch) in CHARACTERS {
        let mut i: usize = 0;
        let mut matches = true;

        while i < 8 {
            if morse[i] == None {
                break;
            }
            if code[i] == None {
                // This is only reached if val is longer than character,
                // which makes any matches impossible.
                matches = false;
                break;
            }

            matches = matches && (code[i] == morse[i]);
            i += 1;
        }

        if matches {
            return Some(ch);
        }
    }
    return None;
}
