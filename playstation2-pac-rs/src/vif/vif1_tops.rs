#[doc = "Register `VIF1_TOPS` reader"]
pub type R = crate::R<Vif1TopsSpec>;
#[doc = "Field `TOPS` reader - Data decompression buffer address"]
pub type TopsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data decompression buffer address"]
    #[inline(always)]
    pub fn tops(&self) -> TopsR {
        TopsR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Next TOP value/data write address\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_tops::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1TopsSpec;
impl crate::RegisterSpec for Vif1TopsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_tops::R`](R) reader structure"]
impl crate::Readable for Vif1TopsSpec {}
#[doc = "`reset()` method sets VIF1_TOPS to value 0"]
impl crate::Resettable for Vif1TopsSpec {}
