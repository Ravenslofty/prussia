//! Functions for initialising the exception vectors and handling exceptions.

use core::{arch::asm, fmt::Write};

use prussia_debug::EEOut;

use crate::cop0::{Cause, CoP0Dump, L1Exception};

/// Address for the V_COMMON exception vector.
pub const V_COMMON_EXCEPTION_VECTOR: usize = 0x8000_0180;
/// Address for the bootstrap V_COMMON exception vector.
pub const V_COMMON_EXCEPTION_BOOTSTRAP_VECTOR: usize = 0xBFC0_0380;

/// Address pointer to an EE-side exception handler.
pub struct EEExceptionVector {
    /// Destination address for the vector.
    pub location: usize,
    /// Associated function pointer to put at the vector.
    pub handler: unsafe extern "C" fn(),
}

/// Purposefully trigger a break exception. For testing purposes.
pub fn trigger_break_exception() {
    extern "C" {
        fn _trigger_break_exception();
    }

    writeln!(
        EEOut,
        "Triggering break exception, which should call the handler at {:0>8p}",
        EXCEPTION_HANDLER_TABLE[0].handler
    )
    .unwrap();
    unsafe { _trigger_break_exception() };
    writeln!(EEOut, "Returned from break exception.").unwrap();
}

/// Vector to the V_COMMON exception handler.
pub const EXCEPTION_HANDLER_TABLE: [EEExceptionVector; 2] = [
    EEExceptionVector {
        location: V_COMMON_EXCEPTION_VECTOR,
        handler: _v_common_exception_vec,
    },
    EEExceptionVector {
        location: V_COMMON_EXCEPTION_BOOTSTRAP_VECTOR,
        handler: _v_common_exception_vec,
    },
];

/// Contains 16 function vectors to V_COMMON exception code handlers.
/// Initialise by running [initialise_exception_vectors].
#[no_mangle]
static mut V_COMMON_HANDLERS: [usize; 16] = [0; 16];

/// Load all exception vectors into their respective destinations.
pub fn initialise_exception_vectors() {
    for vector in &EXCEPTION_HANDLER_TABLE {
        unsafe {
            core::intrinsics::volatile_copy_nonoverlapping_memory(
                vector.location as *mut u32,
                vector.handler as *const u32,
                4,
            );
        };
    }
    writeln!(EEOut, "Exception vectors loaded.").unwrap();

    for x in unsafe { V_COMMON_HANDLERS.iter_mut() } {
        *x = unimplemented_v_common_handler as usize;
    }

    unsafe {
        V_COMMON_HANDLERS[9] = v_common_breakpoint_handler as usize;
    }
}

#[no_mangle]
extern "C" fn unimplemented_v_common_handler() {
    let cause = Cause::load();
    let Some(exc_code) = L1Exception::try_from_common_cause(cause) else {
        panic!("Unexpected exception code found in Cause register ({cause:?})");
    };
    unimplemented!("Unhandled V_COMMON exception. Code: {exc_code}");
}

/// First level of exception-handling. Trampolines to the main exception handler.
#[naked]
pub unsafe extern "C" fn _v_common_exception_vec() {
    asm! {
        ".set noreorder
         .set noat

         la $k0, _v_common_exception_handler
         jr $k0
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
pub unsafe extern "C" fn _v_common_exception_handler() {
    asm! {
        ".set noreorder
         .set noat

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
         addi    $sp, 20                 // Pop stack.

         .int 0x0000000f  // Sync all pending instructions.
         .int 0x42000018  // Return from the exception.

         .set at
         .set reorder",
         options(noreturn)
    }
}

#[no_mangle]
extern "C" fn v_common_breakpoint_handler() {
    let cop0_dump = CoP0Dump::load();

    writeln!(EEOut, "CoP0 registers: {cop0_dump:#?}").unwrap();

    let Some(exc_code) = L1Exception::try_from_common_cause(cop0_dump.cause) else {
        writeln!(EEOut, "UNKNOWN CAUSE CODE. RETURNING PREMATURELY.").unwrap();
        return;
    };

    writeln!(EEOut, "Exception cause: {:?}", exc_code).unwrap();
    match exc_code {
        L1Exception::Breakpoint | L1Exception::ReservedInsn | L1Exception::SystemCall => {
            writeln!(EEOut, "Incrementing exception exit address.").unwrap();
            unsafe {
                asm!(
                    "
                    mfc0 $k1, $14
                    addiu $k1, 4
                    mtc0 $k1, $14
                    "
                )
            }
        }
        _ => {}
    }
}
