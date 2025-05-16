#[doc = "Register `D0_MADR` reader"]
pub type R = crate::R<D0MadrSpec>;
#[doc = "Register `D0_MADR` writer"]
pub type W = crate::W<D0MadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch0 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d0_madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0_madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0MadrSpec;
impl crate::RegisterSpec for D0MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d0_madr::R`](R) reader structure"]
impl crate::Readable for D0MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d0_madr::W`](W) writer structure"]
impl crate::Writable for D0MadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D0_MADR to value 0"]
impl crate::Resettable for D0MadrSpec {}
