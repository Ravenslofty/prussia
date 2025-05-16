#[doc = "Register `SYNCV` writer"]
pub type W = crate::W<SyncvSpec>;
#[doc = "Field `VFP` writer - Vertical front porch interval. Halflines with colour burst after video data."]
pub type VfpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VFPE` writer - Vertical front porch interval end. Halflines without colour burst after VFP."]
pub type VfpeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VBP` writer - Vertical back porch interval. Halflines with colour burst after VBPE."]
pub type VbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VBPE` writer - Vertical back porch interval end. Halflines without colour burst after VS."]
pub type VbpeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VDP` writer - Vertical differential phase. Halflines with video data."]
pub type VdpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VS` writer - Vertical Synchronisation timing. Halflines with VSYNC."]
pub type VsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl W {
    #[doc = "Bits 0:9 - Vertical front porch interval. Halflines with colour burst after video data."]
    #[inline(always)]
    pub fn vfp(&mut self) -> VfpW<SyncvSpec> {
        VfpW::new(self, 0)
    }
    #[doc = "Bits 10:19 - Vertical front porch interval end. Halflines without colour burst after VFP."]
    #[inline(always)]
    pub fn vfpe(&mut self) -> VfpeW<SyncvSpec> {
        VfpeW::new(self, 10)
    }
    #[doc = "Bits 20:31 - Vertical back porch interval. Halflines with colour burst after VBPE."]
    #[inline(always)]
    pub fn vbp(&mut self) -> VbpW<SyncvSpec> {
        VbpW::new(self, 20)
    }
    #[doc = "Bits 32:41 - Vertical back porch interval end. Halflines without colour burst after VS."]
    #[inline(always)]
    pub fn vbpe(&mut self) -> VbpeW<SyncvSpec> {
        VbpeW::new(self, 32)
    }
    #[doc = "Bit 42 - Vertical differential phase. Halflines with video data."]
    #[inline(always)]
    pub fn vdp(&mut self) -> VdpW<SyncvSpec> {
        VdpW::new(self, 42)
    }
    #[doc = "Bits 53:63 - Vertical Synchronisation timing. Halflines with VSYNC."]
    #[inline(always)]
    pub fn vs(&mut self) -> VsW<SyncvSpec> {
        VsW::new(self, 53)
    }
}
#[doc = "VSync-related interval timins (?)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncvSpec;
impl crate::RegisterSpec for SyncvSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`syncv::W`](W) writer structure"]
impl crate::Writable for SyncvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCV to value 0"]
impl crate::Resettable for SyncvSpec {}
