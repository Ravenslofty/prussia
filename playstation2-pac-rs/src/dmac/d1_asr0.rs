#[doc = "Register `D1_ASR0` reader"]
pub type R = crate::R<D1Asr0Spec>;
#[doc = "Register `D1_ASR0` writer"]
pub type W = crate::W<D1Asr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch1 address stack 0\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_asr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_asr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1Asr0Spec;
impl crate::RegisterSpec for D1Asr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1_asr0::R`](R) reader structure"]
impl crate::Readable for D1Asr0Spec {}
#[doc = "`write(|w| ..)` method takes [`d1_asr0::W`](W) writer structure"]
impl crate::Writable for D1Asr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1_ASR0 to value 0"]
impl crate::Resettable for D1Asr0Spec {}
