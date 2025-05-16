#[doc = "Register `D8_MADR` reader"]
pub type R = crate::R<D8MadrSpec>;
#[doc = "Register `D8_MADR` writer"]
pub type W = crate::W<D8MadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch8 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d8_madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d8_madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D8MadrSpec;
impl crate::RegisterSpec for D8MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d8_madr::R`](R) reader structure"]
impl crate::Readable for D8MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d8_madr::W`](W) writer structure"]
impl crate::Writable for D8MadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D8_MADR to value 0"]
impl crate::Resettable for D8MadrSpec {}
