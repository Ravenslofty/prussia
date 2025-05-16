#[doc = "Register `D8_SADR` reader"]
pub type R = crate::R<D8SadrSpec>;
#[doc = "Register `D8_SADR` writer"]
pub type W = crate::W<D8SadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ch8 SPR address\n\nYou can [`read`](crate::Reg::read) this register and get [`d8_sadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d8_sadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D8SadrSpec;
impl crate::RegisterSpec for D8SadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d8_sadr::R`](R) reader structure"]
impl crate::Readable for D8SadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d8_sadr::W`](W) writer structure"]
impl crate::Writable for D8SadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D8_SADR to value 0"]
impl crate::Resettable for D8SadrSpec {}
