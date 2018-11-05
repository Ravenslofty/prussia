//! Startup / runtime for the Sony PlayStation 2.
//!
//! This crate provides routines to bootstrap a Rust environment for the PS2.

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![no_std]

use core::ptr;

// Static data initialised to zero goes in the .bss segment, which is essentially a pointer to
// uninitialised memory. We need to zero .bss before the main program runs.
//
// This could be done in assembly (and is in PS2SDK), but I figured this was not speed-critical.
unsafe fn zero_bss() {
    // Due to a quirk of the linker, these symbols are given addresses, but there is no space
    // allocated for them. Instead the addresses *are* the values, in this case referring to the
    // start and end of .bss.
    extern "C" {
        static mut START_OF_BSS: u8;
        static END_OF_BSS: u8;
    }

    let start = &START_OF_BSS as *const u8 as usize;
    let end = &END_OF_BSS as *const u8 as usize;
    let size = end - start;
    ptr::write_bytes(&mut START_OF_BSS as *mut u8, 0, size);
}

// A PS2 program's execution flow begins in _start, which calls the EE kernel to set up this thread
// and then calls _rust_start. Our job here is to provide a safe Rust startup and then call main().
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn _rust_start() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    zero_bss();

    main()
}
