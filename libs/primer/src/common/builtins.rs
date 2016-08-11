//! Functions required by the code rustc/LLVM generates.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use rlibc;
use core;
// Doing board specific things here is bad
use board::launchpad as board;

// ****************************************************************************
//
// Public Types
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
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Required by the compiler.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> ! {
    board::panic();
}

/// Required by the compiler.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> ! {
    board::panic();
}

/// Required by modules that haven't been build with panic = "abort"
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    board::panic();
}

/// Required by the compiler.
#[lang="eh_personality"]
extern "C" fn eh_personality() {}

/// Entry point of panic from the libcore crate.
///
/// Required by the compiler.
#[lang="panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    board::panic();
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
