//! Startup / runtime for the Sony PlayStation 2.
//!
//! This crate provides routines to bootstrap a Rust environment for the PS2.
//!
//! Your main function needs to have the following declaration:
//! ```
//! #[no_mangle]
//! fn main() -> !;
//! ```

#![no_std]
#![deny(missing_docs)]
#![feature(asm_experimental_arch)]
#![feature(naked_functions)]
#![feature(strict_provenance)]

use core::panic::PanicInfo;
use core::fmt::Write;

use panic::panic_entrypoint;
use prussia_debug::EEOut;

use crate::exceptions::initialise_exception_vectors;

pub mod atomic;
pub mod cop0;
pub mod exceptions;
pub mod interrupts;
pub mod panic;

// Static data initialised to zero goes in the .bss segment, which is essentially a pointer to
// uninitialised memory. We need to zero .bss before the main program runs.
//
// This could be done in assembly (and is in PS2SDK), but I figured this was not speed-critical.
unsafe fn zero_bss() {
    // Due to a quirk of the linker, these symbols are given addresses, but there is no space
    // allocated for them. Instead the addresses *are* the values, in this case referring to the
    // start and end of .bss.
    extern "C" {
        static mut START_OF_BSS: u32;
        static mut END_OF_BSS: u32;
    }
    
    unsafe {
        let mut i = START_OF_BSS as *mut u32;
        let end = END_OF_BSS as *mut u32;

        while i < end {
            core::ptr::write_volatile(i, core::mem::zeroed());
            i = i.offset(1);
        }
    }
}

// A PS2 program's execution flow begins in _start, which calls the EE kernel to set up this thread
// and then calls _rust_start. Our job here is to provide a safe Rust startup and then call main().
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn _rust_start() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    writeln!(EEOut, "rt - Hello world!").unwrap();

    zero_bss();

    writeln!(EEOut, "rt - BSS zero-ed out.").unwrap();

    initialise_exception_vectors();

    writeln!(EEOut, "rt - Exc vectors set.").unwrap();

    main()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_entrypoint(info);
}
