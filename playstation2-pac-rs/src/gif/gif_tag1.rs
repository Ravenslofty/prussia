#[doc = "Register `GIF_TAG1` reader"]
pub type R = crate::R<GifTag1Spec>;
#[doc = "Field `TAG` reader - (Undefined) Values from bit 45 to bit 32 among the most recently read GIFtag values."]
pub type TagR = crate::FieldReader<u16>;
#[doc = "Field `PRE` reader - Most recently read PRE flag of GIFtag."]
pub type PreR = crate::BitReader;
#[doc = "Field `PRIM` reader - Most recently read PRIM field of GIFtag."]
pub type PrimR = crate::FieldReader<u16>;
#[doc = "Field `FLG` reader - Most recently read FLG field of GIFtag."]
pub type FlgR = crate::FieldReader;
#[doc = "Field `NREG` reader - Most recently read NREG field of GIFtag."]
pub type NregR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:13 - (Undefined) Values from bit 45 to bit 32 among the most recently read GIFtag values."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - Most recently read PRE flag of GIFtag."]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:25 - Most recently read PRIM field of GIFtag."]
    #[inline(always)]
    pub fn prim(&self) -> PrimR {
        PrimR::new(((self.bits >> 15) & 0x07ff) as u16)
    }
    #[doc = "Bits 26:27 - Most recently read FLG field of GIFtag."]
    #[inline(always)]
    pub fn flg(&self) -> FlgR {
        FlgR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Most recently read NREG field of GIFtag."]
    #[inline(always)]
    pub fn nreg(&self) -> NregR {
        NregR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "GIFtag (bits 45-32) immediately before. Only accessible when GIF_CTRL PSE is set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_tag1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifTag1Spec;
impl crate::RegisterSpec for GifTag1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_tag1::R`](R) reader structure"]
impl crate::Readable for GifTag1Spec {}
#[doc = "`reset()` method sets GIF_TAG1 to value 0"]
impl crate::Resettable for GifTag1Spec {}
