#![no_std]
#![no_main]

use arduino_hal::{self as hal};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let mut delay = hal::Delay::new();

    loop {
        delay.delay_ms(1000);
    }
}
