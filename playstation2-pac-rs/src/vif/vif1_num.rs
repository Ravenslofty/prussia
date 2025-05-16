#[doc = "Register `VIF1_NUM` reader"]
pub type R = crate::R<Vif1NumSpec>;
#[doc = "Field `NUM` reader - Amount of untransferred data"]
pub type NumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Amount of untransferred data"]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Amount of non-transferred data\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_num::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1NumSpec;
impl crate::RegisterSpec for Vif1NumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_num::R`](R) reader structure"]
impl crate::Readable for Vif1NumSpec {}
#[doc = "`reset()` method sets VIF1_NUM to value 0"]
impl crate::Resettable for Vif1NumSpec {}
