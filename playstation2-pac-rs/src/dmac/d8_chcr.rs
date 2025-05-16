#[doc = "Register `D8_CHCR` reader"]
pub type R = crate::R<D8ChcrSpec>;
#[doc = "Register `D8_CHCR` writer"]
pub type W = crate::W<D8ChcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch8 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d8_chcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d8_chcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D8ChcrSpec;
impl crate::RegisterSpec for D8ChcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d8_chcr::R`](R) reader structure"]
impl crate::Readable for D8ChcrSpec {}
#[doc = "`write(|w| ..)` method takes [`d8_chcr::W`](W) writer structure"]
impl crate::Writable for D8ChcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D8_CHCR to value 0"]
impl crate::Resettable for D8ChcrSpec {}
