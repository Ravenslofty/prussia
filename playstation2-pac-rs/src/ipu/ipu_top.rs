#[doc = "Register `IPU_TOP` reader"]
pub type R = crate::R<IpuTopSpec>;
#[doc = "Field `BSTOP` reader - Top 32 bits of the bit stream data."]
pub type BstopR = crate::FieldReader<u32>;
#[doc = "Command busy status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: BSTOP field is enabled (ready to read.)"]
    Enable = 0,
    #[doc = "1: BSTOP field is disabled (decoding still in execution.)"]
    Disable = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Command busy status."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Enable,
            true => Busy::Disable,
        }
    }
    #[doc = "BSTOP field is enabled (ready to read.)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Busy::Enable
    }
    #[doc = "BSTOP field is disabled (decoding still in execution.)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Busy::Disable
    }
}
impl R {
    #[doc = "Bits 0:31 - Top 32 bits of the bit stream data."]
    #[inline(always)]
    pub fn bstop(&self) -> BstopR {
        BstopR::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bit 63 - Command busy status."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 63) & 1) != 0)
    }
}
#[doc = "Reads the first 32 bits of the bit stream after execution of a BDEC / IDEC / VDEC / FDEC command completes.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_top::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpuTopSpec;
impl crate::RegisterSpec for IpuTopSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ipu_top::R`](R) reader structure"]
impl crate::Readable for IpuTopSpec {}
#[doc = "`reset()` method sets IPU_TOP to value 0"]
impl crate::Resettable for IpuTopSpec {}
