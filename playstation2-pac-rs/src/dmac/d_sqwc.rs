#[doc = "Register `D_SQWC` reader"]
pub type R = crate::R<DSqwcSpec>;
#[doc = "Register `D_SQWC` writer"]
pub type W = crate::W<DSqwcSpec>;
#[doc = "Field `SQWC` reader - Skip quadword counter. Size of the part not transferred (qword)"]
pub type SqwcR = crate::FieldReader;
#[doc = "Field `SQWC` writer - Skip quadword counter. Size of the part not transferred (qword)"]
pub type SqwcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TQWC` reader - Transfer quadword counter. Size of the part transferred (qword)"]
pub type TqwcR = crate::FieldReader;
#[doc = "Field `TQWC` writer - Transfer quadword counter. Size of the part transferred (qword)"]
pub type TqwcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Skip quadword counter. Size of the part not transferred (qword)"]
    #[inline(always)]
    pub fn sqwc(&self) -> SqwcR {
        SqwcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transfer quadword counter. Size of the part transferred (qword)"]
    #[inline(always)]
    pub fn tqwc(&self) -> TqwcR {
        TqwcR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Skip quadword counter. Size of the part not transferred (qword)"]
    #[inline(always)]
    pub fn sqwc(&mut self) -> SqwcW<DSqwcSpec> {
        SqwcW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Transfer quadword counter. Size of the part transferred (qword)"]
    #[inline(always)]
    pub fn tqwc(&mut self) -> TqwcW<DSqwcSpec> {
        TqwcW::new(self, 16)
    }
}
#[doc = "DMAC skip quad word\n\nYou can [`read`](crate::Reg::read) this register and get [`d_sqwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_sqwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSqwcSpec;
impl crate::RegisterSpec for DSqwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_sqwc::R`](R) reader structure"]
impl crate::Readable for DSqwcSpec {}
#[doc = "`write(|w| ..)` method takes [`d_sqwc::W`](W) writer structure"]
impl crate::Writable for DSqwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_SQWC to value 0"]
impl crate::Resettable for DSqwcSpec {}
