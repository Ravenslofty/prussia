#[doc = "Register `T0_HOLD` reader"]
pub type R = crate::R<T0HoldSpec>;
#[doc = "Register `T0_HOLD` writer"]
pub type W = crate::W<T0HoldSpec>;
#[doc = "Field `HOLD` reader - Hold Value. The value of Tn_COUNT is copied when an SBUS interrupt occurs."]
pub type HoldR = crate::FieldReader<u16>;
#[doc = "Field `HOLD` writer - Hold Value. The value of Tn_COUNT is copied when an SBUS interrupt occurs."]
pub type HoldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Hold Value. The value of Tn_COUNT is copied when an SBUS interrupt occurs."]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Hold Value. The value of Tn_COUNT is copied when an SBUS interrupt occurs."]
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<T0HoldSpec> {
        HoldW::new(self, 0)
    }
}
#[doc = "Hold register\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0HoldSpec;
impl crate::RegisterSpec for T0HoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_hold::R`](R) reader structure"]
impl crate::Readable for T0HoldSpec {}
#[doc = "`write(|w| ..)` method takes [`t0_hold::W`](W) writer structure"]
impl crate::Writable for T0HoldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0_HOLD to value 0"]
impl crate::Resettable for T0HoldSpec {}
