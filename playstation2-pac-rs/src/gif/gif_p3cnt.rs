#[doc = "Register `GIF_P3CNT` reader"]
pub type R = crate::R<GifP3cntSpec>;
#[doc = "Field `P3CNT` reader - The LOOPCNT field value of the GIF_CNT register when transfer via PATH3 is interrupted. This is not updated in the event of a transfer restart."]
pub type P3cntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - The LOOPCNT field value of the GIF_CNT register when transfer via PATH3 is interrupted. This is not updated in the event of a transfer restart."]
    #[inline(always)]
    pub fn p3cnt(&self) -> P3cntR {
        P3cntR::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "PATH3 transfer status counter. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_p3cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifP3cntSpec;
impl crate::RegisterSpec for GifP3cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_p3cnt::R`](R) reader structure"]
impl crate::Readable for GifP3cntSpec {}
#[doc = "`reset()` method sets GIF_P3CNT to value 0"]
impl crate::Resettable for GifP3cntSpec {}
