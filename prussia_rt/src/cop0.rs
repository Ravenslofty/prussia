//! Coprocessor 0 manipulation routines.

use bitflags::bitflags;

bitflags! {
    /// The EE Coprocessor 0 Status register.
    pub struct Status: u32 {
        /// Whether interrupts are enabled. 0 = masked, 1 = enabled.
        const IE  = 1 << 0;
        /// Whether we are in an exception (everything which isn't an error) context.
        const EXL = 1 << 1;
        /// Whether we are in an error (Reset, NMI, performance counter, debug exception) context.
        const ERL = 1 << 2;
        /// Which processor mode we are in. 00 = kernel, 01 = supervisor, 10 = user.
        const KSU = 3 << 3;
        /// Whether interrupt 2 is masked. 0 = masked, 1 = enabled.
        const IM2 = 1 << 10;
        /// Whether interrupt 3 is masked. 0 = masked, 1 = enabled.
        const IM3 = 1 << 11;
        /// Whether bus errors are masked. 0 = masked, 1 = enabled.
        const BEM = 1 << 12;
        /// Whether interrupt 7 is masked. 0 = masked, 1 = enabled.
        const IM7 = 1 << 15;
        /// Whether the IE bit is enabled. 0 = disabled (this disables all interrupts), 1 = enabled.
        const EIE = 1 << 16;
        /// Whether the EI/DI instructions have any effect in user mode.
        /// 0 = no effect in user mode, 1 = normal function in user mode.
        const EDI = 1 << 17;
        /// Status of the last CACHE instruction. 0 = miss, 1 = hit.
        const CH  = 1 << 18;
        /// Whether TLB or general exception handlers are located in RAM or ROM. 0 = RAM, 1 = ROM.
        const BEV = 1 << 22;
        /// Whether performance counter/debug exceptions are located in RAM or ROM.
        /// 0 = RAM, 1 = ROM.
        const DEV = 1 << 23;
        /// Whether coprocessor 0 is usable. 0 = unusable, 1 = usable.
        const CU0 = 1 << 28;
        /// Whether coprocessor 1 is usable. 0 = unusable, 1 = usable.
        const CU1 = 1 << 29;
        /// Whether coprocessor 2 is usable. 0 = unusable, 1 = usable.
        const CU2 = 1 << 30;
        /// Whether coprocessor 3 is usable. 0 = unusable, 1 = usable.
        const CU3 = 1 << 31;
    }
}

impl Status {
    /// Load the contents of the Coprocessor 0 Status register, returning a Status type with the
    /// value of Status.
    pub fn load() -> Self {
        extern "C" {
            fn _read_status() -> u32;
        }

        let status = unsafe { _read_status() };

        Status { bits: status }
    }

    /// Store this object to the Coprocessor 0 Status register.
    pub fn store(self) {
        extern "C" {
            fn _write_status(x: u32);
        }

        unsafe { _write_status(self.bits) }
    }
}
