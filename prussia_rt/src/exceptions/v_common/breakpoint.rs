use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, increment_epc, thread::ThreadControlBlock};

/// Purposefully trigger a break exception. For testing purposes.
pub fn trigger_break_exception() {
    extern "C" {
        fn _trigger_break_exception();
    }

    println_ee!("Triggering break exception.");

    unsafe { _trigger_break_exception() };
    
    println_ee!("Returned from break exception.");
}

/// Breakpoint exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_breakpoint_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("BREAK: Encountered breakpoint!");

    if let Some(tcb) = unsafe {tcb_ptr.as_ref()} {
        println_ee!("BREAK: TCB at time of exception: {tcb:#?}");
    } else {
        println_ee!("BREAK: No TCB information available.");
    };

    increment_epc!()
}