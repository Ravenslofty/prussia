#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `SIGMSK` reader - Signal event interrupt mask."]
pub type SigmskR = crate::BitReader;
#[doc = "Field `FINISHMSK` reader - Finish event interrupt mask."]
pub type FinishmskR = crate::BitReader;
#[doc = "Field `HSMSK` reader - HSync interrupt mask."]
pub type HsmskR = crate::BitReader;
#[doc = "Field `VSMSK` reader - VSync interrupt mask."]
pub type VsmskR = crate::BitReader;
#[doc = "Field `EDWMSK` reader - Rectangle write termination interrupt mask."]
pub type EdwmskR = crate::BitReader;
#[doc = "Field `ONES` reader - All set to one. Behaviour is undefined according to the GS users manual but advises to set them regardless, so likely related to a hardware bug."]
pub type OnesR = crate::FieldReader;
impl R {
    #[doc = "Bit 8 - Signal event interrupt mask."]
    #[inline(always)]
    pub fn sigmsk(&self) -> SigmskR {
        SigmskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Finish event interrupt mask."]
    #[inline(always)]
    pub fn finishmsk(&self) -> FinishmskR {
        FinishmskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSync interrupt mask."]
    #[inline(always)]
    pub fn hsmsk(&self) -> HsmskR {
        HsmskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VSync interrupt mask."]
    #[inline(always)]
    pub fn vsmsk(&self) -> VsmskR {
        VsmskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rectangle write termination interrupt mask."]
    #[inline(always)]
    pub fn edwmsk(&self) -> EdwmskR {
        EdwmskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - All set to one. Behaviour is undefined according to the GS users manual but advises to set them regardless, so likely related to a hardware bug."]
    #[inline(always)]
    pub fn ones(&self) -> OnesR {
        OnesR::new(((self.bits >> 13) & 3) as u8)
    }
}
#[doc = "Interrupt mask control.\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
