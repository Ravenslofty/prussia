use prussia_debug::println_ee;

use crate::{cop0::{Cause, CoP0Dump}, increment_epc, thread::ThreadControlBlock};

/// Syscall exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_syscall_handler(tcb_ptr: *mut ThreadControlBlock) {
    const SYSCALL_CODE_FIELD_MASK: u32 = 0x3FF_FFC0;

    let cop0_dump = CoP0Dump::load();

    let in_delay_slot = cop0_dump.cause.intersects(Cause::BD);
    let syscall_addr = cop0_dump.returning_addr();
    let epc_addr = cop0_dump.epc;

    let syscall_code_field = (syscall_addr as u32) & SYSCALL_CODE_FIELD_MASK >> 6;

    println_ee!("SYSCALL: Syscall triggered. In branch delay slot: {in_delay_slot}");
    println_ee!("SYSCALL: EPC address: {epc_addr:?}, instruction address: {syscall_addr:?}, code field: {syscall_code_field:#0x}");

    if let Some(tcb) = unsafe {tcb_ptr.as_ref()} {
        println_ee!("SYSCALL: TCB at time of exception: {tcb:#?}");
    } else {
        println_ee!("SYSCALL: No TCB information available.");
    };

    increment_epc!()
}
