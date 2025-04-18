use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, thread::ThreadControlBlock};

/// Coprocessor unusable exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_cop_unusable_handler(tcb_ptr: *mut ThreadControlBlock) {
    // cop0::Status::load();
    let cop0_dump = CoP0Dump::load();

    println_ee!("COPXUNUS: Exception encountered trying to use unusable Coprocessor.");

    println_ee!("COPXUNUS: Returning.");
}
