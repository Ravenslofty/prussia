#[doc = "Register `T0_COMP` reader"]
pub type R = crate::R<T0CompSpec>;
#[doc = "Register `T0_COMP` writer"]
pub type W = crate::W<T0CompSpec>;
#[doc = "Field `COMP` reader - Compare Value. Reference value to be compared with Tn_COUNT."]
pub type CompR = crate::FieldReader<u16>;
#[doc = "Field `COMP` writer - Compare Value. Reference value to be compared with Tn_COUNT."]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Value. Reference value to be compared with Tn_COUNT."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Value. Reference value to be compared with Tn_COUNT."]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<T0CompSpec> {
        CompW::new(self, 0)
    }
}
#[doc = "Comparison register\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_comp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_comp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0CompSpec;
impl crate::RegisterSpec for T0CompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_comp::R`](R) reader structure"]
impl crate::Readable for T0CompSpec {}
#[doc = "`write(|w| ..)` method takes [`t0_comp::W`](W) writer structure"]
impl crate::Writable for T0CompSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0_COMP to value 0"]
impl crate::Resettable for T0CompSpec {}
