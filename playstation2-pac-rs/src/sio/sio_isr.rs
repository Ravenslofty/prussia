#[doc = "Register `SIO_ISR` reader"]
pub type R = crate::R<SioIsrSpec>;
#[doc = "Register `SIO_ISR` writer"]
pub type W = crate::W<SioIsrSpec>;
#[doc = "RX data is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdata {
    #[doc = "0: No data."]
    NoData = 0,
    #[doc = "1: data present."]
    Data = 1,
}
impl From<Rxdata> for bool {
    #[inline(always)]
    fn from(variant: Rxdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDATA` reader - RX data is available."]
pub type RxdataR = crate::BitReader<Rxdata>;
impl RxdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdata {
        match self.bits {
            false => Rxdata::NoData,
            true => Rxdata::Data,
        }
    }
    #[doc = "No data."]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Rxdata::NoData
    }
    #[doc = "data present."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Rxdata::Data
    }
}
#[doc = "Field `RXDATA` writer - RX data is available."]
pub type RxdataW<'a, REG> = crate::BitWriter<'a, REG, Rxdata>;
impl<'a, REG> RxdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No data."]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdata::NoData)
    }
    #[doc = "data present."]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdata::Data)
    }
}
#[doc = "TX is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempty {
    #[doc = "0: Data is present."]
    Data = 0,
    #[doc = "1: No data."]
    Empty = 1,
}
impl From<Txempty> for bool {
    #[inline(always)]
    fn from(variant: Txempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - TX is empty."]
pub type TxemptyR = crate::BitReader<Txempty>;
impl TxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txempty {
        match self.bits {
            false => Txempty::Data,
            true => Txempty::Empty,
        }
    }
    #[doc = "Data is present."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Txempty::Data
    }
    #[doc = "No data."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Txempty::Empty
    }
}
#[doc = "Field `TXEMPTY` writer - TX is empty."]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG, Txempty>;
impl<'a, REG> TxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is present."]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Data)
    }
    #[doc = "No data."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Empty)
    }
}
#[doc = "RX is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxempty {
    #[doc = "0: Data is present."]
    Data = 0,
    #[doc = "1: No data."]
    Empty = 1,
}
impl From<Rxempty> for bool {
    #[inline(always)]
    fn from(variant: Rxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPTY` reader - RX is empty."]
pub type RxemptyR = crate::BitReader<Rxempty>;
impl RxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxempty {
        match self.bits {
            false => Rxempty::Data,
            true => Rxempty::Empty,
        }
    }
    #[doc = "Data is present."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Rxempty::Data
    }
    #[doc = "No data."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rxempty::Empty
    }
}
#[doc = "Field `RXEMPTY` writer - RX is empty."]
pub type RxemptyW<'a, REG> = crate::BitWriter<'a, REG, Rxempty>;
impl<'a, REG> RxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is present."]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxempty::Data)
    }
    #[doc = "No data."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxempty::Empty)
    }
}
#[doc = "Trigger Modem Status Interrupt if the appropriate flag in Line Status Register is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Umsii {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Umsii> for bool {
    #[inline(always)]
    fn from(variant: Umsii) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UMSII` reader - Trigger Modem Status Interrupt if the appropriate flag in Line Status Register is set."]
pub type UmsiiR = crate::BitReader<Umsii>;
impl UmsiiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Umsii {
        match self.bits {
            false => Umsii::Disabled,
            true => Umsii::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Umsii::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Umsii::Enabled
    }
}
#[doc = "Field `UMSII` writer - Trigger Modem Status Interrupt if the appropriate flag in Line Status Register is set."]
pub type UmsiiW<'a, REG> = crate::BitWriter<'a, REG, Umsii>;
impl<'a, REG> UmsiiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Umsii::Disabled)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Umsii::Enabled)
    }
}
#[doc = "Field `UCRUB` reader - Count of received unread bytes."]
pub type UcrubR = crate::FieldReader;
#[doc = "Field `UCBT` reader - Count of bytes pending transmission."]
pub type UcbtR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - RX data is available."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX is empty."]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX is empty."]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger Modem Status Interrupt if the appropriate flag in Line Status Register is set."]
    #[inline(always)]
    pub fn umsii(&self) -> UmsiiR {
        UmsiiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Count of received unread bytes."]
    #[inline(always)]
    pub fn ucrub(&self) -> UcrubR {
        UcrubR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Count of bytes pending transmission."]
    #[inline(always)]
    pub fn ucbt(&self) -> UcbtR {
        UcbtR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX data is available."]
    #[inline(always)]
    pub fn rxdata(&mut self) -> RxdataW<SioIsrSpec> {
        RxdataW::new(self, 0)
    }
    #[doc = "Bit 1 - TX is empty."]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<SioIsrSpec> {
        TxemptyW::new(self, 1)
    }
    #[doc = "Bit 2 - RX is empty."]
    #[inline(always)]
    pub fn rxempty(&mut self) -> RxemptyW<SioIsrSpec> {
        RxemptyW::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger Modem Status Interrupt if the appropriate flag in Line Status Register is set."]
    #[inline(always)]
    pub fn umsii(&mut self) -> UmsiiW<SioIsrSpec> {
        UmsiiW::new(self, 3)
    }
}
#[doc = "Interrupt Status Register. TODO: Add documentation on register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioIsrSpec;
impl crate::RegisterSpec for SioIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sio_isr::R`](R) reader structure"]
impl crate::Readable for SioIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sio_isr::W`](W) writer structure"]
impl crate::Writable for SioIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIO_ISR to value 0"]
impl crate::Resettable for SioIsrSpec {}
