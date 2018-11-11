use crate::devices::traits;

/// Scratchpad RAM, for fast data reads.
pub struct SpramFrom;
/// Scratchpad RAM, for fast data writes.
pub struct SpramTo;

impl traits::Address for SpramFrom {
    const CONTROL: *mut usize = 0x1000_d000 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_d010 as *mut usize;
    const COUNT: *mut usize = 0x1000_d020 as *mut usize;
}

impl traits::ReadChannel for SpramFrom {}

impl traits::Address for SpramTo {
    const CONTROL: *mut usize = 0x1000_d400 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_d410 as *mut usize;
    const COUNT: *mut usize = 0x1000_d420 as *mut usize;
}

impl traits::WriteChannel for SpramTo {}
