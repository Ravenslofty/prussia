#[doc = "Register `PMODE` writer"]
pub type W = crate::W<PmodeSpec>;
#[doc = "Enable / disable Read Circuit 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En1 {
    #[doc = "0: Disable Read Circuit 1."]
    Disable = 0,
    #[doc = "1: Enable Read Circuit 1."]
    Enable = 1,
}
impl From<En1> for bool {
    #[inline(always)]
    fn from(variant: En1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` writer - Enable / disable Read Circuit 1."]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG, En1>;
impl<'a, REG> En1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Read Circuit 1."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En1::Disable)
    }
    #[doc = "Enable Read Circuit 1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En1::Enable)
    }
}
#[doc = "Enable / disable Read Circuit 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En2 {
    #[doc = "0: Disable Read Circuit 2."]
    Disable = 0,
    #[doc = "1: Enable Read Circuit 2."]
    Enable = 1,
}
impl From<En2> for bool {
    #[inline(always)]
    fn from(variant: En2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN2` writer - Enable / disable Read Circuit 2."]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG, En2>;
impl<'a, REG> En2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Read Circuit 2."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En2::Disable)
    }
    #[doc = "Enable Read Circuit 2."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En2::Enable)
    }
}
#[doc = "CRT output switching. Always set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crtmd {
    #[doc = "1: Only (known) valid value."]
    Out1 = 1,
}
impl From<Crtmd> for u8 {
    #[inline(always)]
    fn from(variant: Crtmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crtmd {
    type Ux = u8;
}
impl crate::IsEnum for Crtmd {}
#[doc = "Field `CRTMD` writer - CRT output switching. Always set to 1."]
pub type CrtmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Crtmd>;
impl<'a, REG> CrtmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only (known) valid value."]
    #[inline(always)]
    pub fn out1(self) -> &'a mut crate::W<REG> {
        self.variant(Crtmd::Out1)
    }
}
#[doc = "Alpha blending value source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmod {
    #[doc = "0: Get alpha from ALP field."]
    BlendAlp = 0,
    #[doc = "1: Get alpha from output of Read Circuit 1."]
    BlendRc1 = 1,
}
impl From<Mmod> for bool {
    #[inline(always)]
    fn from(variant: Mmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMOD` writer - Alpha blending value source."]
pub type MmodW<'a, REG> = crate::BitWriter<'a, REG, Mmod>;
impl<'a, REG> MmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Get alpha from ALP field."]
    #[inline(always)]
    pub fn blend_alp(self) -> &'a mut crate::W<REG> {
        self.variant(Mmod::BlendAlp)
    }
    #[doc = "Get alpha from output of Read Circuit 1."]
    #[inline(always)]
    pub fn blend_rc1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmod::BlendRc1)
    }
}
#[doc = "Alpha source to CRTMD chosen output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amod {
    #[doc = "0: Alpha value from Read Circuit 1 for output selection."]
    Rc1 = 0,
    #[doc = "1: Alpha value from Read Circuit 2 for output selection."]
    Rc2 = 1,
}
impl From<Amod> for bool {
    #[inline(always)]
    fn from(variant: Amod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMOD` writer - Alpha source to CRTMD chosen output."]
pub type AmodW<'a, REG> = crate::BitWriter<'a, REG, Amod>;
impl<'a, REG> AmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alpha value from Read Circuit 1 for output selection."]
    #[inline(always)]
    pub fn rc1(self) -> &'a mut crate::W<REG> {
        self.variant(Amod::Rc1)
    }
    #[doc = "Alpha value from Read Circuit 2 for output selection."]
    #[inline(always)]
    pub fn rc2(self) -> &'a mut crate::W<REG> {
        self.variant(Amod::Rc2)
    }
}
#[doc = "Alpha blending method.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slbg {
    #[doc = "0: Blend alpha with output of Read Circuit 2."]
    Circuit2 = 0,
    #[doc = "1: Use background color."]
    Bgcolor = 1,
}
impl From<Slbg> for bool {
    #[inline(always)]
    fn from(variant: Slbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLBG` writer - Alpha blending method."]
pub type SlbgW<'a, REG> = crate::BitWriter<'a, REG, Slbg>;
impl<'a, REG> SlbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Blend alpha with output of Read Circuit 2."]
    #[inline(always)]
    pub fn circuit2(self) -> &'a mut crate::W<REG> {
        self.variant(Slbg::Circuit2)
    }
    #[doc = "Use background color."]
    #[inline(always)]
    pub fn bgcolor(self) -> &'a mut crate::W<REG> {
        self.variant(Slbg::Bgcolor)
    }
}
#[doc = "Field `ALP` writer - Alpha blending value."]
pub type AlpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ONES` writer - All set to one. Behaviour is undefined according to the GS users manual but advises to set them regardless, so likely related to a hardware bug."]
pub type OnesW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable / disable Read Circuit 1."]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<PmodeSpec> {
        En1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable / disable Read Circuit 2."]
    #[inline(always)]
    pub fn en2(&mut self) -> En2W<PmodeSpec> {
        En2W::new(self, 1)
    }
    #[doc = "Bits 2:4 - CRT output switching. Always set to 1."]
    #[inline(always)]
    pub fn crtmd(&mut self) -> CrtmdW<PmodeSpec> {
        CrtmdW::new(self, 2)
    }
    #[doc = "Bit 5 - Alpha blending value source."]
    #[inline(always)]
    pub fn mmod(&mut self) -> MmodW<PmodeSpec> {
        MmodW::new(self, 5)
    }
    #[doc = "Bit 6 - Alpha source to CRTMD chosen output."]
    #[inline(always)]
    pub fn amod(&mut self) -> AmodW<PmodeSpec> {
        AmodW::new(self, 6)
    }
    #[doc = "Bit 7 - Alpha blending method."]
    #[inline(always)]
    pub fn slbg(&mut self) -> SlbgW<PmodeSpec> {
        SlbgW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Alpha blending value."]
    #[inline(always)]
    pub fn alp(&mut self) -> AlpW<PmodeSpec> {
        AlpW::new(self, 8)
    }
    #[doc = "Bit 16 - All set to one. Behaviour is undefined according to the GS users manual but advises to set them regardless, so likely related to a hardware bug."]
    #[inline(always)]
    pub fn ones(&mut self) -> OnesW<PmodeSpec> {
        OnesW::new(self, 16)
    }
}
#[doc = "Various PCRTC controls. Accessible via EE.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmodeSpec;
impl crate::RegisterSpec for PmodeSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`pmode::W`](W) writer structure"]
impl crate::Writable for PmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMODE to value 0"]
impl crate::Resettable for PmodeSpec {}
