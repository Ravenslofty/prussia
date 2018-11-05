#![deny(missing_docs)]

// Whether a channel can be read from.
pub trait ReadChannel {}

// Whether a channel can be written to.
pub trait WriteChannel {}

// Memory addresses for writing to a DMA channel. Not all channels have all addresses, so they are
// marked as Option<u32>.
pub trait Address {
    // Metadata and status control word.
    const CONTROL: *mut usize;
    // Memory address to read to/write from.
    const ADDRESS: *mut usize;
    // Number of 128-bit quadwords to read/write.
    const COUNT: *mut usize;
    // TAG_ADDRESS is used for devices that support DMA Chain Mode. However, we currently don't use
    // DMA Chain Mode, so it is irrelevant for now.
    // const TAG_ADDRESS: Option<u32>;
}
