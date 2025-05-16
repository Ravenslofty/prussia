#[doc = "Register `VIF1_CODE` reader"]
pub type R = crate::R<Vif1CodeSpec>;
#[doc = "Field `IMMEDIATE` reader - VIFcode IMMEDIATE processed most recently"]
pub type ImmediateR = crate::FieldReader<u16>;
#[doc = "Field `NUM` reader - VIFcode NUM processed most recently"]
pub type NumR = crate::FieldReader;
#[doc = "Field `CMD` reader - VIFcode CMD processed most recently"]
pub type CmdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - VIFcode IMMEDIATE processed most recently"]
    #[inline(always)]
    pub fn immediate(&self) -> ImmediateR {
        ImmediateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - VIFcode NUM processed most recently"]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - VIFcode CMD processed most recently"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Last processed VIFcode\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_code::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1CodeSpec;
impl crate::RegisterSpec for Vif1CodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_code::R`](R) reader structure"]
impl crate::Readable for Vif1CodeSpec {}
#[doc = "`reset()` method sets VIF1_CODE to value 0"]
impl crate::Resettable for Vif1CodeSpec {}
