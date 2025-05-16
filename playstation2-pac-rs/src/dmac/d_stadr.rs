#[doc = "Register `D_STADR` reader"]
pub type R = crate::R<DStadrSpec>;
#[doc = "Register `D_STADR` writer"]
pub type W = crate::W<DStadrSpec>;
#[doc = "Field `ADDR` reader - Stall address (qword alignment)"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Stall address (qword alignment)"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 4:30 - Stall address (qword alignment)"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 4) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:30 - Stall address (qword alignment)"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<DStadrSpec> {
        AddrW::new(self, 4)
    }
}
#[doc = "DMAC stall address\n\nYou can [`read`](crate::Reg::read) this register and get [`d_stadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_stadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DStadrSpec;
impl crate::RegisterSpec for DStadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_stadr::R`](R) reader structure"]
impl crate::Readable for DStadrSpec {}
#[doc = "`write(|w| ..)` method takes [`d_stadr::W`](W) writer structure"]
impl crate::Writable for DStadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_STADR to value 0"]
impl crate::Resettable for DStadrSpec {}
