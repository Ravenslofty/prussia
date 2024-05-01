//! Functions for initialising the exception vectors and handling exceptions.

use core::{arch::asm, fmt::Write};

use prussia_debug::EEOut;

use crate::cop0::{CoP0Dump, L1Exception};

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

         # Load the return address
         mfc0 $k1, $14

         # Call the main cause handler.
         jal _dispatch_common_exc_handler
         nop

         # Load the return address and jump to it. Does not handle address incrementing
         # if exception is break, syscall, or similar which does not auto-advance the epc.
         lw $ra, 120($k0)
         jr $k0
         nop

         .set at
         .set reorder",
         options(noreturn)
    }
}

#[no_mangle]
extern "C" fn _dispatch_common_exc_handler() {
    writeln!(EEOut, "Encountered trap exception. Got here.").unwrap();

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
            unsafe { asm!("addiu $k1, 4") }
        }
        _ => {}
    }
}
