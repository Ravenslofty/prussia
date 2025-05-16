#[doc = "Register `D0_QWC` reader"]
pub type R = crate::R<D0QwcSpec>;
#[doc = "Register `D0_QWC` writer"]
pub type W = crate::W<D0QwcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch0 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_qwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_qwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0QwcSpec;
impl crate::RegisterSpec for D0QwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d0_qwc::R`](R) reader structure"]
impl crate::Readable for D0QwcSpec {}
#[doc = "`write(|w| ..)` method takes [`d0_qwc::W`](W) writer structure"]
impl crate::Writable for D0QwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D0_QWC to value 0"]
impl crate::Resettable for D0QwcSpec {}
