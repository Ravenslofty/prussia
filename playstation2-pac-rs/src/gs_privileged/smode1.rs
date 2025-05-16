#[doc = "Register `SMODE1` writer"]
pub type W = crate::W<Smode1Spec>;
#[doc = "Field `RC` writer - Phase Lock Loop (PLL) reference divider."]
pub type RcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LC` writer - Phase Lock Loop (PLL) loop divider."]
pub type LcW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `T128` writer - Phase Lock Loop (PLL) output divider."]
pub type T128W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "UNKNOWN. Appears to always be set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slck {
    #[doc = "0: Only ever seen set to 0."]
    Default = 0,
}
impl From<Slck> for bool {
    #[inline(always)]
    fn from(variant: Slck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLCK` writer - UNKNOWN. Appears to always be set to 0."]
pub type SlckW<'a, REG> = crate::BitWriter<'a, REG, Slck>;
impl<'a, REG> SlckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only ever seen set to 0."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Slck::Default)
    }
}
#[doc = "Subcarrier frequency (display mode.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmod {
    #[doc = "0: Output a VESA signal."]
    Vesa = 0,
    #[doc = "2: Output an NTSC signal."]
    Ntsc = 2,
    #[doc = "3: Output a PAL signal."]
    Pal = 3,
}
impl From<Cmod> for u8 {
    #[inline(always)]
    fn from(variant: Cmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmod {
    type Ux = u8;
}
impl crate::IsEnum for Cmod {}
#[doc = "Field `CMOD` writer - Subcarrier frequency (display mode.)"]
pub type CmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmod>;
impl<'a, REG> CmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output a VESA signal."]
    #[inline(always)]
    pub fn vesa(self) -> &'a mut crate::W<REG> {
        self.variant(Cmod::Vesa)
    }
    #[doc = "Output an NTSC signal."]
    #[inline(always)]
    pub fn ntsc(self) -> &'a mut crate::W<REG> {
        self.variant(Cmod::Ntsc)
    }
    #[doc = "Output a PAL signal."]
    #[inline(always)]
    pub fn pal(self) -> &'a mut crate::W<REG> {
        self.variant(Cmod::Pal)
    }
}
#[doc = "Field `EX` writer - UNKNOWN."]
pub type ExW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reset the PCRTC circuit. A special procedure should be followed to use properly (see GS Mode Selector app documentation.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prst {
    #[doc = "0: (Initial value) Does not trigger a reset."]
    Nothing = 0,
    #[doc = "1: Trigger a reset."]
    Reset = 1,
}
impl From<Prst> for bool {
    #[inline(always)]
    fn from(variant: Prst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRST` writer - Reset the PCRTC circuit. A special procedure should be followed to use properly (see GS Mode Selector app documentation.)"]
pub type PrstW<'a, REG> = crate::BitWriter<'a, REG, Prst>;
impl<'a, REG> PrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(Initial value) Does not trigger a reset."]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(Prst::Nothing)
    }
    #[doc = "Trigger a reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Prst::Reset)
    }
}
#[doc = "Activate / Deactivate the Phase Lock Loop (PLL) circuit. A special procedure should be followed to use properly (see GS Mode Selector app documentation.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sint {
    #[doc = "0: Turn off the PLL."]
    Off = 0,
    #[doc = "1: Turn on the PLL."]
    On = 1,
}
impl From<Sint> for bool {
    #[inline(always)]
    fn from(variant: Sint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINT` writer - Activate / Deactivate the Phase Lock Loop (PLL) circuit. A special procedure should be followed to use properly (see GS Mode Selector app documentation.)"]
pub type SintW<'a, REG> = crate::BitWriter<'a, REG, Sint>;
impl<'a, REG> SintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turn off the PLL."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sint::Off)
    }
    #[doc = "Turn on the PLL."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sint::On)
    }
}
#[doc = "Field `XPCK` writer - UNKNOWN."]
pub type XpckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK2` writer - UNKNOWN."]
pub type Pck2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPML` writer - Sub-pixel magnification level."]
pub type SpmlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Output colour format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcont {
    #[doc = "0: RGBYc format."]
    Rgbyc = 0,
    #[doc = "1: YCrCb format."]
    Ycrcb = 1,
}
impl From<Gcont> for bool {
    #[inline(always)]
    fn from(variant: Gcont) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCONT` writer - Output colour format."]
pub type GcontW<'a, REG> = crate::BitWriter<'a, REG, Gcont>;
impl<'a, REG> GcontW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGBYc format."]
    #[inline(always)]
    pub fn rgbyc(self) -> &'a mut crate::W<REG> {
        self.variant(Gcont::Rgbyc)
    }
    #[doc = "YCrCb format."]
    #[inline(always)]
    pub fn ycrcb(self) -> &'a mut crate::W<REG> {
        self.variant(Gcont::Ycrcb)
    }
}
#[doc = "Field `PHS` writer - UNKNOWN. Related to HSync output."]
pub type PhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVS` writer - UNKNOWN. Related to VSync output."]
pub type PvsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEHS` writer - UNKNOWN."]
pub type PehsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEVS` writer - UNKNOWN."]
pub type PevsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` writer - UNKNOWN."]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NVCK` writer - UNKNOWN."]
pub type NvckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLCK2` writer - UNKNOWN."]
pub type Slck2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCKSEL` writer - UNKNOWN."]
pub type VckselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Scan mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhp {
    #[doc = "0: Interlace mode."]
    Interlaced = 0,
    #[doc = "1: Progressive mode."]
    Progressive = 1,
}
impl From<Vhp> for bool {
    #[inline(always)]
    fn from(variant: Vhp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VHP` writer - Scan mode."]
pub type VhpW<'a, REG> = crate::BitWriter<'a, REG, Vhp>;
impl<'a, REG> VhpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interlace mode."]
    #[inline(always)]
    pub fn interlaced(self) -> &'a mut crate::W<REG> {
        self.variant(Vhp::Interlaced)
    }
    #[doc = "Progressive mode."]
    #[inline(always)]
    pub fn progressive(self) -> &'a mut crate::W<REG> {
        self.variant(Vhp::Progressive)
    }
}
impl W {
    #[doc = "Bits 0:2 - Phase Lock Loop (PLL) reference divider."]
    #[inline(always)]
    pub fn rc(&mut self) -> RcW<Smode1Spec> {
        RcW::new(self, 0)
    }
    #[doc = "Bits 3:9 - Phase Lock Loop (PLL) loop divider."]
    #[inline(always)]
    pub fn lc(&mut self) -> LcW<Smode1Spec> {
        LcW::new(self, 3)
    }
    #[doc = "Bits 10:11 - Phase Lock Loop (PLL) output divider."]
    #[inline(always)]
    pub fn t128(&mut self) -> T128W<Smode1Spec> {
        T128W::new(self, 10)
    }
    #[doc = "Bit 12 - UNKNOWN. Appears to always be set to 0."]
    #[inline(always)]
    pub fn slck(&mut self) -> SlckW<Smode1Spec> {
        SlckW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Subcarrier frequency (display mode.)"]
    #[inline(always)]
    pub fn cmod(&mut self) -> CmodW<Smode1Spec> {
        CmodW::new(self, 13)
    }
    #[doc = "Bit 15 - UNKNOWN."]
    #[inline(always)]
    pub fn ex(&mut self) -> ExW<Smode1Spec> {
        ExW::new(self, 15)
    }
    #[doc = "Bit 16 - Reset the PCRTC circuit. A special procedure should be followed to use properly (see GS Mode Selector app documentation.)"]
    #[inline(always)]
    pub fn prst(&mut self) -> PrstW<Smode1Spec> {
        PrstW::new(self, 16)
    }
    #[doc = "Bit 17 - Activate / Deactivate the Phase Lock Loop (PLL) circuit. A special procedure should be followed to use properly (see GS Mode Selector app documentation.)"]
    #[inline(always)]
    pub fn sint(&mut self) -> SintW<Smode1Spec> {
        SintW::new(self, 17)
    }
    #[doc = "Bit 18 - UNKNOWN."]
    #[inline(always)]
    pub fn xpck(&mut self) -> XpckW<Smode1Spec> {
        XpckW::new(self, 18)
    }
    #[doc = "Bits 19:20 - UNKNOWN."]
    #[inline(always)]
    pub fn pck2(&mut self) -> Pck2W<Smode1Spec> {
        Pck2W::new(self, 19)
    }
    #[doc = "Bits 21:24 - Sub-pixel magnification level."]
    #[inline(always)]
    pub fn spml(&mut self) -> SpmlW<Smode1Spec> {
        SpmlW::new(self, 21)
    }
    #[doc = "Bit 25 - Output colour format."]
    #[inline(always)]
    pub fn gcont(&mut self) -> GcontW<Smode1Spec> {
        GcontW::new(self, 25)
    }
    #[doc = "Bit 26 - UNKNOWN. Related to HSync output."]
    #[inline(always)]
    pub fn phs(&mut self) -> PhsW<Smode1Spec> {
        PhsW::new(self, 26)
    }
    #[doc = "Bit 27 - UNKNOWN. Related to VSync output."]
    #[inline(always)]
    pub fn pvs(&mut self) -> PvsW<Smode1Spec> {
        PvsW::new(self, 27)
    }
    #[doc = "Bit 28 - UNKNOWN."]
    #[inline(always)]
    pub fn pehs(&mut self) -> PehsW<Smode1Spec> {
        PehsW::new(self, 28)
    }
    #[doc = "Bit 29 - UNKNOWN."]
    #[inline(always)]
    pub fn pevs(&mut self) -> PevsW<Smode1Spec> {
        PevsW::new(self, 29)
    }
    #[doc = "Bits 30:31 - UNKNOWN."]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<Smode1Spec> {
        ClkselW::new(self, 30)
    }
    #[doc = "Bit 32 - UNKNOWN."]
    #[inline(always)]
    pub fn nvck(&mut self) -> NvckW<Smode1Spec> {
        NvckW::new(self, 32)
    }
    #[doc = "Bit 33 - UNKNOWN."]
    #[inline(always)]
    pub fn slck2(&mut self) -> Slck2W<Smode1Spec> {
        Slck2W::new(self, 33)
    }
    #[doc = "Bits 34:35 - UNKNOWN."]
    #[inline(always)]
    pub fn vcksel(&mut self) -> VckselW<Smode1Spec> {
        VckselW::new(self, 34)
    }
    #[doc = "Bit 36 - Scan mode."]
    #[inline(always)]
    pub fn vhp(&mut self) -> VhpW<Smode1Spec> {
        VhpW::new(self, 36)
    }
}
#[doc = "Video signal settings. Sync parameters for the PCRTC.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smode1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smode1Spec;
impl crate::RegisterSpec for Smode1Spec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`smode1::W`](W) writer structure"]
impl crate::Writable for Smode1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMODE1 to value 0"]
impl crate::Resettable for Smode1Spec {}
