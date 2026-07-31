#![no_std]
#![no_main]

mod translation;

use arduino_hal::{self as hal};
use panic_halt as _;

/// The `MorseCharacters` type.
///
/// Contains valid characters morse code can contain.
#[derive(PartialEq)]
enum MorseCharacters {
    Dot,
    Dash,
    Space,
}

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = hal::Adc::new(dp.ADC, Default::default());

    let input_pin = pins.a0.into_analog_input(&mut adc);
    // Measured in ms.
    // More precision than ms should not be required.
    // Only concern is if nothing happens for over a minute issues may arise.
    let mut time_since_change: u16 = 0;
    let mut last_input = false;

    // Stores up to 8 morse code characters.
    // Gets cleared when a valid character is inputted or the length goes past the maximum.
    // In practice 8 slots is more than enough.
    let mut chars: [Option<MorseCharacters>; 8] = Default::default();

    loop {
        let input_active = adc.read_blocking(&input_pin) <= 10;
        // I assume that the time to execute code is negligible, even though it likely won't be.
        // This avoids the need to actually track time, and still should keep enough accuracy to be usable.
        hal::delay_ms(10);

        // Timing updates.
        if last_input == input_active {
            time_since_change += 10;
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
                if matches!(ch, MorseCharacters::Space) {
                    chars = Default::default();
                } else {
                    let mut index: usize = 0;
                    while matches!(&chars[index], Some(_x)) {
                        index += 1;
                    }
                    chars[index] = Some(ch);
                }
            }
        }

        // I've tried splitting this into a seperate function but:
        // 1. I cannot get `serial` to have the right type annotation.
        // 2. Due to the lack of the standard library, I cannot use `String`s.
        let _ = ufmt::uwrite!(&mut serial, "Letter: ");
        for ch in chars.iter() {
            match ch {
                Some(MorseCharacters::Dot) => {
                    let _ = ufmt::uwrite!(&mut serial, ".");
                }
                Some(MorseCharacters::Dash) => {
                    let _ = ufmt::uwrite!(&mut serial, "-");
                }
                _ => {}
            }
        }
        let _ = ufmt::uwriteln!(&mut serial, "");
    }
}

/// Get the morse code character from a given duration of time.
///
/// Returns None if the duration of time is longer than a dash.
fn get_morse_char(time: u16) -> Option<MorseCharacters> {
    if time < 30 {
        None
    } else if time < 200 {
        Some(MorseCharacters::Dot)
    } else if time < 1000 {
        Some(MorseCharacters::Dash)
    } else {
        Some(MorseCharacters::Space)
    }
}
