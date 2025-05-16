#[doc = "Register `VIF1_OFST` reader"]
pub type R = crate::R<Vif1OfstSpec>;
#[doc = "Field `OFFSET` reader - Data buffer offset"]
pub type OffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data buffer offset"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Offset of double buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_ofst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1OfstSpec;
impl crate::RegisterSpec for Vif1OfstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_ofst::R`](R) reader structure"]
impl crate::Readable for Vif1OfstSpec {}
#[doc = "`reset()` method sets VIF1_OFST to value 0"]
impl crate::Resettable for Vif1OfstSpec {}
