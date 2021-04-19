#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate embedded_hal;
extern crate hd44780_driver;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use core::alloc::Layout;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use stellaris_launchpad::board;
use tm4c123x_hal::gpio::GpioExt;

#[no_mangle]
pub fn stellaris_main(mut board: stellaris_launchpad::board::Board) {
    let mut delay = tm4c123x_hal::delay::Delay::new(
        board.core_peripherals.SYST,
        stellaris_launchpad::board::clocks(),
    );

    let pins_a = board.GPIO_PORTA.split(&board.power_control);
    let pins_c = board.GPIO_PORTC.split(&board.power_control);
    let pins_d = board.GPIO_PORTD.split(&board.power_control);

    let rs = pins_a.pa2.into_push_pull_output();
    let en = pins_d.pd6.into_push_pull_output();
    let b4 = pins_c.pc7.into_push_pull_output();
    let b5 = pins_c.pc6.into_push_pull_output();
    let b6 = pins_c.pc5.into_push_pull_output();
    let b7 = pins_c.pc4.into_push_pull_output();

    //let mut lcd = HD44780::new_4bit(rs, en, b4, b5, b6, b7, delay);
    //lcd.reset();
    //lcd.clear();

    let mut lcd = HD44780::new_4bit(rs, en, b4, b5, b6, b7, &mut delay).unwrap();
    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();

    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Invisible,
            cursor_blink: CursorBlink::Off,
        },
        &mut delay,
    )
    .unwrap();

    lcd.write_str("BK-HCMUT 2021-04", &mut delay).unwrap();
    lcd.set_cursor_pos(40, &mut delay).unwrap();
    lcd.write_char('R', &mut delay).unwrap();
    lcd.write_char('U', &mut delay).unwrap();
    lcd.write_char('D', &mut delay).unwrap();
    lcd.write_char('O', &mut delay).unwrap();

    loop {
        board.led_green.set_high().unwrap();
        delay.delay_ms(500u32);
        board.led_green.set_low().unwrap();
        board.led_blue.set_high().unwrap();
        delay.delay_ms(500u32);
        board.led_blue.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    board::panic();
}
