#[doc = "Register `VIF1_ITOP` reader"]
pub type R = crate::R<Vif1ItopSpec>;
#[doc = "Field `ITOP` reader - ITOP value"]
pub type ItopR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - ITOP value"]
    #[inline(always)]
    pub fn itop(&self) -> ItopR {
        ItopR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "ITOP value\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_itop::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1ItopSpec;
impl crate::RegisterSpec for Vif1ItopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_itop::R`](R) reader structure"]
impl crate::Readable for Vif1ItopSpec {}
#[doc = "`reset()` method sets VIF1_ITOP to value 0"]
impl crate::Resettable for Vif1ItopSpec {}
