#[doc = "Register `D6_CHCR` reader"]
pub type R = crate::R<D6ChcrSpec>;
#[doc = "Register `D6_CHCR` writer"]
pub type W = crate::W<D6ChcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch6 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d6_chcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6_chcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D6ChcrSpec;
impl crate::RegisterSpec for D6ChcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d6_chcr::R`](R) reader structure"]
impl crate::Readable for D6ChcrSpec {}
#[doc = "`write(|w| ..)` method takes [`d6_chcr::W`](W) writer structure"]
impl crate::Writable for D6ChcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D6_CHCR to value 0"]
impl crate::Resettable for D6ChcrSpec {}
