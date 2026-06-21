#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use arduino_hal::{self as hal};
use lcd_lcm1602_i2c::{Backlight, sync_lcd::Lcd};

const LCD_ADDRESS: u8 = 0x27;

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut i2c = hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        10000,
    );
    let mut delay = hal::Delay::new();

    let mut lcd = Lcd::new(&mut i2c, &mut delay)
        .with_address(LCD_ADDRESS)
        .with_cursor_on(true)
        .with_rows(2)
        .init().unwrap();

    let mut delay = hal::Delay::new();
    delay.delay_ms(100);

    let _ = lcd.clear();
    let _ = lcd.return_home();
    let _ = lcd.write_str("Hello");
    let _ = lcd.backlight(Backlight::On);

    loop {
        delay.delay_ms(1000);
        let _ = lcd.write_str("h");
    }
}
