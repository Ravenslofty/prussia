#[doc = "Register `DISPFB1` writer"]
pub type W = crate::W<Dispfb1Spec>;
#[doc = "Field `FBP` writer - Base pointer address / 2048."]
pub type FbpW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FBW` writer - Buffer width / 64."]
pub type FbwW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PSM` writer - Pixel storage format."]
pub type PsmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBX` writer - Upper left X position."]
pub type DbxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DBY` writer - Upper left Y position."]
pub type DbyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl W {
    #[doc = "Bits 0:8 - Base pointer address / 2048."]
    #[inline(always)]
    pub fn fbp(&mut self) -> FbpW<Dispfb1Spec> {
        FbpW::new(self, 0)
    }
    #[doc = "Bits 9:14 - Buffer width / 64."]
    #[inline(always)]
    pub fn fbw(&mut self) -> FbwW<Dispfb1Spec> {
        FbwW::new(self, 9)
    }
    #[doc = "Bits 15:19 - Pixel storage format."]
    #[inline(always)]
    pub fn psm(&mut self) -> PsmW<Dispfb1Spec> {
        PsmW::new(self, 15)
    }
    #[doc = "Bits 20:31 - Upper left X position."]
    #[inline(always)]
    pub fn dbx(&mut self) -> DbxW<Dispfb1Spec> {
        DbxW::new(self, 20)
    }
    #[doc = "Bits 32:43 - Upper left Y position."]
    #[inline(always)]
    pub fn dby(&mut self) -> DbyW<Dispfb1Spec> {
        DbyW::new(self, 32)
    }
}
#[doc = "Framebuffer register for Output Circuit 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispfb1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dispfb1Spec;
impl crate::RegisterSpec for Dispfb1Spec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`dispfb1::W`](W) writer structure"]
impl crate::Writable for Dispfb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISPFB1 to value 0"]
impl crate::Resettable for Dispfb1Spec {}
