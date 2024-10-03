mod breakpoint;
mod overflow;
mod syscall;
mod address;
mod bus;
mod reserved_instruction;
mod cop_unusable;
mod trap;

use core::arch::asm;

use address::{v_common_addr_load_handler, v_common_addr_store_handler};
use breakpoint::v_common_breakpoint_handler;
use bus::{v_common_bus_load_handler, v_common_bus_store_handler};
use cop_unusable::v_common_cop_unusable_handler;
use overflow::v_common_overflow_handler;
use prussia_debug::println_ee;
use reserved_instruction::v_common_reserved_instruction_handler;
use syscall::v_common_syscall_handler;
use trap::v_common_trap_handler;

use crate::{cop0::{CoP0Dump, L1Exception}, thread::ThreadControlBlock};

pub use self::{
    address::trigger_addrload_exception,
    breakpoint::trigger_break_exception,
    bus::trigger_bus_load_exception,
    overflow::trigger_overflow_exception,
    reserved_instruction::trgger_reserved_instruction_handler,
    trap::trigger_trap_exception,
};

/// Address for the V_COMMON exception vector.
pub(super) const V_COMMON_EXCEPTION_VECTOR: usize = 0x8000_0180;
/// Address for the bootstrap V_COMMON exception vector.
pub(super) const V_COMMON_EXCEPTION_BOOTSTRAP_VECTOR: usize = 0xBFC0_0380;

/// Increments the EPC register by 1 instruction (4 bytes).
/// 
/// WARNING: This **DOES NOT** account for branch delay slots.
#[macro_export]
macro_rules! increment_epc {
    () => {
        unsafe {
            core::arch::asm!(
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
        V_COMMON_HANDLERS[4] = v_common_addr_load_handler as usize;
        V_COMMON_HANDLERS[5] = v_common_addr_store_handler as usize;
        V_COMMON_HANDLERS[6] = v_common_bus_load_handler as usize;
        V_COMMON_HANDLERS[7] = v_common_bus_store_handler as usize;
        V_COMMON_HANDLERS[8] = v_common_syscall_handler as usize;
        V_COMMON_HANDLERS[9] = v_common_breakpoint_handler as usize;
        V_COMMON_HANDLERS[10] = v_common_reserved_instruction_handler as usize;
        V_COMMON_HANDLERS[11] = v_common_cop_unusable_handler as usize;
        V_COMMON_HANDLERS[12] = v_common_overflow_handler as usize;
        V_COMMON_HANDLERS[13] = v_common_trap_handler as usize;
    }
}

#[no_mangle]
pub(super) extern "C" fn unimplemented_v_common_handler(tcb_ptr: *mut ThreadControlBlock) {
    println_ee!("EXCUNKWN: Unimplemented/unrecognised V_COMMON exception!");

    let cop0_dump = CoP0Dump::load();

    println_ee!("EXCUNKWN: CoP0 registers: {cop0_dump:#?}");

    if let Some(tcb) = unsafe {tcb_ptr.as_ref()} {
        println_ee!("EXCUNKWN: TCB at time of exception: {tcb:#?}");
    } else {
        println_ee!("EXCUNKWN: No TCB information available.");
    };

    match L1Exception::try_from_common_cause(cop0_dump.cause) {
        Some(exc_code) => println_ee!("EXCUNKWN: Exception cause: {:?}", exc_code),
        None => println_ee!("EXCUNKWN: UNKNOWN CAUSE CODE. Cause: {:?}.", cop0_dump.cause),
    }

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
