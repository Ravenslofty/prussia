#[doc = "Register `VIF1_MASK` reader"]
pub type R = crate::R<Vif1MaskSpec>;
#[doc = "Field `M0` reader - Decompressed data"]
pub type M0R = crate::FieldReader;
#[doc = "Field `M1` reader - Row register value corresponding to the field"]
pub type M1R = crate::FieldReader;
#[doc = "Field `M2` reader - Col register value corresponding to the write cycle"]
pub type M2R = crate::FieldReader;
#[doc = "Field `M3` reader - Write protect (Mask)"]
pub type M3R = crate::FieldReader;
#[doc = "Field `M4` reader - Decompressed data"]
pub type M4R = crate::FieldReader;
#[doc = "Field `M5` reader - Row register value corresponding to the field"]
pub type M5R = crate::FieldReader;
#[doc = "Field `M6` reader - Col register value corresponding to the write cycle"]
pub type M6R = crate::FieldReader;
#[doc = "Field `M7` reader - Write protect (Mask)"]
pub type M7R = crate::FieldReader;
#[doc = "Field `M8` reader - Decompressed data"]
pub type M8R = crate::FieldReader;
#[doc = "Field `M9` reader - Row register value corresponding to the field"]
pub type M9R = crate::FieldReader;
#[doc = "Field `M10` reader - Col register value corresponding to the write cycle"]
pub type M10R = crate::FieldReader;
#[doc = "Field `M11` reader - Write protect (Mask)"]
pub type M11R = crate::FieldReader;
#[doc = "Field `M12` reader - Decompressed data"]
pub type M12R = crate::FieldReader;
#[doc = "Field `M13` reader - Row register value corresponding to the field"]
pub type M13R = crate::FieldReader;
#[doc = "Field `M14` reader - Col register value corresponding to the write cycle"]
pub type M14R = crate::FieldReader;
#[doc = "Field `M15` reader - Write protect (Mask)"]
pub type M15R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Decompressed data"]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Row register value corresponding to the field"]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Col register value corresponding to the write cycle"]
    #[inline(always)]
    pub fn m2(&self) -> M2R {
        M2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Write protect (Mask)"]
    #[inline(always)]
    pub fn m3(&self) -> M3R {
        M3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Decompressed data"]
    #[inline(always)]
    pub fn m4(&self) -> M4R {
        M4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Row register value corresponding to the field"]
    #[inline(always)]
    pub fn m5(&self) -> M5R {
        M5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Col register value corresponding to the write cycle"]
    #[inline(always)]
    pub fn m6(&self) -> M6R {
        M6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Write protect (Mask)"]
    #[inline(always)]
    pub fn m7(&self) -> M7R {
        M7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Decompressed data"]
    #[inline(always)]
    pub fn m8(&self) -> M8R {
        M8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Row register value corresponding to the field"]
    #[inline(always)]
    pub fn m9(&self) -> M9R {
        M9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Col register value corresponding to the write cycle"]
    #[inline(always)]
    pub fn m10(&self) -> M10R {
        M10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Write protect (Mask)"]
    #[inline(always)]
    pub fn m11(&self) -> M11R {
        M11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Decompressed data"]
    #[inline(always)]
    pub fn m12(&self) -> M12R {
        M12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Row register value corresponding to the field"]
    #[inline(always)]
    pub fn m13(&self) -> M13R {
        M13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Col register value corresponding to the write cycle"]
    #[inline(always)]
    pub fn m14(&self) -> M14R {
        M14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Write protect (Mask)"]
    #[inline(always)]
    pub fn m15(&self) -> M15R {
        M15R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Write mask pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1MaskSpec;
impl crate::RegisterSpec for Vif1MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_mask::R`](R) reader structure"]
impl crate::Readable for Vif1MaskSpec {}
#[doc = "`reset()` method sets VIF1_MASK to value 0"]
impl crate::Resettable for Vif1MaskSpec {}
