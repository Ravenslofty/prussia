//! Functions for initialising the exception vectors and handling exceptions.

mod v_common;

use core::ptr::copy_nonoverlapping;

use prussia_debug::println_ee;

use self::v_common::{
    _v_common_exception_vec, init_v_common_handlers_table, unimplemented_v_common_handler,
    V_COMMON_EXCEPTION_BOOTSTRAP_VECTOR, V_COMMON_EXCEPTION_VECTOR,
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
            copy_nonoverlapping(vector.handler as *const u32, vector.location as *mut u32, 4,);
        };
    }

    println_ee!("Exception vectors loaded.");

    init_v_common_handlers_table()
}
