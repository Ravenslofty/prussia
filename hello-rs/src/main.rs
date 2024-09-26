#![no_std]
#![no_main]

use core::fmt::Write;
use debug::EEOut;
use rt::cop0::Status;

extern crate prussia_debug as debug;
extern crate prussia_bios as bios;
extern crate prussia_dma as dma;
extern crate prussia_rt as rt;

#[no_mangle]
fn main() -> ! {

    writeln!(EEOut, "main - Hello world!").unwrap();

    let status = Status::load();

    writeln!(EEOut, "main - Status: {status:?}").unwrap();

    loop {}

    bios::exit(0);
}
