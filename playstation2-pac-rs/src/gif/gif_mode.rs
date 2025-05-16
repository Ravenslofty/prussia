#[doc = "Register `GIF_MODE` writer"]
pub type W = crate::W<GifModeSpec>;
#[doc = "PATH3 MASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M3r {
    #[doc = "0: Mask cancel(Initial value)"]
    MaskCancel = 0,
    #[doc = "1: Enables the PATH3 mask. If set during a transmission, the mask is enabled after the end of transmission."]
    Mask = 1,
}
impl From<M3r> for bool {
    #[inline(always)]
    fn from(variant: M3r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M3R` writer - PATH3 MASK"]
pub type M3rW<'a, REG> = crate::BitWriter<'a, REG, M3r>;
impl<'a, REG> M3rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask cancel(Initial value)"]
    #[inline(always)]
    pub fn mask_cancel(self) -> &'a mut crate::W<REG> {
        self.variant(M3r::MaskCancel)
    }
    #[doc = "Enables the PATH3 mask. If set during a transmission, the mask is enabled after the end of transmission."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(M3r::Mask)
    }
}
#[doc = "PATH3 transfer mode specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imt {
    #[doc = "0: (Initial value) Continuous transfer mode"]
    ContinuousTransfer = 0,
    #[doc = "1: Intermittent transfer mode in every 8 qwords."]
    IntermittentTransferMode = 1,
}
impl From<Imt> for bool {
    #[inline(always)]
    fn from(variant: Imt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMT` writer - PATH3 transfer mode specification"]
pub type ImtW<'a, REG> = crate::BitWriter<'a, REG, Imt>;
impl<'a, REG> ImtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(Initial value) Continuous transfer mode"]
    #[inline(always)]
    pub fn continuous_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Imt::ContinuousTransfer)
    }
    #[doc = "Intermittent transfer mode in every 8 qwords."]
    #[inline(always)]
    pub fn intermittent_transfer_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Imt::IntermittentTransferMode)
    }
}
impl W {
    #[doc = "Bit 0 - PATH3 MASK"]
    #[inline(always)]
    pub fn m3r(&mut self) -> M3rW<GifModeSpec> {
        M3rW::new(self, 0)
    }
    #[doc = "Bit 2 - PATH3 transfer mode specification"]
    #[inline(always)]
    pub fn imt(&mut self) -> ImtW<GifModeSpec> {
        ImtW::new(self, 2)
    }
}
#[doc = "GIF mode setting\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gif_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifModeSpec;
impl crate::RegisterSpec for GifModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gif_mode::W`](W) writer structure"]
impl crate::Writable for GifModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GIF_MODE to value 0"]
impl crate::Resettable for GifModeSpec {}
