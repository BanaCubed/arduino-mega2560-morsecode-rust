#![no_std]
#![no_main]

use arduino_hal::{self as hal};
use panic_halt as _;

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        let _ = ufmt::uwriteln!(&mut serial, "Hello World!\r");
        hal::delay_ms(1000);
    }
}
