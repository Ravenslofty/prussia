//! A PlayStation 2 BIOS written in Rust.

#![no_std]
#![no_main]
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts
)]
#![feature(asm)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(global_asm)]
#![feature(naked_functions)]

extern crate prussia_dma as dma;
extern crate prussia_intc as intc;

use core::fmt::Write;
use core::panic::PanicInfo;

use prussia_debug::EEOut;
use prussia_rt::{cop0, interrupts};

mod exceptions;
mod romdir;
mod thread;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Write the panic message to EE output.
    let mut out = EEOut;
    writeln!(out, "[EE] {}", info).unwrap();

    // Then crash to trigger the emulator.
    unsafe { asm!("break") };
    unreachable!();
}

#[no_mangle]
fn main() -> ! {
    let mut out = EEOut;
    writeln!(out, "[EE] Hello, world!").unwrap();

    // Disable spurious interrupts while setting up.
    interrupts::disable();

    // Stop interrupts from the interrupt controller.
    // Remember that writing 1 to a mask toggles it; so writing it to itself clears it.
    intc::Mask::load().store();

    // And from the DMA controller.
    dma::Status::load().store();

    // Set up the exception tables.
    exceptions::init();

    // Switch to our exception tables.
    let mut status = cop0::Status::load();
    status &= !cop0::Status::BEV;
    status.store();

    // Enable interrupts now that we've set everything up.
    interrupts::enable();

    writeln!(out, "[EE] Init OK").unwrap();

    // Fetch the payload address.
    let elf_addr = romdir::lookup(b"PILLGEN\0\0\0").expect("Failed to find PILLGEN in ROM");

    panic!("Found ELF in ROM at {}", elf_addr);
}
