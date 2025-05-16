#[doc = "Register `BGCOLOR` writer"]
pub type W = crate::W<BgcolorSpec>;
#[doc = "Field `R` writer - Red channel."]
pub type RW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `G` writer - Green channel."]
pub type GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `B` writer - Blue channel."]
pub type BW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Red channel."]
    #[inline(always)]
    pub fn r(&mut self) -> RW<BgcolorSpec> {
        RW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green channel."]
    #[inline(always)]
    pub fn g(&mut self) -> GW<BgcolorSpec> {
        GW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Blue channel."]
    #[inline(always)]
    pub fn b(&mut self) -> BW<BgcolorSpec> {
        BW::new(self, 16)
    }
}
#[doc = "Background colour.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolor::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgcolorSpec;
impl crate::RegisterSpec for BgcolorSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`bgcolor::W`](W) writer structure"]
impl crate::Writable for BgcolorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BGCOLOR to value 0"]
impl crate::Resettable for BgcolorSpec {}
