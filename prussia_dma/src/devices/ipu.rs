use crate::devices::traits;

/// The Image Processing Unit, for sending MPEG2 frames to be decoded.
pub struct IpuFrom;
/// The Image Processing Unit for receiving decoded MPEG2 frames.
pub struct IpuTo;

impl traits::Address for IpuFrom {
    const CONTROL: *mut usize = 0x1000_b000 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_b010 as *mut usize;
    const COUNT: *mut usize = 0x1000_b020 as *mut usize;
}

impl traits::ReadChannel for IpuFrom {}

impl traits::Address for IpuTo {
    const CONTROL: *mut usize = 0x1000_b400 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_b410 as *mut usize;
    const COUNT: *mut usize = 0x1000_b420 as *mut usize;
}

impl traits::WriteChannel for IpuTo {}
