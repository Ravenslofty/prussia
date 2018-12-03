//! DMA Controller interrupt handling.

use core::ptr;

use bitflags::bitflags;

static mut DMAC_STAT: *mut u32 = 0x1000_e010 as *mut u32;

bitflags! {
    /// DMA Controller interrupt status register.
    pub struct Status: u32 {
        /// Whether there is an interrupt on channel 0. 0 = no interrupt, 1 = interrupt.
        const CIS0 = 1;
        /// Whether there is an interrupt on channel 1. 0 = no interrupt, 1 = interrupt.
        const CIS1 = 1 << 1;
        /// Whether there is an interrupt on channel 2. 0 = no interrupt, 1 = interrupt.
        const CIS2 = 1 << 2;
        /// Whether there is an interrupt on channel 3. 0 = no interrupt, 1 = interrupt.
        const CIS3 = 1 << 3;
        /// Whether there is an interrupt on channel 4. 0 = no interrupt, 1 = interrupt.
        const CIS4 = 1 << 4;
        /// Whether there is an interrupt on channel 5. 0 = no interrupt, 1 = interrupt.
        const CIS5 = 1 << 5;
        /// Whether there is an interrupt on channel 6. 0 = no interrupt, 1 = interrupt.
        const CIS6 = 1 << 6;
        /// Whether there is an interrupt on channel 7. 0 = no interrupt, 1 = interrupt.
        const CIS7 = 1 << 7;
        /// Whether there is an interrupt on channel 8. 0 = no interrupt, 1 = interrupt.
        const CIS8 = 1 << 8;
        /// Whether there is an interrupt on channel 9. 0 = no interrupt, 1 = interrupt.
        const CIS9 = 1 << 9;
        /// Whether a DMA transfer has stalled. Requires Control::STD. 0 = no stall, 1 = stall at
        /// some point.
        const SIS = 1 << 13;
        /// Whether the DMA memory FIFO is empty. Requires Control::MFD. 0 = not empty, 1 = empty
        /// at some point.
        const MEIS = 1 << 14;
        /// Whether the DMA controller has encountered a bus error. 0 = no error, 1 = bus error.
        const BEIS = 1 << 15;
        /// Whether interrupts on channel 0 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM0 = 1 << 16;
        /// Whether interrupts on channel 1 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM1 = 1 << 17;
        /// Whether interrupts on channel 2 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM2 = 1 << 18;
        /// Whether interrupts on channel 3 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM3 = 1 << 19;
        /// Whether interrupts on channel 4 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM4 = 1 << 20;
        /// Whether interrupts on channel 5 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM5 = 1 << 21;
        /// Whether interrupts on channel 6 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM6 = 1 << 22;
        /// Whether interrupts on channel 7 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM7 = 1 << 23;
        /// Whether interrupts on channel 8 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM8 = 1 << 24;
        /// Whether interrupts on channel 9 are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const CIM9 = 1 << 25;
        /// Whether DMA stall interrupts are active. 0 = disabled, 1 = enabled. Write 1 to toggle.
        const SIM = 1 << 29;
        /// Whether DMA memory FIFO interrupts are active. 0 = disabled, 1 = enabled. Write 1 to
        /// toggle.
        const MEIM = 1 << 30;
    }
}

impl Status {
    /// Load the DMA interrupt status register.
    pub fn load() -> Self {
        let status = unsafe { ptr::read_volatile(DMAC_STAT) };
        Status { bits: status }
    }

    /// Store the DMA interrupt status register.
    pub fn store(self) {
        unsafe { ptr::write_volatile(DMAC_STAT, self.bits) }
    }
}
