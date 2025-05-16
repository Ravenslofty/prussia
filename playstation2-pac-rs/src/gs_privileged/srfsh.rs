#[doc = "Register `SRFSH` writer"]
pub type W = crate::W<SrfshSpec>;
#[doc = "Field `RFSH` writer - GS DRAM refresh rate."]
pub type RfshW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - GS DRAM refresh rate."]
    #[inline(always)]
    pub fn rfsh(&mut self) -> RfshW<SrfshSpec> {
        RfshW::new(self, 0)
    }
}
#[doc = "GS DRAM refresh rate.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srfsh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrfshSpec;
impl crate::RegisterSpec for SrfshSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`srfsh::W`](W) writer structure"]
impl crate::Writable for SrfshSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRFSH to value 0"]
impl crate::Resettable for SrfshSpec {}
