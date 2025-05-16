#[doc = "Register `GIF_TAG3` reader"]
pub type R = crate::R<GifTag3Spec>;
#[doc = "Field `REGS` reader - Values from bit 127 to bit 96 among most recently read GIFtag values (Lower part of REGS field.)"]
pub type RegsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Values from bit 127 to bit 96 among most recently read GIFtag values (Lower part of REGS field.)"]
    #[inline(always)]
    pub fn regs(&self) -> RegsR {
        RegsR::new(self.bits)
    }
}
#[doc = "GIFtag (bits 127-96) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifTag3Spec;
impl crate::RegisterSpec for GifTag3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_tag3::R`](R) reader structure"]
impl crate::Readable for GifTag3Spec {}
#[doc = "`reset()` method sets GIF_TAG3 to value 0"]
impl crate::Resettable for GifTag3Spec {}
