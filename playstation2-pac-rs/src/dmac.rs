#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    d0_chcr: D0Chcr,
    _reserved1: [u8; 0x0c],
    d0_madr: D0Madr,
    _reserved2: [u8; 0x0c],
    d0_qwc: D0Qwc,
    _reserved3: [u8; 0x0c],
    d0_tadr: D0Tadr,
    _reserved4: [u8; 0x0c],
    d0_asr0: D0Asr0,
    _reserved5: [u8; 0x0c],
    d0_asr1: D0Asr1,
    _reserved6: [u8; 0x0fac],
    d1_chcr: D1Chcr,
    _reserved7: [u8; 0x0c],
    d1_madr: D1Madr,
    _reserved8: [u8; 0x0c],
    d1_qwc: D1Qwc,
    _reserved9: [u8; 0x0c],
    d1_tadr: D1Tadr,
    _reserved10: [u8; 0x0c],
    d1_asr0: D1Asr0,
    _reserved11: [u8; 0x0c],
    d1_asr1: D1Asr1,
    _reserved12: [u8; 0x0fac],
    d2_chcr: D2Chcr,
    _reserved13: [u8; 0x0c],
    d2_madr: D2Madr,
    _reserved14: [u8; 0x0c],
    d2_qwc: D2Qwc,
    _reserved15: [u8; 0x0c],
    d2_tadr: D2Tadr,
    _reserved16: [u8; 0x0c],
    d2_asr0: D2Asr0,
    _reserved17: [u8; 0x0c],
    d2_asr1: D2Asr1,
    _reserved18: [u8; 0x0fac],
    d3_chcr: D3Chcr,
    _reserved19: [u8; 0x0c],
    d3_madr: D3Madr,
    _reserved20: [u8; 0x0c],
    d3_qwc: D3Qwc,
    _reserved21: [u8; 0x03dc],
    d4_chcr: D4Chcr,
    _reserved22: [u8; 0x0c],
    d4_madr: D4Madr,
    _reserved23: [u8; 0x0c],
    d4_qwc: D4Qwc,
    _reserved24: [u8; 0x0c],
    d4_tadr: D4Tadr,
    _reserved25: [u8; 0x0bcc],
    d5_chcr: D5Chcr,
    _reserved26: [u8; 0x0c],
    d5_madr: D5Madr,
    _reserved27: [u8; 0x0c],
    d5_qwc: D5Qwc,
    _reserved28: [u8; 0x03dc],
    d6_chcr: D6Chcr,
    _reserved29: [u8; 0x0c],
    d6_madr: D6Madr,
    _reserved30: [u8; 0x0c],
    d6_qwc: D6Qwc,
    _reserved31: [u8; 0x0c],
    d6_tadr: D6Tadr,
    _reserved32: [u8; 0x03cc],
    d7_chcr: D7Chcr,
    _reserved33: [u8; 0x0c],
    d7_madr: D7Madr,
    _reserved34: [u8; 0x0c],
    d7_qwc: D7Qwc,
    _reserved35: [u8; 0x07dc],
    d8_chcr: D8Chcr,
    _reserved36: [u8; 0x0c],
    d8_madr: D8Madr,
    _reserved37: [u8; 0x0c],
    d8_qwc: D8Qwc,
    _reserved38: [u8; 0x5c],
    d8_sadr: D8Sadr,
    _reserved39: [u8; 0x037c],
    d9_chcr: D9Chcr,
    _reserved40: [u8; 0x0c],
    d9_madr: D9Madr,
    _reserved41: [u8; 0x0c],
    d9_qwc: D9Qwc,
    _reserved42: [u8; 0x5c],
    d9_sadr: D9Sadr,
    _reserved43: [u8; 0x0b7c],
    d_ctrl: DCtrl,
    _reserved44: [u8; 0x0c],
    d_stat: DStat,
    _reserved45: [u8; 0x0c],
    d_pcr: DPcr,
    _reserved46: [u8; 0x0c],
    d_sqwc: DSqwc,
    _reserved47: [u8; 0x0c],
    d_rbsr: DRbsr,
    _reserved48: [u8; 0x0c],
    d_rbor: DRbor,
    _reserved49: [u8; 0x0c],
    d_stadr: DStadr,
    _reserved50: [u8; 0x14bc],
    d_enabler: DEnabler,
    _reserved51: [u8; 0x6c],
    d_enablew: DEnablew,
}
impl RegisterBlock {
    #[doc = "0x00 - Ch0 channel control"]
    #[inline(always)]
    pub const fn d0_chcr(&self) -> &D0Chcr {
        &self.d0_chcr
    }
    #[doc = "0x10 - Ch0 memory address"]
    #[inline(always)]
    pub const fn d0_madr(&self) -> &D0Madr {
        &self.d0_madr
    }
    #[doc = "0x20 - Ch0 quad word count"]
    #[inline(always)]
    pub const fn d0_qwc(&self) -> &D0Qwc {
        &self.d0_qwc
    }
    #[doc = "0x30 - Ch0 tag address"]
    #[inline(always)]
    pub const fn d0_tadr(&self) -> &D0Tadr {
        &self.d0_tadr
    }
    #[doc = "0x40 - Ch0 address stack 0"]
    #[inline(always)]
    pub const fn d0_asr0(&self) -> &D0Asr0 {
        &self.d0_asr0
    }
    #[doc = "0x50 - Ch0 address stack 1"]
    #[inline(always)]
    pub const fn d0_asr1(&self) -> &D0Asr1 {
        &self.d0_asr1
    }
    #[doc = "0x1000 - Ch1 channel control"]
    #[inline(always)]
    pub const fn d1_chcr(&self) -> &D1Chcr {
        &self.d1_chcr
    }
    #[doc = "0x1010 - Ch1 memory address"]
    #[inline(always)]
    pub const fn d1_madr(&self) -> &D1Madr {
        &self.d1_madr
    }
    #[doc = "0x1020 - Ch1 quad word count"]
    #[inline(always)]
    pub const fn d1_qwc(&self) -> &D1Qwc {
        &self.d1_qwc
    }
    #[doc = "0x1030 - Ch1 tag address"]
    #[inline(always)]
    pub const fn d1_tadr(&self) -> &D1Tadr {
        &self.d1_tadr
    }
    #[doc = "0x1040 - Ch1 address stack 0"]
    #[inline(always)]
    pub const fn d1_asr0(&self) -> &D1Asr0 {
        &self.d1_asr0
    }
    #[doc = "0x1050 - Ch1 address stack 1"]
    #[inline(always)]
    pub const fn d1_asr1(&self) -> &D1Asr1 {
        &self.d1_asr1
    }
    #[doc = "0x2000 - Ch2 channel control"]
    #[inline(always)]
    pub const fn d2_chcr(&self) -> &D2Chcr {
        &self.d2_chcr
    }
    #[doc = "0x2010 - Ch2 memory address"]
    #[inline(always)]
    pub const fn d2_madr(&self) -> &D2Madr {
        &self.d2_madr
    }
    #[doc = "0x2020 - Ch2 quad word count"]
    #[inline(always)]
    pub const fn d2_qwc(&self) -> &D2Qwc {
        &self.d2_qwc
    }
    #[doc = "0x2030 - Ch2 tag address"]
    #[inline(always)]
    pub const fn d2_tadr(&self) -> &D2Tadr {
        &self.d2_tadr
    }
    #[doc = "0x2040 - Ch2 address stack 0"]
    #[inline(always)]
    pub const fn d2_asr0(&self) -> &D2Asr0 {
        &self.d2_asr0
    }
    #[doc = "0x2050 - Ch2 address stack 1"]
    #[inline(always)]
    pub const fn d2_asr1(&self) -> &D2Asr1 {
        &self.d2_asr1
    }
    #[doc = "0x3000 - Ch3 channel control"]
    #[inline(always)]
    pub const fn d3_chcr(&self) -> &D3Chcr {
        &self.d3_chcr
    }
    #[doc = "0x3010 - Ch3 memory address"]
    #[inline(always)]
    pub const fn d3_madr(&self) -> &D3Madr {
        &self.d3_madr
    }
    #[doc = "0x3020 - Ch3 quad word count"]
    #[inline(always)]
    pub const fn d3_qwc(&self) -> &D3Qwc {
        &self.d3_qwc
    }
    #[doc = "0x3400 - Ch4 channel control"]
    #[inline(always)]
    pub const fn d4_chcr(&self) -> &D4Chcr {
        &self.d4_chcr
    }
    #[doc = "0x3410 - Ch4 memory address"]
    #[inline(always)]
    pub const fn d4_madr(&self) -> &D4Madr {
        &self.d4_madr
    }
    #[doc = "0x3420 - Ch4 quad word count"]
    #[inline(always)]
    pub const fn d4_qwc(&self) -> &D4Qwc {
        &self.d4_qwc
    }
    #[doc = "0x3430 - Ch4 tag address"]
    #[inline(always)]
    pub const fn d4_tadr(&self) -> &D4Tadr {
        &self.d4_tadr
    }
    #[doc = "0x4000 - Ch5 channel control"]
    #[inline(always)]
    pub const fn d5_chcr(&self) -> &D5Chcr {
        &self.d5_chcr
    }
    #[doc = "0x4010 - Ch5 memory address"]
    #[inline(always)]
    pub const fn d5_madr(&self) -> &D5Madr {
        &self.d5_madr
    }
    #[doc = "0x4020 - Ch5 quad word count"]
    #[inline(always)]
    pub const fn d5_qwc(&self) -> &D5Qwc {
        &self.d5_qwc
    }
    #[doc = "0x4400 - Ch6 channel control"]
    #[inline(always)]
    pub const fn d6_chcr(&self) -> &D6Chcr {
        &self.d6_chcr
    }
    #[doc = "0x4410 - Ch6 memory address"]
    #[inline(always)]
    pub const fn d6_madr(&self) -> &D6Madr {
        &self.d6_madr
    }
    #[doc = "0x4420 - Ch6 quad word count"]
    #[inline(always)]
    pub const fn d6_qwc(&self) -> &D6Qwc {
        &self.d6_qwc
    }
    #[doc = "0x4430 - Ch6 tag address"]
    #[inline(always)]
    pub const fn d6_tadr(&self) -> &D6Tadr {
        &self.d6_tadr
    }
    #[doc = "0x4800 - Ch7 channel control"]
    #[inline(always)]
    pub const fn d7_chcr(&self) -> &D7Chcr {
        &self.d7_chcr
    }
    #[doc = "0x4810 - Ch7 memory address"]
    #[inline(always)]
    pub const fn d7_madr(&self) -> &D7Madr {
        &self.d7_madr
    }
    #[doc = "0x4820 - Ch7 quad word count"]
    #[inline(always)]
    pub const fn d7_qwc(&self) -> &D7Qwc {
        &self.d7_qwc
    }
    #[doc = "0x5000 - Ch8 channel control"]
    #[inline(always)]
    pub const fn d8_chcr(&self) -> &D8Chcr {
        &self.d8_chcr
    }
    #[doc = "0x5010 - Ch8 memory address"]
    #[inline(always)]
    pub const fn d8_madr(&self) -> &D8Madr {
        &self.d8_madr
    }
    #[doc = "0x5020 - Ch8 quad word count"]
    #[inline(always)]
    pub const fn d8_qwc(&self) -> &D8Qwc {
        &self.d8_qwc
    }
    #[doc = "0x5080 - Ch8 SPR address"]
    #[inline(always)]
    pub const fn d8_sadr(&self) -> &D8Sadr {
        &self.d8_sadr
    }
    #[doc = "0x5400 - Ch9 channel control"]
    #[inline(always)]
    pub const fn d9_chcr(&self) -> &D9Chcr {
        &self.d9_chcr
    }
    #[doc = "0x5410 - Ch9 memory address"]
    #[inline(always)]
    pub const fn d9_madr(&self) -> &D9Madr {
        &self.d9_madr
    }
    #[doc = "0x5420 - Ch9 quad word count"]
    #[inline(always)]
    pub const fn d9_qwc(&self) -> &D9Qwc {
        &self.d9_qwc
    }
    #[doc = "0x5480 - Ch9 SPR address"]
    #[inline(always)]
    pub const fn d9_sadr(&self) -> &D9Sadr {
        &self.d9_sadr
    }
    #[doc = "0x6000 - DMAC control"]
    #[inline(always)]
    pub const fn d_ctrl(&self) -> &DCtrl {
        &self.d_ctrl
    }
    #[doc = "0x6010 - DMAC status"]
    #[inline(always)]
    pub const fn d_stat(&self) -> &DStat {
        &self.d_stat
    }
    #[doc = "0x6020 - DMAC priority control"]
    #[inline(always)]
    pub const fn d_pcr(&self) -> &DPcr {
        &self.d_pcr
    }
    #[doc = "0x6030 - DMAC skip quad word"]
    #[inline(always)]
    pub const fn d_sqwc(&self) -> &DSqwc {
        &self.d_sqwc
    }
    #[doc = "0x6040 - DMAC ring buffer size"]
    #[inline(always)]
    pub const fn d_rbsr(&self) -> &DRbsr {
        &self.d_rbsr
    }
    #[doc = "0x6050 - DMAC ring buffer offset"]
    #[inline(always)]
    pub const fn d_rbor(&self) -> &DRbor {
        &self.d_rbor
    }
    #[doc = "0x6060 - DMAC stall address"]
    #[inline(always)]
    pub const fn d_stadr(&self) -> &DStadr {
        &self.d_stadr
    }
    #[doc = "0x7520 - Acquistion of DMA suspend status"]
    #[inline(always)]
    pub const fn d_enabler(&self) -> &DEnabler {
        &self.d_enabler
    }
    #[doc = "0x7590 - DMA suspend control"]
    #[inline(always)]
    pub const fn d_enablew(&self) -> &DEnablew {
        &self.d_enablew
    }
}
#[doc = "D0_CHCR (rw) register accessor: Ch0 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0_chcr`] module"]
#[doc(alias = "D0_CHCR")]
pub type D0Chcr = crate::Reg<d0_chcr::D0ChcrSpec>;
#[doc = "Ch0 channel control"]
pub mod d0_chcr;
#[doc = "D0_MADR (rw) register accessor: Ch0 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0_madr`] module"]
#[doc(alias = "D0_MADR")]
pub type D0Madr = crate::Reg<d0_madr::D0MadrSpec>;
#[doc = "Ch0 memory address"]
pub mod d0_madr;
#[doc = "D0_QWC (rw) register accessor: Ch0 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0_qwc`] module"]
#[doc(alias = "D0_QWC")]
pub type D0Qwc = crate::Reg<d0_qwc::D0QwcSpec>;
#[doc = "Ch0 quad word count"]
pub mod d0_qwc;
#[doc = "D0_TADR (rw) register accessor: Ch0 tag address\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_tadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_tadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0_tadr`] module"]
#[doc(alias = "D0_TADR")]
pub type D0Tadr = crate::Reg<d0_tadr::D0TadrSpec>;
#[doc = "Ch0 tag address"]
pub mod d0_tadr;
#[doc = "D0_ASR0 (rw) register accessor: Ch0 address stack 0\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_asr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_asr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0_asr0`] module"]
#[doc(alias = "D0_ASR0")]
pub type D0Asr0 = crate::Reg<d0_asr0::D0Asr0Spec>;
#[doc = "Ch0 address stack 0"]
pub mod d0_asr0;
#[doc = "D0_ASR1 (rw) register accessor: Ch0 address stack 1\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_asr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_asr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0_asr1`] module"]
#[doc(alias = "D0_ASR1")]
pub type D0Asr1 = crate::Reg<d0_asr1::D0Asr1Spec>;
#[doc = "Ch0 address stack 1"]
pub mod d0_asr1;
#[doc = "D1_CHCR (rw) register accessor: Ch1 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1_chcr`] module"]
#[doc(alias = "D1_CHCR")]
pub type D1Chcr = crate::Reg<d1_chcr::D1ChcrSpec>;
#[doc = "Ch1 channel control"]
pub mod d1_chcr;
#[doc = "D1_MADR (rw) register accessor: Ch1 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1_madr`] module"]
#[doc(alias = "D1_MADR")]
pub type D1Madr = crate::Reg<d1_madr::D1MadrSpec>;
#[doc = "Ch1 memory address"]
pub mod d1_madr;
#[doc = "D1_QWC (rw) register accessor: Ch1 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1_qwc`] module"]
#[doc(alias = "D1_QWC")]
pub type D1Qwc = crate::Reg<d1_qwc::D1QwcSpec>;
#[doc = "Ch1 quad word count"]
pub mod d1_qwc;
#[doc = "D1_TADR (rw) register accessor: Ch1 tag address\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_tadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_tadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1_tadr`] module"]
#[doc(alias = "D1_TADR")]
pub type D1Tadr = crate::Reg<d1_tadr::D1TadrSpec>;
#[doc = "Ch1 tag address"]
pub mod d1_tadr;
#[doc = "D1_ASR0 (rw) register accessor: Ch1 address stack 0\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_asr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_asr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1_asr0`] module"]
#[doc(alias = "D1_ASR0")]
pub type D1Asr0 = crate::Reg<d1_asr0::D1Asr0Spec>;
#[doc = "Ch1 address stack 0"]
pub mod d1_asr0;
#[doc = "D1_ASR1 (rw) register accessor: Ch1 address stack 1\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_asr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_asr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1_asr1`] module"]
#[doc(alias = "D1_ASR1")]
pub type D1Asr1 = crate::Reg<d1_asr1::D1Asr1Spec>;
#[doc = "Ch1 address stack 1"]
pub mod d1_asr1;
#[doc = "D2_CHCR (rw) register accessor: Ch2 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2_chcr`] module"]
#[doc(alias = "D2_CHCR")]
pub type D2Chcr = crate::Reg<d2_chcr::D2ChcrSpec>;
#[doc = "Ch2 channel control"]
pub mod d2_chcr;
#[doc = "D2_MADR (rw) register accessor: Ch2 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2_madr`] module"]
#[doc(alias = "D2_MADR")]
pub type D2Madr = crate::Reg<d2_madr::D2MadrSpec>;
#[doc = "Ch2 memory address"]
pub mod d2_madr;
#[doc = "D2_QWC (rw) register accessor: Ch2 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2_qwc`] module"]
#[doc(alias = "D2_QWC")]
pub type D2Qwc = crate::Reg<d2_qwc::D2QwcSpec>;
#[doc = "Ch2 quad word count"]
pub mod d2_qwc;
#[doc = "D2_TADR (rw) register accessor: Ch2 tag address\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_tadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_tadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2_tadr`] module"]
#[doc(alias = "D2_TADR")]
pub type D2Tadr = crate::Reg<d2_tadr::D2TadrSpec>;
#[doc = "Ch2 tag address"]
pub mod d2_tadr;
#[doc = "D2_ASR0 (rw) register accessor: Ch2 address stack 0\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_asr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_asr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2_asr0`] module"]
#[doc(alias = "D2_ASR0")]
pub type D2Asr0 = crate::Reg<d2_asr0::D2Asr0Spec>;
#[doc = "Ch2 address stack 0"]
pub mod d2_asr0;
#[doc = "D2_ASR1 (rw) register accessor: Ch2 address stack 1\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_asr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_asr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2_asr1`] module"]
#[doc(alias = "D2_ASR1")]
pub type D2Asr1 = crate::Reg<d2_asr1::D2Asr1Spec>;
#[doc = "Ch2 address stack 1"]
pub mod d2_asr1;
#[doc = "D3_CHCR (rw) register accessor: Ch3 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d3_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3_chcr`] module"]
#[doc(alias = "D3_CHCR")]
pub type D3Chcr = crate::Reg<d3_chcr::D3ChcrSpec>;
#[doc = "Ch3 channel control"]
pub mod d3_chcr;
#[doc = "D3_MADR (rw) register accessor: Ch3 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d3_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3_madr`] module"]
#[doc(alias = "D3_MADR")]
pub type D3Madr = crate::Reg<d3_madr::D3MadrSpec>;
#[doc = "Ch3 memory address"]
pub mod d3_madr;
#[doc = "D3_QWC (rw) register accessor: Ch3 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d3_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3_qwc`] module"]
#[doc(alias = "D3_QWC")]
pub type D3Qwc = crate::Reg<d3_qwc::D3QwcSpec>;
#[doc = "Ch3 quad word count"]
pub mod d3_qwc;
#[doc = "D4_CHCR (rw) register accessor: Ch4 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d4_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d4_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d4_chcr`] module"]
#[doc(alias = "D4_CHCR")]
pub type D4Chcr = crate::Reg<d4_chcr::D4ChcrSpec>;
#[doc = "Ch4 channel control"]
pub mod d4_chcr;
#[doc = "D4_MADR (rw) register accessor: Ch4 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d4_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d4_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d4_madr`] module"]
#[doc(alias = "D4_MADR")]
pub type D4Madr = crate::Reg<d4_madr::D4MadrSpec>;
#[doc = "Ch4 memory address"]
pub mod d4_madr;
#[doc = "D4_QWC (rw) register accessor: Ch4 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d4_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d4_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d4_qwc`] module"]
#[doc(alias = "D4_QWC")]
pub type D4Qwc = crate::Reg<d4_qwc::D4QwcSpec>;
#[doc = "Ch4 quad word count"]
pub mod d4_qwc;
#[doc = "D4_TADR (rw) register accessor: Ch4 tag address\n\nYou can [`read`](crate::Reg::read) this register and get [`d4_tadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d4_tadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d4_tadr`] module"]
#[doc(alias = "D4_TADR")]
pub type D4Tadr = crate::Reg<d4_tadr::D4TadrSpec>;
#[doc = "Ch4 tag address"]
pub mod d4_tadr;
#[doc = "D5_CHCR (rw) register accessor: Ch5 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d5_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d5_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d5_chcr`] module"]
#[doc(alias = "D5_CHCR")]
pub type D5Chcr = crate::Reg<d5_chcr::D5ChcrSpec>;
#[doc = "Ch5 channel control"]
pub mod d5_chcr;
#[doc = "D5_MADR (rw) register accessor: Ch5 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d5_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d5_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d5_madr`] module"]
#[doc(alias = "D5_MADR")]
pub type D5Madr = crate::Reg<d5_madr::D5MadrSpec>;
#[doc = "Ch5 memory address"]
pub mod d5_madr;
#[doc = "D5_QWC (rw) register accessor: Ch5 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d5_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d5_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d5_qwc`] module"]
#[doc(alias = "D5_QWC")]
pub type D5Qwc = crate::Reg<d5_qwc::D5QwcSpec>;
#[doc = "Ch5 quad word count"]
pub mod d5_qwc;
#[doc = "D6_CHCR (rw) register accessor: Ch6 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d6_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d6_chcr`] module"]
#[doc(alias = "D6_CHCR")]
pub type D6Chcr = crate::Reg<d6_chcr::D6ChcrSpec>;
#[doc = "Ch6 channel control"]
pub mod d6_chcr;
#[doc = "D6_MADR (rw) register accessor: Ch6 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d6_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d6_madr`] module"]
#[doc(alias = "D6_MADR")]
pub type D6Madr = crate::Reg<d6_madr::D6MadrSpec>;
#[doc = "Ch6 memory address"]
pub mod d6_madr;
#[doc = "D6_QWC (rw) register accessor: Ch6 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d6_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d6_qwc`] module"]
#[doc(alias = "D6_QWC")]
pub type D6Qwc = crate::Reg<d6_qwc::D6QwcSpec>;
#[doc = "Ch6 quad word count"]
pub mod d6_qwc;
#[doc = "D6_TADR (rw) register accessor: Ch6 tag address\n\nYou can [`read`](crate::Reg::read) this register and get [`d6_tadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6_tadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d6_tadr`] module"]
#[doc(alias = "D6_TADR")]
pub type D6Tadr = crate::Reg<d6_tadr::D6TadrSpec>;
#[doc = "Ch6 tag address"]
pub mod d6_tadr;
#[doc = "D7_CHCR (rw) register accessor: Ch7 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d7_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d7_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d7_chcr`] module"]
#[doc(alias = "D7_CHCR")]
pub type D7Chcr = crate::Reg<d7_chcr::D7ChcrSpec>;
#[doc = "Ch7 channel control"]
pub mod d7_chcr;
#[doc = "D7_MADR (rw) register accessor: Ch7 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d7_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d7_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d7_madr`] module"]
#[doc(alias = "D7_MADR")]
pub type D7Madr = crate::Reg<d7_madr::D7MadrSpec>;
#[doc = "Ch7 memory address"]
pub mod d7_madr;
#[doc = "D7_QWC (rw) register accessor: Ch7 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d7_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d7_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d7_qwc`] module"]
#[doc(alias = "D7_QWC")]
pub type D7Qwc = crate::Reg<d7_qwc::D7QwcSpec>;
#[doc = "Ch7 quad word count"]
pub mod d7_qwc;
#[doc = "D8_CHCR (rw) register accessor: Ch8 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d8_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d8_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d8_chcr`] module"]
#[doc(alias = "D8_CHCR")]
pub type D8Chcr = crate::Reg<d8_chcr::D8ChcrSpec>;
#[doc = "Ch8 channel control"]
pub mod d8_chcr;
#[doc = "D8_MADR (rw) register accessor: Ch8 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d8_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d8_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d8_madr`] module"]
#[doc(alias = "D8_MADR")]
pub type D8Madr = crate::Reg<d8_madr::D8MadrSpec>;
#[doc = "Ch8 memory address"]
pub mod d8_madr;
#[doc = "D8_QWC (rw) register accessor: Ch8 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d8_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d8_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d8_qwc`] module"]
#[doc(alias = "D8_QWC")]
pub type D8Qwc = crate::Reg<d8_qwc::D8QwcSpec>;
#[doc = "Ch8 quad word count"]
pub mod d8_qwc;
#[doc = "D8_SADR (rw) register accessor: Ch8 SPR address\n\nYou can [`read`](crate::Reg::read) this register and get [`d8_sadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d8_sadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d8_sadr`] module"]
#[doc(alias = "D8_SADR")]
pub type D8Sadr = crate::Reg<d8_sadr::D8SadrSpec>;
#[doc = "Ch8 SPR address"]
pub mod d8_sadr;
#[doc = "D9_CHCR (rw) register accessor: Ch9 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d9_chcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d9_chcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d9_chcr`] module"]
#[doc(alias = "D9_CHCR")]
pub type D9Chcr = crate::Reg<d9_chcr::D9ChcrSpec>;
#[doc = "Ch9 channel control"]
pub mod d9_chcr;
#[doc = "D9_MADR (rw) register accessor: Ch9 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d9_madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d9_madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d9_madr`] module"]
#[doc(alias = "D9_MADR")]
pub type D9Madr = crate::Reg<d9_madr::D9MadrSpec>;
#[doc = "Ch9 memory address"]
pub mod d9_madr;
#[doc = "D9_QWC (rw) register accessor: Ch9 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d9_qwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d9_qwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d9_qwc`] module"]
#[doc(alias = "D9_QWC")]
pub type D9Qwc = crate::Reg<d9_qwc::D9QwcSpec>;
#[doc = "Ch9 quad word count"]
pub mod d9_qwc;
#[doc = "D9_SADR (rw) register accessor: Ch9 SPR address\n\nYou can [`read`](crate::Reg::read) this register and get [`d9_sadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d9_sadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d9_sadr`] module"]
#[doc(alias = "D9_SADR")]
pub type D9Sadr = crate::Reg<d9_sadr::D9SadrSpec>;
#[doc = "Ch9 SPR address"]
pub mod d9_sadr;
#[doc = "D_CTRL (rw) register accessor: DMAC control\n\nYou can [`read`](crate::Reg::read) this register and get [`d_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_ctrl`] module"]
#[doc(alias = "D_CTRL")]
pub type DCtrl = crate::Reg<d_ctrl::DCtrlSpec>;
#[doc = "DMAC control"]
pub mod d_ctrl;
#[doc = "D_STAT (rw) register accessor: DMAC status\n\nYou can [`read`](crate::Reg::read) this register and get [`d_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_stat`] module"]
#[doc(alias = "D_STAT")]
pub type DStat = crate::Reg<d_stat::DStatSpec>;
#[doc = "DMAC status"]
pub mod d_stat;
#[doc = "D_PCR (rw) register accessor: DMAC priority control\n\nYou can [`read`](crate::Reg::read) this register and get [`d_pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_pcr`] module"]
#[doc(alias = "D_PCR")]
pub type DPcr = crate::Reg<d_pcr::DPcrSpec>;
#[doc = "DMAC priority control"]
pub mod d_pcr;
#[doc = "D_SQWC (rw) register accessor: DMAC skip quad word\n\nYou can [`read`](crate::Reg::read) this register and get [`d_sqwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_sqwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_sqwc`] module"]
#[doc(alias = "D_SQWC")]
pub type DSqwc = crate::Reg<d_sqwc::DSqwcSpec>;
#[doc = "DMAC skip quad word"]
pub mod d_sqwc;
#[doc = "D_RBSR (rw) register accessor: DMAC ring buffer size\n\nYou can [`read`](crate::Reg::read) this register and get [`d_rbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_rbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_rbsr`] module"]
#[doc(alias = "D_RBSR")]
pub type DRbsr = crate::Reg<d_rbsr::DRbsrSpec>;
#[doc = "DMAC ring buffer size"]
pub mod d_rbsr;
#[doc = "D_RBOR (rw) register accessor: DMAC ring buffer offset\n\nYou can [`read`](crate::Reg::read) this register and get [`d_rbor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_rbor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_rbor`] module"]
#[doc(alias = "D_RBOR")]
pub type DRbor = crate::Reg<d_rbor::DRborSpec>;
#[doc = "DMAC ring buffer offset"]
pub mod d_rbor;
#[doc = "D_STADR (rw) register accessor: DMAC stall address\n\nYou can [`read`](crate::Reg::read) this register and get [`d_stadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_stadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_stadr`] module"]
#[doc(alias = "D_STADR")]
pub type DStadr = crate::Reg<d_stadr::DStadrSpec>;
#[doc = "DMAC stall address"]
pub mod d_stadr;
#[doc = "D_ENABLER (r) register accessor: Acquistion of DMA suspend status\n\nYou can [`read`](crate::Reg::read) this register and get [`d_enabler::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_enabler`] module"]
#[doc(alias = "D_ENABLER")]
pub type DEnabler = crate::Reg<d_enabler::DEnablerSpec>;
#[doc = "Acquistion of DMA suspend status"]
pub mod d_enabler;
#[doc = "D_ENABLEW (w) register accessor: DMA suspend control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_enablew::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_enablew`] module"]
#[doc(alias = "D_ENABLEW")]
pub type DEnablew = crate::Reg<d_enablew::DEnablewSpec>;
#[doc = "DMA suspend control"]
pub mod d_enablew;
