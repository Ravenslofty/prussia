//! Routines for BIOS debug output.
#![no_std]
#![deny(missing_docs)]

use core::{fmt, ptr};

/// A zero-sized type for printing to the EE debug console.
pub struct EEOut;

static mut EE_DEBUG_OUT: *mut u8 = 0x1000_f180 as *mut u8;

impl fmt::Write for EEOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            unsafe {
                ptr::write_volatile(EE_DEBUG_OUT, c);
            }
        }
        Ok(())
    }
}

/// Print to the [EEOut] serial interface.
#[macro_export]
macro_rules! print_ee {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!(prussia_debug::EEOut, "{}", format_args!($($arg)*)).unwrap();
        }
    };
}

/// Print lines to the [EEOut] serial interface.
#[macro_export]
macro_rules! println_ee {
    () => {
        prussia_debug::print_ee!("\n")
    };
    ($($arg:tt)*) => (prussia_debug::print_ee!("{}\n", format_args!($($arg)*)));
}
