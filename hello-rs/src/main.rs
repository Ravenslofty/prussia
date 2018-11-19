#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;
extern crate prussia_dma as dma;
extern crate prussia_rt as rt;

#[no_mangle]
fn main() -> ! {
    unsafe {
        asm!("li $$3, 4; move $$4, $$0; syscall");
    }
    unreachable!("The syscall should not return");
}
