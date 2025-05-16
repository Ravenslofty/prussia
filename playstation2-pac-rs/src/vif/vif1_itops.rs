#[doc = "Register `VIF1_ITOPS` reader"]
pub type R = crate::R<Vif1ItopsSpec>;
#[doc = "Field `ITOPS` reader - ITOPS value"]
pub type ItopsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - ITOPS value"]
    #[inline(always)]
    pub fn itops(&self) -> ItopsR {
        ItopsR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Next ITOP value\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_itops::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1ItopsSpec;
impl crate::RegisterSpec for Vif1ItopsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_itops::R`](R) reader structure"]
impl crate::Readable for Vif1ItopsSpec {}
#[doc = "`reset()` method sets VIF1_ITOPS to value 0"]
impl crate::Resettable for Vif1ItopsSpec {}
