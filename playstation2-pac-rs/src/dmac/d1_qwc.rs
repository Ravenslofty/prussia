#[doc = "Register `D1_QWC` reader"]
pub type R = crate::R<D1QwcSpec>;
#[doc = "Register `D1_QWC` writer"]
pub type W = crate::W<D1QwcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch1 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_qwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_qwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1QwcSpec;
impl crate::RegisterSpec for D1QwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1_qwc::R`](R) reader structure"]
impl crate::Readable for D1QwcSpec {}
#[doc = "`write(|w| ..)` method takes [`d1_qwc::W`](W) writer structure"]
impl crate::Writable for D1QwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1_QWC to value 0"]
impl crate::Resettable for D1QwcSpec {}
