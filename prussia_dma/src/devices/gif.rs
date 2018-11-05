#![deny(missing_docs)]

use crate::devices::traits;

/// The GS Interface handles setting up and changing the registers of the Graphics Synthesizer, the
/// GPU of the PS2. It is write-only, and most use of the GIF requires a GIF-specific header called
/// a GIFtag. For those situations it is better to use prussia_gs than prussia_dma directly.
///
/// This struct is empty, but is used as a type parameter for Transfer.
#[derive(Default)]
pub struct Gif;

impl traits::Address for Gif {
    const CONTROL: *mut usize = 0x1000_a000 as *mut usize;
    const ADDRESS: *mut usize = 0x1000_a010 as *mut usize;
    const COUNT: *mut usize = 0x1000_a020 as *mut usize;
}

impl traits::WriteChannel for Gif {}
