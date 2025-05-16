#[doc = "Register `D9_SADR` reader"]
pub type R = crate::R<D9SadrSpec>;
#[doc = "Register `D9_SADR` writer"]
pub type W = crate::W<D9SadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch9 SPR address\n\nYou can [`read`](crate::Reg::read) this register and get [`d9_sadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d9_sadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D9SadrSpec;
impl crate::RegisterSpec for D9SadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d9_sadr::R`](R) reader structure"]
impl crate::Readable for D9SadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d9_sadr::W`](W) writer structure"]
impl crate::Writable for D9SadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D9_SADR to value 0"]
impl crate::Resettable for D9SadrSpec {}
