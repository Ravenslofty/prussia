#[doc = "Register `GIF_CTRL` writer"]
pub type W = crate::W<GifCtrlSpec>;
#[doc = "GIF reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rst {
    #[doc = "0: Does nothing."]
    NoReset = 0,
    #[doc = "1: When set, the GIF is reset and the internal register returns to the initial value."]
    Reset = 1,
}
impl From<Rst> for bool {
    #[inline(always)]
    fn from(variant: Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` writer - GIF reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rst>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does nothing."]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::NoReset)
    }
    #[doc = "When set, the GIF is reset and the internal register returns to the initial value."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Reset)
    }
}
#[doc = "Temporary transfer stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pse {
    #[doc = "0: Restarts transfer processing. When set, the other GIF registers can no longer be read."]
    TransferRestart = 0,
    #[doc = "1: Temporarily stops transfer processing. When set, allows reading the other GIF registers for debugging."]
    TempStop = 1,
}
impl From<Pse> for bool {
    #[inline(always)]
    fn from(variant: Pse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSE` writer - Temporary transfer stop"]
pub type PseW<'a, REG> = crate::BitWriter<'a, REG, Pse>;
impl<'a, REG> PseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Restarts transfer processing. When set, the other GIF registers can no longer be read."]
    #[inline(always)]
    pub fn transfer_restart(self) -> &'a mut crate::W<REG> {
        self.variant(Pse::TransferRestart)
    }
    #[doc = "Temporarily stops transfer processing. When set, allows reading the other GIF registers for debugging."]
    #[inline(always)]
    pub fn temp_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Pse::TempStop)
    }
}
impl W {
    #[doc = "Bit 0 - GIF reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<GifCtrlSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bit 3 - Temporary transfer stop"]
    #[inline(always)]
    pub fn pse(&mut self) -> PseW<GifCtrlSpec> {
        PseW::new(self, 3)
    }
}
#[doc = "GIF control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gif_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifCtrlSpec;
impl crate::RegisterSpec for GifCtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gif_ctrl::W`](W) writer structure"]
impl crate::Writable for GifCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GIF_CTRL to value 0"]
impl crate::Resettable for GifCtrlSpec {}
