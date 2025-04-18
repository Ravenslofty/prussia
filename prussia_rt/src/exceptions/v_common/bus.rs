use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, thread::ThreadControlBlock};

/// Purposefully trigger a Bus Load exception.
pub unsafe fn trigger_bus_load_exception() {
    println_ee!("Triggering Instruction Load exception.");

    let addr = 0xA1000000 as *const u32;
    let val = core::ptr::read_volatile(addr);

    println_ee!("Returned from exception handler.");
}

/// Bus (instruction fetch/load) exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_bus_load_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("BUSLOAD: Instruction Fetch/Load error encountered.");

    println_ee!("BUSLOAD: Returning.");
}

/// Bus (store) exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_bus_store_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("BUSSTOR: Store error encountered.");

    println_ee!("BUSSTOR: Returning.");
}
