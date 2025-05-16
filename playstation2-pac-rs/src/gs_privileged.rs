#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmode: Pmode,
    _reserved1: [u8; 0x08],
    smode1: Smode1,
    _reserved2: [u8; 0x08],
    smode2: Smode2,
    _reserved3: [u8; 0x08],
    srfsh: Srfsh,
    _reserved4: [u8; 0x08],
    synch1: Synch1,
    _reserved5: [u8; 0x08],
    synch2: Synch2,
    _reserved6: [u8; 0x08],
    syncv: Syncv,
    _reserved7: [u8; 0x08],
    dispfb1: Dispfb1,
    _reserved8: [u8; 0x08],
    display1: Display1,
    _reserved9: [u8; 0x08],
    dispfb2: Dispfb2,
    _reserved10: [u8; 0x08],
    display2: Display2,
    _reserved11: [u8; 0x08],
    extbuf: Extbuf,
    _reserved12: [u8; 0x08],
    extdata: Extdata,
    _reserved13: [u8; 0x08],
    extwrite: Extwrite,
    _reserved14: [u8; 0x08],
    bgcolor: Bgcolor,
    _reserved15: [u8; 0x0f18],
    csr: Csr,
    _reserved16: [u8; 0x08],
    imr: Imr,
    _reserved17: [u8; 0x28],
    busdir: Busdir,
    _reserved18: [u8; 0x38],
    siglblid: Siglblid,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Various PCRTC controls. Accessible via EE."]
    #[inline(always)]
    pub const fn pmode(&self) -> &Pmode {
        &self.pmode
    }
    #[doc = "0x10..0x18 - Video signal settings. Sync parameters for the PCRTC."]
    #[inline(always)]
    pub const fn smode1(&self) -> &Smode1 {
        &self.smode1
    }
    #[doc = "0x20..0x28 - Video mode settings for synchronisation."]
    #[inline(always)]
    pub const fn smode2(&self) -> &Smode2 {
        &self.smode2
    }
    #[doc = "0x30..0x38 - GS DRAM refresh rate."]
    #[inline(always)]
    pub const fn srfsh(&self) -> &Srfsh {
        &self.srfsh
    }
    #[doc = "0x40..0x48 - Sync-related. Specific purpose unknown."]
    #[inline(always)]
    pub const fn synch1(&self) -> &Synch1 {
        &self.synch1
    }
    #[doc = "0x50..0x58 - Sync-related. Specific purpose unknown."]
    #[inline(always)]
    pub const fn synch2(&self) -> &Synch2 {
        &self.synch2
    }
    #[doc = "0x60..0x68 - VSync-related interval timins (?)"]
    #[inline(always)]
    pub const fn syncv(&self) -> &Syncv {
        &self.syncv
    }
    #[doc = "0x70..0x78 - Framebuffer register for Output Circuit 1."]
    #[inline(always)]
    pub const fn dispfb1(&self) -> &Dispfb1 {
        &self.dispfb1
    }
    #[doc = "0x80..0x88 - Rectangular viewport from output of Read Circuit 1."]
    #[inline(always)]
    pub const fn display1(&self) -> &Display1 {
        &self.display1
    }
    #[doc = "0x90..0x98 - Framebuffer register for Output Circuit 2."]
    #[inline(always)]
    pub const fn dispfb2(&self) -> &Dispfb2 {
        &self.dispfb2
    }
    #[doc = "0xa0..0xa8 - Rectangular viewport from output of Read Circuit 1."]
    #[inline(always)]
    pub const fn display2(&self) -> &Display2 {
        &self.display2
    }
    #[doc = "0xb0..0xb8 - Feedback write buffer."]
    #[inline(always)]
    pub const fn extbuf(&self) -> &Extbuf {
        &self.extbuf
    }
    #[doc = "0xc0..0xc8 - Feedback write setting."]
    #[inline(always)]
    pub const fn extdata(&self) -> &Extdata {
        &self.extdata
    }
    #[doc = "0xd0..0xd8 - Feedback write function control."]
    #[inline(always)]
    pub const fn extwrite(&self) -> &Extwrite {
        &self.extwrite
    }
    #[doc = "0xe0..0xe8 - Background colour."]
    #[inline(always)]
    pub const fn bgcolor(&self) -> &Bgcolor {
        &self.bgcolor
    }
    #[doc = "0x1000..0x1008 - GS system status and control registers."]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x1010..0x1018 - Interrupt mask control."]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1040..0x1048 - GS bus direction."]
    #[inline(always)]
    pub const fn busdir(&self) -> &Busdir {
        &self.busdir
    }
    #[doc = "0x1080..0x1088 - Signal Id value."]
    #[inline(always)]
    pub const fn siglblid(&self) -> &Siglblid {
        &self.siglblid
    }
}
#[doc = "PMODE (w) register accessor: Various PCRTC controls. Accessible via EE.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmode`] module"]
#[doc(alias = "PMODE")]
pub type Pmode = crate::Reg<pmode::PmodeSpec>;
#[doc = "Various PCRTC controls. Accessible via EE."]
pub mod pmode;
#[doc = "SMODE1 (w) register accessor: Video signal settings. Sync parameters for the PCRTC.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smode1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smode1`] module"]
#[doc(alias = "SMODE1")]
pub type Smode1 = crate::Reg<smode1::Smode1Spec>;
#[doc = "Video signal settings. Sync parameters for the PCRTC."]
pub mod smode1;
#[doc = "SMODE2 (w) register accessor: Video mode settings for synchronisation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smode2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smode2`] module"]
#[doc(alias = "SMODE2")]
pub type Smode2 = crate::Reg<smode2::Smode2Spec>;
#[doc = "Video mode settings for synchronisation."]
pub mod smode2;
#[doc = "SRFSH (w) register accessor: GS DRAM refresh rate.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srfsh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srfsh`] module"]
#[doc(alias = "SRFSH")]
pub type Srfsh = crate::Reg<srfsh::SrfshSpec>;
#[doc = "GS DRAM refresh rate."]
pub mod srfsh;
#[doc = "SYNCH1 (w) register accessor: Sync-related. Specific purpose unknown.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synch1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synch1`] module"]
#[doc(alias = "SYNCH1")]
pub type Synch1 = crate::Reg<synch1::Synch1Spec>;
#[doc = "Sync-related. Specific purpose unknown."]
pub mod synch1;
#[doc = "SYNCH2 (w) register accessor: Sync-related. Specific purpose unknown.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synch2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synch2`] module"]
#[doc(alias = "SYNCH2")]
pub type Synch2 = crate::Reg<synch2::Synch2Spec>;
#[doc = "Sync-related. Specific purpose unknown."]
pub mod synch2;
#[doc = "SYNCV (w) register accessor: VSync-related interval timins (?)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncv`] module"]
#[doc(alias = "SYNCV")]
pub type Syncv = crate::Reg<syncv::SyncvSpec>;
#[doc = "VSync-related interval timins (?)"]
pub mod syncv;
#[doc = "DISPFB1 (w) register accessor: Framebuffer register for Output Circuit 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispfb1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dispfb1`] module"]
#[doc(alias = "DISPFB1")]
pub type Dispfb1 = crate::Reg<dispfb1::Dispfb1Spec>;
#[doc = "Framebuffer register for Output Circuit 1."]
pub mod dispfb1;
#[doc = "DISPLAY1 (w) register accessor: Rectangular viewport from output of Read Circuit 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`display1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@display1`] module"]
#[doc(alias = "DISPLAY1")]
pub type Display1 = crate::Reg<display1::Display1Spec>;
#[doc = "Rectangular viewport from output of Read Circuit 1."]
pub mod display1;
pub use dispfb1 as dispfb2;
pub use display1 as display2;
pub use Dispfb1 as Dispfb2;
pub use Display1 as Display2;
#[doc = "EXTBUF (w) register accessor: Feedback write buffer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extbuf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extbuf`] module"]
#[doc(alias = "EXTBUF")]
pub type Extbuf = crate::Reg<extbuf::ExtbufSpec>;
#[doc = "Feedback write buffer."]
pub mod extbuf;
#[doc = "EXTDATA (w) register accessor: Feedback write setting.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extdata`] module"]
#[doc(alias = "EXTDATA")]
pub type Extdata = crate::Reg<extdata::ExtdataSpec>;
#[doc = "Feedback write setting."]
pub mod extdata;
#[doc = "EXTWRITE (w) register accessor: Feedback write function control.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extwrite::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extwrite`] module"]
#[doc(alias = "EXTWRITE")]
pub type Extwrite = crate::Reg<extwrite::ExtwriteSpec>;
#[doc = "Feedback write function control."]
pub mod extwrite;
#[doc = "BGCOLOR (w) register accessor: Background colour.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolor::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcolor`] module"]
#[doc(alias = "BGCOLOR")]
pub type Bgcolor = crate::Reg<bgcolor::BgcolorSpec>;
#[doc = "Background colour."]
pub mod bgcolor;
#[doc = "CSR (rw) register accessor: GS system status and control registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "GS system status and control registers."]
pub mod csr;
#[doc = "IMR (r) register accessor: Interrupt mask control.\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt mask control."]
pub mod imr;
#[doc = "BUSDIR (r) register accessor: GS bus direction.\n\nYou can [`read`](crate::Reg::read) this register and get [`busdir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busdir`] module"]
#[doc(alias = "BUSDIR")]
pub type Busdir = crate::Reg<busdir::BusdirSpec>;
#[doc = "GS bus direction."]
pub mod busdir;
#[doc = "SIGLBLID (r) register accessor: Signal Id value.\n\nYou can [`read`](crate::Reg::read) this register and get [`siglblid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@siglblid`] module"]
#[doc(alias = "SIGLBLID")]
pub type Siglblid = crate::Reg<siglblid::SiglblidSpec>;
#[doc = "Signal Id value."]
pub mod siglblid;
