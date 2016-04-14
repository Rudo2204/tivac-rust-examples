//! A blinky-LED example application
//! This example uses Primer, a library for simple bare-metal ARM programming.

#![no_std]
#![no_main]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate primer;

use primer::gpio;
use primer::launchpad;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

pub fn led_on() {
    gpio::set(launchpad::LED_RED, gpio::Level::High);
}

pub fn led_off() {
    gpio::set(launchpad::LED_RED, gpio::Level::Low);
}

#[no_mangle]
pub extern "C" fn primer_start() {
    launchpad::init();
    loop {
        led_on();
        primer::delay(250);
        led_off();
        primer::delay(250);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
