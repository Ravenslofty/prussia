#![no_std]
#![no_main]
extern crate prussia_rt as rt;
extern crate prussia_dma as dma;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
        loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    loop {}
}
