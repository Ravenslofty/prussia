#[doc = "Register `D2_MADR` reader"]
pub type R = crate::R<D2MadrSpec>;
#[doc = "Register `D2_MADR` writer"]
pub type W = crate::W<D2MadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch2 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d2_madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2_madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2MadrSpec;
impl crate::RegisterSpec for D2MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2_madr::R`](R) reader structure"]
impl crate::Readable for D2MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d2_madr::W`](W) writer structure"]
impl crate::Writable for D2MadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D2_MADR to value 0"]
impl crate::Resettable for D2MadrSpec {}
