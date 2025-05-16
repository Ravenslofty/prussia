#[doc = "Register `D4_MADR` reader"]
pub type R = crate::R<D4MadrSpec>;
#[doc = "Register `D4_MADR` writer"]
pub type W = crate::W<D4MadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch4 memory address\n\nYou can [`read`](crate::Reg::read) this register and get [`d4_madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d4_madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D4MadrSpec;
impl crate::RegisterSpec for D4MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d4_madr::R`](R) reader structure"]
impl crate::Readable for D4MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d4_madr::W`](W) writer structure"]
impl crate::Writable for D4MadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D4_MADR to value 0"]
impl crate::Resettable for D4MadrSpec {}
