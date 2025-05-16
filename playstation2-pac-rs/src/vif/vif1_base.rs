#[doc = "Register `VIF1_BASE` reader"]
pub type R = crate::R<Vif1BaseSpec>;
#[doc = "Field `BASE` reader - Data buffer base address"]
pub type BaseR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data buffer base address"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Base address of double buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_base::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1BaseSpec;
impl crate::RegisterSpec for Vif1BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_base::R`](R) reader structure"]
impl crate::Readable for Vif1BaseSpec {}
#[doc = "`reset()` method sets VIF1_BASE to value 0"]
impl crate::Resettable for Vif1BaseSpec {}
