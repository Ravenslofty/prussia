#[doc = "Register `SIGLBLID` reader"]
pub type R = crate::R<SiglblidSpec>;
#[doc = "Field `SIGID` reader - Id value set by SIGNAL register."]
pub type SigidR = crate::FieldReader<u32>;
#[doc = "Field `LBLID` reader - Id value set by LABEL register."]
pub type LblidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Id value set by SIGNAL register."]
    #[inline(always)]
    pub fn sigid(&self) -> SigidR {
        SigidR::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bits 32:63 - Id value set by LABEL register."]
    #[inline(always)]
    pub fn lblid(&self) -> LblidR {
        LblidR::new(((self.bits >> 32) & 0xffff_ffff) as u32)
    }
}
#[doc = "Signal Id value.\n\nYou can [`read`](crate::Reg::read) this register and get [`siglblid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SiglblidSpec;
impl crate::RegisterSpec for SiglblidSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`siglblid::R`](R) reader structure"]
impl crate::Readable for SiglblidSpec {}
#[doc = "`reset()` method sets SIGLBLID to value 0"]
impl crate::Resettable for SiglblidSpec {}
