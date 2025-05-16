#[doc = "Register `SYNCH1` writer"]
pub type W = crate::W<Synch1Spec>;
#[doc = "Field `HFP` writer - Horizonal front porch."]
pub type HfpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HBP` writer - Horizonal back porch."]
pub type HbpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HSEQ` writer - UNKNOWN."]
pub type HseqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HSVS` writer - UNKNOWN."]
pub type HsvsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HS` writer - UNKNOWN."]
pub type HsW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl W {
    #[doc = "Bits 0:10 - Horizonal front porch."]
    #[inline(always)]
    pub fn hfp(&mut self) -> HfpW<Synch1Spec> {
        HfpW::new(self, 0)
    }
    #[doc = "Bits 11:21 - Horizonal back porch."]
    #[inline(always)]
    pub fn hbp(&mut self) -> HbpW<Synch1Spec> {
        HbpW::new(self, 11)
    }
    #[doc = "Bits 22:31 - UNKNOWN."]
    #[inline(always)]
    pub fn hseq(&mut self) -> HseqW<Synch1Spec> {
        HseqW::new(self, 22)
    }
    #[doc = "Bits 32:42 - UNKNOWN."]
    #[inline(always)]
    pub fn hsvs(&mut self) -> HsvsW<Synch1Spec> {
        HsvsW::new(self, 32)
    }
    #[doc = "Bits 43:63 - UNKNOWN."]
    #[inline(always)]
    pub fn hs(&mut self) -> HsW<Synch1Spec> {
        HsW::new(self, 43)
    }
}
#[doc = "Sync-related. Specific purpose unknown.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synch1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Synch1Spec;
impl crate::RegisterSpec for Synch1Spec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`synch1::W`](W) writer structure"]
impl crate::Writable for Synch1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCH1 to value 0"]
impl crate::Resettable for Synch1Spec {}
