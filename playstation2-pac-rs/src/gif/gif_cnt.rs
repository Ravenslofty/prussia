#[doc = "Register `GIF_CNT` reader"]
pub type R = crate::R<GifCntSpec>;
#[doc = "Field `LOOPCNT` reader - Value of current loop counter (backward counter from GIF_TAG0 NLOOP field). Decremented when starting processing of the GIF_TAG1 NREGSth register descriptor."]
pub type LoopcntR = crate::FieldReader<u16>;
#[doc = "Field `REGCNT` reader - Register descriptor number (0-15) currently in process (Becomes indeterminate in IMAGE mode). * 1 - Lowest significant register descriptor of REGS field. * 2 - 2nd lowest significant register descriptor of REGS field. .. * 15 - 15th lowest significant register descriptor of REGS field. * 0 - Most significant register descriptor of REGS field."]
pub type RegcntR = crate::FieldReader;
#[doc = "Field `VUADDR` reader - Address of VU memory under transfer."]
pub type VuaddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Value of current loop counter (backward counter from GIF_TAG0 NLOOP field). Decremented when starting processing of the GIF_TAG1 NREGSth register descriptor."]
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:19 - Register descriptor number (0-15) currently in process (Becomes indeterminate in IMAGE mode). * 1 - Lowest significant register descriptor of REGS field. * 2 - 2nd lowest significant register descriptor of REGS field. .. * 15 - 15th lowest significant register descriptor of REGS field. * 0 - Most significant register descriptor of REGS field."]
    #[inline(always)]
    pub fn regcnt(&self) -> RegcntR {
        RegcntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:29 - Address of VU memory under transfer."]
    #[inline(always)]
    pub fn vuaddr(&self) -> VuaddrR {
        VuaddrR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[doc = "Transfer status counter. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifCntSpec;
impl crate::RegisterSpec for GifCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_cnt::R`](R) reader structure"]
impl crate::Readable for GifCntSpec {}
#[doc = "`reset()` method sets GIF_CNT to value 0"]
impl crate::Resettable for GifCntSpec {}
