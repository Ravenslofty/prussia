#[doc = "Register `D5_CHCR` reader"]
pub type R = crate::R<D5ChcrSpec>;
#[doc = "Register `D5_CHCR` writer"]
pub type W = crate::W<D5ChcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch5 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d5_chcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d5_chcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D5ChcrSpec;
impl crate::RegisterSpec for D5ChcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d5_chcr::R`](R) reader structure"]
impl crate::Readable for D5ChcrSpec {}
#[doc = "`write(|w| ..)` method takes [`d5_chcr::W`](W) writer structure"]
impl crate::Writable for D5ChcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D5_CHCR to value 0"]
impl crate::Resettable for D5ChcrSpec {}
