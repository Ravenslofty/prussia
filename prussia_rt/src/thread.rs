//! Contains functions and structs related to EE threading and runtime debugging.

use core::fmt::Debug;

/// Represents information about the EE registers at a moment in execution.
/// Usually used on exception-handling to save the register state before
/// jumping to an exception handler.
#[repr(C)]
pub(crate) struct ThreadControlBlock {
    pub at: u64,
    pub v0: u64,
    pub v1: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub t0: u64,
    pub t1: u64,
    pub t2: u64,
    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,
    pub t7: u64,
    pub s0: u64,
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub t8: u64,
    pub t9: u64,
    pub k0: u64,
    pub k1: u64,
    pub gp: u64,
    pub sp: u64,
    pub fp: u64,
    pub ra: u64,
}

impl Debug for ThreadControlBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ThreadControlBlock")
            .field("at", &format_args!("{:#016X}", self.at))
            .field("v0", &format_args!("{:#016X}", self.v0))
            .field("v1", &format_args!("{:#016X}", self.v1))
            .field("a0", &format_args!("{:#016X}", self.a0))
            .field("a1", &format_args!("{:#016X}", self.a1))
            .field("a2", &format_args!("{:#016X}", self.a2))
            .field("a3", &format_args!("{:#016X}", self.a3))
            .field("t0", &format_args!("{:#016X}", self.t0))
            .field("t1", &format_args!("{:#016X}", self.t1))
            .field("t2", &format_args!("{:#016X}", self.t2))
            .field("t3", &format_args!("{:#016X}", self.t3))
            .field("t4", &format_args!("{:#016X}", self.t4))
            .field("t5", &format_args!("{:#016X}", self.t5))
            .field("t6", &format_args!("{:#016X}", self.t6))
            .field("t7", &format_args!("{:#016X}", self.t7))
            .field("s0", &format_args!("{:#016X}", self.s0))
            .field("s1", &format_args!("{:#016X}", self.s1))
            .field("s2", &format_args!("{:#016X}", self.s2))
            .field("s3", &format_args!("{:#016X}", self.s3))
            .field("s4", &format_args!("{:#016X}", self.s4))
            .field("s5", &format_args!("{:#016X}", self.s5))
            .field("s6", &format_args!("{:#016X}", self.s6))
            .field("s7", &format_args!("{:#016X}", self.s7))
            .field("t8", &format_args!("{:#016X}", self.t8))
            .field("t9", &format_args!("{:#016X}", self.t9))
            .field("k0", &format_args!("{:#016X}", self.k0))
            .field("k1", &format_args!("{:#016X}", self.k1))
            .field("gp", &format_args!("{:#016X}", self.gp))
            .field("sp", &format_args!("{:#016X}", self.sp))
            .field("fp", &format_args!("{:#016X}", self.fp))
            .field("ra", &format_args!("{:#016X}", self.ra))
            .finish()
    }
}