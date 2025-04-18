use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, thread::ThreadControlBlock};

/// Purposefully trigger an overflow exception. For testing purposes.
pub fn trigger_overflow_exception() {
    println_ee!("Triggering overflow.");

    unsafe {
        core::arch::asm!(
            "
            # Trigger overflow
            li $t2, 0x7FFFFFFF
            li $t3, 0x7FFFFFFF
            add $t4, $t2, $t3
            "
        )
    }

    println_ee!("Exited overflow.");
}

// FIXME: Overflow handler could not be verified as working. Retest when possible.
/// Overflow exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_overflow_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    let epc_addr = cop0_dump.returning_addr();

    println_ee!("OVERFLOW: Overflow occurred! Erroring address: {epc_addr:?}");

    if let Some(tcb) = unsafe {tcb_ptr.as_ref()} {
        println_ee!("OVERFLOW: TCB at time of exception: {tcb:#?}");
    } else {
        println_ee!("OVERFLOW: No TCB information available.");
    };
}