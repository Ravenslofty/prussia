#[doc = "Register `D9_CHCR` reader"]
pub type R = crate::R<D9ChcrSpec>;
#[doc = "Register `D9_CHCR` writer"]
pub type W = crate::W<D9ChcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch9 channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`d9_chcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d9_chcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D9ChcrSpec;
impl crate::RegisterSpec for D9ChcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d9_chcr::R`](R) reader structure"]
impl crate::Readable for D9ChcrSpec {}
#[doc = "`write(|w| ..)` method takes [`d9_chcr::W`](W) writer structure"]
impl crate::Writable for D9ChcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D9_CHCR to value 0"]
impl crate::Resettable for D9ChcrSpec {}
