#![no_std]
#![no_main]

extern crate panic_halt;
extern crate prussia_bios as bios;
extern crate prussia_debug as debug;
extern crate prussia_dma as dma;
extern crate prussia_rt as rt;

use core::fmt::Write;

use debug::EEOut;
use rt::cop0::{BadVAddr, TimerCount};

#[no_mangle]
fn main() -> ! {
    writeln!(EEOut, "Testing testing 123.").unwrap();

    let bad_v_addr = BadVAddr::load();

    writeln!(EEOut, "Received this bad address: {bad_v_addr:?}").unwrap();

    let timer_count = TimerCount::working_load();
    let other_timer_count = TimerCount::working_load();

    writeln!(EEOut, "Received this TimerCount: {timer_count:?}").unwrap();

    let yet_other_timer_count = TimerCount::failing_load();

    writeln!(EEOut, "Received this TimerCount: {yet_other_timer_count:?}").unwrap();

    let also_other_timer_count = TimerCount::working_load();

    writeln!(
        EEOut,
        "Received this TimerCount: {also_other_timer_count:?}"
    )
    .unwrap();

    loop {}

    bios::exit(0);
}
