#[doc = "Register `D6_TADR` reader"]
pub type R = crate::R<D6TadrSpec>;
#[doc = "Register `D6_TADR` writer"]
pub type W = crate::W<D6TadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch6 tag address\n\nYou can [`read`](crate::Reg::read) this register and get [`d6_tadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6_tadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D6TadrSpec;
impl crate::RegisterSpec for D6TadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d6_tadr::R`](R) reader structure"]
impl crate::Readable for D6TadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d6_tadr::W`](W) writer structure"]
impl crate::Writable for D6TadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D6_TADR to value 0"]
impl crate::Resettable for D6TadrSpec {}
