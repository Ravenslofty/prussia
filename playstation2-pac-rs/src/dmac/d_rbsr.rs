#[doc = "Register `D_RBSR` reader"]
pub type R = crate::R<DRbsrSpec>;
#[doc = "Register `D_RBSR` writer"]
pub type W = crate::W<DRbsrSpec>;
#[doc = "Field `RMSK` reader - Ring buffer size mask"]
pub type RmskR = crate::FieldReader<u32>;
#[doc = "Field `RMSK` writer - Ring buffer size mask"]
pub type RmskW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 4:30 - Ring buffer size mask"]
    #[inline(always)]
    pub fn rmsk(&self) -> RmskR {
        RmskR::new((self.bits >> 4) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:30 - Ring buffer size mask"]
    #[inline(always)]
    pub fn rmsk(&mut self) -> RmskW<DRbsrSpec> {
        RmskW::new(self, 4)
    }
}
#[doc = "DMAC ring buffer size\n\nYou can [`read`](crate::Reg::read) this register and get [`d_rbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_rbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRbsrSpec;
impl crate::RegisterSpec for DRbsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_rbsr::R`](R) reader structure"]
impl crate::Readable for DRbsrSpec {}
#[doc = "`write(|w| ..)` method takes [`d_rbsr::W`](W) writer structure"]
impl crate::Writable for DRbsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_RBSR to value 0"]
impl crate::Resettable for DRbsrSpec {}
