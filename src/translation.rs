//! Module containing utilities for converting the array of morse code
//! characters into a usable latin alphabet character.

use crate::MorseCharacters;

/// Tuple containing translation data for the latin letter 'A'.
/// ([chars], 'a')
///
/// .- = 'a'
pub const LETTER_A: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'a',
);

/// Tuple containing translation data for the latin letter 'B'.
/// ([chars], 'b')
///
/// -... = 'b'
pub const LETTER_B: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'b',
);

/// Tuple containing translation data for the latin letter 'C'.
/// ([chars], 'c')
///
/// -.-. = 'c'
pub const LETTER_C: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'c',
);

/// Tuple containing translation data for the latin letter 'D'.
/// ([chars], 'd')
///
/// -.. = 'd'
pub const LETTER_D: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'd',
);

/// Tuple containing translation data for the latin letter 'E'.
/// ([chars], 'e')
///
/// . = 'e'
pub const LETTER_E: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'e',
);

/// Tuple containing translation data for the latin letter 'F'.
/// ([chars], 'f')
///
/// ..-. = 'f'
pub const LETTER_F: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'f',
);

/// Tuple containing translation data for the latin letter 'G'.
/// ([chars], 'g')
///
/// --. = 'g'
pub const LETTER_G: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'g',
);

/// Tuple containing translation data for the latin letter 'H'.
/// ([chars], 'h')
///
/// .... = 'h'
pub const LETTER_H: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'h',
);

/// Tuple containing translation data for the latin letter 'I'.
/// ([chars], 'i')
///
/// .. = 'i'
pub const LETTER_I: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'i',
);

/// Tuple containing translation data for the latin letter 'J'.
/// ([chars], 'j')
///
/// .--- = 'j'
pub const LETTER_J: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'j',
);

/// Tuple containing translation data for the latin letter 'K'.
/// ([chars], 'k')
///
/// -.- = 'k'
pub const LETTER_K: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'k',
);

/// Tuple containing translation data for the latin letter 'L'.
/// ([chars], 'l')
///
/// .-.. = 'l'
pub const LETTER_L: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'l',
);

/// Tuple containing translation data for the latin letter 'M'.
/// ([chars], 'm')
///
/// -- = 'm'
pub const LETTER_M: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'm',
);

/// Tuple containing translation data for the latin letter 'N'.
/// ([chars], 'n')
///
/// -. = 'n'
pub const LETTER_N: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'n',
);

/// Tuple containing translation data for the latin letter 'O'.
/// ([chars], 'o')
///
/// --- = 'o'
pub const LETTER_O: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'o',
);

/// Tuple containing translation data for the latin letter 'P'.
/// ([chars], 'p')
///
/// .--. = 'p'
pub const LETTER_P: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'p',
);

/// Tuple containing translation data for the latin letter 'Q'.
/// ([chars], 'q')
///
/// --.- = 'q'
pub const LETTER_Q: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'q',
);

/// Tuple containing translation data for the latin letter 'R'.
/// ([chars], 'r')
///
/// .-. = 'r'
pub const LETTER_R: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'r',
);

/// Tuple containing translation data for the latin letter 'S'.
/// ([chars], 's')
///
/// ... = 's'
pub const LETTER_S: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    's',
);

/// Tuple containing translation data for the latin letter 'T'.
/// ([chars], 't')
///
/// - = 't'
pub const LETTER_T: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    't',
);

/// Tuple containing translation data for the latin letter 'U'.
/// ([chars], 'u')
///
/// ..- = 'u'
pub const LETTER_U: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'u',
);

/// Tuple containing translation data for the latin letter 'V'.
/// ([chars], 'v')
///
/// ...- = 'v'
pub const LETTER_V: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'v',
);

/// Tuple containing translation data for the latin letter 'W'.
/// ([chars], 'w')
///
/// .-- = 'w'
pub const LETTER_W: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'w',
);

/// Tuple containing translation data for the latin letter 'X'.
/// ([chars], 'x')
///
/// -..- = 'x'
pub const LETTER_X: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'x',
);

/// Tuple containing translation data for the latin letter 'Y'.
/// ([chars], 'y')
///
/// -.-- = 'y'
pub const LETTER_Y: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'y',
);

/// Tuple containing translation data for the latin letter 'Z'.
/// ([chars], 'z')
///
/// --.. = 'z'
pub const LETTER_Z: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    'z',
);

/// Tuple containing translation data for arabic numeral '0'.
/// ([chars], '0')
///
/// ----- = '0'
pub const NUMBER_0: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '0',
);

/// Tuple containing translation data for arabic numeral '1'.
/// ([chars], '1')
///
/// .---- = '1'
pub const NUMBER_1: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '1',
);

/// Tuple containing translation data for arabic numeral '2'.
/// ([chars], '2')
///
/// ..--- = '2'
pub const NUMBER_2: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '2',
);

/// Tuple containing translation data for arabic numeral '3'.
/// ([chars], '3')
///
/// ...-- = '3'
pub const NUMBER_3: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '3',
);

/// Tuple containing translation data for arabic numeral '4'.
/// ([chars], '4')
///
/// ....- = '4'
pub const NUMBER_4: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dash),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '4',
);

/// Tuple containing translation data for arabic numeral '5'.
/// ([chars], '5')
///
/// ..... = '5'
pub const NUMBER_5: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '5',
);

/// Tuple containing translation data for arabic numeral '6'.
/// ([chars], '6')
///
/// -.... = '6'
pub const NUMBER_6: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '6',
);

/// Tuple containing translation data for arabic numeral '7'.
/// ([chars], '7')
///
/// --... = '7'
pub const NUMBER_7: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '7',
);

/// Tuple containing translation data for arabic numeral '8'.
/// ([chars], '8')
///
/// ---.. = '8'
pub const NUMBER_8: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '8',
);

/// Tuple containing translation data for arabic numeral '9'.
/// ([chars], '9')
///
/// ----. = '9'
pub const NUMBER_9: ([Option<MorseCharacters>; 16], char) = (
    [
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dash),
        Some(MorseCharacters::Dot),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    '9',
);
