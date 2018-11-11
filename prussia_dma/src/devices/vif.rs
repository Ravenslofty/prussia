use crate::devices::traits;

/// VU Interface 0, for writing data to Vector Unit 0.
pub struct Vif0;
/// VU Interface 1, for reading/writing data to Vector Unit 1.
pub struct Vif1;

impl traits::Address for Vif0 {
    const CONTROL: *mut usize = 0x1000_8000 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_8010 as *mut usize;
    const COUNT: *mut usize = 0x1000_8020 as *mut usize;
}

impl traits::WriteChannel for Vif0 {}

impl traits::Address for Vif1 {
    const CONTROL: *mut usize = 0x1000_9000 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_9010 as *mut usize;
    const COUNT: *mut usize = 0x1000_9020 as *mut usize;
}

impl traits::ReadChannel for Vif1 {}
impl traits::WriteChannel for Vif1 {}
