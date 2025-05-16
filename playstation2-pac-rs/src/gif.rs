#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gif_ctrl: GifCtrl,
    _reserved1: [u8; 0x0c],
    gif_mode: GifMode,
    _reserved2: [u8; 0x0c],
    gif_stat: GifStat,
    _reserved3: [u8; 0x1c],
    gif_tag0: GifTag0,
    _reserved4: [u8; 0x0c],
    gif_tag1: GifTag1,
    _reserved5: [u8; 0x0c],
    gif_tag2: GifTag2,
    _reserved6: [u8; 0x0c],
    gif_tag3: GifTag3,
    _reserved7: [u8; 0x0c],
    gif_cnt: GifCnt,
    _reserved8: [u8; 0x0c],
    gif_p3cnt: GifP3cnt,
    _reserved9: [u8; 0x0c],
    gif_p3tag: GifP3tag,
}
impl RegisterBlock {
    #[doc = "0x00 - GIF control"]
    #[inline(always)]
    pub const fn gif_ctrl(&self) -> &GifCtrl {
        &self.gif_ctrl
    }
    #[doc = "0x10 - GIF mode setting"]
    #[inline(always)]
    pub const fn gif_mode(&self) -> &GifMode {
        &self.gif_mode
    }
    #[doc = "0x20 - GIF status"]
    #[inline(always)]
    pub const fn gif_stat(&self) -> &GifStat {
        &self.gif_stat
    }
    #[doc = "0x40 - GIFtag (bits 31-16) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
    #[inline(always)]
    pub const fn gif_tag0(&self) -> &GifTag0 {
        &self.gif_tag0
    }
    #[doc = "0x50 - GIFtag (bits 45-32) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
    #[inline(always)]
    pub const fn gif_tag1(&self) -> &GifTag1 {
        &self.gif_tag1
    }
    #[doc = "0x60 - GIFtag (bits 95-64) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
    #[inline(always)]
    pub const fn gif_tag2(&self) -> &GifTag2 {
        &self.gif_tag2
    }
    #[doc = "0x70 - GIFtag (bits 127-96) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
    #[inline(always)]
    pub const fn gif_tag3(&self) -> &GifTag3 {
        &self.gif_tag3
    }
    #[doc = "0x80 - Transfer status counter. Only accessible when GIF_CTRL PSE is set to 1."]
    #[inline(always)]
    pub const fn gif_cnt(&self) -> &GifCnt {
        &self.gif_cnt
    }
    #[doc = "0x90 - PATH3 transfer status counter. Only accessible when GIF_CTRL PSE is set to 1."]
    #[inline(always)]
    pub const fn gif_p3cnt(&self) -> &GifP3cnt {
        &self.gif_p3cnt
    }
    #[doc = "0xa0 - PATH3 tag value. Only accessible when GIF_CTRL PSE is set to 1 and when PATH3 is transferring data in IMAGE mode."]
    #[inline(always)]
    pub const fn gif_p3tag(&self) -> &GifP3tag {
        &self.gif_p3tag
    }
}
#[doc = "GIF_CTRL (w) register accessor: GIF control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gif_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_ctrl`] module"]
#[doc(alias = "GIF_CTRL")]
pub type GifCtrl = crate::Reg<gif_ctrl::GifCtrlSpec>;
#[doc = "GIF control"]
pub mod gif_ctrl;
#[doc = "GIF_MODE (w) register accessor: GIF mode setting\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gif_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_mode`] module"]
#[doc(alias = "GIF_MODE")]
pub type GifMode = crate::Reg<gif_mode::GifModeSpec>;
#[doc = "GIF mode setting"]
pub mod gif_mode;
#[doc = "GIF_STAT (r) register accessor: GIF status\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_stat`] module"]
#[doc(alias = "GIF_STAT")]
pub type GifStat = crate::Reg<gif_stat::GifStatSpec>;
#[doc = "GIF status"]
pub mod gif_stat;
#[doc = "GIF_TAG0 (r) register accessor: GIFtag (bits 31-16) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_tag0`] module"]
#[doc(alias = "GIF_TAG0")]
pub type GifTag0 = crate::Reg<gif_tag0::GifTag0Spec>;
#[doc = "GIFtag (bits 31-16) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
pub mod gif_tag0;
#[doc = "GIF_TAG1 (r) register accessor: GIFtag (bits 45-32) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_tag1`] module"]
#[doc(alias = "GIF_TAG1")]
pub type GifTag1 = crate::Reg<gif_tag1::GifTag1Spec>;
#[doc = "GIFtag (bits 45-32) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
pub mod gif_tag1;
#[doc = "GIF_TAG2 (r) register accessor: GIFtag (bits 95-64) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_tag2`] module"]
#[doc(alias = "GIF_TAG2")]
pub type GifTag2 = crate::Reg<gif_tag2::GifTag2Spec>;
#[doc = "GIFtag (bits 95-64) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
pub mod gif_tag2;
#[doc = "GIF_TAG3 (r) register accessor: GIFtag (bits 127-96) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_tag3`] module"]
#[doc(alias = "GIF_TAG3")]
pub type GifTag3 = crate::Reg<gif_tag3::GifTag3Spec>;
#[doc = "GIFtag (bits 127-96) immediately before. Only accessible when GIF_CTRL PSE is set to 1."]
pub mod gif_tag3;
#[doc = "GIF_CNT (r) register accessor: Transfer status counter. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_cnt`] module"]
#[doc(alias = "GIF_CNT")]
pub type GifCnt = crate::Reg<gif_cnt::GifCntSpec>;
#[doc = "Transfer status counter. Only accessible when GIF_CTRL PSE is set to 1."]
pub mod gif_cnt;
#[doc = "GIF_P3CNT (r) register accessor: PATH3 transfer status counter. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_p3cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_p3cnt`] module"]
#[doc(alias = "GIF_P3CNT")]
pub type GifP3cnt = crate::Reg<gif_p3cnt::GifP3cntSpec>;
#[doc = "PATH3 transfer status counter. Only accessible when GIF_CTRL PSE is set to 1."]
pub mod gif_p3cnt;
#[doc = "GIF_P3TAG (r) register accessor: PATH3 tag value. Only accessible when GIF_CTRL PSE is set to 1 and when PATH3 is transferring data in IMAGE mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_p3tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gif_p3tag`] module"]
#[doc(alias = "GIF_P3TAG")]
pub type GifP3tag = crate::Reg<gif_p3tag::GifP3tagSpec>;
#[doc = "PATH3 tag value. Only accessible when GIF_CTRL PSE is set to 1 and when PATH3 is transferring data in IMAGE mode."]
pub mod gif_p3tag;
