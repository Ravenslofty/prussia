use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, thread::ThreadControlBlock};

/// Purposefully triggers an Instruction Load/Store exception for handler testing.
pub unsafe fn trigger_addrload_exception() {
    println_ee!("Triggering Instruction Load exception.");

    // let invalid_address: fn() = core::mem::transmute(0xA000_0000 as usize);

    // invalid_address();

    core::arch::asm!(
        "
        // Set the privile mode to user mode.
        mfc0 $t0, $12
        li $t1, 0x10
        or $t0, $t0, $t1
        mtc0 $t0, $12
        // Try to load a privileged address.
        li $t0, 0x80000008
        jr $t0
        nop"
    );

    println_ee!("Returned from exception handler.");
}

/// Address (instruction fetch/load) exception handler
#[no_mangle]
pub(super) extern "C" fn v_common_addr_load_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    let epc_addr = cop0_dump.epc;

    println_ee!("ADDRLOAD: Instruction Fetch/Load error encountered.");

    println_ee!("ADDRLOAD: Returning.");
}

/// Address (store) exception handler
#[no_mangle]
pub(super) extern "C" fn v_common_addr_store_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    let epc_addr = cop0_dump.epc;

    println_ee!("ADDRSTOR: Store error encountered.");

    println_ee!("ADDRSTOR: Returning.");
}
