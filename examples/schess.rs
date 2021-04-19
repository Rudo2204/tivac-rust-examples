#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate chess_engine;
extern crate embedded_hal;
extern crate hd44780_driver;
extern crate numtoa;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use chess_engine::*;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use numtoa::NumToA;
use stellaris_launchpad::board;
use tm4c123x_hal::gpio::GpioExt;

use core::alloc::Layout;

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

    let chess_board = BoardBuilder::default()
        .piece(Piece::Rook(BLACK, A8))
        .piece(Piece::Knight(BLACK, B8))
        .piece(Piece::Bishop(BLACK, C8))
        .piece(Piece::Queen(BLACK, D8))
        .piece(Piece::King(BLACK, E8))
        .piece(Piece::Bishop(BLACK, F8))
        .piece(Piece::Knight(BLACK, G8))
        .piece(Piece::Rook(BLACK, H8))
        .row(Piece::Pawn(BLACK, A7))
        .piece(Piece::Pawn(WHITE, A2))
        .piece(Piece::Pawn(WHITE, B2))
        .piece(Piece::Pawn(WHITE, C2))
        .piece(Piece::Pawn(WHITE, D4))
        .piece(Piece::Pawn(WHITE, E2))
        .piece(Piece::Pawn(WHITE, F2))
        .piece(Piece::Pawn(WHITE, G2))
        .piece(Piece::Pawn(WHITE, H2))
        .piece(Piece::Rook(WHITE, A1))
        .piece(Piece::Knight(WHITE, B1))
        .piece(Piece::Bishop(WHITE, C1))
        .piece(Piece::Queen(WHITE, D1))
        .piece(Piece::King(WHITE, E1))
        .piece(Piece::Bishop(WHITE, F1))
        .piece(Piece::Knight(WHITE, G1))
        .piece(Piece::Rook(WHITE, H1))
        .enable_castling()
        .build()
        .change_turn();

    lcd.write_str("Evaluating...", &mut delay).unwrap();
    let (_cpu_move, count, _) = chess_board.get_best_next_move(2);
    let mut buffer = [0u8; 10];

    lcd.clear(&mut delay).unwrap();
    lcd.write_str("Finished!", &mut delay).unwrap();
    lcd.set_cursor_pos(40, &mut delay).unwrap();
    lcd.write_str(count.numtoa_str(10, &mut buffer), &mut delay)
        .unwrap();
    lcd.write_str("s evald", &mut delay).unwrap();

    loop {
        board.led_red.set_high().unwrap();
        delay.delay_ms(500u32);
        board.led_red.set_low().unwrap();
        board.led_green.set_high().unwrap();
        delay.delay_ms(500u32);
        board.led_green.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    board::panic();
}
