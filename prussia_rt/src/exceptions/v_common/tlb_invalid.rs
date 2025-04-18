use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, thread::ThreadControlBlock};

/// TLB Invalid (Instruction fetch / load) exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_tlb_invalid_load_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("TLBINVLOAD: TLB Load Invalid exception encountered.");

    println_ee!("TLBINVLOAD: Returning.");
}

/// TLB Invalid (Instruction store) exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_tlb_invalid_store_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("TLBINVSTOR: TLB Store Invalid exception encountered.");

    println_ee!("TLBINVSTOR: Returning.");
}
