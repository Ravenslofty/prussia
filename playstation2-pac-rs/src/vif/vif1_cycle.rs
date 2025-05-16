#[doc = "Register `VIF1_CYCLE` reader"]
pub type R = crate::R<Vif1CycleSpec>;
#[doc = "Field `CL` reader - Cycle length, cycle/block size"]
pub type ClR = crate::FieldReader;
#[doc = "Field `WL` reader - Write cycle length, Block size/cycle"]
pub type WlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Cycle length, cycle/block size"]
    #[inline(always)]
    pub fn cl(&self) -> ClR {
        ClR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Write cycle length, Block size/cycle"]
    #[inline(always)]
    pub fn wl(&self) -> WlR {
        WlR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Data write cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_cycle::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1CycleSpec;
impl crate::RegisterSpec for Vif1CycleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_cycle::R`](R) reader structure"]
impl crate::Readable for Vif1CycleSpec {}
#[doc = "`reset()` method sets VIF1_CYCLE to value 0"]
impl crate::Resettable for Vif1CycleSpec {}
