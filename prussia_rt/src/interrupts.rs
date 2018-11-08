//! Interrupt handling and management.
#![allow(dead_code)]

#[no_mangle]
extern "C" {
    fn _di() -> u32;
    fn _ei();
}

/// Disable interrupts.
pub fn disable() {
    // We need to work around a hardware errata here, which is why _di() returns the contents of
    // the COP0 Status register. If an interrupt occurs immediately after `di` but before the
    // instruction clears the pipeline, the EIE (enable interrupt enable) bit might not be cleared,
    // which requires re-running _di().
    loop {
        let cop0_status = unsafe { _di() };
        const EIE: u32 = 0x00010000;
        if cop0_status & EIE == 0 {
            break;
        }
    }
}

/// Enable interrupts.
pub fn enable() {
    // PS2SDK has a workaround here, but it's undocumented. For now I'll omit it.
    unsafe {
        _ei();
    }
}

/// Execute a closure in an interrupt-free context.
pub fn free<T: FnOnce() -> ()>(f: T) {
    disable();
    f();
    enable();
}
