#[doc = "Register `SMODE2` writer"]
pub type W = crate::W<Smode2Spec>;
#[doc = "Interlace mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: Progressive (noninterlace) mode."]
    Progressive = 0,
    #[doc = "1: Interlace mode."]
    Interlace = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` writer - Interlace mode."]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG, Int>;
impl<'a, REG> IntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Progressive (noninterlace) mode."]
    #[inline(always)]
    pub fn progressive(self) -> &'a mut crate::W<REG> {
        self.variant(Int::Progressive)
    }
    #[doc = "Interlace mode."]
    #[inline(always)]
    pub fn interlace(self) -> &'a mut crate::W<REG> {
        self.variant(Int::Interlace)
    }
}
#[doc = "Field or Frame mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffmd {
    #[doc = "0: Field mode (every other line is read.)"]
    Field = 0,
    #[doc = "1: Frame mode (every line is read.)"]
    Frame = 1,
}
impl From<Ffmd> for bool {
    #[inline(always)]
    fn from(variant: Ffmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFMD` writer - Field or Frame mode."]
pub type FfmdW<'a, REG> = crate::BitWriter<'a, REG, Ffmd>;
impl<'a, REG> FfmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Field mode (every other line is read.)"]
    #[inline(always)]
    pub fn field(self) -> &'a mut crate::W<REG> {
        self.variant(Ffmd::Field)
    }
    #[doc = "Frame mode (every line is read.)"]
    #[inline(always)]
    pub fn frame(self) -> &'a mut crate::W<REG> {
        self.variant(Ffmd::Frame)
    }
}
#[doc = "VESA display power management signalling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dpms {
    #[doc = "0: In use."]
    On = 0,
    #[doc = "1: Blanked, low power."]
    Standby = 1,
    #[doc = "2: Blanked, lower power."]
    Suspend = 2,
    #[doc = "3: Shut off, awaiting activity."]
    Off = 3,
}
impl From<Dpms> for u8 {
    #[inline(always)]
    fn from(variant: Dpms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dpms {
    type Ux = u8;
}
impl crate::IsEnum for Dpms {}
#[doc = "Field `DPMS` writer - VESA display power management signalling."]
pub type DpmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dpms, crate::Safe>;
impl<'a, REG> DpmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "In use."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Dpms::On)
    }
    #[doc = "Blanked, low power."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(Dpms::Standby)
    }
    #[doc = "Blanked, lower power."]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Dpms::Suspend)
    }
    #[doc = "Shut off, awaiting activity."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Dpms::Off)
    }
}
impl W {
    #[doc = "Bit 0 - Interlace mode."]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<Smode2Spec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Field or Frame mode."]
    #[inline(always)]
    pub fn ffmd(&mut self) -> FfmdW<Smode2Spec> {
        FfmdW::new(self, 1)
    }
    #[doc = "Bits 2:3 - VESA display power management signalling."]
    #[inline(always)]
    pub fn dpms(&mut self) -> DpmsW<Smode2Spec> {
        DpmsW::new(self, 2)
    }
}
#[doc = "Video mode settings for synchronisation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smode2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smode2Spec;
impl crate::RegisterSpec for Smode2Spec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`smode2::W`](W) writer structure"]
impl crate::Writable for Smode2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMODE2 to value 0"]
impl crate::Resettable for Smode2Spec {}
