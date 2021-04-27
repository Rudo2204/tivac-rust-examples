#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate keypad;

extern crate embedded_hal;
extern crate hd44780_driver;
extern crate numtoa;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use core::alloc::Layout;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use numtoa::NumToA;
use stellaris_launchpad::board;
use tm4c123x_hal::gpio::GpioExt;

use tm4c123x_hal::gpio::{
    gpioa::PA5, gpioa::PA6, gpioa::PA7, gpiob::PB0, gpiob::PB1, gpiob::PB4, gpioe::PE4, gpioe::PE5,
};
use tm4c123x_hal::gpio::{Input, Output, PullUp, PushPull};

keypad_struct! {
    struct MyKeypad {
        rows: (
            PE4<Input<PullUp>>,
            PB1<Input<PullUp>>,
            PB0<Input<PullUp>>,
        ),
        columns: (
            PA5<Output<PushPull>>,
            PA6<Output<PushPull>>,
            PA7<Output<PushPull>>,
        ),
    }
}

#[no_mangle]
pub fn stellaris_main(mut board: stellaris_launchpad::board::Board) {
    let mut delay = tm4c123x_hal::delay::Delay::new(
        board.core_peripherals.SYST,
        stellaris_launchpad::board::clocks(),
    );

    let pins_a = board.GPIO_PORTA.split(&board.power_control);
    let pins_c = board.GPIO_PORTC.split(&board.power_control);
    let pins_d = board.GPIO_PORTD.split(&board.power_control);
    let pins_b = board.GPIO_PORTB.split(&board.power_control);
    let pins_e = board.GPIO_PORTE.split(&board.power_control);

    let rs = pins_a.pa2.into_push_pull_output();
    let en = pins_d.pd6.into_push_pull_output();
    let b4 = pins_c.pc7.into_push_pull_output();
    let b5 = pins_c.pc6.into_push_pull_output();
    let b6 = pins_c.pc5.into_push_pull_output();
    let b7 = pins_c.pc4.into_push_pull_output();

    let r2 = pins_e.pe4.into_pull_up_input();
    let r3 = pins_b.pb1.into_pull_up_input();
    let r4 = pins_b.pb0.into_pull_up_input();

    let c2 = pins_a.pa5.into_push_pull_output();
    let c3 = pins_a.pa6.into_push_pull_output();
    let c4 = pins_a.pa7.into_push_pull_output();

    let keypad = keypad_new!(MyKeypad {
        rows: (r2, r3, r4),
        columns: (c2, c3, c4),
    });

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

    let keys = keypad.decompose();
    let first_key = &keys[0][0];
    if first_key.is_low().unwrap() {
        lcd.write_str("ifkl 1", &mut delay).unwrap();
    } else {
        lcd.write_str("ifkl 0", &mut delay).unwrap();
    }
    delay.delay_ms(2000u32);

    let mut buffer = [0u8; 10];

    loop {
        for (row_index, row) in keys.iter().enumerate() {
            for (col_index, key) in row.iter().enumerate() {
                if key.is_low().unwrap() {
                    lcd.clear(&mut delay).unwrap();
                    lcd.write_str("pd ", &mut delay).unwrap();
                    lcd.write_str(row_index.numtoa_str(10, &mut buffer), &mut delay)
                        .unwrap();
                    lcd.write_str(" ", &mut delay).unwrap();
                    lcd.write_str(col_index.numtoa_str(10, &mut buffer), &mut delay)
                        .unwrap();
                }
            }
        }
    }

    //lcd.write_str("2021-04", &mut delay).unwrap();
    //lcd.set_cursor_pos(40, &mut delay).unwrap();
    //lcd.write_str("KEYPAD", &mut delay).unwrap();

    //loop {
    //    board.led_green.set_high().unwrap();
    //    delay.delay_ms(500u32);
    //    board.led_green.set_low().unwrap();
    //    board.led_blue.set_high().unwrap();
    //    delay.delay_ms(500u32);
    //    board.led_blue.set_low().unwrap();
    //    delay.delay_ms(500u32);
    //}
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    board::panic();
}
