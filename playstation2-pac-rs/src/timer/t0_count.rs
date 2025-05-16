#[doc = "Register `T0_COUNT` reader"]
pub type R = crate::R<T0CountSpec>;
#[doc = "Register `T0_COUNT` writer"]
pub type W = crate::W<T0CountSpec>;
#[doc = "Field `COUNT` reader - Counter Value. The counter value is incremented according to the conditions of the clock and the gate signal specified in the Tn_MODE."]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Counter Value. The counter value is incremented according to the conditions of the clock and the gate signal specified in the Tn_MODE."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Value. The counter value is incremented according to the conditions of the clock and the gate signal specified in the Tn_MODE."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Value. The counter value is incremented according to the conditions of the clock and the gate signal specified in the Tn_MODE."]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<T0CountSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0CountSpec;
impl crate::RegisterSpec for T0CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_count::R`](R) reader structure"]
impl crate::Readable for T0CountSpec {}
#[doc = "`write(|w| ..)` method takes [`t0_count::W`](W) writer structure"]
impl crate::Writable for T0CountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0_COUNT to value 0"]
impl crate::Resettable for T0CountSpec {}
