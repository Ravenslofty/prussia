use prussia_debug::println_ee;

use crate::{cop0::CoP0Dump, thread::ThreadControlBlock};

/// Purposefully trigger a TLB Modified exception.
pub unsafe fn trigger_tlb_modified_exception() {
    println_ee!("Triggering TLB Modified exception.");

    core::arch::asm!("
        # First prepare a TLB entry by loading a dummy address.
        lui $k0, 0x1234
        lw  $k1, 0($k0)

        # Next, update the TLB entry and then write to the virtual address.
        # This should trigger the exception.
        tlbp
        nop
        tlbwi
        sw  $zero, 0($k0)
    ");

    println_ee!("Returned from exception.");
}

/// TLB Modified exception handler.
#[no_mangle]
pub(super) extern "C" fn v_common_tlb_modified_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("TLBMODIFIED: TLB Modified exception encountered.");

    println_ee!("TLBMODIFIED: Returning.");
}
