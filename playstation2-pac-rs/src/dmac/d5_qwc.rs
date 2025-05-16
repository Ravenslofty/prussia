#[doc = "Register `D5_QWC` reader"]
pub type R = crate::R<D5QwcSpec>;
#[doc = "Register `D5_QWC` writer"]
pub type W = crate::W<D5QwcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch5 quad word count\n\nYou can [`read`](crate::Reg::read) this register and get [`d5_qwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d5_qwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D5QwcSpec;
impl crate::RegisterSpec for D5QwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d5_qwc::R`](R) reader structure"]
impl crate::Readable for D5QwcSpec {}
#[doc = "`write(|w| ..)` method takes [`d5_qwc::W`](W) writer structure"]
impl crate::Writable for D5QwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D5_QWC to value 0"]
impl crate::Resettable for D5QwcSpec {}
