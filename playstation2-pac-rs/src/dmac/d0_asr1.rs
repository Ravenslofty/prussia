#[doc = "Register `D0_ASR1` reader"]
pub type R = crate::R<D0Asr1Spec>;
#[doc = "Register `D0_ASR1` writer"]
pub type W = crate::W<D0Asr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch0 address stack 1\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_asr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_asr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0Asr1Spec;
impl crate::RegisterSpec for D0Asr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d0_asr1::R`](R) reader structure"]
impl crate::Readable for D0Asr1Spec {}
#[doc = "`write(|w| ..)` method takes [`d0_asr1::W`](W) writer structure"]
impl crate::Writable for D0Asr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D0_ASR1 to value 0"]
impl crate::Resettable for D0Asr1Spec {}
