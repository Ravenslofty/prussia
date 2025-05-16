#[doc = "Register `GIF_TAG0` reader"]
pub type R = crate::R<GifTag0Spec>;
#[doc = "Field `NLOOP` reader - Most recently read NLOOP field of GIFtag."]
pub type NloopR = crate::FieldReader<u16>;
#[doc = "Field `EOP` reader - Most recently read EOP flag of GIFtag."]
pub type EopR = crate::BitReader;
#[doc = "Field `TAG` reader - (Undefined) Values from bit 31 to bit 16 among the most recently read GIFtag values."]
pub type TagR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Most recently read NLOOP field of GIFtag."]
    #[inline(always)]
    pub fn nloop(&self) -> NloopR {
        NloopR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Most recently read EOP flag of GIFtag."]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - (Undefined) Values from bit 31 to bit 16 among the most recently read GIFtag values."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "GIFtag (bits 31-16) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifTag0Spec;
impl crate::RegisterSpec for GifTag0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_tag0::R`](R) reader structure"]
impl crate::Readable for GifTag0Spec {}
#[doc = "`reset()` method sets GIF_TAG0 to value 0"]
impl crate::Resettable for GifTag0Spec {}
