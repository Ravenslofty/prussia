//! Routines for the PlayStation 2 Emotion Engine Interrupt Controller.

#![no_std]
#![deny(missing_docs)]
use core::ptr;

use bitflags::bitflags;

static mut INTC_STAT: *mut u32 = 0x1000_f000 as *mut u32;
static mut INTC_MASK: *mut u32 = 0x1000_f010 as *mut u32;

bitflags! {
    /// The interrupt controller status. Each flag is set if the interrupt controller detects a
    /// change in state. Writing a bit acknowledges that interrupt. If the status and
    /// corresponding mask bit are both set, INT0 is raised.
    pub struct Status: u32 {
        /// An interrupt from the Graphics Synthesizer.
        const GS = 1;
        /// An interrupt from the SBUS connection to the IOP.
        const SBUS = 1 << 1;
        /// The start of VBlank.
        const VBON = 1 << 2;
        /// The end of VBlank.
        const VBOF = 1 << 3;
        /// An interrupt from VU Interface 0.
        const VIF0 = 1 << 4;
        /// An interrupt from VU Interface 1.
        const VIF1 = 1 << 5;
        /// An interrupt from Vector Unit 0.
        const VU0 = 1 << 6;
        /// An interrupt from Vector Unit 1.
        const VU1 = 1 << 7;
        /// An interrupt from the Image Processing Unit.
        const IPU = 1 << 8;
        /// An interrupt from Timer 0.
        const TIM0 = 1 << 9;
        /// An interrupt from Timer 1.
        const TIM1 = 1 << 10;
        /// An interrupt from Timer 2.
        const TIM2 = 1 << 11;
        /// An interrupt from Timer 3.
        const TIM3 = 1 << 12;
        /// An interrupt from the SBUS FIFO queue.
        const SFIFO = 1 << 13;
        /// An interrupt from the Vector Unit 0 watchdog.
        const VU0WD = 1 << 14;
    }
}

impl Status {
    /// Load the interrupt controller status.
    pub fn load() -> Self {
        let status = unsafe { ptr::read_volatile(INTC_STAT) };
        Status { bits: status }
    }

    /// Store the interrupt controller status.
    pub fn store(self) {
        unsafe { ptr::write_volatile(INTC_STAT, self.bits) };
    }
}

bitflags! {
    /// The interrupt status mask. Each bit is set if the interrupt is enabled and clear if the
    /// interrupt is disabled. Writing a bit toggles its state. If both the mask bit and the
    /// corresponding status bit are both set, INT0 is raised.
    pub struct Mask: u32 {
        /// An interrupt from the Graphics Synthesizer.
        const GS = 1;
        /// An interrupt from the SBUS connection to the IOP.
        const SBUS = 1 << 1;
        /// The start of VBlank.
        const VBON = 1 << 2;
        /// The end of VBlank.
        const VBOF = 1 << 3;
        /// An interrupt from VU Interface 0.
        const VIF0 = 1 << 4;
        /// An interrupt from VU Interface 1.
        const VIF1 = 1 << 5;
        /// An interrupt from Vector Unit 0.
        const VU0 = 1 << 6;
        /// An interrupt from Vector Unit 1.
        const VU1 = 1 << 7;
        /// An interrupt from the Image Processing Unit.
        const IPU = 1 << 8;
        /// An interrupt from Timer 0.
        const TIM0 = 1 << 9;
        /// An interrupt from Timer 1.
        const TIM1 = 1 << 10;
        /// An interrupt from Timer 2.
        const TIM2 = 1 << 11;
        /// An interrupt from Timer 3.
        const TIM3 = 1 << 12;
        /// An interrupt from the SBUS FIFO queue.
        const SFIFO = 1 << 13;
        /// An interrupt from the Vector Unit 0 watchdog.
        const VU0WD = 1 << 14;
    }
}

impl Mask {
    /// Load the interrupt controller mask.
    pub fn load() -> Self {
        let mask = unsafe { ptr::read_volatile(INTC_MASK) };
        Mask { bits: mask }
    }

    /// Store the interrupt controller mask.
    pub fn store(self) {
        unsafe { ptr::write_volatile(INTC_MASK, self.bits) };
    }
}
