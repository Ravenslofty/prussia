#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i_stat: IStat,
    _reserved1: [u8; 0x0c],
    i_mask: IMask,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Status"]
    #[inline(always)]
    pub const fn i_stat(&self) -> &IStat {
        &self.i_stat
    }
    #[doc = "0x10 - Interrupt Mask"]
    #[inline(always)]
    pub const fn i_mask(&self) -> &IMask {
        &self.i_mask
    }
}
#[doc = "I_STAT (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i_stat`] module"]
#[doc(alias = "I_STAT")]
pub type IStat = crate::Reg<i_stat::IStatSpec>;
#[doc = "Interrupt Status"]
pub mod i_stat;
#[doc = "I_MASK (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`i_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i_mask`] module"]
#[doc(alias = "I_MASK")]
pub type IMask = crate::Reg<i_mask::IMaskSpec>;
#[doc = "Interrupt Mask"]
pub mod i_mask;
