//! Coprocessor 0 manipulation routines.

use core::{arch::asm, fmt::Display};

use bitflags::bitflags;

extern "C" {
    fn _read_badvaddr() -> u32;
    fn _read_timercount() -> u32;
    fn _write_timercount(count: u32);
    fn _read_compare() -> u32;
    fn _write_compare(compare: u32);
    fn _read_epc() -> u32;
    fn _write_epc(epc: u32);
    fn _read_badpaddr() -> u32;
    fn _write_badpaddr(badpaddr: u32);
    fn _read_errorepc() -> u32;
    fn _write_errorepc(errorepc: u32);
    fn _read_pccr() -> u32;
    fn _write_pccr(pccr: u32);
    fn _read_pcr0() -> u32;
    fn _read_pcr1() -> u32;
    fn _write_pcr0(pcr0: u32);
    fn _write_pcr1(pcr1: u32);
}

/// Represents a coredump of the CoP0 registers.
#[derive(Debug)]
pub struct CoP0Dump {
    /// [BadVAddr] value loaded from the BadVAddr register.
    pub bad_v_addr: BadVAddr,
    /// [TimerCount] value loaded from the Count register.
    pub count: TimerCount,
    /// [Compare] value loaded from the Compare register.
    pub compare: Compare,
    /// [Status] value loaded from the Status register.
    pub status: Status,
    /// [Cause] value loaded from the Cause register.
    pub cause: Cause,
    /// [EPC] value loaded from the EPC register.
    pub epc: EPC,
    /// [BadPAddr] value loaded from the BadPAddr register.
    pub bad_p_addr: BadPAddr,
    /// [PerfCounterControl] value loaded from the Perf register.
    pub perf: PerfCounterControl,
    /// [PCCREvent0] value loaded from the Perf.EVENT0 register field.
    pub perf_event0: PCCREvent0,
    /// [PCCREvent1] value loaded from the Perf.EVENT1 register field.
    pub perf_event1: PCCREvent1,
    /// [PerfCounter] value loaded from the PCR0 register.
    pub pcr0: PerfCounter,
    /// [PerfCounter] value loaded from the PCR1 register.
    pub pcr1: PerfCounter,
    /// [ErrorEPC] value loaded from the ErrorEPC register.
    pub error_epc: ErrorEPC,
}

impl CoP0Dump {
    /// Dumps all register contents and constructs an instance of [CoP0Dump].
    pub fn load() -> Self {
        let bad_v_addr = BadVAddr::load();
        let count = TimerCount::load();
        let compare = Compare::load();
        let status = Status::load();
        let cause = Cause::load();
        let epc = EPC::load();
        let bad_p_addr = BadPAddr::load();
        let perf = PerfCounterControl::load();
        let pcr0 = PerfCounter::load_pcr0();
        let pcr1 = PerfCounter::load_pcr1();
        let error_epc = ErrorEPC::load();

        Self {
            bad_v_addr,
            count,
            compare,
            status,
            cause,
            epc,
            bad_p_addr,
            perf,
            perf_event0: perf.into(),
            perf_event1: perf.into(),
            pcr0,
            pcr1,
            error_epc,
        }
    }
}

/// A virtual address responsible for either of the below exceptions:
/// * TLB Invalid
/// * TLB Modified
/// * TLB Refill
/// * Address Error
#[derive(Debug)]
pub struct BadVAddr(u32);

impl BadVAddr {
    /// Load a [BadVAddr] from the _CoP0.BadVAddr_ register (`$12`)
    pub fn load() -> Self {
        let bad_v_addr = unsafe { _read_badvaddr() };

        BadVAddr(bad_v_addr)
    }
}

/// A value read from the CoP0.Count register. The register increments every CPU clock cycle.
/// This register, when equal to the CoP0.Compare register, signals a timer interrupt.
/// FIXME: Pick a method, discard the other.
#[derive(Debug)]
pub struct TimerCount(u32);

impl TimerCount {
    /// Load a [TimerCount] value from the _CoP0.Count_ register (`$9`).
    pub fn load() -> Self {
        let count = unsafe { _read_timercount() };

        TimerCount(count)
    }

    /// Write [TimerCount] to the _CoP0.Count_ register (`$9`).
    pub fn store(self) {
        unsafe { _write_timercount(self.0) }
    }
}

/// A value read from the CoP0.Compare register. The register acts as a timer;
/// when the Count register becomes equal to the Compare register, the interrupt bit in the Cause
/// register is set, and a timer interrupt occurs if enabled.
#[derive(Debug)]
pub struct Compare(u32);

impl Compare {
    /// Load a [Compare] value from the _CoP0.Compare_ register (`$11`).
    pub fn load() -> Self {
        let compare = unsafe { _read_compare() };

        Compare(compare)
    }

    /// Write [Self] to the _CoP0.Compare_ register (`$11`).
    pub fn store(self) {
        unsafe { _write_compare(self.0) }
    }
}

/// A value read from the CoP0.EPC register. The value is the returning virtual address to jump to
/// after handling an exception.
#[derive(Debug)]
pub struct EPC(u32);

impl EPC {
    /// Load an [EPC] value from the _CoP0.EPC_ register (`$14`).
    pub fn load() -> Self {
        let epc = unsafe { _read_epc() };

        EPC(epc)
    }

    /// Write [Self] to the _CoP0.EPC_ register (`$14`).
    pub fn store(self) {
        unsafe { _write_epc(self.0) }
    }
}

/// A physical address read from the CoP0.BadPAddr register. The value is automatically set to the
/// most recent physical address that caused a bus error (assuming bus error masking (Status.BEM) is disabled).
#[derive(Debug)]
pub struct BadPAddr(u32);

impl BadPAddr {
    /// Load a [BadPAddr] value from the _CoP0.BadPAddr_ register (`$23`).
    pub fn load() -> Self {
        let badpaddr = unsafe { _read_badpaddr() };

        BadPAddr(badpaddr)
    }

    /// Write [Self] to the _CoP0.BadPAddr_ register (`$23`).
    pub fn store(self) {
        unsafe { _write_badpaddr(self.0) }
    }
}

/// A virtual address read from the CoP0.ErrorEPC register. The value is a return address set when
/// an NMI, debug, or counter exception occurs and is handled. The address normally points to the
/// instruction which generated the error. If the instruction is in a branch delay slot, the address
/// is set to the branch instruction prior to the erroring instruction (indicated by Cause.BD2 being set to 1).
#[derive(Debug)]
pub struct ErrorEPC(u32);

impl ErrorEPC {
    /// Load an [ErrorEPC] value from _CoP0.ErrorEPC_ register (`$30`).
    pub fn load() -> Self {
        let errorepc = unsafe { _read_errorepc() };

        ErrorEPC(errorepc)
    }

    /// Write [Self] to the _CoP0.ErrorEPC_ register (`$30`).
    pub fn store(self) {
        unsafe { _write_errorepc(self.0) }
    }
}

bitflags! {
    /// A value in the PCCR performance counter control register.
    pub struct PerfCounterControl: u32 {
        /// When set, enables a counting operation in CTR0 when handling a level 1 exception.
        const EXL0 = 1 << 1;
        /// When set, enables a counting operation in CTR0 in Kernel mode.
        const K0 = 1 << 2;
        /// When set, enables a counting operation in CTR0 in Supervisor mode.
        const S0 = 1 << 3;
        /// When set, enables a counting operation in CTR0 in User mode.
        const U0 = 1 << 4;
        /// The Id of the CTR0 events to be counted.
        const EVENT0 = 0x1F << 5;
        /// When set, enables a counting operation in CTR1 when handling a level 1 exception.
        const EXL1 = 1 << 11;
        /// When set, enables a counting operation in CTR1 in Kernel mode.
        const K1 = 1 << 12;
        /// When set, enables a counting operation in CTR1 in Supervisor mode.
        const S1 = 1 << 13;
        /// When set, enables a counting operation in CTR1 in User mode.
        const U1 = 1 << 14;
        /// The Id of the CTR1 events to be counted.
        const EVENT1 = 0x1F << 15;
        /// Enables the counter function.
        const CTE = 1 << 31;
    }
}

impl PerfCounterControl {
    /// Load a [PerfCounterControl] value from the _CoP0.PCCR_ register (via `mfps`).
    pub fn load() -> Self {
        let control = unsafe { _read_pccr() };

        PerfCounterControl::from_bits_truncate(control)
    }

    /// Write [Self] to the _CoP0.PCCR_ register (via `mtps`).
    pub fn store(self) {
        let control = self.bits;
        unsafe {
            _write_pccr(control);
        }
    }
}

/// Represents the CTR0 event Ids supported by the PCCR.EVENT0 register field.
#[derive(Debug)]
#[repr(u8)]
pub enum PCCREvent0 {
    /// Reserved
    Reserved = 0,
    /// Processor cycle
    ProcessorCycle = 1,
    /// Issues single instruction
    IssueSingleInstruction = 2,
    /// Issues branch
    IssuesBranch = 3,
    /// BTAC miss
    BtacMiss = 4,
    /// TLB miss
    TlbMiss = 5,
    /// Instruction cache (I$) miss
    InstructionCacheMiss = 6,
    /// Access DTLB
    AccessDtlb = 7,
    /// Non-blocking load
    NonBlockingLoad = 8,
    /// WBB single request
    WBBSingleRequest = 9,
    /// WBB burst request
    WBBBurstRequest = 10,
    /// CPU address bus busy
    CPUAddressBusBusy = 11,
    /// Completes instruction
    CompletesInstruction = 12,
    /// Completes non-BDS instruction
    CompletesNonBdsInstruction = 13,
    /// Completes COP2 instruction
    CompletesCop2Instruction = 14,
    /// Completes load
    CompletesLoads = 15,
    /// No event
    NoEvent = 16,
}

impl From<PCCREvent0> for PerfCounterControl {
    fn from(value: PCCREvent0) -> Self {
        let event0 = (value as u32) << 5;

        PerfCounterControl::from_bits_truncate(event0)
    }
}

impl From<PerfCounterControl> for PCCREvent0 {
    fn from(value: PerfCounterControl) -> Self {
        let bits: u32 = (value & PerfCounterControl::EVENT0).bits >> 5;

        match bits {
            0 => Self::Reserved,
            1 => Self::ProcessorCycle,
            2 => Self::IssueSingleInstruction,
            3 => Self::IssuesBranch,
            4 => Self::BtacMiss,
            5 => Self::TlbMiss,
            6 => Self::InstructionCacheMiss,
            7 => Self::AccessDtlb,
            8 => Self::NonBlockingLoad,
            9 => Self::WBBSingleRequest,
            10 => Self::WBBBurstRequest,
            11 => Self::CPUAddressBusBusy,
            12 => Self::CompletesInstruction,
            13 => Self::CompletesNonBdsInstruction,
            14 => Self::CompletesCop2Instruction,
            15 => Self::CompletesLoads,
            16 => Self::NoEvent,
            i if i > 16 && i < 32 => Self::Reserved,
            _ => Self::NoEvent,
        }
    }
}

/// Represents the CTR1 event Ids supported by the PCCR.EVENT1 register field.
#[derive(Debug)]
#[repr(u8)]
pub enum PCCREvent1 {
    /// Issues low order branch
    IssuesLowOrderBranch = 0,
    /// Processor cycle
    ProcessorCycle = 1,
    /// Issues double instruction
    IssuesDoubleInstruction = 2,
    /// Branch prediction miss
    BranchPredictionMiss = 3,
    /// TLB miss
    TlbMiss = 4,
    /// DTLB miss
    DtlbMiss = 5,
    /// Data cache (D$) miss
    DataCacheMiss = 6,
    /// WBB single request unusable
    WbbSingleRequestUnusable = 7,
    /// WBB burst request unusable
    WbbBurstRequestUnusable = 8,
    /// WBB burst request almost full
    WbbBurstRequestAlmostFull = 9,
    /// WBB burst request full
    WbbBurstRequestFull = 10,
    /// CPU data bus busy
    CpuDataBusBusy = 11,
    /// Completes instruction
    CompletesInstruction = 12,
    /// Completes non-BDS instruction
    CompletesNonBdsInstruction = 13,
    /// Completes COP1 instruction
    CompletesCop1Instruction = 14,
    /// Completes stores
    CompletesStores = 15,
    /// No event
    NoEvent = 16,
    /// Reserved
    Reserved = 17,
}

impl From<PCCREvent1> for PerfCounterControl {
    fn from(value: PCCREvent1) -> Self {
        let event1 = (value as u32) << 15;

        PerfCounterControl::from_bits_truncate(event1)
    }
}

impl From<PerfCounterControl> for PCCREvent1 {
    fn from(value: PerfCounterControl) -> Self {
        let bits: u32 = (value & PerfCounterControl::EVENT0).bits >> 5;

        match bits {
            0 => Self::IssuesLowOrderBranch,
            1 => Self::ProcessorCycle,
            2 => Self::IssuesDoubleInstruction,
            3 => Self::BranchPredictionMiss,
            4 => Self::TlbMiss,
            5 => Self::DtlbMiss,
            6 => Self::DataCacheMiss,
            7 => Self::WbbSingleRequestUnusable,
            8 => Self::WbbBurstRequestUnusable,
            9 => Self::WbbBurstRequestAlmostFull,
            10 => Self::WbbBurstRequestFull,
            11 => Self::CpuDataBusBusy,
            12 => Self::CompletesInstruction,
            13 => Self::CompletesNonBdsInstruction,
            14 => Self::CompletesCop1Instruction,
            15 => Self::CompletesStores,
            16 => Self::NoEvent,
            i if i > 16 && i < 32 => Self::Reserved,
            _ => Self::NoEvent,
        }
    }
}

/// Represents a value from the _Cop0.PCR0/1_ Performance Counter registers.
#[derive(Debug)]
#[repr(transparent)]
pub struct PerfCounter(u32);

impl PerfCounter {
    /// Mask for the OVFL bit.
    pub const OVFL: u32 = 1 << 31;
    /// Mask for the VALUE field.
    pub const VALUE: u32 = u32::MAX >> 1;

    /// Check if the OVFL bit is enabled.
    #[inline(always)]
    pub fn is_ovfl(&self) -> bool {
        self.0 & Self::OVFL == 1
    }

    /// Get the contents of the VALUE field.
    #[inline(always)]
    pub fn value(&self) -> u32 {
        self.0 & Self::VALUE
    }

    /// Load a [PerfCounter] value from the _CoP0.PCR0_ register (via `mfpc`).
    pub fn load_pcr0() -> Self {
        let pcr0 = unsafe { _read_pcr0() };

        PerfCounter(pcr0)
    }

    /// Load a [PerfCounter] value from the _CoP0.PCR1_ register (via `mfpc`).
    pub fn load_pcr1() -> Self {
        let pcr1 = unsafe { _read_pcr1() };

        PerfCounter(pcr1)
    }

    /// Write [Self] to the _CoP0.PCR0_ register (via `mtpc`).
    pub fn store_pcr0(self) {
        unsafe { _write_pcr0(self.0) }
    }

    /// Write [Self] to the _CoP0.PCR1_ register (via `mtpc`).
    pub fn store_pcr1(self) {
        unsafe { _write_pcr1(self.0) }
    }
}

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
    /// This function returns Option, but if it fails, you're passing in the wrong value.
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

    /// Convert a L1 exception code in the Common exception handler.
    pub fn try_from_common_cause(cause: Cause) -> Option<Self> {
        let bits: u32 = (cause & Cause::EXC_CODE).bits >> 2;

        Self::try_from_common(bits as u8)
    }
}

impl Display for L1Exception {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
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
