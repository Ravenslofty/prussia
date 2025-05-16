#[doc = "Register `GIF_P3TAG` reader"]
pub type R = crate::R<GifP3tagSpec>;
#[doc = "Field `LOOPCNT` reader - The NLOOP field value of the GIFtag read via PATH3 at the end."]
pub type LoopcntR = crate::FieldReader<u16>;
#[doc = "Field `EOP` reader - The EOP flag value of the GIFtag read via PATH 3 at the end."]
pub type EopR = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - The NLOOP field value of the GIFtag read via PATH3 at the end."]
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - The EOP flag value of the GIFtag read via PATH 3 at the end."]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "PATH3 tag value. Only accessible when GIF_CTRL PSE is set to 1 and when PATH3 is transferring data in IMAGE mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_p3tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifP3tagSpec;
impl crate::RegisterSpec for GifP3tagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_p3tag::R`](R) reader structure"]
impl crate::Readable for GifP3tagSpec {}
#[doc = "`reset()` method sets GIF_P3TAG to value 0"]
impl crate::Resettable for GifP3tagSpec {}
