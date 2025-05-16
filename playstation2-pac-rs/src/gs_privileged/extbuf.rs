#[doc = "Register `EXTBUF` writer"]
pub type W = crate::W<ExtbufSpec>;
#[doc = "Field `EXBP` writer - Buffer base pointer / 64."]
pub type ExbpW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `EXBW` writer - Width of buffer / 64."]
pub type ExbwW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Input source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fbin {
    #[doc = "0: Output from Read Circuit 1."]
    Out1 = 0,
    #[doc = "1: Output from Read Circuit 2."]
    Out2 = 1,
}
impl From<Fbin> for u8 {
    #[inline(always)]
    fn from(variant: Fbin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fbin {
    type Ux = u8;
}
impl crate::IsEnum for Fbin {}
#[doc = "Field `FBIN` writer - Input source selection."]
pub type FbinW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fbin>;
impl<'a, REG> FbinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output from Read Circuit 1."]
    #[inline(always)]
    pub fn out1(self) -> &'a mut crate::W<REG> {
        self.variant(Fbin::Out1)
    }
    #[doc = "Output from Read Circuit 2."]
    #[inline(always)]
    pub fn out2(self) -> &'a mut crate::W<REG> {
        self.variant(Fbin::Out2)
    }
}
#[doc = "Interlace mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wffmd {
    #[doc = "0: Write to every other raster."]
    Field = 0,
    #[doc = "1: Write to every raster."]
    Frame = 1,
}
impl From<Wffmd> for bool {
    #[inline(always)]
    fn from(variant: Wffmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFFMD` writer - Interlace mode"]
pub type WffmdW<'a, REG> = crate::BitWriter<'a, REG, Wffmd>;
impl<'a, REG> WffmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to every other raster."]
    #[inline(always)]
    pub fn field(self) -> &'a mut crate::W<REG> {
        self.variant(Wffmd::Field)
    }
    #[doc = "Write to every raster."]
    #[inline(always)]
    pub fn frame(self) -> &'a mut crate::W<REG> {
        self.variant(Wffmd::Frame)
    }
}
#[doc = "Processing of input alpha.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emoda {
    #[doc = "0: Write input alpha as-is from alpha channel."]
    Alpha = 0,
    #[doc = "1: Write input alpha as-is from Y colour channel."]
    Y = 1,
    #[doc = "2: Write input alpha as half of Y colour channel."]
    YHalf = 2,
    #[doc = "3: Alpha is fixed at zero."]
    Zero = 3,
}
impl From<Emoda> for u8 {
    #[inline(always)]
    fn from(variant: Emoda) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emoda {
    type Ux = u8;
}
impl crate::IsEnum for Emoda {}
#[doc = "Field `EMODA` writer - Processing of input alpha."]
pub type EmodaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Emoda, crate::Safe>;
impl<'a, REG> EmodaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write input alpha as-is from alpha channel."]
    #[inline(always)]
    pub fn alpha(self) -> &'a mut crate::W<REG> {
        self.variant(Emoda::Alpha)
    }
    #[doc = "Write input alpha as-is from Y colour channel."]
    #[inline(always)]
    pub fn y(self) -> &'a mut crate::W<REG> {
        self.variant(Emoda::Y)
    }
    #[doc = "Write input alpha as half of Y colour channel."]
    #[inline(always)]
    pub fn y_half(self) -> &'a mut crate::W<REG> {
        self.variant(Emoda::YHalf)
    }
    #[doc = "Alpha is fixed at zero."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Emoda::Zero)
    }
}
#[doc = "Processing of input colour.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emodc {
    #[doc = "0: Read colour from RGB channels."]
    Rgb = 0,
    #[doc = "1: Read colour as Y channel."]
    Y = 1,
    #[doc = "2: Read color from YcBcR channels."]
    Ycbcr = 2,
    #[doc = "3: Read color from alpha channel."]
    Alpha = 3,
}
impl From<Emodc> for u8 {
    #[inline(always)]
    fn from(variant: Emodc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emodc {
    type Ux = u8;
}
impl crate::IsEnum for Emodc {}
#[doc = "Field `EMODC` writer - Processing of input colour."]
pub type EmodcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Emodc, crate::Safe>;
impl<'a, REG> EmodcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read colour from RGB channels."]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut crate::W<REG> {
        self.variant(Emodc::Rgb)
    }
    #[doc = "Read colour as Y channel."]
    #[inline(always)]
    pub fn y(self) -> &'a mut crate::W<REG> {
        self.variant(Emodc::Y)
    }
    #[doc = "Read color from YcBcR channels."]
    #[inline(always)]
    pub fn ycbcr(self) -> &'a mut crate::W<REG> {
        self.variant(Emodc::Ycbcr)
    }
    #[doc = "Read color from alpha channel."]
    #[inline(always)]
    pub fn alpha(self) -> &'a mut crate::W<REG> {
        self.variant(Emodc::Alpha)
    }
}
#[doc = "Field `WDX` writer - Upper left X position."]
pub type WdxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `WDY` writer - Upper left Y position."]
pub type WdyW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl W {
    #[doc = "Bits 0:13 - Buffer base pointer / 64."]
    #[inline(always)]
    pub fn exbp(&mut self) -> ExbpW<ExtbufSpec> {
        ExbpW::new(self, 0)
    }
    #[doc = "Bits 14:19 - Width of buffer / 64."]
    #[inline(always)]
    pub fn exbw(&mut self) -> ExbwW<ExtbufSpec> {
        ExbwW::new(self, 14)
    }
    #[doc = "Bits 20:21 - Input source selection."]
    #[inline(always)]
    pub fn fbin(&mut self) -> FbinW<ExtbufSpec> {
        FbinW::new(self, 20)
    }
    #[doc = "Bit 22 - Interlace mode"]
    #[inline(always)]
    pub fn wffmd(&mut self) -> WffmdW<ExtbufSpec> {
        WffmdW::new(self, 22)
    }
    #[doc = "Bits 23:24 - Processing of input alpha."]
    #[inline(always)]
    pub fn emoda(&mut self) -> EmodaW<ExtbufSpec> {
        EmodaW::new(self, 23)
    }
    #[doc = "Bits 25:26 - Processing of input colour."]
    #[inline(always)]
    pub fn emodc(&mut self) -> EmodcW<ExtbufSpec> {
        EmodcW::new(self, 25)
    }
    #[doc = "Bits 32:42 - Upper left X position."]
    #[inline(always)]
    pub fn wdx(&mut self) -> WdxW<ExtbufSpec> {
        WdxW::new(self, 32)
    }
    #[doc = "Bits 43:53 - Upper left Y position."]
    #[inline(always)]
    pub fn wdy(&mut self) -> WdyW<ExtbufSpec> {
        WdyW::new(self, 43)
    }
}
#[doc = "Feedback write buffer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extbuf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtbufSpec;
impl crate::RegisterSpec for ExtbufSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`extbuf::W`](W) writer structure"]
impl crate::Writable for ExtbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTBUF to value 0"]
impl crate::Resettable for ExtbufSpec {}
