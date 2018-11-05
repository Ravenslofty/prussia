#[no_std]
#[warn(missing_docs)]

use crate::devices::traits;

#[derive(Default)]
pub struct Sif0;

#[derive(Default)]
pub struct Sif1;

#[derive(Default)]
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


