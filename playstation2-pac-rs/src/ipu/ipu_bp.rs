#[doc = "Register `IPU_BP` reader"]
pub type R = crate::R<IpuBpSpec>;
#[doc = "Field `BP` reader - Bit stream pointer to bit position in first 128-bit data to start decoding."]
pub type BpR = crate::FieldReader;
#[doc = "Field `IFC` reader - Input FIFO counter. Equal to number of qwords in IPU_in_FIFO."]
pub type IfcR = crate::FieldReader;
#[doc = "Field `FP` reader - FIFO pointer. Equal to number of qwords remaining in IPU except IPU_in_FIFO."]
pub type FpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Bit stream pointer to bit position in first 128-bit data to start decoding."]
    #[inline(always)]
    pub fn bp(&self) -> BpR {
        BpR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Input FIFO counter. Equal to number of qwords in IPU_in_FIFO."]
    #[inline(always)]
    pub fn ifc(&self) -> IfcR {
        IfcR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - FIFO pointer. Equal to number of qwords remaining in IPU except IPU_in_FIFO."]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "IPU input FIFO control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_bp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpuBpSpec;
impl crate::RegisterSpec for IpuBpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipu_bp::R`](R) reader structure"]
impl crate::Readable for IpuBpSpec {}
#[doc = "`reset()` method sets IPU_BP to value 0"]
impl crate::Resettable for IpuBpSpec {}
