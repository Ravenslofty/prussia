#[doc = "Register `D0_CHCR` reader"]
pub type R = crate::R<D0ChcrSpec>;
#[doc = "Register `D0_CHCR` writer"]
pub type W = crate::W<D0ChcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch0 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_chcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_chcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0ChcrSpec;
impl crate::RegisterSpec for D0ChcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d0_chcr::R`](R) reader structure"]
impl crate::Readable for D0ChcrSpec {}
#[doc = "`write(|w| ..)` method takes [`d0_chcr::W`](W) writer structure"]
impl crate::Writable for D0ChcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D0_CHCR to value 0"]
impl crate::Resettable for D0ChcrSpec {}
