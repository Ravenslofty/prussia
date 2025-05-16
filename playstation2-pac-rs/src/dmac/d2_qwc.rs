#[doc = "Register `D2_QWC` reader"]
pub type R = crate::R<D2QwcSpec>;
#[doc = "Register `D2_QWC` writer"]
pub type W = crate::W<D2QwcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch2 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_qwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_qwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2QwcSpec;
impl crate::RegisterSpec for D2QwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2_qwc::R`](R) reader structure"]
impl crate::Readable for D2QwcSpec {}
#[doc = "`write(|w| ..)` method takes [`d2_qwc::W`](W) writer structure"]
impl crate::Writable for D2QwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D2_QWC to value 0"]
impl crate::Resettable for D2QwcSpec {}
