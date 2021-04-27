#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate embedded_hal;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use core::alloc::Layout;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[no_mangle]
pub fn stellaris_main(mut board: stellaris_launchpad::board::Board) {
    let mut delay = tm4c123x_hal::delay::Delay::new(
        board.core_peripherals.SYST,
        stellaris_launchpad::board::clocks(),
    );

    loop {
        if board.button_one.is_high().unwrap() {
            board.led_red.set_low().unwrap();
            delay.delay_ms(50u32);
        } else {
            board.led_red.set_high().unwrap();
            delay.delay_ms(50u32);
        }
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    stellaris_launchpad::board::panic();
}
