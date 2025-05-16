#[doc = "Register `SIO_IER` reader"]
pub type R = crate::R<SioIerSpec>;
#[doc = "Register `SIO_IER` writer"]
pub type W = crate::W<SioIerSpec>;
#[doc = "Toggles interrupting once the Rx FIFO contains data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erbfi {
    #[doc = "0: Disable the interrupt."]
    Disable = 0,
    #[doc = "1: Enable the interrupt."]
    Enable = 1,
}
impl From<Erbfi> for bool {
    #[inline(always)]
    fn from(variant: Erbfi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERBFI` reader - Toggles interrupting once the Rx FIFO contains data."]
pub type ErbfiR = crate::BitReader<Erbfi>;
impl ErbfiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erbfi {
        match self.bits {
            false => Erbfi::Disable,
            true => Erbfi::Enable,
        }
    }
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Erbfi::Disable
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Erbfi::Enable
    }
}
#[doc = "Field `ERBFI` writer - Toggles interrupting once the Rx FIFO contains data."]
pub type ErbfiW<'a, REG> = crate::BitWriter<'a, REG, Erbfi>;
impl<'a, REG> ErbfiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Erbfi::Disable)
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Erbfi::Enable)
    }
}
#[doc = "Toggles interrupting when the Tx FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etbei {
    #[doc = "0: Disable the interrupt."]
    Disable = 0,
    #[doc = "1: Enable the interrupt."]
    Enable = 1,
}
impl From<Etbei> for bool {
    #[inline(always)]
    fn from(variant: Etbei) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETBEI` reader - Toggles interrupting when the Tx FIFO is empty."]
pub type EtbeiR = crate::BitReader<Etbei>;
impl EtbeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etbei {
        match self.bits {
            false => Etbei::Disable,
            true => Etbei::Enable,
        }
    }
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Etbei::Disable
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Etbei::Enable
    }
}
#[doc = "Field `ETBEI` writer - Toggles interrupting when the Tx FIFO is empty."]
pub type EtbeiW<'a, REG> = crate::BitWriter<'a, REG, Etbei>;
impl<'a, REG> EtbeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Etbei::Disable)
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Etbei::Enable)
    }
}
#[doc = "When enabled, generates Rx Status Interrupts if either of bits 1, 2, 3, or 4, of the Line Status Register are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elsi {
    #[doc = "0: Disable the interrupt."]
    Disable = 0,
    #[doc = "1: Enable the interrupt."]
    Enable = 1,
}
impl From<Elsi> for bool {
    #[inline(always)]
    fn from(variant: Elsi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELSI` reader - When enabled, generates Rx Status Interrupts if either of bits 1, 2, 3, or 4, of the Line Status Register are set."]
pub type ElsiR = crate::BitReader<Elsi>;
impl ElsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Elsi {
        match self.bits {
            false => Elsi::Disable,
            true => Elsi::Enable,
        }
    }
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Elsi::Disable
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Elsi::Enable
    }
}
#[doc = "Field `ELSI` writer - When enabled, generates Rx Status Interrupts if either of bits 1, 2, 3, or 4, of the Line Status Register are set."]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG, Elsi>;
impl<'a, REG> ElsiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Elsi::Disable)
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Elsi::Enable)
    }
}
#[doc = "When enabled, generates Modem Status Interrupts if either of bits 0, 1, 2, or 3, of the Modem Status Register are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edssi {
    #[doc = "0: Disable the interrupt."]
    Disable = 0,
    #[doc = "1: Enable the interrupt."]
    Enable = 1,
}
impl From<Edssi> for bool {
    #[inline(always)]
    fn from(variant: Edssi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDSSI` reader - When enabled, generates Modem Status Interrupts if either of bits 0, 1, 2, or 3, of the Modem Status Register are set."]
pub type EdssiR = crate::BitReader<Edssi>;
impl EdssiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edssi {
        match self.bits {
            false => Edssi::Disable,
            true => Edssi::Enable,
        }
    }
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Edssi::Disable
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Edssi::Enable
    }
}
#[doc = "Field `EDSSI` writer - When enabled, generates Modem Status Interrupts if either of bits 0, 1, 2, or 3, of the Modem Status Register are set."]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG, Edssi>;
impl<'a, REG> EdssiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Edssi::Disable)
    }
    #[doc = "Enable the interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Edssi::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Toggles interrupting once the Rx FIFO contains data."]
    #[inline(always)]
    pub fn erbfi(&self) -> ErbfiR {
        ErbfiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Toggles interrupting when the Tx FIFO is empty."]
    #[inline(always)]
    pub fn etbei(&self) -> EtbeiR {
        EtbeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When enabled, generates Rx Status Interrupts if either of bits 1, 2, 3, or 4, of the Line Status Register are set."]
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 2 - When enabled, generates Modem Status Interrupts if either of bits 0, 1, 2, or 3, of the Modem Status Register are set."]
    #[inline(always)]
    pub fn edssi(&self) -> EdssiR {
        EdssiR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Toggles interrupting once the Rx FIFO contains data."]
    #[inline(always)]
    pub fn erbfi(&mut self) -> ErbfiW<SioIerSpec> {
        ErbfiW::new(self, 0)
    }
    #[doc = "Bit 1 - Toggles interrupting when the Tx FIFO is empty."]
    #[inline(always)]
    pub fn etbei(&mut self) -> EtbeiW<SioIerSpec> {
        EtbeiW::new(self, 1)
    }
    #[doc = "Bit 2 - When enabled, generates Rx Status Interrupts if either of bits 1, 2, 3, or 4, of the Line Status Register are set."]
    #[inline(always)]
    pub fn elsi(&mut self) -> ElsiW<SioIerSpec> {
        ElsiW::new(self, 2)
    }
    #[doc = "Bit 2 - When enabled, generates Modem Status Interrupts if either of bits 0, 1, 2, or 3, of the Modem Status Register are set."]
    #[inline(always)]
    pub fn edssi(&mut self) -> EdssiW<SioIerSpec> {
        EdssiW::new(self, 2)
    }
}
#[doc = "Interrupt Enable Register. Handles the enabling/disabling of SIO-specific interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioIerSpec;
impl crate::RegisterSpec for SioIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sio_ier::R`](R) reader structure"]
impl crate::Readable for SioIerSpec {}
#[doc = "`write(|w| ..)` method takes [`sio_ier::W`](W) writer structure"]
impl crate::Writable for SioIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIO_IER to value 0"]
impl crate::Resettable for SioIerSpec {}
