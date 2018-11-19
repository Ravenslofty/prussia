//! Atomic reads and writes.

use crate::interrupts;

extern "C" {
    fn _read_u64(addr: usize) -> u64;
    fn _write_u64(addr: usize, value: u64);
}

/// Read a 64-bit value from an address.
pub fn read_u64(addr: usize) -> u64 {
    let mut val = 0;
    interrupts::free(|| unsafe {
        val = _read_u64(addr);
    });
    val
}

/// Write a 64-bit value to an address.
pub fn write_u64(addr: usize, value: u64) {
    interrupts::free(|| unsafe { _write_u64(addr, value) });
}
