#[doc = "Register `D7_QWC` reader"]
pub type R = crate::R<D7QwcSpec>;
#[doc = "Register `D7_QWC` writer"]
pub type W = crate::W<D7QwcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch7 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d7_qwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d7_qwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D7QwcSpec;
impl crate::RegisterSpec for D7QwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d7_qwc::R`](R) reader structure"]
impl crate::Readable for D7QwcSpec {}
#[doc = "`write(|w| ..)` method takes [`d7_qwc::W`](W) writer structure"]
impl crate::Writable for D7QwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D7_QWC to value 0"]
impl crate::Resettable for D7QwcSpec {}
