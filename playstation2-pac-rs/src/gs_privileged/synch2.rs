#[doc = "Register `SYNCH2` writer"]
pub type W = crate::W<Synch2Spec>;
#[doc = "Field `HF` writer - UNKNOWN."]
pub type HfW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HB` writer - UNKNOWN."]
pub type HbW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl W {
    #[doc = "Bits 0:10 - UNKNOWN."]
    #[inline(always)]
    pub fn hf(&mut self) -> HfW<Synch2Spec> {
        HfW::new(self, 0)
    }
    #[doc = "Bits 11:21 - UNKNOWN."]
    #[inline(always)]
    pub fn hb(&mut self) -> HbW<Synch2Spec> {
        HbW::new(self, 11)
    }
}
#[doc = "Sync-related. Specific purpose unknown.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synch2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Synch2Spec;
impl crate::RegisterSpec for Synch2Spec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`synch2::W`](W) writer structure"]
impl crate::Writable for Synch2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCH2 to value 0"]
impl crate::Resettable for Synch2Spec {}
