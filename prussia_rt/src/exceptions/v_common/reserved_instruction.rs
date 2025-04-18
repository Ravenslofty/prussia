use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, increment_epc, thread::ThreadControlBlock};

/// Purposefully triggers a Reserved Instruction exception.
#[no_mangle]
pub unsafe fn trgger_reserved_instruction_handler() {
    println_ee!("Triggering Reserved Instruction exception.");

    core::arch::asm!(".word 0xDEADBEEF");

    println_ee!("Returned from exception.");
}

/// Reserved Instruction exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_reserved_instruction_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("RESINSTR: Reserved instruction error encountered.");

    increment_epc!();

    println_ee!("RESINSTR: Returning.");
}
