#[doc = "Register `D_ENABLEW` writer"]
pub type W = crate::W<DEnablewSpec>;
#[doc = "DMA transfer is held\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpnd {
    #[doc = "0: Enables all channel transfer (restarts)"]
    Enable = 0,
    #[doc = "1: Holds all channel transfer (suspends)"]
    Disable = 1,
}
impl From<Cpnd> for bool {
    #[inline(always)]
    fn from(variant: Cpnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPND` writer - DMA transfer is held"]
pub type CpndW<'a, REG> = crate::BitWriter<'a, REG, Cpnd>;
impl<'a, REG> CpndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables all channel transfer (restarts)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnd::Enable)
    }
    #[doc = "Holds all channel transfer (suspends)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnd::Disable)
    }
}
impl W {
    #[doc = "Bit 16 - DMA transfer is held"]
    #[inline(always)]
    pub fn cpnd(&mut self) -> CpndW<DEnablewSpec> {
        CpndW::new(self, 16)
    }
}
#[doc = "DMA suspend control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_enablew::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEnablewSpec;
impl crate::RegisterSpec for DEnablewSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`d_enablew::W`](W) writer structure"]
impl crate::Writable for DEnablewSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_ENABLEW to value 0"]
impl crate::Resettable for DEnablewSpec {}
