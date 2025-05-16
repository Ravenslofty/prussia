#[doc = "Register `VIF1_MARK` reader"]
pub type R = crate::R<Vif1MarkSpec>;
#[doc = "Register `VIF1_MARK` writer"]
pub type W = crate::W<Vif1MarkSpec>;
#[doc = "Field `MARK` reader - Mark value most recently set"]
pub type MarkR = crate::FieldReader<u16>;
#[doc = "Field `MARK` writer - Mark value most recently set"]
pub type MarkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Mark value most recently set"]
    #[inline(always)]
    pub fn mark(&self) -> MarkR {
        MarkR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mark value most recently set"]
    #[inline(always)]
    pub fn mark(&mut self) -> MarkW<Vif1MarkSpec> {
        MarkW::new(self, 0)
    }
}
#[doc = "Mark value\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_mark::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_mark::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1MarkSpec;
impl crate::RegisterSpec for Vif1MarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_mark::R`](R) reader structure"]
impl crate::Readable for Vif1MarkSpec {}
#[doc = "`write(|w| ..)` method takes [`vif1_mark::W`](W) writer structure"]
impl crate::Writable for Vif1MarkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIF1_MARK to value 0"]
impl crate::Resettable for Vif1MarkSpec {}
