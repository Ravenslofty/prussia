#[doc = "Register `IPU_CMD` reader"]
pub type R = crate::R<IpuCmdSpec>;
#[doc = "Register `IPU_CMD` writer"]
pub type W = crate::W<IpuCmdSpec>;
#[doc = "Field `OPTION` writer - Command Option. Contents differ depending on executed command."]
pub type OptionW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `DATA` reader - VDEC / FDEC decoded value."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `CODE` writer - Command code."]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "VDEC / FDEC command busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: DATA field enabled (available to read.)"]
    Enable = 0,
    #[doc = "1: DATA field disabled (VDEC / FDEC still in execution.)"]
    Disable = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - VDEC / FDEC command busy"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Enable,
            true => Busy::Disable,
        }
    }
    #[doc = "DATA field enabled (available to read.)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Busy::Enable
    }
    #[doc = "DATA field disabled (VDEC / FDEC still in execution.)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Busy::Disable
    }
}
impl R {
    #[doc = "Bits 0:31 - VDEC / FDEC decoded value."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bit 63 - VDEC / FDEC command busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Command Option. Contents differ depending on executed command."]
    #[inline(always)]
    pub fn option(&mut self) -> OptionW<IpuCmdSpec> {
        OptionW::new(self, 0)
    }
    #[doc = "Bits 28:31 - Command code."]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<IpuCmdSpec> {
        CodeW::new(self, 28)
    }
}
#[doc = "IPU Decoded-code read register / command register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipu_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpuCmdSpec;
impl crate::RegisterSpec for IpuCmdSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ipu_cmd::R`](R) reader structure"]
impl crate::Readable for IpuCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ipu_cmd::W`](W) writer structure"]
impl crate::Writable for IpuCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPU_CMD to value 0"]
impl crate::Resettable for IpuCmdSpec {}
