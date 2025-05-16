#[doc = "Register `D_RBOR` reader"]
pub type R = crate::R<DRborSpec>;
#[doc = "Register `D_RBOR` writer"]
pub type W = crate::W<DRborSpec>;
#[doc = "Field `ADDR` reader - Ring buffer offset address (qword alignment)"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Ring buffer offset address (qword alignment)"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 4:30 - Ring buffer offset address (qword alignment)"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 4) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:30 - Ring buffer offset address (qword alignment)"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<DRborSpec> {
        AddrW::new(self, 4)
    }
}
#[doc = "DMAC ring buffer offset\n\nYou can [`read`](crate::Reg::read) this register and get [`d_rbor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_rbor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRborSpec;
impl crate::RegisterSpec for DRborSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_rbor::R`](R) reader structure"]
impl crate::Readable for DRborSpec {}
#[doc = "`write(|w| ..)` method takes [`d_rbor::W`](W) writer structure"]
impl crate::Writable for DRborSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_RBOR to value 0"]
impl crate::Resettable for DRborSpec {}
