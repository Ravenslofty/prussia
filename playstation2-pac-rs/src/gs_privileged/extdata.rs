#[doc = "Register `EXTDATA` writer"]
pub type W = crate::W<ExtdataSpec>;
#[doc = "Field `SX` writer - Upper left X position (px)."]
pub type SxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SY` writer - Upper left Y position (px)."]
pub type SyW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SMPH` writer - Horizontal sampling rate interval (measured in scanlines)."]
pub type SmphW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMPV` writer - Vertical sampling rate interval (VCK)."]
pub type SmpvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WW` writer - Rectangular area width - 1."]
pub type WwW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WH` writer - Rectangular area height - 1."]
pub type WhW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl W {
    #[doc = "Bits 0:11 - Upper left X position (px)."]
    #[inline(always)]
    pub fn sx(&mut self) -> SxW<ExtdataSpec> {
        SxW::new(self, 0)
    }
    #[doc = "Bits 12:22 - Upper left Y position (px)."]
    #[inline(always)]
    pub fn sy(&mut self) -> SyW<ExtdataSpec> {
        SyW::new(self, 12)
    }
    #[doc = "Bits 23:26 - Horizontal sampling rate interval (measured in scanlines)."]
    #[inline(always)]
    pub fn smph(&mut self) -> SmphW<ExtdataSpec> {
        SmphW::new(self, 23)
    }
    #[doc = "Bits 27:28 - Vertical sampling rate interval (VCK)."]
    #[inline(always)]
    pub fn smpv(&mut self) -> SmpvW<ExtdataSpec> {
        SmpvW::new(self, 27)
    }
    #[doc = "Bits 32:43 - Rectangular area width - 1."]
    #[inline(always)]
    pub fn ww(&mut self) -> WwW<ExtdataSpec> {
        WwW::new(self, 32)
    }
    #[doc = "Bits 44:54 - Rectangular area height - 1."]
    #[inline(always)]
    pub fn wh(&mut self) -> WhW<ExtdataSpec> {
        WhW::new(self, 44)
    }
}
#[doc = "Feedback write setting.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtdataSpec;
impl crate::RegisterSpec for ExtdataSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`extdata::W`](W) writer structure"]
impl crate::Writable for ExtdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTDATA to value 0"]
impl crate::Resettable for ExtdataSpec {}
