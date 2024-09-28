use core::arch::asm;

use prussia_debug::println_ee;

use crate::{
    cop0::{Cause, CoP0Dump, L1Exception},
    exceptions::EXCEPTION_HANDLER_TABLE, thread::ThreadControlBlock,
};

/// Address for the V_COMMON exception vector.
pub(super) const V_COMMON_EXCEPTION_VECTOR: usize = 0x8000_0180;
/// Address for the bootstrap V_COMMON exception vector.
pub(super) const V_COMMON_EXCEPTION_BOOTSTRAP_VECTOR: usize = 0xBFC0_0380;

macro_rules! increment_epc {
    () => {
        unsafe {
            asm!(
                "
                mfc0 $k1, $14
                addiu $k1, 4
                mtc0 $k1, $14
                "
            )
        }
    };
}

/// Contains 16 function vectors to V_COMMON exception code handlers.
#[no_mangle]
pub(super) static mut V_COMMON_HANDLERS: [usize; 16] = [0; 16];

pub(super) fn init_v_common_handlers_table() {
    for x in unsafe { V_COMMON_HANDLERS.iter_mut() } {
        *x = unimplemented_v_common_handler as usize;
    }

    unsafe {
        V_COMMON_HANDLERS[8] = v_common_syscall_handler as usize;
        V_COMMON_HANDLERS[9] = v_common_breakpoint_handler as usize;
        V_COMMON_HANDLERS[12] = v_common_overflow_handler as usize;
    }
}

/// Purposefully trigger a break exception. For testing purposes.
pub fn trigger_break_exception() {
    extern "C" {
        fn _trigger_break_exception();
    }

    println_ee!(
        "Triggering break exception, which should call the handler at {:0>8p}",
        EXCEPTION_HANDLER_TABLE[0].handler
    );
    unsafe { _trigger_break_exception() };
    println_ee!("Returned from break exception.");
}

/// Purposefully trigger an overflow exception. For testing purposes.
pub fn trigger_overflow_exception() {
    println_ee!("Triggering overflow.");

    unsafe {
        asm!(
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

#[no_mangle]
pub(super) extern "C" fn unimplemented_v_common_handler() {
    println_ee!("Unimplemented v_common exception!");

    let cop0_dump = CoP0Dump::load();

    println_ee!("CoP0 registers: {cop0_dump:#?}");

    let Some(exc_code) = L1Exception::try_from_common_cause(cop0_dump.cause) else {
        println_ee!("UNKNOWN CAUSE CODE. RETURNING PREMATURELY.");
        return;
    };

    println_ee!("Exception cause: {:?}", exc_code);

    loop {}

    unreachable!("Unhandled exceptions should not reach this point.");
}

/// First level of exception-handling. Trampolines to the main exception handler.
#[naked]
pub(super) unsafe extern "C" fn _v_common_exception_vec() {
    asm! {
        ".set noreorder
         .set noat

         la   $k0, _v_common_exception_handler
         jr   $k0
         nop

         .set at
         .set reorder",
        options(noreturn)
    }
}

/// Second level of exception-handling. Load arguments, call the main handler,
/// then load and jump to the main return address.
#[naked]
#[no_mangle]
unsafe extern "C" fn _v_common_exception_handler() {
    asm! {
        ".set noreorder
         .set noat

         // Allocate space for a TCB debug struct and save address.
         addi $sp, $sp, -0x100
         sd   $a0, 48($sp)
         move $a0, $sp

         // Fill TCB struct.
         sd      $at, 0($a0)            // Save $at
         sd      $v0, 8($a0)            // Save $v0
         sd      $v1, 16($a0)           // Save $v1
         sd      $a0, 24($a0)           // Save $a0
         sd      $a1, 32($a0)           // Save $a1
         sd      $a2, 40($a0)           // Save $a2
         sd      $a3, 48($a0)           // Save $a3
         sd      $t0, 56($a0)           // Save $t0
         sd      $t1, 64($a0)           // Save $t1
         sd      $t2, 72($a0)           // Save $t2
         sd      $t3, 80($a0)           // Save $t3
         sd      $t4, 88($a0)           // Save $t4
         sd      $t5, 96($a0)           // Save $t5
         sd      $t6, 104($a0)          // Save $t6
         sd      $t7, 112($a0)          // Save $t7
         sd      $s0, 120($a0)          // Save $s0
         sd      $s1, 128($a0)          // Save $s1
         sd      $s2, 136($a0)          // Save $s2
         sd      $s3, 144($a0)          // Save $s3
         sd      $s4, 152($a0)          // Save $s4
         sd      $s5, 160($a0)          // Save $s5
         sd      $s6, 168($a0)          // Save $s6
         sd      $s7, 176($a0)          // Save $s7
         sd      $t8, 184($a0)          // Save $t8
         sd      $t9, 192($a0)          // Save $t9
         sd      $k0, 200($a0)          // Save $k0
         sd      $k1, 208($a0)          // Save $k1
         sd      $gp, 216($a0)          // Save $gp
         sd      $sp, 224($a0)          // Save $sp
         sd      $fp, 232($a0)          // Save $fp
         sd      $ra, 240($a0)          // Save $ra

         // Handler jump section.
         mfc0    $k0, $13
         andi    $k0, 0x3F

         addi    $sp, -20
         sw      $ra, 0($sp)
         sw      $t0, 16($sp)

         la      $t0, V_COMMON_HANDLERS  // Load start of handler table.
         add     $k0, $t0                // Add ExcCode to handler table.

         lw      $t0, 16($sp)            // Restore $t0 from stack.

         lw      $k0, 0($k0)             // Get the handler address.
         jalr    $k0                     // Jump to the handler.
         nop

         lw      $ra, 0($sp)             // Restore $ra from stack.
         addi    $sp, 20                 // Pop stack, undo handler.

         // Pop TCB struct.
         addi    $sp, 0x100

         .int 0x0000000f  // Sync all pending instructions.
         .int 0x42000018  // Return from the exception.

         .set at
         .set reorder",
         options(noreturn)
    }
}

/// Breakpoint exception handler
#[no_mangle]
pub(super) extern "C" fn v_common_breakpoint_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    println_ee!("BREAK: Encountered breakpoint!");

    increment_epc!()
}

// FIXME: Overflow handler could not be verified as working. Retest when possible.
/// Overflow exception handler
#[no_mangle]
pub(super) extern "C" fn v_common_overflow_handler(tcb_ptr: *mut ThreadControlBlock) {
    let cop0_dump = CoP0Dump::load();

    let epc_addr = cop0_dump.epc;
    let erroring_addr = unsafe {epc_addr.as_raw_ptr().offset(-1)};

    println_ee!("OVERFLOW: Overflow occurred! Erroring address: {erroring_addr:?}, returning address: {epc_addr:?}");
}

/// Syscall exception handler
#[no_mangle]
pub(super) extern "C" fn v_common_syscall_handler(tcb_ptr: *mut ThreadControlBlock) {
    const SYSCALL_CODE_FIELD_MASK: u32 = 0x3FF_FFC0;

    let cop0_dump = CoP0Dump::load();

    let epc_addr = cop0_dump.epc.as_raw_ptr();
    let in_delay_slot = cop0_dump.cause.intersection(Cause::BD) == Cause::BD;
    let syscall_addr = if in_delay_slot {
        unsafe {
            epc_addr.offset(1)
        }
    } else {
        epc_addr
    };

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
