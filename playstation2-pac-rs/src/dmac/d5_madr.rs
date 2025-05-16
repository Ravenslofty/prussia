#[doc = "Register `D5_MADR` reader"]
pub type R = crate::R<D5MadrSpec>;
#[doc = "Register `D5_MADR` writer"]
pub type W = crate::W<D5MadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch5 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d5_madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d5_madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D5MadrSpec;
impl crate::RegisterSpec for D5MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d5_madr::R`](R) reader structure"]
impl crate::Readable for D5MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d5_madr::W`](W) writer structure"]
impl crate::Writable for D5MadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D5_MADR to value 0"]
impl crate::Resettable for D5MadrSpec {}
