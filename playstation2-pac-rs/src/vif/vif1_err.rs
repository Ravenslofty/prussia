#[doc = "Register `VIF1_ERR` reader"]
pub type R = crate::R<Vif1ErrSpec>;
#[doc = "Register `VIF1_ERR` writer"]
pub type W = crate::W<Vif1ErrSpec>;
#[doc = "Masks an interrupt by the i bit of the VIFcode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mii {
    #[doc = "0: Unmask (i bit interrupt enable)"]
    Unmask = 0,
    #[doc = "1: Mask (i bit interrupt disable)"]
    Mask = 1,
}
impl From<Mii> for bool {
    #[inline(always)]
    fn from(variant: Mii) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MII` reader - Masks an interrupt by the i bit of the VIFcode"]
pub type MiiR = crate::BitReader<Mii>;
impl MiiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mii {
        match self.bits {
            false => Mii::Unmask,
            true => Mii::Mask,
        }
    }
    #[doc = "Unmask (i bit interrupt enable)"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == Mii::Unmask
    }
    #[doc = "Mask (i bit interrupt disable)"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Mii::Mask
    }
}
#[doc = "Field `MII` writer - Masks an interrupt by the i bit of the VIFcode"]
pub type MiiW<'a, REG> = crate::BitWriter<'a, REG, Mii>;
impl<'a, REG> MiiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unmask (i bit interrupt enable)"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut crate::W<REG> {
        self.variant(Mii::Unmask)
    }
    #[doc = "Mask (i bit interrupt disable)"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Mii::Mask)
    }
}
#[doc = "DMAtag Mismatch error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Me0 {
    #[doc = "0: Unmask (stalls when an error occurs.)"]
    Unmask = 0,
    #[doc = "1: Mask (ignores a DMAtag Mismatch error.)"]
    Mask = 1,
}
impl From<Me0> for bool {
    #[inline(always)]
    fn from(variant: Me0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ME0` reader - DMAtag Mismatch error mask"]
pub type Me0R = crate::BitReader<Me0>;
impl Me0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Me0 {
        match self.bits {
            false => Me0::Unmask,
            true => Me0::Mask,
        }
    }
    #[doc = "Unmask (stalls when an error occurs.)"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == Me0::Unmask
    }
    #[doc = "Mask (ignores a DMAtag Mismatch error.)"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Me0::Mask
    }
}
#[doc = "Field `ME0` writer - DMAtag Mismatch error mask"]
pub type Me0W<'a, REG> = crate::BitWriter<'a, REG, Me0>;
impl<'a, REG> Me0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unmask (stalls when an error occurs.)"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut crate::W<REG> {
        self.variant(Me0::Unmask)
    }
    #[doc = "Mask (ignores a DMAtag Mismatch error.)"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Me0::Mask)
    }
}
#[doc = "VIFcode error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Me1 {
    #[doc = "0: Unmask (stalls when an error occurs.)"]
    Unmask = 0,
    #[doc = "1: Mask (considered as VIFcode NOP)"]
    Mask = 1,
}
impl From<Me1> for bool {
    #[inline(always)]
    fn from(variant: Me1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ME1` reader - VIFcode error mask"]
pub type Me1R = crate::BitReader<Me1>;
impl Me1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Me1 {
        match self.bits {
            false => Me1::Unmask,
            true => Me1::Mask,
        }
    }
    #[doc = "Unmask (stalls when an error occurs.)"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == Me1::Unmask
    }
    #[doc = "Mask (considered as VIFcode NOP)"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Me1::Mask
    }
}
#[doc = "Field `ME1` writer - VIFcode error mask"]
pub type Me1W<'a, REG> = crate::BitWriter<'a, REG, Me1>;
impl<'a, REG> Me1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unmask (stalls when an error occurs.)"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut crate::W<REG> {
        self.variant(Me1::Unmask)
    }
    #[doc = "Mask (considered as VIFcode NOP)"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Me1::Mask)
    }
}
impl R {
    #[doc = "Bit 0 - Masks an interrupt by the i bit of the VIFcode"]
    #[inline(always)]
    pub fn mii(&self) -> MiiR {
        MiiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAtag Mismatch error mask"]
    #[inline(always)]
    pub fn me0(&self) -> Me0R {
        Me0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VIFcode error mask"]
    #[inline(always)]
    pub fn me1(&self) -> Me1R {
        Me1R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masks an interrupt by the i bit of the VIFcode"]
    #[inline(always)]
    pub fn mii(&mut self) -> MiiW<Vif1ErrSpec> {
        MiiW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAtag Mismatch error mask"]
    #[inline(always)]
    pub fn me0(&mut self) -> Me0W<Vif1ErrSpec> {
        Me0W::new(self, 1)
    }
    #[doc = "Bit 2 - VIFcode error mask"]
    #[inline(always)]
    pub fn me1(&mut self) -> Me1W<Vif1ErrSpec> {
        Me1W::new(self, 2)
    }
}
#[doc = "Error status\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1ErrSpec;
impl crate::RegisterSpec for Vif1ErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_err::R`](R) reader structure"]
impl crate::Readable for Vif1ErrSpec {}
#[doc = "`write(|w| ..)` method takes [`vif1_err::W`](W) writer structure"]
impl crate::Writable for Vif1ErrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIF1_ERR to value 0"]
impl crate::Resettable for Vif1ErrSpec {}
