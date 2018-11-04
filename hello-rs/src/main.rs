#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
        loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    loop {}
}
