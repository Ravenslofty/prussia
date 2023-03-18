//! Coprocessor 0 manipulation routines.

use core::arch::asm;

use bitflags::bitflags;

bitflags! {
    /// The current processor state.
    pub struct Status: u32 {
        /// Whether interrupts are enabled. 0 = masked, 1 = enabled.
        const IE  = 1;
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
        let status;
        unsafe { asm!(".set noat; mfc0 {}, $12", out(reg) status) };

        Status { bits: status }
    }

    /// Store this object to the Coprocessor 0 Status register.
    pub fn store(self) {
        unsafe { asm!("mtc0 {}, $12", in(reg) self.bits) };
    }
}

bitflags! {
    /// Cause of the most recent exception.
    pub struct Cause: u32 {
        /// Level 1 exception code.
        const EXC_CODE = 0x1F << 2;
        /// Interrupt from the Interrupt Controller is pending.
        const IP2 = 1 << 10;
        /// Interrupt from the DMA Controller is pending.
        const IP3 = 1 << 11;
        /// Interrupt from the timer is pending.
        const IP7 = 1 << 15;
        /// Level 2 exception code.
        const EXC2 = 0x7 << 16;
        /// Coprocessor number of a Coprocessor Unusable exception.
        const CE = 0x3 << 28;
        /// Level 2 exception occurred in a branch delay slot.
        const BD2 = 1 << 30;
        /// Level 1 exception occurred in a branch delay slot.
        const BD = 1 << 31;
    }
}

impl Cause {
    /// Load the contents of the Coprocessor 0 Cause register, returning a Cause type with the
    /// value of Cause.
    pub fn load() -> Self {
        let cause;
        unsafe { asm!(".set noat; mfc0 {}, $13", out(reg) cause) };

        Cause { bits: cause }
    }
}

/// Level 1 exception types.
#[derive(Debug)]
pub enum L1Exception {
    /// An interrupt occurred.
    Interrupt,
    /// A TLB page marked not Dirty (not writable) was written to.
    TlbModified,
    /// No TLB entry was found while loading data or instructions from memory.
    TlbMissLoad,
    /// No TLB entry was found while storing data to memory.
    TlbMissStore,
    /// A TLB page marked not Valid (not mapped) was read from.
    TlbInvalidLoad,
    /// A TLB page marked not Valid (not mapped) was written to.
    TlbInvalidStore,
    /// An address error occurred while loading data or instructions from memory.
    AddrErrLoad,
    /// An address error occurred while storing data to memory.
    AddrErrStore,
    /// A bus error occurred while loading instructions.
    InsnBusError,
    /// A bus error occurred while loading/storing data.
    DataBusError,
    /// A system call was made.
    SystemCall,
    /// A debug breakpoint was executed.
    Breakpoint,
    /// An unrecognised instruction was executed.
    ReservedInsn,
    /// A coprocessor marked as unusable was used.
    CoprocessorUnusable,
    /// Two's complement arithmetic overflow occurred.
    Overflow,
    /// A trap instruction was executed.
    Trap,
}

impl L1Exception {
    /// Convert a L1 exception code in the TLB Miss exception handler.
    /// This function returns Option, but if it fails, you're passing in the wrong value.
    pub fn try_from_tlbmiss(x: u8) -> Option<Self> {
        match x {
            2 => Some(L1Exception::TlbMissLoad),
            3 => Some(L1Exception::TlbMissStore),
            _ => None,
        }
    }

    /// Convert a L1 exception code in the Common exception handler.
    /// This function rturns Option, but if it fails, you're passing in the wrong value.
    pub fn try_from_common(x: u8) -> Option<Self> {
        match x {
            1 => Some(L1Exception::TlbModified),
            2 => Some(L1Exception::TlbInvalidLoad),
            3 => Some(L1Exception::TlbInvalidStore),
            4 => Some(L1Exception::AddrErrLoad),
            5 => Some(L1Exception::AddrErrStore),
            6 => Some(L1Exception::InsnBusError),
            7 => Some(L1Exception::DataBusError),
            8 => Some(L1Exception::SystemCall),
            9 => Some(L1Exception::Breakpoint),
            10 => Some(L1Exception::ReservedInsn),
            11 => Some(L1Exception::CoprocessorUnusable),
            12 => Some(L1Exception::Overflow),
            13 => Some(L1Exception::Trap),
            _ => None,
        }
    }
}

/// Level 2 exception types.
#[derive(Debug)]
pub enum L2Exception {
    /// The processor was reset.
    Reset,
    /// A non-maskable interrupt occurred.
    NMI,
    /// The performance counter overflowed.
    PerfCounter,
    /// A breakpoint condition occurred.
    Debug,
}

impl L2Exception {
    /// Convert a level 2 exception code in the Reset handler.
    pub fn try_from_reset(x: u8) -> Option<Self> {
        match x {
            0 => Some(L2Exception::Reset),
            1 => Some(L2Exception::NMI),
            _ => None,
        }
    }

    /// Convert a level 2 exception code in the Performance Counter handler.
    pub fn try_from_counter(x: u8) -> Option<Self> {
        match x {
            2 => Some(L2Exception::PerfCounter),
            _ => None,
        }
    }

    /// Convert a level 2 exception code in the Debug handler.
    pub fn try_from_debug(x: u8) -> Option<Self> {
        match x {
            4 => Some(L2Exception::Debug),
            _ => None,
        }
    }
}
