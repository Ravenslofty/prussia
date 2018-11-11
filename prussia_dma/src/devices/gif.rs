use crate::devices::traits;

/// The GS Interface, for writing data for the Graphics Synthesizer to render later.
pub struct Gif;

impl traits::Address for Gif {
    const CONTROL: *mut usize = 0x1000_a000 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_a010 as *mut usize;
    const COUNT: *mut usize = 0x1000_a020 as *mut usize;
}

impl traits::WriteChannel for Gif {}
