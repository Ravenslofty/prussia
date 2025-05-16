#[doc = "Register `D_ENABLER` reader"]
pub type R = crate::R<DEnablerSpec>;
#[doc = "DMA transfer hold state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpnd {
    #[doc = "0: All channel transfer enabled"]
    Enable = 0,
    #[doc = "1: All channel transfer being held (suspended)"]
    Disable = 1,
}
impl From<Cpnd> for bool {
    #[inline(always)]
    fn from(variant: Cpnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPND` reader - DMA transfer hold state"]
pub type CpndR = crate::BitReader<Cpnd>;
impl CpndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpnd {
        match self.bits {
            false => Cpnd::Enable,
            true => Cpnd::Disable,
        }
    }
    #[doc = "All channel transfer enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpnd::Enable
    }
    #[doc = "All channel transfer being held (suspended)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpnd::Disable
    }
}
impl R {
    #[doc = "Bit 16 - DMA transfer hold state"]
    #[inline(always)]
    pub fn cpnd(&self) -> CpndR {
        CpndR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Acquistion of DMA suspend status\n\nYou can [`read`](crate::Reg::read) this register and get [`d_enabler::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEnablerSpec;
impl crate::RegisterSpec for DEnablerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_enabler::R`](R) reader structure"]
impl crate::Readable for DEnablerSpec {}
#[doc = "`reset()` method sets D_ENABLER to value 0"]
impl crate::Resettable for DEnablerSpec {}
