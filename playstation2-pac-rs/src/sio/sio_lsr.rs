#[doc = "Register `SIO_LSR` reader"]
pub type R = crate::R<SioLsrSpec>;
#[doc = "Indicates whether or not data is available in the Rx FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dr {
    #[doc = "0: Receive FIFO is empty."]
    No = 0,
    #[doc = "1: Receive FIFO has data for reading."]
    Yes = 1,
}
impl From<Dr> for bool {
    #[inline(always)]
    fn from(variant: Dr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DR` reader - Indicates whether or not data is available in the Rx FIFO."]
pub type DrR = crate::BitReader<Dr>;
impl DrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dr {
        match self.bits {
            false => Dr::No,
            true => Dr::Yes,
        }
    }
    #[doc = "Receive FIFO is empty."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Dr::No
    }
    #[doc = "Receive FIFO has data for reading."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Dr::Yes
    }
}
#[doc = "Indicates an overrun error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uoer {
    #[doc = "0: No error has occurred."]
    NoError = 0,
    #[doc = "1: An overrun error has occurred."]
    Error = 1,
}
impl From<Uoer> for bool {
    #[inline(always)]
    fn from(variant: Uoer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UOER` reader - Indicates an overrun error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
pub type UoerR = crate::BitReader<Uoer>;
impl UoerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uoer {
        match self.bits {
            false => Uoer::NoError,
            true => Uoer::Error,
        }
    }
    #[doc = "No error has occurred."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Uoer::NoError
    }
    #[doc = "An overrun error has occurred."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Uoer::Error
    }
}
#[doc = "Indicates a parity error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uper {
    #[doc = "0: No error has occurred."]
    NoError = 0,
    #[doc = "1: A parity error has occurred."]
    Error = 1,
}
impl From<Uper> for bool {
    #[inline(always)]
    fn from(variant: Uper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPER` reader - Indicates a parity error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
pub type UperR = crate::BitReader<Uper>;
impl UperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uper {
        match self.bits {
            false => Uper::NoError,
            true => Uper::Error,
        }
    }
    #[doc = "No error has occurred."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Uper::NoError
    }
    #[doc = "A parity error has occurred."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Uper::Error
    }
}
#[doc = "Indicates a framing error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ufer {
    #[doc = "0: No error has occurred."]
    NoError = 0,
    #[doc = "1: A framing error has occurred."]
    Error = 1,
}
impl From<Ufer> for bool {
    #[inline(always)]
    fn from(variant: Ufer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UFER` reader - Indicates a framing error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
pub type UferR = crate::BitReader<Ufer>;
impl UferR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ufer {
        match self.bits {
            false => Ufer::NoError,
            true => Ufer::Error,
        }
    }
    #[doc = "No error has occurred."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Ufer::NoError
    }
    #[doc = "A framing error has occurred."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Ufer::Error
    }
}
#[doc = "Indicates a break exception is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ubrk {
    #[doc = "0: No error has occurred."]
    NoError = 0,
    #[doc = "1: A framing error has occurred."]
    Error = 1,
}
impl From<Ubrk> for bool {
    #[inline(always)]
    fn from(variant: Ubrk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UBRK` reader - Indicates a break exception is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
pub type UbrkR = crate::BitReader<Ubrk>;
impl UbrkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ubrk {
        match self.bits {
            false => Ubrk::NoError,
            true => Ubrk::Error,
        }
    }
    #[doc = "No error has occurred."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Ubrk::NoError
    }
    #[doc = "A framing error has occurred."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Ubrk::Error
    }
}
#[doc = "Tx FIFO Empty. Indicates there is data in the Tx FIFO pending transmission. This field is updated every time the Tx FIFO is written to or cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uthr {
    #[doc = "0: There is data in the FIFO."]
    Data = 0,
    #[doc = "1: The FIFO is empty."]
    NoData = 1,
}
impl From<Uthr> for bool {
    #[inline(always)]
    fn from(variant: Uthr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTHR` reader - Tx FIFO Empty. Indicates there is data in the Tx FIFO pending transmission. This field is updated every time the Tx FIFO is written to or cleared."]
pub type UthrR = crate::BitReader<Uthr>;
impl UthrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uthr {
        match self.bits {
            false => Uthr::Data,
            true => Uthr::NoData,
        }
    }
    #[doc = "There is data in the FIFO."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Uthr::Data
    }
    #[doc = "The FIFO is empty."]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Uthr::NoData
    }
}
#[doc = "Transmitter empty. Indicates there is data in the Tx FIFO and Tx Shift registers. This field is updated every time the Tx FIFO is written to or cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uter {
    #[doc = "0: There is data in the FIFO and Shift."]
    Data = 0,
    #[doc = "1: The FIFO or Shift are empty."]
    NoData = 1,
}
impl From<Uter> for bool {
    #[inline(always)]
    fn from(variant: Uter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTER` reader - Transmitter empty. Indicates there is data in the Tx FIFO and Tx Shift registers. This field is updated every time the Tx FIFO is written to or cleared."]
pub type UterR = crate::BitReader<Uter>;
impl UterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uter {
        match self.bits {
            false => Uter::Data,
            true => Uter::NoData,
        }
    }
    #[doc = "There is data in the FIFO and Shift."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Uter::Data
    }
    #[doc = "The FIFO or Shift are empty."]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Uter::NoData
    }
}
#[doc = "Rx FIFO Error. Set to 1 when at least one error bit in the Line Status Register is set. When the LSR is read, this field is cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ufder {
    #[doc = "0: No errors in the LSR."]
    Data = 0,
    #[doc = "1: At least one error in the LSR."]
    NoData = 1,
}
impl From<Ufder> for bool {
    #[inline(always)]
    fn from(variant: Ufder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UFDER` reader - Rx FIFO Error. Set to 1 when at least one error bit in the Line Status Register is set. When the LSR is read, this field is cleared."]
pub type UfderR = crate::BitReader<Ufder>;
impl UfderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ufder {
        match self.bits {
            false => Ufder::Data,
            true => Ufder::NoData,
        }
    }
    #[doc = "No errors in the LSR."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Ufder::Data
    }
    #[doc = "At least one error in the LSR."]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Ufder::NoData
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether or not data is available in the Rx FIFO."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates an overrun error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
    #[inline(always)]
    pub fn uoer(&self) -> UoerR {
        UoerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates a parity error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
    #[inline(always)]
    pub fn uper(&self) -> UperR {
        UperR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates a framing error is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
    #[inline(always)]
    pub fn ufer(&self) -> UferR {
        UferR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates a break exception is present in the next Rx FIFO data to be read. This field is updated every time the Rx FIFO is read."]
    #[inline(always)]
    pub fn ubrk(&self) -> UbrkR {
        UbrkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx FIFO Empty. Indicates there is data in the Tx FIFO pending transmission. This field is updated every time the Tx FIFO is written to or cleared."]
    #[inline(always)]
    pub fn uthr(&self) -> UthrR {
        UthrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter empty. Indicates there is data in the Tx FIFO and Tx Shift registers. This field is updated every time the Tx FIFO is written to or cleared."]
    #[inline(always)]
    pub fn uter(&self) -> UterR {
        UterR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Error. Set to 1 when at least one error bit in the Line Status Register is set. When the LSR is read, this field is cleared."]
    #[inline(always)]
    pub fn ufder(&self) -> UfderR {
        UfderR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register (aka DMA/Interrupt Status Register). Provides status information about the SIO1 (serial port) channel. Some documentation is loosely based on the TMPR4937 CPU, particularly The SIDISR0/1 register, however it mainly draws from the TX7901.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioLsrSpec;
impl crate::RegisterSpec for SioLsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sio_lsr::R`](R) reader structure"]
impl crate::Readable for SioLsrSpec {}
#[doc = "`reset()` method sets SIO_LSR to value 0"]
impl crate::Resettable for SioLsrSpec {}
