#[doc = "Register `VIF1_R0` reader"]
pub type R = crate::R<Vif1R0Spec>;
#[doc = "Field `R` reader - Row data for filling when decompressing"]
pub type RR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Row data for filling when decompressing"]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new(self.bits)
    }
}
#[doc = "Filling data R0 (Row register)\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1R0Spec;
impl crate::RegisterSpec for Vif1R0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_r0::R`](R) reader structure"]
impl crate::Readable for Vif1R0Spec {}
#[doc = "`reset()` method sets VIF1_R0 to value 0"]
impl crate::Resettable for Vif1R0Spec {}
