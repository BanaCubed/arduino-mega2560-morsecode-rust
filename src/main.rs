#![no_std]
#![no_main]

mod translation;

use arduino_hal::{self as hal};
use panic_halt as _;

/// The frequency of the baudrate clock.
///
/// I don't fully understand how this works, given that it is the default given
/// by arduino_hal.
const BAUDRATE: u32 = 57600;

/// Time in milliseconds between input polling.
///
/// Can be lowered if the "flickering" causes issues or the execution time for
/// code becomes impactful.
///
/// Can be increased if inputs get voided.
const POLL_DURATION: u32 = 10;

/// Cuttof value for `adc.read_blocking()` below which the button is treated as
/// pressed.
///
/// If this is set too high, the button may be considered pressed when it
/// isn't.
const INPUT_BLOCKING_THRESHOLD: u16 = 10;

/// The minimum time the button must be held down to count as an input.
///
/// This solves the issue of push buttons "flickering" when being pressed or
/// unpressed.
///
/// Acts as the minimum length for a dot.
const MIN_INPUT_LENGTH: u32 = 30;

/// The maximum time a dot can last.
///
/// Acts as the maximum length for a dot and the minimum length for a dash.
const MAX_DOT_LENGTH: u32 = 200;

/// The longest the button can be held before being treated as a manual
/// submission.
///
/// Acts as the maximum length for a dash.
const MAX_DASH_LENGTH: u32 = 1000;

/// Prints a line to the serial.
///
/// Has no return type since the lack of standard library with [arduino_hal]
/// removes access to `String`s.
///
/// Abstract types are so confusing.
fn print_to_serial<W>(
    serial: &mut W,
    morse: &[Option<translation::MorseCharacters>],
    ch: Option<char>,
) where
    W: ufmt::uWrite,
{
    let _ = ufmt::uwrite!(serial, "{}: " match ch { None => {'#'}, Some(x) => {x} });
    for ch in morse.iter() {
        match ch {
            Some(translation::MorseCharacters::Dot) => {
                let _ = ufmt::uwrite!(serial, ".");
            }
            Some(translation::MorseCharacters::Dash) => {
                let _ = ufmt::uwrite!(serial, "-");
            }
            _ => {}
        }
    }
    let _ = ufmt::uwriteln!(serial, "");
}

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, BAUDRATE);
    let mut adc = hal::Adc::new(dp.ADC, Default::default());

    let input_pin = pins.a0.into_analog_input(&mut adc);
    // Measured in ms.
    // More precision than ms should not be required.
    // Only concern is if nothing happens for over a minute issues may arise.
    let mut time_since_change: u32 = 0;
    let mut last_input = false;

    // Stores up to 8 morse code characters.
    // Gets cleared when a valid character is inputted or the length goes past the maximum.
    // In practice 8 slots is more than enough.
    let mut chars: [Option<translation::MorseCharacters>; 8] = Default::default();

    loop {
        let input_active = adc.read_blocking(&input_pin) <= INPUT_BLOCKING_THRESHOLD;
        // I assume that the time to execute code is negligible, even though it
        // likely won't be. This avoids the need to actually track time, and
        // still should keep enough accuracy to be usable. In the event that
        // the execution time becomes problematic, increasing `POLL_DURATION`
        // should help.
        hal::delay_ms(POLL_DURATION);

        // Timing updates.
        if last_input == input_active {
            time_since_change += POLL_DURATION;
            continue;
        }

        last_input = input_active;
        if input_active {
            time_since_change = 0;
            continue;
        }

        let morse_char = get_morse_char(time_since_change);
        time_since_change = 0;

        match morse_char {
            None => {
                continue;
            }
            Some(ch) => {
                if !matches!(ch, translation::MorseCharacters::Space) {
                    let mut index: usize = 0;
                    while matches!(&chars[index], Some(_x)) {
                        index += 1;
                    }
                    chars[index] = Some(ch);
                }
            }
        }

        print_to_serial(&mut serial, &chars, translation::get_character(&chars));
    }
}

/// Get the morse code character from a given duration of time.
///
/// Returns None if the duration of time is longer than a dash.
fn get_morse_char(time: u32) -> Option<translation::MorseCharacters> {
    if time < MIN_INPUT_LENGTH {
        None
    } else if time < MAX_DOT_LENGTH {
        Some(translation::MorseCharacters::Dot)
    } else if time < MAX_DASH_LENGTH {
        Some(translation::MorseCharacters::Dash)
    } else {
        Some(translation::MorseCharacters::Space)
    }
}
