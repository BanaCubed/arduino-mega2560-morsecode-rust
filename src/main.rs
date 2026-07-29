#![no_std]
#![no_main]

mod translation;

use arduino_hal::{self as hal};
use panic_halt as _;

/// The `MorseCharacters` type.
///
/// Contains valid characters morse code can contain.
enum MorseCharacters {
    Dot,
    Dash,
}

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins: hal::Pins = hal::pins!(dp);
    let mut _serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = hal::Adc::new(dp.ADC, Default::default());

    let input_pin = pins.a0.into_analog_input(&mut adc);
    // Measured in ms.
    // More precision than ms should not be required.
    let mut time_since_change: u16 = 0;
    let mut last_input = false;

    // Stores up to 16 morse code characters.
    // Gets cleared when a valid character is inputted or the length goes past the maximum.
    // In practice 16 slots is more than enough.
    let mut chars: [Option<MorseCharacters>; 16] = Default::default();

    loop {
        let input_active = adc.read_blocking(&input_pin) <= 10;
        // I assume that the time to execute code is negligible, which it won't be.
        // This avoids the need to actually track time, and still should keep enough accuracy to be usable.
        hal::delay_ms(10);

        // Timing updates.
        if last_input == input_active {
            time_since_change += 10;
            // Skips to next iteration, since nothing needs to be done.
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
            None => chars = Default::default(),
            Some(ch) => {
                let mut index: usize = 0;
                while matches!(&chars[index], Some(_x)) {
                    index += 1;
                }
                chars[index] = Some(ch);
            }
        }

        let _ = ufmt::uwrite!(&mut _serial, "Letter: ");
        for ch in chars.iter() {
            match ch {
                Some(MorseCharacters::Dot) => {
                    let _ = ufmt::uwrite!(&mut _serial, ".");
                }
                Some(MorseCharacters::Dash) => {
                    let _ = ufmt::uwrite!(&mut _serial, "-");
                }
                None => {}
            }
        }
        let _ = ufmt::uwriteln!(&mut _serial, "");
    }
}

/// Get the morse code character from a given duration of time.
///
/// Returns None if the duration of time is longer than a dash.
fn get_morse_char(time: u16) -> Option<MorseCharacters> {
    if time < 250 {
        return Some(MorseCharacters::Dot);
    } else if time < 2500 {
        return Some(MorseCharacters::Dash);
    } else {
        return None;
    }
}
