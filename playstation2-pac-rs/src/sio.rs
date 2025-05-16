#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sio_lcr: SioLcr,
    _reserved1: [u8; 0x0c],
    sio_lsr: SioLsr,
    _reserved2: [u8; 0x0c],
    sio_ier: SioIer,
    _reserved3: [u8; 0x0c],
    sio_isr: SioIsr,
    _reserved4: [u8; 0x0c],
    sio_fcr: SioFcr,
    _reserved5: [u8; 0x0c],
    sio_bgr: SioBgr,
    _reserved6: [u8; 0x2c],
    _reserved_6_sio: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Line Control Register. Controls the serial data format. Matches the same register found in the TMPR4937 CPU."]
    #[inline(always)]
    pub const fn sio_lcr(&self) -> &SioLcr {
        &self.sio_lcr
    }
    #[doc = "0x10 - Line Status Register (aka DMA/Interrupt Status Register). Provides status information about the SIO1 (serial port) channel. Some documentation is loosely based on the TMPR4937 CPU, particularly The SIDISR0/1 register, however it mainly draws from the TX7901."]
    #[inline(always)]
    pub const fn sio_lsr(&self) -> &SioLsr {
        &self.sio_lsr
    }
    #[doc = "0x20 - Interrupt Enable Register. Handles the enabling/disabling of SIO-specific interrupts."]
    #[inline(always)]
    pub const fn sio_ier(&self) -> &SioIer {
        &self.sio_ier
    }
    #[doc = "0x30 - Interrupt Status Register. TODO: Add documentation on register."]
    #[inline(always)]
    pub const fn sio_isr(&self) -> &SioIsr {
        &self.sio_isr
    }
    #[doc = "0x40 - Manages the Rx and Tx FIFO buffers. Additionally, controls the Receive FIFO Trigger level and sets DMA mode 1. Based on TX7901 docs."]
    #[inline(always)]
    pub const fn sio_fcr(&self) -> &SioFcr {
        &self.sio_fcr
    }
    #[doc = "0x50 - Baud Rate Control Register. Configures the pre-scaler and clock frequency for the Baud Rate Generator. The clock signal used is calculated based on the formula `fc / (prescaler x divisor x 16)` where fc is the clock chosen in the Line Control Register (bits 6:5). Based on the TX49 docs."]
    #[inline(always)]
    pub const fn sio_bgr(&self) -> &SioBgr {
        &self.sio_bgr
    }
    #[doc = "0x80 - Rx FIFO buffer."]
    #[inline(always)]
    pub const fn sio_rxfifo(&self) -> &SioRxfifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - Tx FIFO buffer."]
    #[inline(always)]
    pub const fn sio_txfifo(&self) -> &SioTxfifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
}
#[doc = "SIO_LCR (rw) register accessor: Line Control Register. Controls the serial data format. Matches the same register found in the TMPR4937 CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_lcr`] module"]
#[doc(alias = "SIO_LCR")]
pub type SioLcr = crate::Reg<sio_lcr::SioLcrSpec>;
#[doc = "Line Control Register. Controls the serial data format. Matches the same register found in the TMPR4937 CPU."]
pub mod sio_lcr;
#[doc = "SIO_LSR (r) register accessor: Line Status Register (aka DMA/Interrupt Status Register). Provides status information about the SIO1 (serial port) channel. Some documentation is loosely based on the TMPR4937 CPU, particularly The SIDISR0/1 register, however it mainly draws from the TX7901.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_lsr`] module"]
#[doc(alias = "SIO_LSR")]
pub type SioLsr = crate::Reg<sio_lsr::SioLsrSpec>;
#[doc = "Line Status Register (aka DMA/Interrupt Status Register). Provides status information about the SIO1 (serial port) channel. Some documentation is loosely based on the TMPR4937 CPU, particularly The SIDISR0/1 register, however it mainly draws from the TX7901."]
pub mod sio_lsr;
#[doc = "SIO_IER (rw) register accessor: Interrupt Enable Register. Handles the enabling/disabling of SIO-specific interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_ier`] module"]
#[doc(alias = "SIO_IER")]
pub type SioIer = crate::Reg<sio_ier::SioIerSpec>;
#[doc = "Interrupt Enable Register. Handles the enabling/disabling of SIO-specific interrupts."]
pub mod sio_ier;
#[doc = "SIO_ISR (rw) register accessor: Interrupt Status Register. TODO: Add documentation on register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_isr`] module"]
#[doc(alias = "SIO_ISR")]
pub type SioIsr = crate::Reg<sio_isr::SioIsrSpec>;
#[doc = "Interrupt Status Register. TODO: Add documentation on register."]
pub mod sio_isr;
#[doc = "SIO_FCR (rw) register accessor: Manages the Rx and Tx FIFO buffers. Additionally, controls the Receive FIFO Trigger level and sets DMA mode 1. Based on TX7901 docs.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_fcr`] module"]
#[doc(alias = "SIO_FCR")]
pub type SioFcr = crate::Reg<sio_fcr::SioFcrSpec>;
#[doc = "Manages the Rx and Tx FIFO buffers. Additionally, controls the Receive FIFO Trigger level and sets DMA mode 1. Based on TX7901 docs."]
pub mod sio_fcr;
#[doc = "SIO_BGR (rw) register accessor: Baud Rate Control Register. Configures the pre-scaler and clock frequency for the Baud Rate Generator. The clock signal used is calculated based on the formula `fc / (prescaler x divisor x 16)` where fc is the clock chosen in the Line Control Register (bits 6:5). Based on the TX49 docs.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_bgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_bgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_bgr`] module"]
#[doc(alias = "SIO_BGR")]
pub type SioBgr = crate::Reg<sio_bgr::SioBgrSpec>;
#[doc = "Baud Rate Control Register. Configures the pre-scaler and clock frequency for the Baud Rate Generator. The clock signal used is calculated based on the formula `fc / (prescaler x divisor x 16)` where fc is the clock chosen in the Line Control Register (bits 6:5). Based on the TX49 docs."]
pub mod sio_bgr;
#[doc = "SIO_TXFIFO (w) register accessor: Tx FIFO buffer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_txfifo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_txfifo`] module"]
#[doc(alias = "SIO_TXFIFO")]
pub type SioTxfifo = crate::Reg<sio_txfifo::SioTxfifoSpec>;
#[doc = "Tx FIFO buffer."]
pub mod sio_txfifo;
#[doc = "SIO_RXFIFO (r) register accessor: Rx FIFO buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_rxfifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sio_rxfifo`] module"]
#[doc(alias = "SIO_RXFIFO")]
pub type SioRxfifo = crate::Reg<sio_rxfifo::SioRxfifoSpec>;
#[doc = "Rx FIFO buffer."]
pub mod sio_rxfifo;
