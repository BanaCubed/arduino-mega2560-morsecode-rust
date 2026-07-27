#![no_std]
#![no_main]

use arduino_hal::{self as hal};
use panic_halt as _;

/// The `MorseCharacters` type. Contains valid characters morse code can contain.
enum MorseCharacters {
    Dot,
    Dash,
}

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins: hal::Pins = hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = hal::Adc::new(dp.ADC, Default::default());

    let input_pin = pins.a0.into_analog_input(&mut adc);
    // Measured in ms.
    // More precision than ms should not be required.
    let mut time_since_change: u16 = 0;

    loop {
        let input_active = adc.read_blocking(&input_pin) <= 10;
        let _ = ufmt::uwriteln!(&mut serial, "{}", input_active);
        hal::delay_ms(50);
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
