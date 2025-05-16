#[doc = "Register `SIO_FCR` reader"]
pub type R = crate::R<SioFcrSpec>;
#[doc = "Register `SIO_FCR` writer"]
pub type W = crate::W<SioFcrSpec>;
#[doc = "Enables or disables the FIFOs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ufifoe {
    #[doc = "0: FIFOs are disabled."]
    Disabled = 0,
    #[doc = "1: FIFOs are enabled."]
    Enabled = 1,
}
impl From<Ufifoe> for bool {
    #[inline(always)]
    fn from(variant: Ufifoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UFIFOE` reader - Enables or disables the FIFOs."]
pub type UfifoeR = crate::BitReader<Ufifoe>;
impl UfifoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ufifoe {
        match self.bits {
            false => Ufifoe::Disabled,
            true => Ufifoe::Enabled,
        }
    }
    #[doc = "FIFOs are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ufifoe::Disabled
    }
    #[doc = "FIFOs are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ufifoe::Enabled
    }
}
#[doc = "Field `UFIFOE` writer - Enables or disables the FIFOs."]
pub type UfifoeW<'a, REG> = crate::BitWriter<'a, REG, Ufifoe>;
impl<'a, REG> UfifoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFOs are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ufifoe::Disabled)
    }
    #[doc = "FIFOs are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ufifoe::Enabled)
    }
}
#[doc = "Clears the Rx FIFO when set. Self-clears once set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uclrr {
    #[doc = "0: Does nothing."]
    Nothing = 0,
    #[doc = "1: Clear the Rx FIFO."]
    Clear = 1,
}
impl From<Uclrr> for bool {
    #[inline(always)]
    fn from(variant: Uclrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCLRR` reader - Clears the Rx FIFO when set. Self-clears once set to 1."]
pub type UclrrR = crate::BitReader<Uclrr>;
impl UclrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uclrr {
        match self.bits {
            false => Uclrr::Nothing,
            true => Uclrr::Clear,
        }
    }
    #[doc = "Does nothing."]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == Uclrr::Nothing
    }
    #[doc = "Clear the Rx FIFO."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Uclrr::Clear
    }
}
#[doc = "Field `UCLRR` writer - Clears the Rx FIFO when set. Self-clears once set to 1."]
pub type UclrrW<'a, REG> = crate::BitWriter<'a, REG, Uclrr>;
impl<'a, REG> UclrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does nothing."]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(Uclrr::Nothing)
    }
    #[doc = "Clear the Rx FIFO."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Uclrr::Clear)
    }
}
#[doc = "Clears the Tx FIFO when set. Self-clears once set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uclrt {
    #[doc = "0: Does nothing."]
    Nothing = 0,
    #[doc = "1: Clear the Tx FIFO."]
    Clear = 1,
}
impl From<Uclrt> for bool {
    #[inline(always)]
    fn from(variant: Uclrt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCLRT` reader - Clears the Tx FIFO when set. Self-clears once set to 1."]
pub type UclrtR = crate::BitReader<Uclrt>;
impl UclrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uclrt {
        match self.bits {
            false => Uclrt::Nothing,
            true => Uclrt::Clear,
        }
    }
    #[doc = "Does nothing."]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == Uclrt::Nothing
    }
    #[doc = "Clear the Tx FIFO."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Uclrt::Clear
    }
}
#[doc = "Field `UCLRT` writer - Clears the Tx FIFO when set. Self-clears once set to 1."]
pub type UclrtW<'a, REG> = crate::BitWriter<'a, REG, Uclrt>;
impl<'a, REG> UclrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does nothing."]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(Uclrt::Nothing)
    }
    #[doc = "Clear the Tx FIFO."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Uclrt::Clear)
    }
}
#[doc = "Sets DMA mode 1 for when the Tx/Rx ready interrupts trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udma1 {
    #[doc = "0: DMA mode 0."]
    Mode0 = 0,
    #[doc = "1: DMA mode 1."]
    Mode1 = 1,
}
impl From<Udma1> for bool {
    #[inline(always)]
    fn from(variant: Udma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDMA1` reader - Sets DMA mode 1 for when the Tx/Rx ready interrupts trigger."]
pub type Udma1R = crate::BitReader<Udma1>;
impl Udma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udma1 {
        match self.bits {
            false => Udma1::Mode0,
            true => Udma1::Mode1,
        }
    }
    #[doc = "DMA mode 0."]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Udma1::Mode0
    }
    #[doc = "DMA mode 1."]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Udma1::Mode1
    }
}
#[doc = "Field `UDMA1` writer - Sets DMA mode 1 for when the Tx/Rx ready interrupts trigger."]
pub type Udma1W<'a, REG> = crate::BitWriter<'a, REG, Udma1>;
impl<'a, REG> Udma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode 0."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Udma1::Mode0)
    }
    #[doc = "DMA mode 1."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Udma1::Mode1)
    }
}
#[doc = "Determines the amount of bytes needed in the Rx FIFO to trigger an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Urfifol {
    #[doc = "0: One byte to trigger the interrupt."]
    Bytes1 = 0,
    #[doc = "1: Four bytes to trigger the interrupt."]
    Bytes4 = 1,
    #[doc = "2: Eight bytes to trigger the interrupt."]
    Bytes8 = 2,
    #[doc = "3: Fourteen bytes to trigger the interrupt."]
    Bytes14 = 3,
}
impl From<Urfifol> for u8 {
    #[inline(always)]
    fn from(variant: Urfifol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Urfifol {
    type Ux = u8;
}
impl crate::IsEnum for Urfifol {}
#[doc = "Field `URFIFOL` reader - Determines the amount of bytes needed in the Rx FIFO to trigger an interrupt."]
pub type UrfifolR = crate::FieldReader<Urfifol>;
impl UrfifolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Urfifol {
        match self.bits {
            0 => Urfifol::Bytes1,
            1 => Urfifol::Bytes4,
            2 => Urfifol::Bytes8,
            3 => Urfifol::Bytes14,
            _ => unreachable!(),
        }
    }
    #[doc = "One byte to trigger the interrupt."]
    #[inline(always)]
    pub fn is_bytes_1(&self) -> bool {
        *self == Urfifol::Bytes1
    }
    #[doc = "Four bytes to trigger the interrupt."]
    #[inline(always)]
    pub fn is_bytes_4(&self) -> bool {
        *self == Urfifol::Bytes4
    }
    #[doc = "Eight bytes to trigger the interrupt."]
    #[inline(always)]
    pub fn is_bytes_8(&self) -> bool {
        *self == Urfifol::Bytes8
    }
    #[doc = "Fourteen bytes to trigger the interrupt."]
    #[inline(always)]
    pub fn is_bytes_14(&self) -> bool {
        *self == Urfifol::Bytes14
    }
}
#[doc = "Field `URFIFOL` writer - Determines the amount of bytes needed in the Rx FIFO to trigger an interrupt."]
pub type UrfifolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Urfifol, crate::Safe>;
impl<'a, REG> UrfifolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One byte to trigger the interrupt."]
    #[inline(always)]
    pub fn bytes_1(self) -> &'a mut crate::W<REG> {
        self.variant(Urfifol::Bytes1)
    }
    #[doc = "Four bytes to trigger the interrupt."]
    #[inline(always)]
    pub fn bytes_4(self) -> &'a mut crate::W<REG> {
        self.variant(Urfifol::Bytes4)
    }
    #[doc = "Eight bytes to trigger the interrupt."]
    #[inline(always)]
    pub fn bytes_8(self) -> &'a mut crate::W<REG> {
        self.variant(Urfifol::Bytes8)
    }
    #[doc = "Fourteen bytes to trigger the interrupt."]
    #[inline(always)]
    pub fn bytes_14(self) -> &'a mut crate::W<REG> {
        self.variant(Urfifol::Bytes14)
    }
}
impl R {
    #[doc = "Bit 0 - Enables or disables the FIFOs."]
    #[inline(always)]
    pub fn ufifoe(&self) -> UfifoeR {
        UfifoeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clears the Rx FIFO when set. Self-clears once set to 1."]
    #[inline(always)]
    pub fn uclrr(&self) -> UclrrR {
        UclrrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clears the Tx FIFO when set. Self-clears once set to 1."]
    #[inline(always)]
    pub fn uclrt(&self) -> UclrtR {
        UclrtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sets DMA mode 1 for when the Tx/Rx ready interrupts trigger."]
    #[inline(always)]
    pub fn udma1(&self) -> Udma1R {
        Udma1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Determines the amount of bytes needed in the Rx FIFO to trigger an interrupt."]
    #[inline(always)]
    pub fn urfifol(&self) -> UrfifolR {
        UrfifolR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables the FIFOs."]
    #[inline(always)]
    pub fn ufifoe(&mut self) -> UfifoeW<SioFcrSpec> {
        UfifoeW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the Rx FIFO when set. Self-clears once set to 1."]
    #[inline(always)]
    pub fn uclrr(&mut self) -> UclrrW<SioFcrSpec> {
        UclrrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears the Tx FIFO when set. Self-clears once set to 1."]
    #[inline(always)]
    pub fn uclrt(&mut self) -> UclrtW<SioFcrSpec> {
        UclrtW::new(self, 2)
    }
    #[doc = "Bit 3 - Sets DMA mode 1 for when the Tx/Rx ready interrupts trigger."]
    #[inline(always)]
    pub fn udma1(&mut self) -> Udma1W<SioFcrSpec> {
        Udma1W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Determines the amount of bytes needed in the Rx FIFO to trigger an interrupt."]
    #[inline(always)]
    pub fn urfifol(&mut self) -> UrfifolW<SioFcrSpec> {
        UrfifolW::new(self, 6)
    }
}
#[doc = "Manages the Rx and Tx FIFO buffers. Additionally, controls the Receive FIFO Trigger level and sets DMA mode 1. Based on TX7901 docs.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioFcrSpec;
impl crate::RegisterSpec for SioFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sio_fcr::R`](R) reader structure"]
impl crate::Readable for SioFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sio_fcr::W`](W) writer structure"]
impl crate::Writable for SioFcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIO_FCR to value 0"]
impl crate::Resettable for SioFcrSpec {}
