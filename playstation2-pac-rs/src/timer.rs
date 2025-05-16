#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    t0_count: T0Count,
    _reserved1: [u8; 0x0c],
    t0_mode: T0Mode,
    _reserved2: [u8; 0x0c],
    t0_comp: T0Comp,
    _reserved3: [u8; 0x0c],
    t0_hold: T0Hold,
    _reserved4: [u8; 0x07cc],
    t1_count: T1Count,
    _reserved5: [u8; 0x0c],
    t1_mode: T1Mode,
    _reserved6: [u8; 0x0c],
    _reserved_6_t1_comp: [u8; 0x04],
    _reserved7: [u8; 0x0c],
    t1_hold: T1Hold,
    _reserved8: [u8; 0x07cc],
    t2_count: T2Count,
    _reserved9: [u8; 0x0c],
    t2_mode: T2Mode,
    _reserved10: [u8; 0x07ec],
    t3_count: T3Count,
    _reserved11: [u8; 0x0c],
    t3_mode: T3Mode,
    _reserved12: [u8; 0x0c],
    t3_comp: T3Comp,
}
impl RegisterBlock {
    #[doc = "0x00 - Counter register"]
    #[inline(always)]
    pub const fn t0_count(&self) -> &T0Count {
        &self.t0_count
    }
    #[doc = "0x10 - Register for setting modes and reading status"]
    #[inline(always)]
    pub const fn t0_mode(&self) -> &T0Mode {
        &self.t0_mode
    }
    #[doc = "0x20 - Comparison register"]
    #[inline(always)]
    pub const fn t0_comp(&self) -> &T0Comp {
        &self.t0_comp
    }
    #[doc = "0x30 - Hold register"]
    #[inline(always)]
    pub const fn t0_hold(&self) -> &T0Hold {
        &self.t0_hold
    }
    #[doc = "0x800 - Counter register"]
    #[inline(always)]
    pub const fn t1_count(&self) -> &T1Count {
        &self.t1_count
    }
    #[doc = "0x810 - Register for setting modes and reading status"]
    #[inline(always)]
    pub const fn t1_mode(&self) -> &T1Mode {
        &self.t1_mode
    }
    #[doc = "0x820 - Comparison register"]
    #[inline(always)]
    pub const fn t2_comp(&self) -> &T2Comp {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2080).cast() }
    }
    #[doc = "0x820 - Comparison register"]
    #[inline(always)]
    pub const fn t1_comp(&self) -> &T1Comp {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2080).cast() }
    }
    #[doc = "0x830 - Hold register"]
    #[inline(always)]
    pub const fn t1_hold(&self) -> &T1Hold {
        &self.t1_hold
    }
    #[doc = "0x1000 - Counter register"]
    #[inline(always)]
    pub const fn t2_count(&self) -> &T2Count {
        &self.t2_count
    }
    #[doc = "0x1010 - Register for setting modes and reading status"]
    #[inline(always)]
    pub const fn t2_mode(&self) -> &T2Mode {
        &self.t2_mode
    }
    #[doc = "0x1800 - Counter register"]
    #[inline(always)]
    pub const fn t3_count(&self) -> &T3Count {
        &self.t3_count
    }
    #[doc = "0x1810 - Register for setting modes and reading status"]
    #[inline(always)]
    pub const fn t3_mode(&self) -> &T3Mode {
        &self.t3_mode
    }
    #[doc = "0x1820 - Comparison register"]
    #[inline(always)]
    pub const fn t3_comp(&self) -> &T3Comp {
        &self.t3_comp
    }
}
#[doc = "T0_COUNT (rw) register accessor: Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_count`] module"]
#[doc(alias = "T0_COUNT")]
pub type T0Count = crate::Reg<t0_count::T0CountSpec>;
#[doc = "Counter register"]
pub mod t0_count;
#[doc = "T0_MODE (rw) register accessor: Register for setting modes and reading status\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_mode`] module"]
#[doc(alias = "T0_MODE")]
pub type T0Mode = crate::Reg<t0_mode::T0ModeSpec>;
#[doc = "Register for setting modes and reading status"]
pub mod t0_mode;
#[doc = "T0_COMP (rw) register accessor: Comparison register\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_comp`] module"]
#[doc(alias = "T0_COMP")]
pub type T0Comp = crate::Reg<t0_comp::T0CompSpec>;
#[doc = "Comparison register"]
pub mod t0_comp;
#[doc = "T0_HOLD (rw) register accessor: Hold register\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_hold`] module"]
#[doc(alias = "T0_HOLD")]
pub type T0Hold = crate::Reg<t0_hold::T0HoldSpec>;
#[doc = "Hold register"]
pub mod t0_hold;
pub use t0_comp as t1_comp;
pub use t0_comp as t2_comp;
pub use t0_comp as t3_comp;
pub use t0_count as t1_count;
pub use t0_count as t2_count;
pub use t0_count as t3_count;
pub use t0_hold as t1_hold;
pub use t0_mode as t1_mode;
pub use t0_mode as t2_mode;
pub use t0_mode as t3_mode;
pub use T0Comp as T1Comp;
pub use T0Comp as T2Comp;
pub use T0Comp as T3Comp;
pub use T0Count as T1Count;
pub use T0Count as T2Count;
pub use T0Count as T3Count;
pub use T0Hold as T1Hold;
pub use T0Mode as T1Mode;
pub use T0Mode as T2Mode;
pub use T0Mode as T3Mode;
