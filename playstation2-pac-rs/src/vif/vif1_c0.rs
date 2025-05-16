#[doc = "Register `VIF1_C0` reader"]
pub type R = crate::R<Vif1C0Spec>;
#[doc = "Register `VIF1_C0` writer"]
pub type W = crate::W<Vif1C0Spec>;
#[doc = "Field `C` reader - Column data for filling when decompressing"]
pub type CR = crate::FieldReader<u32>;
#[doc = "Field `C` writer - Column data for filling when decompressing"]
pub type CW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Column data for filling when decompressing"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Column data for filling when decompressing"]
    #[inline(always)]
    pub fn c(&mut self) -> CW<Vif1C0Spec> {
        CW::new(self, 0)
    }
}
#[doc = "Filling data C0 (Col register)\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1C0Spec;
impl crate::RegisterSpec for Vif1C0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_c0::R`](R) reader structure"]
impl crate::Readable for Vif1C0Spec {}
#[doc = "`write(|w| ..)` method takes [`vif1_c0::W`](W) writer structure"]
impl crate::Writable for Vif1C0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIF1_C0 to value 0"]
impl crate::Resettable for Vif1C0Spec {}
