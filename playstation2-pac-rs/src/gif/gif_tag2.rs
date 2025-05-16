#[doc = "Register `GIF_TAG2` reader"]
pub type R = crate::R<GifTag2Spec>;
#[doc = "Field `REGS` reader - Values from bit 95 to bit 64 among most recently read GIFtag values (Lower part of REGS field.)"]
pub type RegsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Values from bit 95 to bit 64 among most recently read GIFtag values (Lower part of REGS field.)"]
    #[inline(always)]
    pub fn regs(&self) -> RegsR {
        RegsR::new(self.bits)
    }
}
#[doc = "GIFtag (bits 95-64) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifTag2Spec;
impl crate::RegisterSpec for GifTag2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_tag2::R`](R) reader structure"]
impl crate::Readable for GifTag2Spec {}
#[doc = "`reset()` method sets GIF_TAG2 to value 0"]
impl crate::Resettable for GifTag2Spec {}
