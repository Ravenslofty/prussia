#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipu_cmd: IpuCmd,
    _reserved1: [u8; 0x08],
    ipu_ctrl: IpuCtrl,
    _reserved2: [u8; 0x0c],
    ipu_bp: IpuBp,
    _reserved3: [u8; 0x0c],
    ipu_top: IpuTop,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - IPU Decoded-code read register / command register"]
    #[inline(always)]
    pub const fn ipu_cmd(&self) -> &IpuCmd {
        &self.ipu_cmd
    }
    #[doc = "0x10 - IPU control"]
    #[inline(always)]
    pub const fn ipu_ctrl(&self) -> &IpuCtrl {
        &self.ipu_ctrl
    }
    #[doc = "0x20 - IPU input FIFO control."]
    #[inline(always)]
    pub const fn ipu_bp(&self) -> &IpuBp {
        &self.ipu_bp
    }
    #[doc = "0x30..0x38 - Reads the first 32 bits of the bit stream after execution of a BDEC / IDEC / VDEC / FDEC command completes."]
    #[inline(always)]
    pub const fn ipu_top(&self) -> &IpuTop {
        &self.ipu_top
    }
}
#[doc = "IPU_CMD (rw) register accessor: IPU Decoded-code read register / command register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipu_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipu_cmd`] module"]
#[doc(alias = "IPU_CMD")]
pub type IpuCmd = crate::Reg<ipu_cmd::IpuCmdSpec>;
#[doc = "IPU Decoded-code read register / command register"]
pub mod ipu_cmd;
#[doc = "IPU_CTRL (rw) register accessor: IPU control\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipu_ctrl`] module"]
#[doc(alias = "IPU_CTRL")]
pub type IpuCtrl = crate::Reg<ipu_ctrl::IpuCtrlSpec>;
#[doc = "IPU control"]
pub mod ipu_ctrl;
#[doc = "IPU_BP (r) register accessor: IPU input FIFO control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_bp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipu_bp`] module"]
#[doc(alias = "IPU_BP")]
pub type IpuBp = crate::Reg<ipu_bp::IpuBpSpec>;
#[doc = "IPU input FIFO control."]
pub mod ipu_bp;
#[doc = "IPU_TOP (r) register accessor: Reads the first 32 bits of the bit stream after execution of a BDEC / IDEC / VDEC / FDEC command completes.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_top::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipu_top`] module"]
#[doc(alias = "IPU_TOP")]
pub type IpuTop = crate::Reg<ipu_top::IpuTopSpec>;
#[doc = "Reads the first 32 bits of the bit stream after execution of a BDEC / IDEC / VDEC / FDEC command completes."]
pub mod ipu_top;
