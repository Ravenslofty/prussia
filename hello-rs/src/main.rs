#![no_std]
#![no_main]

use debug::println_ee;
use rt::cop0::Status;

extern crate prussia_debug as debug;
extern crate prussia_bios as bios;
extern crate prussia_dma as dma;
extern crate prussia_rt as rt;

#[no_mangle]
fn main() -> ! {

    println_ee!("main - Hello world!");

    let status = Status::load();

    println_ee!("main - Status: {status:?}");

    println_ee!("main - Testing overflow.");
    
    bios::set_gs_crt(bios::Interlacing::Noninterlaced, bios::VideoMode::Pal, bios::FieldFrameMode::Frame);

    println_ee!("main - Returned.");

    loop {}

    bios::exit(0);
}
