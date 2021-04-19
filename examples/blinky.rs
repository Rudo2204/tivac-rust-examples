#![no_std]
#![no_main]

extern crate embedded_hal;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

#[no_mangle]
pub fn stellaris_main(mut board: stellaris_launchpad::board::Board) {
    let mut delay = tm4c123x_hal::delay::Delay::new(
        board.core_peripherals.SYST,
        stellaris_launchpad::board::clocks(),
    );

    loop {
        board.led_green.set_high().unwrap();
        delay.delay_ms(500u32);
        board.led_green.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}
