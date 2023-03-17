//! A PlayStation 2 BIOS written in Rust.

#![no_std]
#![no_main]
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts
)]
#![feature(asm_experimental_arch)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

extern crate prussia_dma as dma;
extern crate prussia_intc as intc;

use core::fmt::Write;
use core::intrinsics;
use core::mem;
use core::panic::PanicInfo;
use core::ptr;
use core::slice;
use core::str;

use prussia_debug::EEOut;
use prussia_rt::{cop0, interrupts};
use xmas_elf::ElfFile;

mod cache;
mod exceptions;
mod gs;
mod romdir;
mod thread;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Write the panic message to EE output.
    let mut out = EEOut;
    writeln!(out, "[EE] {}", info).unwrap();

    // Then crash to trigger the emulator.
    // This gets converted to `break` by LLVM.
    intrinsics::abort();
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
    let (addr, entry) = romdir::lookup(b"PILLGEN\0\0\0").expect("Failed to find PILLGEN in ROM");

    // Load the ELF file.
    writeln!(
        out,
        "[EE] Found {} at {:x}",
        str::from_utf8(&entry.name).unwrap(),
        addr
    )
    .unwrap();

    let data = unsafe { slice::from_raw_parts(addr as *mut u8, entry.size as usize) };
    let elf = ElfFile::new(data).expect("ELF should be valid. This indicates a broken ROM build.");

    for header in elf.program_iter() {
        unsafe {
            ptr::copy_nonoverlapping(
                data[header.offset() as usize..].as_ptr(),
                header.physical_addr() as usize as *mut u8,
                header.file_size() as usize,
            );
            writeln!(
                out,
                "[EE] copied {} bytes from {:x} to {:x}",
                header.file_size(),
                addr + (header.offset() as usize),
                header.physical_addr()
            )
            .unwrap();
        }
    }

    let elf_entry = elf.header.pt2.entry_point() as usize;

    writeln!(out, "[EE] Entry point is {:x}", elf_entry).unwrap();

    let elf_entry: fn() = unsafe { mem::transmute(elf_entry as *mut u8) };

    writeln!(out, "[EE] Launching payload").unwrap();

    elf_entry();

    unreachable!("Somehow returned from entry point");
}
