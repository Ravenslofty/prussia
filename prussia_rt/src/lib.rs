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

use core::panic::PanicInfo;

use panic::panic_entrypoint;

pub mod atomic;
pub mod cop0;
pub mod interrupts;
mod panic;

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

    r0::zero_bss::<u32>(&mut START_OF_BSS as *mut u32, &mut END_OF_BSS as *mut u32);
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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_entrypoint(info);
}
