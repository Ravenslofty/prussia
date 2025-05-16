#[doc = "Register `SIO_BGR` reader"]
pub type R = crate::R<SioBgrSpec>;
#[doc = "Register `SIO_BGR` writer"]
pub type W = crate::W<SioBgrSpec>;
#[doc = "Field `UBRD` reader - Configures the baud rate divisor value. Default value is 0xFF."]
pub type UbrdR = crate::FieldReader;
#[doc = "Field `UBRD` writer - Configures the baud rate divisor value. Default value is 0xFF."]
pub type UbrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Configures the Baud Rate Generator Clock. Default setting is 0x3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bclk {
    #[doc = "0: FC / 2"]
    Fc2 = 0,
    #[doc = "1: FC / 8"]
    Fc8 = 1,
    #[doc = "2: FC / 32"]
    Fc32 = 2,
    #[doc = "3: FC / 128"]
    Fc128 = 3,
}
impl From<Bclk> for u8 {
    #[inline(always)]
    fn from(variant: Bclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bclk {
    type Ux = u8;
}
impl crate::IsEnum for Bclk {}
#[doc = "Field `BCLK` reader - Configures the Baud Rate Generator Clock. Default setting is 0x3."]
pub type BclkR = crate::FieldReader<Bclk>;
impl BclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bclk {
        match self.bits {
            0 => Bclk::Fc2,
            1 => Bclk::Fc8,
            2 => Bclk::Fc32,
            3 => Bclk::Fc128,
            _ => unreachable!(),
        }
    }
    #[doc = "FC / 2"]
    #[inline(always)]
    pub fn is_fc_2(&self) -> bool {
        *self == Bclk::Fc2
    }
    #[doc = "FC / 8"]
    #[inline(always)]
    pub fn is_fc_8(&self) -> bool {
        *self == Bclk::Fc8
    }
    #[doc = "FC / 32"]
    #[inline(always)]
    pub fn is_fc_32(&self) -> bool {
        *self == Bclk::Fc32
    }
    #[doc = "FC / 128"]
    #[inline(always)]
    pub fn is_fc_128(&self) -> bool {
        *self == Bclk::Fc128
    }
}
#[doc = "Field `BCLK` writer - Configures the Baud Rate Generator Clock. Default setting is 0x3."]
pub type BclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bclk, crate::Safe>;
impl<'a, REG> BclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FC / 2"]
    #[inline(always)]
    pub fn fc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Bclk::Fc2)
    }
    #[doc = "FC / 8"]
    #[inline(always)]
    pub fn fc_8(self) -> &'a mut crate::W<REG> {
        self.variant(Bclk::Fc8)
    }
    #[doc = "FC / 32"]
    #[inline(always)]
    pub fn fc_32(self) -> &'a mut crate::W<REG> {
        self.variant(Bclk::Fc32)
    }
    #[doc = "FC / 128"]
    #[inline(always)]
    pub fn fc_128(self) -> &'a mut crate::W<REG> {
        self.variant(Bclk::Fc128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Configures the baud rate divisor value. Default value is 0xFF."]
    #[inline(always)]
    pub fn ubrd(&self) -> UbrdR {
        UbrdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Configures the Baud Rate Generator Clock. Default setting is 0x3."]
    #[inline(always)]
    pub fn bclk(&self) -> BclkR {
        BclkR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the baud rate divisor value. Default value is 0xFF."]
    #[inline(always)]
    pub fn ubrd(&mut self) -> UbrdW<SioBgrSpec> {
        UbrdW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Configures the Baud Rate Generator Clock. Default setting is 0x3."]
    #[inline(always)]
    pub fn bclk(&mut self) -> BclkW<SioBgrSpec> {
        BclkW::new(self, 8)
    }
}
#[doc = "Baud Rate Control Register. Configures the pre-scaler and clock frequency for the Baud Rate Generator. The clock signal used is calculated based on the formula `fc / (prescaler x divisor x 16)` where fc is the clock chosen in the Line Control Register (bits 6:5). Based on the TX49 docs.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_bgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_bgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioBgrSpec;
impl crate::RegisterSpec for SioBgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sio_bgr::R`](R) reader structure"]
impl crate::Readable for SioBgrSpec {}
#[doc = "`write(|w| ..)` method takes [`sio_bgr::W`](W) writer structure"]
impl crate::Writable for SioBgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIO_BGR to value 0"]
impl crate::Resettable for SioBgrSpec {}
