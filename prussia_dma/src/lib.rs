//! DMA Controller routines for the Sony PlayStation 2.
//!
//! The DMA Controller (DMAC) provides memory transfer for the peripherals of the PS2. Specifically
//! it links to the GS Interface (GIF), the VU Interfaces (VIF0/VIF1), the SBUS Interface (SIF, for
//! the Input/Output Processor) and the Image Processing Unit (IPU).
//!
//! Most DMA transfers are represented through the `Transfer` struct, and can be created through
//! the `Transfer::from_mem` and `Transfer::to_mem` functions.
//!
//! Ownership of a channel is represented through the `Vif0`, `Vif1`, `Gif` (etc) types, which are
//! moved into a `Transfer` while it is in progress to avoid multiple transfers on the same
//! channel. A singleton for these channels is provided in `Channels::take()`.
//!
//! # Examples
//!
//! ```
//! use prussia_dma as dma;
//! fn write_transfer(gif: dma::Gif) -> dma::Gif {
//!     // Note: don't actually send this to the GIF.
//!     static mut DATA: Aligned<A16, [u32; 4]> = Aligned([0xdeadbeef, 0xcafed00d, 0x0c0ffee0, 0xcafebabe]);
//!
//!     let tx = dma::Transfer::from_mem(gif, unsafe { &mut DATA });
//!
//!     // Transfer in progress; don't touch DATA.
//!     // In future versions, there may be a better solution for avoiding memory aliasing.
//!
//!     let (gif, _) = tx.wait();
//!
//!     // Modifying DATA is now safe.
//!
//!     gif
//! }
//! ```

#![no_std]
#![allow(dead_code)]
#![deny(missing_docs)]

extern crate aligned;

use aligned::{Aligned, A16};
use core::{mem, ptr, sync::atomic};

mod control;
mod devices;

pub use crate::control::Status;
pub use crate::devices::{Gif, IpuFrom, IpuTo, Sif0, Sif1, Sif2, SpramFrom, SpramTo, Vif0, Vif1};

/// Represents the channels of the DMA controller.
pub struct Channels {
    vif0: Vif0,
    vif1: Vif1,
    gif: Gif,
    ipu_from: IpuFrom,
    ipu_to: IpuTo,
    sif0: Sif0,
    sif1: Sif1,
    sif2: Sif2,
    spram_from: SpramFrom,
    spram_to: SpramTo,
}

impl Channels {
    /// Return the DMA channels, *once*.
    pub fn take() -> Option<Self> {
        static mut TAKEN: bool = false;
        unsafe {
            if !TAKEN {
                TAKEN = true;
                Some(Self::steal())
            } else {
                None
            }
        }
    }

    unsafe fn steal() -> Self {
        Channels {
            vif0: Vif0 {},
            vif1: Vif1 {},
            gif: Gif {},
            ipu_from: IpuFrom {},
            ipu_to: IpuTo {},
            sif0: Sif0 {},
            sif1: Sif1 {},
            sif2: Sif2 {},
            spram_from: SpramFrom {},
            spram_to: SpramTo {},
        }
    }

    /// Initialise a DMA channel.
    fn init<T: devices::Address>(_dev: &mut T) {
        unsafe {
            // Zero the registers of the device.
            ptr::write_volatile(T::CONTROL, 0);
            ptr::write_volatile(T::ADDRESS, 0);
            ptr::write_volatile(T::COUNT, 0);
        }

        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }

    /// Initialise and return the DMA channels.
    pub fn split(
        mut self,
    ) -> (
        Vif0,
        Vif1,
        Gif,
        IpuFrom,
        IpuTo,
        Sif0,
        Sif1,
        Sif2,
        SpramFrom,
        SpramTo,
    ) {
        Channels::init(&mut self.vif0);
        Channels::init(&mut self.vif1);
        Channels::init(&mut self.gif);
        Channels::init(&mut self.ipu_from);
        Channels::init(&mut self.ipu_to);
        Channels::init(&mut self.sif0);
        Channels::init(&mut self.sif1);
        Channels::init(&mut self.sif2);
        Channels::init(&mut self.spram_from);
        Channels::init(&mut self.spram_to);

        (
            self.vif0,
            self.vif1,
            self.gif,
            self.ipu_from,
            self.ipu_to,
            self.sif0,
            self.sif1,
            self.sif2,
            self.spram_from,
            self.spram_to,
        )
    }
}

enum TransferDirection {
    ToMem,
    FromMem,
}

/// An in-progress DMA transfer.
///
/// This struct holds a mutable reference to the data that it is operating on to ensure it does not
/// go out of scope while the transfer is in progress. It also owns the channel that it is
/// operating on to avoid multithread races.
pub struct Transfer<DEVICE: devices::Address, T: 'static> {
    data: &'static mut Aligned<A16, [T]>,
    dev: DEVICE,
}

impl<DEVICE: devices::ReadChannel + devices::Address, T> Transfer<DEVICE, T> {
    /// Perform a DMA transfer from a device to memory, and return a `Transfer` object bound to
    /// the destination object.
    ///
    /// Reading from a write-only device is prevented at compile time through use of internal marker
    /// traits. Reading zero quadwords has no effect. Correct alignment is ensured through use of
    /// `Aligned<A16, [T]>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut data = Aligned([0u32; 4]);
    /// {
    ///     let transfer = Transfer::to_mem(SIF0, &mut data);
    ///     // Do work.
    /// }
    /// // The contents of data are now one quadword from SIF0 (the IOP).
    /// ```
    pub fn to_mem(dev: DEVICE, data: &'static mut Aligned<A16, [T]>) -> Transfer<DEVICE, T> {
        Transfer::transfer(TransferDirection::ToMem, dev, data)
    }
}

impl<DEVICE: devices::WriteChannel + devices::Address, T> Transfer<DEVICE, T> {
    /// Perform a DMA transfer from memory to a device, and return a `Transfer` object bound to
    /// the source object.
    ///
    /// Writing to a read-only device is prevented at compile time through use of internal marker
    /// traits. Writing zero quadwords has no effect. Correct alignment is ensured through use of
    /// `Aligned<A16, T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut data = Aligned[0u32; 4];
    /// {
    ///     let transfer: Transfer<Gif, u32> = Transfer::from_mem(GIF, &mut data);
    ///     // Do work.
    /// }
    /// // The contents of data have now been transferred to the GIF.
    /// ```
    pub fn from_mem(dev: DEVICE, data: &'static mut Aligned<A16, [T]>) -> Transfer<DEVICE, T> {
        Transfer::transfer(TransferDirection::FromMem, dev, data)
    }
}

impl<DEVICE: devices::Address, T: 'static> Transfer<DEVICE, T> {
    fn transfer(
        dir: TransferDirection,
        dev: DEVICE,
        data: &'static mut Aligned<A16, [T]>,
    ) -> Transfer<DEVICE, T> {
        let address = data.as_ptr() as usize;
        // This assumes that data is on a 16-byte boundary.
        let qword_count: usize = (data.len() * mem::size_of::<T>()) / 16;
        // The 1st bit of CHANNEL_CONTROL changes direction (we want to transfer from memory).
        // The 9th bit of CHANNEL_CONTROL starts a transfer.
        let control = unsafe { ptr::read_volatile(DEVICE::CONTROL) } | (1 << 8);
        let control = match dir {
            TransferDirection::ToMem => control & !1,
            TransferDirection::FromMem => control | 1,
        };

        // The EE User's Manual (5.10 Restrictions) states not to initiate a DMA transfer with a
        // qword count of zero (possibly because it transfers zero qwords?). Either way, avoid this
        // situation, and because we don't set the STR bit, drop() will see a finished transaction
        // immediately.
        if qword_count != 0 {
            unsafe {
                // Memory address to read data from.
                ptr::write_volatile(DEVICE::ADDRESS, address);
                // Number of 128-bit quadwords to read.
                ptr::write_volatile(DEVICE::COUNT, qword_count);
                // Avoid compiler reordering.
                atomic::compiler_fence(atomic::Ordering::SeqCst);
                // Start the transfer.
                ptr::write_volatile(DEVICE::CONTROL, control);
            }
        }

        Transfer {
            data: data,
            dev: dev,
        }
    }

    /// Returns true if the DMA transfer has completed.
    pub fn is_done(&self) -> bool {
        // Check if the STR bit is zero. If we skipped transferring zero qwords in
        // to_mem()/from_mem() then this will be zero anyway.
        let control = unsafe { ptr::read_volatile(DEVICE::CONTROL) };
        let control = control & (1 << 8);

        control == 0
    }

    /// Wait for a transfer to complete, returning the buffer used for the transfer.
    ///
    /// Failing to wait for a DMA transfer means it will continue to completion in the
    /// background, but the channel will remain locked.
    pub fn wait(self) -> (DEVICE, &'static mut Aligned<A16, [T]>) {
        while !self.is_done() {}

        atomic::compiler_fence(atomic::Ordering::SeqCst);

        (self.dev, self.data)
    }
}

mod tests {
    use aligned::{Aligned, A16};

    use crate::Channels;
    use crate::Transfer;
    use crate::{Gif, IpuFrom, IpuTo, Sif0, Vif0, Vif1};

    fn test_vif0(vif0: Vif0) -> Vif0 {
        static mut DATA: Aligned<A16, [u32; 4]> = Aligned([0u32; 4]);

        let dma = Transfer::from_mem(vif0, unsafe { &mut DATA });
        let (vif0, _) = dma.wait();

        vif0
    }

    fn test_vif1(vif1: Vif1) -> Vif1 {
        static mut DATA: Aligned<A16, [u32; 4]> = Aligned([0u32; 4]);

        let dma = Transfer::from_mem(vif1, unsafe { &mut DATA });
        let (vif1, _) = dma.wait();

        let dma = Transfer::to_mem(vif1, unsafe { &mut DATA });
        let (vif1, _) = dma.wait();

        vif1
    }

    fn test_gif(gif: Gif) -> Gif {
        static mut DATA: Aligned<A16, [u32; 4]> = Aligned([0u32; 4]);

        let dma = Transfer::from_mem(gif, unsafe { &mut DATA });
        let (gif, _) = dma.wait();

        gif
    }

    fn test_ipu(ipu_from: IpuFrom, ipu_to: IpuTo) -> (IpuFrom, IpuTo) {
        static mut READ_DATA: Aligned<A16, [u32; 4]> = Aligned([0u32; 4]);
        static mut WRITE_DATA: Aligned<A16, [u32; 4]> = Aligned([0u32; 4]);

        let dma_to = Transfer::from_mem(ipu_to, unsafe { &mut READ_DATA });
        let dma_from = Transfer::to_mem(ipu_from, unsafe { &mut WRITE_DATA });

        // Multiple transfers at once for fun and profit.
        // Note the separate read and write buffers; using the same buffer for multiple
        // simultaneous transfer is Undefined Behaviour.

        let (ipu_to, _) = dma_to.wait();
        let (ipu_from, _) = dma_from.wait();

        (ipu_from, ipu_to)
    }

    fn test_sif0(sif0: Sif0) -> Sif0 {
        static mut DATA: Aligned<A16, [u32; 4]> = Aligned([0u32; 4]);

        let dma = Transfer::to_mem(sif0, unsafe { &mut DATA });
        let (sif0, _) = dma.wait();

        sif0
    }

    // If this function compiles, this crate *should* be safe.
    fn test() {
        let chans = Channels::take().unwrap().split();

        test_vif0(chans.0);
        test_vif1(chans.1);
        test_gif(chans.2);
        test_sif0(chans.5);
    }
}
