#[doc = "Register `DISPLAY1` writer"]
pub type W = crate::W<Display1Spec>;
#[doc = "Field `DX` writer - Display X position (VCK)."]
pub type DxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DY` writer - Display Y position (px)."]
pub type DyW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MAGH` writer - Horizontal magnification in VCKs. Magnification = factor -1 (0 = 1x, 1 = 2x, etc.)"]
pub type MaghW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAGV` writer - Vertical magnification in scanlines. Magnification = factor -1 (0 = 1x, 1 = 2x, etc.)"]
pub type MagvW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DW` writer - Display area width - 1 (VCK)."]
pub type DwW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DH` writer - Display area height - 1 (px)."]
pub type DhW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl W {
    #[doc = "Bits 10:11 - Display X position (VCK)."]
    #[inline(always)]
    pub fn dx(&mut self) -> DxW<Display1Spec> {
        DxW::new(self, 10)
    }
    #[doc = "Bits 12:22 - Display Y position (px)."]
    #[inline(always)]
    pub fn dy(&mut self) -> DyW<Display1Spec> {
        DyW::new(self, 12)
    }
    #[doc = "Bits 23:24 - Horizontal magnification in VCKs. Magnification = factor -1 (0 = 1x, 1 = 2x, etc.)"]
    #[inline(always)]
    pub fn magh(&mut self) -> MaghW<Display1Spec> {
        MaghW::new(self, 23)
    }
    #[doc = "Bits 27:38 - Vertical magnification in scanlines. Magnification = factor -1 (0 = 1x, 1 = 2x, etc.)"]
    #[inline(always)]
    pub fn magv(&mut self) -> MagvW<Display1Spec> {
        MagvW::new(self, 27)
    }
    #[doc = "Bits 32:43 - Display area width - 1 (VCK)."]
    #[inline(always)]
    pub fn dw(&mut self) -> DwW<Display1Spec> {
        DwW::new(self, 32)
    }
    #[doc = "Bits 44:54 - Display area height - 1 (px)."]
    #[inline(always)]
    pub fn dh(&mut self) -> DhW<Display1Spec> {
        DhW::new(self, 44)
    }
}
#[doc = "Rectangular viewport from output of Read Circuit 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`display1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Display1Spec;
impl crate::RegisterSpec for Display1Spec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`display1::W`](W) writer structure"]
impl crate::Writable for Display1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISPLAY1 to value 0"]
impl crate::Resettable for Display1Spec {}
