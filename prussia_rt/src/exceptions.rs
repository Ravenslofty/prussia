//! Functions for initialising the exception vectors and handling exceptions.

mod v_common;

use core::{arch::asm, fmt::Write};

use prussia_debug::EEOut;

use crate::cop0::{Cause, CoP0Dump, L1Exception};

use self::v_common::{
    _v_common_exception_vec, unimplemented_v_common_handler, v_common_breakpoint_handler,
    V_COMMON_EXCEPTION_BOOTSTRAP_VECTOR, V_COMMON_EXCEPTION_VECTOR, V_COMMON_HANDLERS,
};

pub use self::v_common::trigger_break_exception;

/// Address pointer to an EE-side exception handler.
pub struct EEExceptionVector {
    /// Destination address for the vector.
    pub location: usize,
    /// Associated function pointer to put at the vector.
    pub handler: unsafe extern "C" fn(),
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

    for x in unsafe { V_COMMON_HANDLERS.iter_mut() } {
        *x = unimplemented_v_common_handler as usize;
    }

    unsafe {
        V_COMMON_HANDLERS[9] = v_common_breakpoint_handler as usize;
    }
}
