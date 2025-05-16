#[doc = "Register `D1_MADR` reader"]
pub type R = crate::R<D1MadrSpec>;
#[doc = "Register `D1_MADR` writer"]
pub type W = crate::W<D1MadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch1 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d1_madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1_madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1MadrSpec;
impl crate::RegisterSpec for D1MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1_madr::R`](R) reader structure"]
impl crate::Readable for D1MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d1_madr::W`](W) writer structure"]
impl crate::Writable for D1MadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1_MADR to value 0"]
impl crate::Resettable for D1MadrSpec {}
