#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vif0_stat: Vif0Stat,
    _reserved1: [u8; 0x0c],
    vif0_fbrst: Vif0Fbrst,
    _reserved2: [u8; 0x0c],
    vif0_err: Vif0Err,
    _reserved3: [u8; 0x0c],
    vif0_mark: Vif0Mark,
    _reserved4: [u8; 0x0c],
    vif0_cycle: Vif0Cycle,
    _reserved5: [u8; 0x0c],
    vif0_mode: Vif0Mode,
    _reserved6: [u8; 0x0c],
    vif0_num: Vif0Num,
    _reserved7: [u8; 0x0c],
    vif0_mask: Vif0Mask,
    _reserved8: [u8; 0x0c],
    vif0_code: Vif0Code,
    _reserved9: [u8; 0x0c],
    vif0_itops: Vif0Itops,
    _reserved10: [u8; 0x3c],
    vif0_itop: Vif0Itop,
    _reserved11: [u8; 0x2c],
    vif0_r0: Vif0R0,
    _reserved12: [u8; 0x0c],
    vif0_r1: Vif0R1,
    _reserved13: [u8; 0x0c],
    vif0_r2: Vif0R2,
    _reserved14: [u8; 0x0c],
    vif0_r3: Vif0R3,
    _reserved15: [u8; 0x0c],
    vif0_c0: Vif0C0,
    _reserved16: [u8; 0x0c],
    vif0_c1: Vif0C1,
    _reserved17: [u8; 0x0c],
    vif0_c2: Vif0C2,
    _reserved18: [u8; 0x0c],
    vif0_c3: Vif0C3,
    _reserved19: [u8; 0x028c],
    vif1_stat: Vif1Stat,
    _reserved20: [u8; 0x0c],
    vif1_fbrst: Vif1Fbrst,
    _reserved21: [u8; 0x0c],
    vif1_err: Vif1Err,
    _reserved22: [u8; 0x0c],
    vif1_mark: Vif1Mark,
    _reserved23: [u8; 0x0c],
    vif1_cycle: Vif1Cycle,
    _reserved24: [u8; 0x0c],
    vif1_mode: Vif1Mode,
    _reserved25: [u8; 0x0c],
    vif1_num: Vif1Num,
    _reserved26: [u8; 0x0c],
    vif1_mask: Vif1Mask,
    _reserved27: [u8; 0x0c],
    vif1_code: Vif1Code,
    _reserved28: [u8; 0x0c],
    vif1_itops: Vif1Itops,
    _reserved29: [u8; 0x0c],
    vif1_base: Vif1Base,
    _reserved30: [u8; 0x0c],
    vif1_ofst: Vif1Ofst,
    _reserved31: [u8; 0x0c],
    vif1_tops: Vif1Tops,
    _reserved32: [u8; 0x0c],
    vif1_itop: Vif1Itop,
    _reserved33: [u8; 0x2c],
    vif1_r0: Vif1R0,
    _reserved34: [u8; 0x0c],
    vif1_r1: Vif1R1,
    _reserved35: [u8; 0x0c],
    vif1_r2: Vif1R2,
    _reserved36: [u8; 0x0c],
    vif1_r3: Vif1R3,
    _reserved37: [u8; 0x0c],
    vif1_c0: Vif1C0,
    _reserved38: [u8; 0x0c],
    vif1_c1: Vif1C1,
    _reserved39: [u8; 0x0c],
    vif1_c2: Vif1C2,
    _reserved40: [u8; 0x0c],
    vif1_c3: Vif1C3,
}
impl RegisterBlock {
    #[doc = "0x00 - Status"]
    #[inline(always)]
    pub const fn vif0_stat(&self) -> &Vif0Stat {
        &self.vif0_stat
    }
    #[doc = "0x10 - Operation control"]
    #[inline(always)]
    pub const fn vif0_fbrst(&self) -> &Vif0Fbrst {
        &self.vif0_fbrst
    }
    #[doc = "0x20 - Error status"]
    #[inline(always)]
    pub const fn vif0_err(&self) -> &Vif0Err {
        &self.vif0_err
    }
    #[doc = "0x30 - Mark value"]
    #[inline(always)]
    pub const fn vif0_mark(&self) -> &Vif0Mark {
        &self.vif0_mark
    }
    #[doc = "0x40 - Data write cycle"]
    #[inline(always)]
    pub const fn vif0_cycle(&self) -> &Vif0Cycle {
        &self.vif0_cycle
    }
    #[doc = "0x50 - ADD mode"]
    #[inline(always)]
    pub const fn vif0_mode(&self) -> &Vif0Mode {
        &self.vif0_mode
    }
    #[doc = "0x60 - Amount of non-transferred data"]
    #[inline(always)]
    pub const fn vif0_num(&self) -> &Vif0Num {
        &self.vif0_num
    }
    #[doc = "0x70 - Write mask pattern"]
    #[inline(always)]
    pub const fn vif0_mask(&self) -> &Vif0Mask {
        &self.vif0_mask
    }
    #[doc = "0x80 - Last processed VIFcode"]
    #[inline(always)]
    pub const fn vif0_code(&self) -> &Vif0Code {
        &self.vif0_code
    }
    #[doc = "0x90 - Next ITOP value"]
    #[inline(always)]
    pub const fn vif0_itops(&self) -> &Vif0Itops {
        &self.vif0_itops
    }
    #[doc = "0xd0 - ITOP value"]
    #[inline(always)]
    pub const fn vif0_itop(&self) -> &Vif0Itop {
        &self.vif0_itop
    }
    #[doc = "0x100 - Filling data R0 (Row register)"]
    #[inline(always)]
    pub const fn vif0_r0(&self) -> &Vif0R0 {
        &self.vif0_r0
    }
    #[doc = "0x110 - Filling data R1 (Row register)"]
    #[inline(always)]
    pub const fn vif0_r1(&self) -> &Vif0R1 {
        &self.vif0_r1
    }
    #[doc = "0x120 - Filling data R2 (Row register)"]
    #[inline(always)]
    pub const fn vif0_r2(&self) -> &Vif0R2 {
        &self.vif0_r2
    }
    #[doc = "0x130 - Filling data R3 (Row register)"]
    #[inline(always)]
    pub const fn vif0_r3(&self) -> &Vif0R3 {
        &self.vif0_r3
    }
    #[doc = "0x140 - Filling data C0 (Col register)"]
    #[inline(always)]
    pub const fn vif0_c0(&self) -> &Vif0C0 {
        &self.vif0_c0
    }
    #[doc = "0x150 - Filling data C1 (Col register)"]
    #[inline(always)]
    pub const fn vif0_c1(&self) -> &Vif0C1 {
        &self.vif0_c1
    }
    #[doc = "0x160 - Filling data C2 (Col register)"]
    #[inline(always)]
    pub const fn vif0_c2(&self) -> &Vif0C2 {
        &self.vif0_c2
    }
    #[doc = "0x170 - Filling data C3 (Col register)"]
    #[inline(always)]
    pub const fn vif0_c3(&self) -> &Vif0C3 {
        &self.vif0_c3
    }
    #[doc = "0x400 - Status"]
    #[inline(always)]
    pub const fn vif1_stat(&self) -> &Vif1Stat {
        &self.vif1_stat
    }
    #[doc = "0x410 - Operation control"]
    #[inline(always)]
    pub const fn vif1_fbrst(&self) -> &Vif1Fbrst {
        &self.vif1_fbrst
    }
    #[doc = "0x420 - Error status"]
    #[inline(always)]
    pub const fn vif1_err(&self) -> &Vif1Err {
        &self.vif1_err
    }
    #[doc = "0x430 - Mark value"]
    #[inline(always)]
    pub const fn vif1_mark(&self) -> &Vif1Mark {
        &self.vif1_mark
    }
    #[doc = "0x440 - Data write cycle"]
    #[inline(always)]
    pub const fn vif1_cycle(&self) -> &Vif1Cycle {
        &self.vif1_cycle
    }
    #[doc = "0x450 - ADD mode"]
    #[inline(always)]
    pub const fn vif1_mode(&self) -> &Vif1Mode {
        &self.vif1_mode
    }
    #[doc = "0x460 - Amount of non-transferred data"]
    #[inline(always)]
    pub const fn vif1_num(&self) -> &Vif1Num {
        &self.vif1_num
    }
    #[doc = "0x470 - Write mask pattern"]
    #[inline(always)]
    pub const fn vif1_mask(&self) -> &Vif1Mask {
        &self.vif1_mask
    }
    #[doc = "0x480 - Last processed VIFcode"]
    #[inline(always)]
    pub const fn vif1_code(&self) -> &Vif1Code {
        &self.vif1_code
    }
    #[doc = "0x490 - Next ITOP value"]
    #[inline(always)]
    pub const fn vif1_itops(&self) -> &Vif1Itops {
        &self.vif1_itops
    }
    #[doc = "0x4a0 - Base address of double buffer"]
    #[inline(always)]
    pub const fn vif1_base(&self) -> &Vif1Base {
        &self.vif1_base
    }
    #[doc = "0x4b0 - Offset of double buffer"]
    #[inline(always)]
    pub const fn vif1_ofst(&self) -> &Vif1Ofst {
        &self.vif1_ofst
    }
    #[doc = "0x4c0 - Next TOP value/data write address"]
    #[inline(always)]
    pub const fn vif1_tops(&self) -> &Vif1Tops {
        &self.vif1_tops
    }
    #[doc = "0x4d0 - ITOP value"]
    #[inline(always)]
    pub const fn vif1_itop(&self) -> &Vif1Itop {
        &self.vif1_itop
    }
    #[doc = "0x500 - Filling data R0 (Row register)"]
    #[inline(always)]
    pub const fn vif1_r0(&self) -> &Vif1R0 {
        &self.vif1_r0
    }
    #[doc = "0x510 - Filling data R1 (Row register)"]
    #[inline(always)]
    pub const fn vif1_r1(&self) -> &Vif1R1 {
        &self.vif1_r1
    }
    #[doc = "0x520 - Filling data R2 (Row register)"]
    #[inline(always)]
    pub const fn vif1_r2(&self) -> &Vif1R2 {
        &self.vif1_r2
    }
    #[doc = "0x530 - Filling data R3 (Row register)"]
    #[inline(always)]
    pub const fn vif1_r3(&self) -> &Vif1R3 {
        &self.vif1_r3
    }
    #[doc = "0x540 - Filling data C0 (Col register)"]
    #[inline(always)]
    pub const fn vif1_c0(&self) -> &Vif1C0 {
        &self.vif1_c0
    }
    #[doc = "0x550 - Filling data C1 (Col register)"]
    #[inline(always)]
    pub const fn vif1_c1(&self) -> &Vif1C1 {
        &self.vif1_c1
    }
    #[doc = "0x560 - Filling data C2 (Col register)"]
    #[inline(always)]
    pub const fn vif1_c2(&self) -> &Vif1C2 {
        &self.vif1_c2
    }
    #[doc = "0x570 - Filling data C3 (Col register)"]
    #[inline(always)]
    pub const fn vif1_c3(&self) -> &Vif1C3 {
        &self.vif1_c3
    }
}
#[doc = "VIF0_STAT (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vif0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif0_stat`] module"]
#[doc(alias = "VIF0_STAT")]
pub type Vif0Stat = crate::Reg<vif0_stat::Vif0StatSpec>;
#[doc = "Status"]
pub mod vif0_stat;
pub use vif1_c0 as vif0_c0;
pub use vif1_c0 as vif0_c1;
pub use vif1_c0 as vif0_c2;
pub use vif1_c0 as vif0_c3;
pub use vif1_code as vif0_code;
pub use vif1_cycle as vif0_cycle;
pub use vif1_err as vif0_err;
pub use vif1_fbrst as vif0_fbrst;
pub use vif1_itop as vif0_itop;
pub use vif1_itops as vif0_itops;
pub use vif1_mark as vif0_mark;
pub use vif1_mask as vif0_mask;
pub use vif1_mode as vif0_mode;
pub use vif1_num as vif0_num;
pub use vif1_r0 as vif0_r0;
pub use vif1_r0 as vif0_r1;
pub use vif1_r0 as vif0_r2;
pub use vif1_r0 as vif0_r3;
pub use Vif1C0 as Vif0C0;
pub use Vif1C0 as Vif0C1;
pub use Vif1C0 as Vif0C2;
pub use Vif1C0 as Vif0C3;
pub use Vif1Code as Vif0Code;
pub use Vif1Cycle as Vif0Cycle;
pub use Vif1Err as Vif0Err;
pub use Vif1Fbrst as Vif0Fbrst;
pub use Vif1Itop as Vif0Itop;
pub use Vif1Itops as Vif0Itops;
pub use Vif1Mark as Vif0Mark;
pub use Vif1Mask as Vif0Mask;
pub use Vif1Mode as Vif0Mode;
pub use Vif1Num as Vif0Num;
pub use Vif1R0 as Vif0R0;
pub use Vif1R0 as Vif0R1;
pub use Vif1R0 as Vif0R2;
pub use Vif1R0 as Vif0R3;
#[doc = "VIF1_STAT (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_stat`] module"]
#[doc(alias = "VIF1_STAT")]
pub type Vif1Stat = crate::Reg<vif1_stat::Vif1StatSpec>;
#[doc = "Status"]
pub mod vif1_stat;
#[doc = "VIF1_FBRST (w) register accessor: Operation control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_fbrst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_fbrst`] module"]
#[doc(alias = "VIF1_FBRST")]
pub type Vif1Fbrst = crate::Reg<vif1_fbrst::Vif1FbrstSpec>;
#[doc = "Operation control"]
pub mod vif1_fbrst;
#[doc = "VIF1_ERR (rw) register accessor: Error status\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_err`] module"]
#[doc(alias = "VIF1_ERR")]
pub type Vif1Err = crate::Reg<vif1_err::Vif1ErrSpec>;
#[doc = "Error status"]
pub mod vif1_err;
#[doc = "VIF1_MARK (rw) register accessor: Mark value\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_mark::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_mark::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_mark`] module"]
#[doc(alias = "VIF1_MARK")]
pub type Vif1Mark = crate::Reg<vif1_mark::Vif1MarkSpec>;
#[doc = "Mark value"]
pub mod vif1_mark;
#[doc = "VIF1_CYCLE (r) register accessor: Data write cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_cycle::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_cycle`] module"]
#[doc(alias = "VIF1_CYCLE")]
pub type Vif1Cycle = crate::Reg<vif1_cycle::Vif1CycleSpec>;
#[doc = "Data write cycle"]
pub mod vif1_cycle;
#[doc = "VIF1_MODE (r) register accessor: ADD mode\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_mode`] module"]
#[doc(alias = "VIF1_MODE")]
pub type Vif1Mode = crate::Reg<vif1_mode::Vif1ModeSpec>;
#[doc = "ADD mode"]
pub mod vif1_mode;
#[doc = "VIF1_NUM (r) register accessor: Amount of non-transferred data\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_num::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_num`] module"]
#[doc(alias = "VIF1_NUM")]
pub type Vif1Num = crate::Reg<vif1_num::Vif1NumSpec>;
#[doc = "Amount of non-transferred data"]
pub mod vif1_num;
#[doc = "VIF1_MASK (r) register accessor: Write mask pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_mask`] module"]
#[doc(alias = "VIF1_MASK")]
pub type Vif1Mask = crate::Reg<vif1_mask::Vif1MaskSpec>;
#[doc = "Write mask pattern"]
pub mod vif1_mask;
#[doc = "VIF1_CODE (r) register accessor: Last processed VIFcode\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_code::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_code`] module"]
#[doc(alias = "VIF1_CODE")]
pub type Vif1Code = crate::Reg<vif1_code::Vif1CodeSpec>;
#[doc = "Last processed VIFcode"]
pub mod vif1_code;
#[doc = "VIF1_ITOPS (r) register accessor: Next ITOP value\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_itops::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_itops`] module"]
#[doc(alias = "VIF1_ITOPS")]
pub type Vif1Itops = crate::Reg<vif1_itops::Vif1ItopsSpec>;
#[doc = "Next ITOP value"]
pub mod vif1_itops;
#[doc = "VIF1_BASE (r) register accessor: Base address of double buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_base::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_base`] module"]
#[doc(alias = "VIF1_BASE")]
pub type Vif1Base = crate::Reg<vif1_base::Vif1BaseSpec>;
#[doc = "Base address of double buffer"]
pub mod vif1_base;
#[doc = "VIF1_OFST (r) register accessor: Offset of double buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_ofst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_ofst`] module"]
#[doc(alias = "VIF1_OFST")]
pub type Vif1Ofst = crate::Reg<vif1_ofst::Vif1OfstSpec>;
#[doc = "Offset of double buffer"]
pub mod vif1_ofst;
#[doc = "VIF1_TOPS (r) register accessor: Next TOP value/data write address\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_tops::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_tops`] module"]
#[doc(alias = "VIF1_TOPS")]
pub type Vif1Tops = crate::Reg<vif1_tops::Vif1TopsSpec>;
#[doc = "Next TOP value/data write address"]
pub mod vif1_tops;
#[doc = "VIF1_ITOP (r) register accessor: ITOP value\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_itop::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_itop`] module"]
#[doc(alias = "VIF1_ITOP")]
pub type Vif1Itop = crate::Reg<vif1_itop::Vif1ItopSpec>;
#[doc = "ITOP value"]
pub mod vif1_itop;
#[doc = "VIF1_R0 (r) register accessor: Filling data R0 (Row register)\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_r0`] module"]
#[doc(alias = "VIF1_R0")]
pub type Vif1R0 = crate::Reg<vif1_r0::Vif1R0Spec>;
#[doc = "Filling data R0 (Row register)"]
pub mod vif1_r0;
pub use vif1_r0 as vif1_r1;
pub use vif1_r0 as vif1_r2;
pub use vif1_r0 as vif1_r3;
pub use Vif1R0 as Vif1R1;
pub use Vif1R0 as Vif1R2;
pub use Vif1R0 as Vif1R3;
#[doc = "VIF1_C0 (rw) register accessor: Filling data C0 (Col register)\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vif1_c0`] module"]
#[doc(alias = "VIF1_C0")]
pub type Vif1C0 = crate::Reg<vif1_c0::Vif1C0Spec>;
#[doc = "Filling data C0 (Col register)"]
pub mod vif1_c0;
pub use vif1_c0 as vif1_c1;
pub use vif1_c0 as vif1_c2;
pub use vif1_c0 as vif1_c3;
pub use Vif1C0 as Vif1C1;
pub use Vif1C0 as Vif1C2;
pub use Vif1C0 as Vif1C3;
