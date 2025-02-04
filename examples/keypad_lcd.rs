#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate keypad;

extern crate embedded_hal;
extern crate hd44780_driver;
//extern crate numtoa;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use core::alloc::Layout;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
//use numtoa::NumToA;
use stellaris_launchpad::board;
use tm4c123x_hal::gpio::GpioExt;

use tm4c123x_hal::gpio::{
    gpioa::{PA2, PA5, PA6, PA7},
    gpiob::{PB0, PB1, PB4},
    gpioc::{PC4, PC5, PC6, PC7},
    gpiod::PD6,
    gpioe::{PE4, PE5},
};
use tm4c123x_hal::gpio::{Input, Output, PullUp, PushPull};

keypad_struct! {
    struct MyKeypad {
        rows: (
            PE5<Input<PullUp>>,
            PE4<Input<PullUp>>,
            PB1<Input<PullUp>>,
            PB0<Input<PullUp>>,
        ),
        columns: (
            PB4<Output<PushPull>>,
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

    let r1 = pins_e.pe5.into_pull_up_input();
    let r2 = pins_e.pe4.into_pull_up_input();
    let r3 = pins_b.pb1.into_pull_up_input();
    let r4 = pins_b.pb0.into_pull_up_input();

    let c1 = pins_b.pb4.into_push_pull_output();
    let c2 = pins_a.pa5.into_push_pull_output();
    let c3 = pins_a.pa6.into_push_pull_output();
    let c4 = pins_a.pa7.into_push_pull_output();

    let keypad = keypad_new!(MyKeypad {
        rows: (r1, r2, r3, r4),
        columns: (c1, c2, c3, c4),
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

    //let keys = keypad.decompose();
    //let first_key = &keys[0][0];
    //if first_key.is_low().unwrap() {
    //    lcd.write_str("ifkl 1", &mut delay).unwrap();
    //} else {
    //    lcd.write_str("ifkl 0", &mut delay).unwrap();
    //}
    //delay.delay_ms(1000u32);

    loop {
        lcd.clear(&mut delay).unwrap();

        let from_file = get_chess_file(&keypad);
        lcd_write_file(from_file, &mut lcd, &mut delay);
        let from_rank = get_chess_rank(&keypad);
        lcd_write_rank(from_rank, &mut lcd, &mut delay);

        let to_file = get_chess_file(&keypad);
        lcd_write_file(to_file, &mut lcd, &mut delay);
        let to_rank = get_chess_rank(&keypad);
        lcd_write_rank(to_rank, &mut lcd, &mut delay);

        lcd.set_cursor_pos(40, &mut delay).unwrap();
        lcd.write_str("Done cycle", &mut delay).unwrap();
        board.led_blue.set_high().unwrap();
        delay.delay_ms(500u32);
        board.led_blue.set_low().unwrap();
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

fn lcd_write_file(
    file: i32,
    lcd: &mut HD44780<
        hd44780_driver::bus::FourBitBus<
            PA2<Output<PushPull>>,
            PD6<Output<PushPull>>,
            PC7<Output<PushPull>>,
            PC6<Output<PushPull>>,
            PC5<Output<PushPull>>,
            PC4<Output<PushPull>>,
        >,
    >,
    delay: &mut tm4c123x_hal::delay::Delay,
) {
    match file {
        0 => lcd.write_char('a', delay).unwrap(),
        1 => lcd.write_char('b', delay).unwrap(),
        2 => lcd.write_char('c', delay).unwrap(),
        3 => lcd.write_char('d', delay).unwrap(),
        4 => lcd.write_char('e', delay).unwrap(),
        5 => lcd.write_char('f', delay).unwrap(),
        6 => lcd.write_char('g', delay).unwrap(),
        7 => lcd.write_char('h', delay).unwrap(),
        _ => {
            lcd.write_str("Unreachable", delay).unwrap();
        }
    }
}

fn lcd_write_rank(
    rank: i32,
    lcd: &mut HD44780<
        hd44780_driver::bus::FourBitBus<
            PA2<Output<PushPull>>,
            PD6<Output<PushPull>>,
            PC7<Output<PushPull>>,
            PC6<Output<PushPull>>,
            PC5<Output<PushPull>>,
            PC4<Output<PushPull>>,
        >,
    >,
    delay: &mut tm4c123x_hal::delay::Delay,
) {
    match rank {
        0 => lcd.write_char('1', delay).unwrap(),
        1 => lcd.write_char('2', delay).unwrap(),
        2 => lcd.write_char('3', delay).unwrap(),
        3 => lcd.write_char('4', delay).unwrap(),
        4 => lcd.write_char('5', delay).unwrap(),
        5 => lcd.write_char('6', delay).unwrap(),
        6 => lcd.write_char('7', delay).unwrap(),
        7 => lcd.write_char('8', delay).unwrap(),
        _ => {
            lcd.write_str("Unreachable", delay).unwrap();
        }
    }
}

fn get_chess_file(keypad: &MyKeypad) -> i32 {
    // row column - file - ret
    // 33 a 0; 32 b 1; 31 c 2; 30 d 3
    // 23 e 4; 22 f 5; 21 g 6; 20 h 7
    let keys = keypad.decompose();
    loop {
        for (row_index, row) in keys.iter().enumerate() {
            for (col_index, key) in row.iter().enumerate() {
                if key.is_low().unwrap() {
                    match (row_index, col_index) {
                        (3, 3) => return 0,
                        (3, 2) => return 1,
                        (3, 1) => return 2,
                        (3, 0) => return 3,
                        (2, 3) => return 4,
                        (2, 2) => return 5,
                        (2, 1) => return 6,
                        (2, 0) => return 7,
                        (_, _) => continue,
                    };
                }
            }
        }
    }
}

fn get_chess_rank(keypad: &MyKeypad) -> i32 {
    // row column - rank - ret
    // 13 1 0; 12 2 1; 11 3 2; 10 4 3
    // 03 5 4; 02 6 5; 01 7 6; 00 8 7
    let keys = keypad.decompose();
    loop {
        for (row_index, row) in keys.iter().enumerate() {
            for (col_index, key) in row.iter().enumerate() {
                if key.is_low().unwrap() {
                    match (row_index, col_index) {
                        (1, 3) => return 0,
                        (1, 2) => return 1,
                        (1, 1) => return 2,
                        (1, 0) => return 3,
                        (0, 3) => return 4,
                        (0, 2) => return 5,
                        (0, 1) => return 6,
                        (0, 0) => return 7,
                        (_, _) => continue,
                    };
                }
            }
        }
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    board::panic();
}
