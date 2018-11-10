use crate::devices::traits;

pub struct Sif0;
pub struct Sif1;
pub struct Sif2;

impl traits::Address for Sif0 {
    const CONTROL: *mut usize = 0x1000_c000 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_c010 as *mut usize;
    const COUNT: *mut usize = 0x1000_c020 as *mut usize;
}

impl traits::ReadChannel for Sif0 {}

impl traits::Address for Sif1 {
    const CONTROL: *mut usize = 0x1000_c400 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_c410 as *mut usize;
    const COUNT: *mut usize = 0x1000_c420 as *mut usize;
}

impl traits::WriteChannel for Sif1 {}

impl traits::Address for Sif2 {
    const CONTROL: *mut usize = 0x1000_c800 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_c810 as *mut usize;
    const COUNT: *mut usize = 0x1000_c820 as *mut usize;
}

impl traits::ReadChannel for Sif2 {}
impl traits::WriteChannel for Sif2 {}
