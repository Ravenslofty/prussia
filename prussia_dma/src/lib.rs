//! DMA Controller routines for the Sony PlayStation 2.
//!
//! The DMA Controller (DMAC) provides memory transfer for the peripherals of the PS2. Specifically
//! it links to the GS Interface (GIF), the VU Interfaces (VIF0/VIF1), the SBUS Interface (SIF, for
//! the Input/Output Processor) and the Image Processing Unit (IPU).
//!
//! Most things in the PS2 rely on the DMA Controller, and you should call dma::init_all() first in
//! main().

#![no_std]
#![deny(missing_docs)]

use core::{mem, ptr};

mod devices;

pub use crate::devices::{Gif, Sif0, Sif1, Sif2};

/// Represents an in-progress DMA transfer. This struct holds a mutable reference to the data that
/// it is operating on to ensure it does not go out of scope while the transfer is in progress. It
/// uses Drop to ensure that the DMA transfer has completed, but this could be slow for large
/// transfers, so you should perform as much work as possible in between transfers to maximise
/// performance.
pub struct Transfer<'a, T: devices::Address + Default, U> {
    data: &'a mut [U],
    dev: T,
}

impl<'a, T: devices::ReadChannel + devices::Address + Default, U> Transfer<'a, T, U> {
    /// Perform a DMA transfer from a device to memory, and return a Transfer object bound to the
    /// destination object. Reading from a write-only device is prevented at compile time through
    /// use of internal marker traits. Writing zero quadwords has no effect.
    ///
    /// # Safety
    ///
    /// `data` must be 16-byte aligned and a multiple of 16 bytes long. A misaligned memory
    /// location is undefined behaviour, both in software and in hardware.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assume this is aligned.
    /// let mut data: [u32; 4] = [0; 4];
    /// {
    ///     let transfer: Transfer<Sif0, u32> = Transfer::to_mem(&mut data);
    ///     // Do work.
    /// }
    /// // The contents of data are now one quadword from SIF0 (the IOP). 
    /// ```
    pub fn to_mem(data: &'a mut [U]) -> Transfer<'a, T, U> {
        let address = data.as_mut_ptr() as usize;
        // This assumes that data is on a 16-byte boundary.
        let qword_count: usize = (data.len() * mem::size_of::<U>()) / 16;
        // The 1st bit of CHANNEL_CONTROL changes direction (we want to transfer to memory).
        // The 9th bit of CHANNEL_CONTROL starts a transfer.
        let control = unsafe { ptr::read_volatile(T::CONTROL) };
        let control = control | (1 << 8) & !1;
    
        // The EE User's Manual (5.10 Restrictions) states not to initiate a DMA transfer with a
        // qword count of zero (possibly because it transfers zero qwords?). Either way, avoid this
        // situation, and because we don't set the STR bit, drop() will see a finished transaction
        // immediately.
        if qword_count != 0 {
            unsafe {
                // Memory address to write data to.
                ptr::write_volatile(T::ADDRESS, address);
                // Number of 128-bit quadwords to write.
                ptr::write_volatile(T::COUNT, qword_count);
                // Start the transfer.
                ptr::write_volatile(T::CONTROL, control);
            }
        }

        Transfer {
            data: data,
            dev: Default::default()
        }
    }
}

impl<'a, T: devices::WriteChannel + devices::Address + Default, U> Transfer<'a, T, U> {
    /// Perform a DMA transfer from memory to a device, and return a Transfer object bound to the
    /// source object. Writing to a read-only device is prevented at compile time through use
    /// of internal marker traits. Writing zero quadwords has no effect.
    ///
    /// # Safety
    ///
    /// `data` must be 16-byte aligned and a multiple of 16 bytes long. A misaligned memory
    /// location is undefined behaviour, both in software and in hardware.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assume this is aligned.
    /// let mut data: [u32; 4] = [0; 4];
    /// {
    ///     let transfer: Transfer<Gif, u32> = Transfer::from_mem(&mut data);
    ///     // Do work.
    /// }
    /// // The contents of data have now been transferred to the GIF.
    /// ```
    pub fn from_mem(data: &'a mut [U]) -> Transfer<'a, T, U> {
        let address = data.as_mut_ptr() as usize;
        // This assumes that data is on a 16-byte boundary.
        let qword_count: usize = (data.len() * mem::size_of::<U>()) / 16;
        // The 1st bit of CHANNEL_CONTROL changes direction (we want to transfer from memory).
        // The 9th bit of CHANNEL_CONTROL starts a transfer.
        let control = unsafe { ptr::read_volatile(T::CONTROL) };
        let control = control | (1 << 8) | 1;
        
        // The EE User's Manual (5.10 Restrictions) states not to initiate a DMA transfer with a
        // qword count of zero (possibly because it transfers zero qwords?). Either way, avoid this
        // situation, and because we don't set the STR bit, drop() will see a finished transaction
        // immediately.
        if qword_count != 0 {
            unsafe {
                // Memory address to read data from.
                ptr::write_volatile(T::ADDRESS, address);
                // Number of 128-bit quadwords to read.
                ptr::write_volatile(T::COUNT, qword_count);
                // Start the transfer.
                ptr::write_volatile(T::CONTROL, control);
            }
        }

        Transfer {
            data: data,
            dev: Default::default()
        }
    }
}

impl<'a, T: devices::Address + Default, U> Drop for Transfer<'a, T, U> {
    fn drop(&mut self) {
        loop {
            // Check if the STR bit is zero. If we skipped transferring zero qwords in
            // read()/write() then this will be zero anyway.
            let control = unsafe { ptr::read_volatile(T::CONTROL) };
            let control = control & (1 << 8);
            if control == 0 {
                break;
            }
        }
    }
}

mod tests {
    use crate::Transfer;
    use crate::devices::Gif;

    fn test_gif_write() {
        let mut data = [0u32; 4];
        let dma: Transfer<Gif, u32> = Transfer::from_mem(&mut data);
    }
}
