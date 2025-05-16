#[doc = "Register `VIF1_FBRST` writer"]
pub type W = crate::W<Vif1FbrstSpec>;
#[doc = "Field `RST` writer - Reset. Resets the whole VIF (including VIF FIFO) when 1 is written."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBK` writer - ForceBreak. Causes a ForceBreak to the VIF when 1 is written. (Stall occurrence)"]
pub type FbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP` writer - Stop. Stops after end of VIDcode in process when 1 is written. (Stall occurrence)"]
pub type StpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STC` writer - Stall Cancel. Cancels the VIF stall and clears the VSS, VFS, VIS, INT, ER0 and ER1 of the VIF_STAT register to 0 when 1 is written"]
pub type StcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reset. Resets the whole VIF (including VIF FIFO) when 1 is written."]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<Vif1FbrstSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bit 1 - ForceBreak. Causes a ForceBreak to the VIF when 1 is written. (Stall occurrence)"]
    #[inline(always)]
    pub fn fbk(&mut self) -> FbkW<Vif1FbrstSpec> {
        FbkW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop. Stops after end of VIDcode in process when 1 is written. (Stall occurrence)"]
    #[inline(always)]
    pub fn stp(&mut self) -> StpW<Vif1FbrstSpec> {
        StpW::new(self, 2)
    }
    #[doc = "Bit 3 - Stall Cancel. Cancels the VIF stall and clears the VSS, VFS, VIS, INT, ER0 and ER1 of the VIF_STAT register to 0 when 1 is written"]
    #[inline(always)]
    pub fn stc(&mut self) -> StcW<Vif1FbrstSpec> {
        StcW::new(self, 3)
    }
}
#[doc = "Operation control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_fbrst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1FbrstSpec;
impl crate::RegisterSpec for Vif1FbrstSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`vif1_fbrst::W`](W) writer structure"]
impl crate::Writable for Vif1FbrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIF1_FBRST to value 0"]
impl crate::Resettable for Vif1FbrstSpec {}
