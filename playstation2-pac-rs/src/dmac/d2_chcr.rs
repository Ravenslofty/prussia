#[doc = "Register `D2_CHCR` reader"]
pub type R = crate::R<D2ChcrSpec>;
#[doc = "Register `D2_CHCR` writer"]
pub type W = crate::W<D2ChcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch2 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_chcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_chcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2ChcrSpec;
impl crate::RegisterSpec for D2ChcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2_chcr::R`](R) reader structure"]
impl crate::Readable for D2ChcrSpec {}
#[doc = "`write(|w| ..)` method takes [`d2_chcr::W`](W) writer structure"]
impl crate::Writable for D2ChcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D2_CHCR to value 0"]
impl crate::Resettable for D2ChcrSpec {}
