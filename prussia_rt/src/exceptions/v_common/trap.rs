use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, thread::ThreadControlBlock};

/// Trigger Trap exception.
pub unsafe fn trigger_trap_exception() {
    println_ee!("Triggering trap.");

    core::arch::asm!("
        li $t1, 0x5
        li $t2, 0x5
        teq $t1, $t2
    ");

    println_ee!("Returned from exception.");
}

/// Trap exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_trap_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("TRAP: Trap instruction evaluated to true.");

    println_ee!("TRAP: Returning.");
}
