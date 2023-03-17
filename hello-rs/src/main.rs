#![no_std]
#![no_main]

extern crate panic_halt;
extern crate prussia_bios as bios;
extern crate prussia_dma as dma;
extern crate prussia_rt as rt;

#[no_mangle]
fn main() -> ! {
    bios::exit(0);
}
