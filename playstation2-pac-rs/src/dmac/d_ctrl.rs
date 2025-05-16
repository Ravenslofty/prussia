#[doc = "Register `D_CTRL` reader"]
pub type R = crate::R<DCtrlSpec>;
#[doc = "Register `D_CTRL` writer"]
pub type W = crate::W<DCtrlSpec>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmae {
    #[doc = "0: Disables all DMAs"]
    Disable = 0,
    #[doc = "1: Enables all DMAs"]
    Enable = 1,
}
impl From<Dmae> for bool {
    #[inline(always)]
    fn from(variant: Dmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAE` reader - DMA Enable"]
pub type DmaeR = crate::BitReader<Dmae>;
impl DmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmae {
        match self.bits {
            false => Dmae::Disable,
            true => Dmae::Enable,
        }
    }
    #[doc = "Disables all DMAs"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmae::Disable
    }
    #[doc = "Enables all DMAs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmae::Enable
    }
}
#[doc = "Field `DMAE` writer - DMA Enable"]
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG, Dmae>;
impl<'a, REG> DmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables all DMAs"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmae::Disable)
    }
    #[doc = "Enables all DMAs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmae::Enable)
    }
}
#[doc = "Release signal enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rele {
    #[doc = "0: Cycle stealing off"]
    Off = 0,
    #[doc = "1: Cycle stealing on"]
    On = 1,
}
impl From<Rele> for bool {
    #[inline(always)]
    fn from(variant: Rele) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELE` reader - Release signal enable"]
pub type ReleR = crate::BitReader<Rele>;
impl ReleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rele {
        match self.bits {
            false => Rele::Off,
            true => Rele::On,
        }
    }
    #[doc = "Cycle stealing off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Rele::Off
    }
    #[doc = "Cycle stealing on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Rele::On
    }
}
#[doc = "Field `RELE` writer - Release signal enable"]
pub type ReleW<'a, REG> = crate::BitWriter<'a, REG, Rele>;
impl<'a, REG> ReleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cycle stealing off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Rele::Off)
    }
    #[doc = "Cycle stealing on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Rele::On)
    }
}
#[doc = "Memory FIFO drain channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mfd {
    #[doc = "0: Does not use MFIFO function."]
    Off = 0,
    #[doc = "2: VIF1 channel (ch-1)"]
    Vif1 = 2,
    #[doc = "3: GIF channel (ch-2)"]
    Gif = 3,
}
impl From<Mfd> for u8 {
    #[inline(always)]
    fn from(variant: Mfd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mfd {
    type Ux = u8;
}
impl crate::IsEnum for Mfd {}
#[doc = "Field `MFD` reader - Memory FIFO drain channel"]
pub type MfdR = crate::FieldReader<Mfd>;
impl MfdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mfd> {
        match self.bits {
            0 => Some(Mfd::Off),
            2 => Some(Mfd::Vif1),
            3 => Some(Mfd::Gif),
            _ => None,
        }
    }
    #[doc = "Does not use MFIFO function."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mfd::Off
    }
    #[doc = "VIF1 channel (ch-1)"]
    #[inline(always)]
    pub fn is_vif1(&self) -> bool {
        *self == Mfd::Vif1
    }
    #[doc = "GIF channel (ch-2)"]
    #[inline(always)]
    pub fn is_gif(&self) -> bool {
        *self == Mfd::Gif
    }
}
#[doc = "Field `MFD` writer - Memory FIFO drain channel"]
pub type MfdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mfd>;
impl<'a, REG> MfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Does not use MFIFO function."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mfd::Off)
    }
    #[doc = "VIF1 channel (ch-1)"]
    #[inline(always)]
    pub fn vif1(self) -> &'a mut crate::W<REG> {
        self.variant(Mfd::Vif1)
    }
    #[doc = "GIF channel (ch-2)"]
    #[inline(always)]
    pub fn gif(self) -> &'a mut crate::W<REG> {
        self.variant(Mfd::Gif)
    }
}
#[doc = "Stall Control source channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sts {
    #[doc = "0: Non-specified (Does not update D_STADR)"]
    None = 0,
    #[doc = "1: SIF0 channel (ch-5)"]
    Sif0 = 1,
    #[doc = "2: fromSPR channel (ch-8)"]
    FromSpr = 2,
    #[doc = "3: fromIPU channel (ch-3)"]
    FromIpu = 3,
}
impl From<Sts> for u8 {
    #[inline(always)]
    fn from(variant: Sts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sts {
    type Ux = u8;
}
impl crate::IsEnum for Sts {}
#[doc = "Field `STS` reader - Stall Control source channel"]
pub type StsR = crate::FieldReader<Sts>;
impl StsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sts {
        match self.bits {
            0 => Sts::None,
            1 => Sts::Sif0,
            2 => Sts::FromSpr,
            3 => Sts::FromIpu,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-specified (Does not update D_STADR)"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sts::None
    }
    #[doc = "SIF0 channel (ch-5)"]
    #[inline(always)]
    pub fn is_sif0(&self) -> bool {
        *self == Sts::Sif0
    }
    #[doc = "fromSPR channel (ch-8)"]
    #[inline(always)]
    pub fn is_from_spr(&self) -> bool {
        *self == Sts::FromSpr
    }
    #[doc = "fromIPU channel (ch-3)"]
    #[inline(always)]
    pub fn is_from_ipu(&self) -> bool {
        *self == Sts::FromIpu
    }
}
#[doc = "Field `STS` writer - Stall Control source channel"]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sts, crate::Safe>;
impl<'a, REG> StsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-specified (Does not update D_STADR)"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::None)
    }
    #[doc = "SIF0 channel (ch-5)"]
    #[inline(always)]
    pub fn sif0(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sif0)
    }
    #[doc = "fromSPR channel (ch-8)"]
    #[inline(always)]
    pub fn from_spr(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::FromSpr)
    }
    #[doc = "fromIPU channel (ch-3)"]
    #[inline(always)]
    pub fn from_ipu(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::FromIpu)
    }
}
#[doc = "Stall Control drain channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Std {
    #[doc = "0: Does not perform stall control"]
    Off = 0,
    #[doc = "1: VIF1 channel (ch-1)"]
    Vif1 = 1,
    #[doc = "2: GIF channel (ch-2)"]
    Gif = 2,
    #[doc = "3: SIF1 channel (ch-6)"]
    Sif1 = 3,
}
impl From<Std> for u8 {
    #[inline(always)]
    fn from(variant: Std) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Std {
    type Ux = u8;
}
impl crate::IsEnum for Std {}
#[doc = "Field `STD` reader - Stall Control drain channel"]
pub type StdR = crate::FieldReader<Std>;
impl StdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Std {
        match self.bits {
            0 => Std::Off,
            1 => Std::Vif1,
            2 => Std::Gif,
            3 => Std::Sif1,
            _ => unreachable!(),
        }
    }
    #[doc = "Does not perform stall control"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Std::Off
    }
    #[doc = "VIF1 channel (ch-1)"]
    #[inline(always)]
    pub fn is_vif1(&self) -> bool {
        *self == Std::Vif1
    }
    #[doc = "GIF channel (ch-2)"]
    #[inline(always)]
    pub fn is_gif(&self) -> bool {
        *self == Std::Gif
    }
    #[doc = "SIF1 channel (ch-6)"]
    #[inline(always)]
    pub fn is_sif1(&self) -> bool {
        *self == Std::Sif1
    }
}
#[doc = "Field `STD` writer - Stall Control drain channel"]
pub type StdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Std, crate::Safe>;
impl<'a, REG> StdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Does not perform stall control"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Std::Off)
    }
    #[doc = "VIF1 channel (ch-1)"]
    #[inline(always)]
    pub fn vif1(self) -> &'a mut crate::W<REG> {
        self.variant(Std::Vif1)
    }
    #[doc = "GIF channel (ch-2)"]
    #[inline(always)]
    pub fn gif(self) -> &'a mut crate::W<REG> {
        self.variant(Std::Gif)
    }
    #[doc = "SIF1 channel (ch-6)"]
    #[inline(always)]
    pub fn sif1(self) -> &'a mut crate::W<REG> {
        self.variant(Std::Sif1)
    }
}
#[doc = "Release cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcyc {
    #[doc = "0: Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    Cycle8 = 0,
    #[doc = "1: Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    Cycle16 = 1,
    #[doc = "2: Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    Cycle32 = 2,
    #[doc = "3: Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    Cycle64 = 3,
    #[doc = "4: Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    Cycle128 = 4,
    #[doc = "5: Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    Cycle256 = 5,
}
impl From<Rcyc> for u8 {
    #[inline(always)]
    fn from(variant: Rcyc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcyc {
    type Ux = u8;
}
impl crate::IsEnum for Rcyc {}
#[doc = "Field `RCYC` reader - Release cycle"]
pub type RcycR = crate::FieldReader<Rcyc>;
impl RcycR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rcyc> {
        match self.bits {
            0 => Some(Rcyc::Cycle8),
            1 => Some(Rcyc::Cycle16),
            2 => Some(Rcyc::Cycle32),
            3 => Some(Rcyc::Cycle64),
            4 => Some(Rcyc::Cycle128),
            5 => Some(Rcyc::Cycle256),
            _ => None,
        }
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn is_cycle_8(&self) -> bool {
        *self == Rcyc::Cycle8
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn is_cycle_16(&self) -> bool {
        *self == Rcyc::Cycle16
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn is_cycle_32(&self) -> bool {
        *self == Rcyc::Cycle32
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn is_cycle_64(&self) -> bool {
        *self == Rcyc::Cycle64
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn is_cycle_128(&self) -> bool {
        *self == Rcyc::Cycle128
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn is_cycle_256(&self) -> bool {
        *self == Rcyc::Cycle256
    }
}
#[doc = "Field `RCYC` writer - Release cycle"]
pub type RcycW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rcyc>;
impl<'a, REG> RcycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn cycle_8(self) -> &'a mut crate::W<REG> {
        self.variant(Rcyc::Cycle8)
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn cycle_16(self) -> &'a mut crate::W<REG> {
        self.variant(Rcyc::Cycle16)
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn cycle_32(self) -> &'a mut crate::W<REG> {
        self.variant(Rcyc::Cycle32)
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn cycle_64(self) -> &'a mut crate::W<REG> {
        self.variant(Rcyc::Cycle64)
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn cycle_128(self) -> &'a mut crate::W<REG> {
        self.variant(Rcyc::Cycle128)
    }
    #[doc = "Number of cycles to release the bbus to EE core when Cycle Stealing is on"]
    #[inline(always)]
    pub fn cycle_256(self) -> &'a mut crate::W<REG> {
        self.variant(Rcyc::Cycle256)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Release signal enable"]
    #[inline(always)]
    pub fn rele(&self) -> ReleR {
        ReleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Memory FIFO drain channel"]
    #[inline(always)]
    pub fn mfd(&self) -> MfdR {
        MfdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Stall Control source channel"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Stall Control drain channel"]
    #[inline(always)]
    pub fn std(&self) -> StdR {
        StdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Release cycle"]
    #[inline(always)]
    pub fn rcyc(&self) -> RcycR {
        RcycR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<DCtrlSpec> {
        DmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - Release signal enable"]
    #[inline(always)]
    pub fn rele(&mut self) -> ReleW<DCtrlSpec> {
        ReleW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Memory FIFO drain channel"]
    #[inline(always)]
    pub fn mfd(&mut self) -> MfdW<DCtrlSpec> {
        MfdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Stall Control source channel"]
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<DCtrlSpec> {
        StsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Stall Control drain channel"]
    #[inline(always)]
    pub fn std(&mut self) -> StdW<DCtrlSpec> {
        StdW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Release cycle"]
    #[inline(always)]
    pub fn rcyc(&mut self) -> RcycW<DCtrlSpec> {
        RcycW::new(self, 8)
    }
}
#[doc = "DMAC control\n\nYou can [`read`](crate::Reg::read) this register and get [`d_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCtrlSpec;
impl crate::RegisterSpec for DCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_ctrl::R`](R) reader structure"]
impl crate::Readable for DCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`d_ctrl::W`](W) writer structure"]
impl crate::Writable for DCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_CTRL to value 0"]
impl crate::Resettable for DCtrlSpec {}
