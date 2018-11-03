//! Startup / runtime for the Sony PlayStation 2.
//!
//! This crate provides routines to bootstrap a Rust environment for the PS2.

#![deny(missing_docs)]
#![no_std]

// A PS2 program's execution flow begins in _start, which calls the EE kernel to set up this thread
// and then calls _rust_start. Our job here is to provide a safe Rust startup and then call main().
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn _rust_start() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    main()
}
