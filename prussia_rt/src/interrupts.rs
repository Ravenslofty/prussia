//! Interrupt handling and management.
#![allow(dead_code)]

use crate::cop0;

/// Disable interrupts.
pub fn disable() {
    // By not using the `di` instruction, we sidestep the hardware errata.
    let mut status = cop0::Status::load();
    status.remove(cop0::Status::EIE); // `di`/`ei` use the EIE bit, rather than IE.
    status.remove(cop0::Status::IE);
    status.store();
}

/// Whether interrupts are enabled or disabled.
pub enum Status {
    /// Interrupts are disabled.
    Disabled,
    /// Interrupts are enabled.
    Enabled,
}

/// Get current interrupt status.
pub fn status() -> Status {
    let status = cop0::Status::load();
    if status.contains(cop0::Status::IE) && status.contains(cop0::Status::EIE) {
        Status::Enabled
    } else {
        Status::Disabled
    }
}

/// Enable interrupts.
pub fn enable() {
    let mut status = cop0::Status::load();
    status.insert(cop0::Status::EIE);
    status.insert(cop0::Status::IE);
    status.store();
}

/// Execute a closure in an interrupt-free context, preserving previous interrupt state.
pub fn free<T: FnOnce() -> ()>(f: T) {
    match status() {
        Status::Enabled => {
            disable();
            f();
            enable();
        }
        Status::Disabled => {
            f();
        }
    }
}
