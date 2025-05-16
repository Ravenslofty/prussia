#[doc = "Register `D_STAT` reader"]
pub type R = crate::R<DStatSpec>;
#[doc = "Register `D_STAT` writer"]
pub type W = crate::W<DStatSpec>;
#[doc = "Field `CIS0` reader - Channel interrupt status (ch-0)"]
pub type Cis0R = crate::BitReader;
#[doc = "Field `CIS0` writer - Channel interrupt status (ch-0)"]
pub type Cis0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS1` reader - Channel interrupt status (ch-1)"]
pub type Cis1R = crate::BitReader;
#[doc = "Field `CIS1` writer - Channel interrupt status (ch-1)"]
pub type Cis1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS2` reader - Channel interrupt status (ch-2)"]
pub type Cis2R = crate::BitReader;
#[doc = "Field `CIS2` writer - Channel interrupt status (ch-2)"]
pub type Cis2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS3` reader - Channel interrupt status (ch-3)"]
pub type Cis3R = crate::BitReader;
#[doc = "Field `CIS3` writer - Channel interrupt status (ch-3)"]
pub type Cis3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS4` reader - Channel interrupt status (ch-4)"]
pub type Cis4R = crate::BitReader;
#[doc = "Field `CIS4` writer - Channel interrupt status (ch-4)"]
pub type Cis4W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS5` reader - Channel interrupt status (ch-5)"]
pub type Cis5R = crate::BitReader;
#[doc = "Field `CIS5` writer - Channel interrupt status (ch-5)"]
pub type Cis5W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS6` reader - Channel interrupt status (ch-6)"]
pub type Cis6R = crate::BitReader;
#[doc = "Field `CIS6` writer - Channel interrupt status (ch-6)"]
pub type Cis6W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS7` reader - Channel interrupt status (ch-7)"]
pub type Cis7R = crate::BitReader;
#[doc = "Field `CIS7` writer - Channel interrupt status (ch-7)"]
pub type Cis7W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS8` reader - Channel interrupt status (ch-8)"]
pub type Cis8R = crate::BitReader;
#[doc = "Field `CIS8` writer - Channel interrupt status (ch-8)"]
pub type Cis8W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIS9` reader - Channel interrupt status (ch-9)"]
pub type Cis9R = crate::BitReader;
#[doc = "Field `CIS9` writer - Channel interrupt status (ch-9)"]
pub type Cis9W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SIS` reader - DMA Stall interrupt status"]
pub type SisR = crate::BitReader;
#[doc = "Field `SIS` writer - DMA Stall interrupt status"]
pub type SisW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MEIS` reader - MFIFO empty interrupt status"]
pub type MeisR = crate::BitReader;
#[doc = "Field `MEIS` writer - MFIFO empty interrupt status"]
pub type MeisW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BEIS` reader - BUSERR interrupt status"]
pub type BeisR = crate::BitReader;
#[doc = "Field `BEIS` writer - BUSERR interrupt status"]
pub type BeisW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Channel interrupt mask (ch-0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim0 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim0> for bool {
    #[inline(always)]
    fn from(variant: Cim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM0` reader - Channel interrupt mask (ch-0)"]
pub type Cim0R = crate::BitReader<Cim0>;
impl Cim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim0 {
        match self.bits {
            false => Cim0::Disable,
            true => Cim0::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim0::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim0::Enable
    }
}
#[doc = "Field `CIM0` writer - Channel interrupt mask (ch-0)"]
pub type Cim0W<'a, REG> = crate::BitWriter<'a, REG, Cim0>;
impl<'a, REG> Cim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim0::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim0::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim1 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim1> for bool {
    #[inline(always)]
    fn from(variant: Cim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM1` reader - Channel interrupt mask (ch-1)"]
pub type Cim1R = crate::BitReader<Cim1>;
impl Cim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim1 {
        match self.bits {
            false => Cim1::Disable,
            true => Cim1::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim1::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim1::Enable
    }
}
#[doc = "Field `CIM1` writer - Channel interrupt mask (ch-1)"]
pub type Cim1W<'a, REG> = crate::BitWriter<'a, REG, Cim1>;
impl<'a, REG> Cim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim1::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim1::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim2 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim2> for bool {
    #[inline(always)]
    fn from(variant: Cim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM2` reader - Channel interrupt mask (ch-2)"]
pub type Cim2R = crate::BitReader<Cim2>;
impl Cim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim2 {
        match self.bits {
            false => Cim2::Disable,
            true => Cim2::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim2::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim2::Enable
    }
}
#[doc = "Field `CIM2` writer - Channel interrupt mask (ch-2)"]
pub type Cim2W<'a, REG> = crate::BitWriter<'a, REG, Cim2>;
impl<'a, REG> Cim2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim2::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim2::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim3 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim3> for bool {
    #[inline(always)]
    fn from(variant: Cim3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM3` reader - Channel interrupt mask (ch-3)"]
pub type Cim3R = crate::BitReader<Cim3>;
impl Cim3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim3 {
        match self.bits {
            false => Cim3::Disable,
            true => Cim3::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim3::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim3::Enable
    }
}
#[doc = "Field `CIM3` writer - Channel interrupt mask (ch-3)"]
pub type Cim3W<'a, REG> = crate::BitWriter<'a, REG, Cim3>;
impl<'a, REG> Cim3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim3::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim3::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim4 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim4> for bool {
    #[inline(always)]
    fn from(variant: Cim4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM4` reader - Channel interrupt mask (ch-4)"]
pub type Cim4R = crate::BitReader<Cim4>;
impl Cim4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim4 {
        match self.bits {
            false => Cim4::Disable,
            true => Cim4::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim4::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim4::Enable
    }
}
#[doc = "Field `CIM4` writer - Channel interrupt mask (ch-4)"]
pub type Cim4W<'a, REG> = crate::BitWriter<'a, REG, Cim4>;
impl<'a, REG> Cim4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim4::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim4::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-5)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim5 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim5> for bool {
    #[inline(always)]
    fn from(variant: Cim5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM5` reader - Channel interrupt mask (ch-5)"]
pub type Cim5R = crate::BitReader<Cim5>;
impl Cim5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim5 {
        match self.bits {
            false => Cim5::Disable,
            true => Cim5::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim5::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim5::Enable
    }
}
#[doc = "Field `CIM5` writer - Channel interrupt mask (ch-5)"]
pub type Cim5W<'a, REG> = crate::BitWriter<'a, REG, Cim5>;
impl<'a, REG> Cim5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim5::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim5::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-6)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim6 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim6> for bool {
    #[inline(always)]
    fn from(variant: Cim6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM6` reader - Channel interrupt mask (ch-6)"]
pub type Cim6R = crate::BitReader<Cim6>;
impl Cim6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim6 {
        match self.bits {
            false => Cim6::Disable,
            true => Cim6::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim6::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim6::Enable
    }
}
#[doc = "Field `CIM6` writer - Channel interrupt mask (ch-6)"]
pub type Cim6W<'a, REG> = crate::BitWriter<'a, REG, Cim6>;
impl<'a, REG> Cim6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim6::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim6::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim7 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim7> for bool {
    #[inline(always)]
    fn from(variant: Cim7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM7` reader - Channel interrupt mask (ch-7)"]
pub type Cim7R = crate::BitReader<Cim7>;
impl Cim7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim7 {
        match self.bits {
            false => Cim7::Disable,
            true => Cim7::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim7::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim7::Enable
    }
}
#[doc = "Field `CIM7` writer - Channel interrupt mask (ch-7)"]
pub type Cim7W<'a, REG> = crate::BitWriter<'a, REG, Cim7>;
impl<'a, REG> Cim7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim7::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim7::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim8 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim8> for bool {
    #[inline(always)]
    fn from(variant: Cim8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM8` reader - Channel interrupt mask (ch-8)"]
pub type Cim8R = crate::BitReader<Cim8>;
impl Cim8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim8 {
        match self.bits {
            false => Cim8::Disable,
            true => Cim8::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim8::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim8::Enable
    }
}
#[doc = "Field `CIM8` writer - Channel interrupt mask (ch-8)"]
pub type Cim8W<'a, REG> = crate::BitWriter<'a, REG, Cim8>;
impl<'a, REG> Cim8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim8::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim8::Enable)
    }
}
#[doc = "Channel interrupt mask (ch-9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cim9 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cim9> for bool {
    #[inline(always)]
    fn from(variant: Cim9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIM9` reader - Channel interrupt mask (ch-9)"]
pub type Cim9R = crate::BitReader<Cim9>;
impl Cim9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim9 {
        match self.bits {
            false => Cim9::Disable,
            true => Cim9::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cim9::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cim9::Enable
    }
}
#[doc = "Field `CIM9` writer - Channel interrupt mask (ch-9)"]
pub type Cim9W<'a, REG> = crate::BitWriter<'a, REG, Cim9>;
impl<'a, REG> Cim9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim9::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cim9::Enable)
    }
}
#[doc = "DMA stall interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sim {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Sim> for bool {
    #[inline(always)]
    fn from(variant: Sim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIM` reader - DMA stall interrupt mask"]
pub type SimR = crate::BitReader<Sim>;
impl SimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sim {
        match self.bits {
            false => Sim::Disable,
            true => Sim::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sim::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sim::Enable
    }
}
#[doc = "Field `SIM` writer - DMA stall interrupt mask"]
pub type SimW<'a, REG> = crate::BitWriter<'a, REG, Sim>;
impl<'a, REG> SimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sim::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sim::Enable)
    }
}
#[doc = "MFIFO empty interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meim {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Meim> for bool {
    #[inline(always)]
    fn from(variant: Meim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEIM` reader - MFIFO empty interrupt mask"]
pub type MeimR = crate::BitReader<Meim>;
impl MeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Meim {
        match self.bits {
            false => Meim::Disable,
            true => Meim::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Meim::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Meim::Enable
    }
}
#[doc = "Field `MEIM` writer - MFIFO empty interrupt mask"]
pub type MeimW<'a, REG> = crate::BitWriter<'a, REG, Meim>;
impl<'a, REG> MeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Meim::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Meim::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Channel interrupt status (ch-0)"]
    #[inline(always)]
    pub fn cis0(&self) -> Cis0R {
        Cis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel interrupt status (ch-1)"]
    #[inline(always)]
    pub fn cis1(&self) -> Cis1R {
        Cis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel interrupt status (ch-2)"]
    #[inline(always)]
    pub fn cis2(&self) -> Cis2R {
        Cis2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel interrupt status (ch-3)"]
    #[inline(always)]
    pub fn cis3(&self) -> Cis3R {
        Cis3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel interrupt status (ch-4)"]
    #[inline(always)]
    pub fn cis4(&self) -> Cis4R {
        Cis4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel interrupt status (ch-5)"]
    #[inline(always)]
    pub fn cis5(&self) -> Cis5R {
        Cis5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel interrupt status (ch-6)"]
    #[inline(always)]
    pub fn cis6(&self) -> Cis6R {
        Cis6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel interrupt status (ch-7)"]
    #[inline(always)]
    pub fn cis7(&self) -> Cis7R {
        Cis7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel interrupt status (ch-8)"]
    #[inline(always)]
    pub fn cis8(&self) -> Cis8R {
        Cis8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel interrupt status (ch-9)"]
    #[inline(always)]
    pub fn cis9(&self) -> Cis9R {
        Cis9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Stall interrupt status"]
    #[inline(always)]
    pub fn sis(&self) -> SisR {
        SisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MFIFO empty interrupt status"]
    #[inline(always)]
    pub fn meis(&self) -> MeisR {
        MeisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BUSERR interrupt status"]
    #[inline(always)]
    pub fn beis(&self) -> BeisR {
        BeisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel interrupt mask (ch-0)"]
    #[inline(always)]
    pub fn cim0(&self) -> Cim0R {
        Cim0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel interrupt mask (ch-1)"]
    #[inline(always)]
    pub fn cim1(&self) -> Cim1R {
        Cim1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel interrupt mask (ch-2)"]
    #[inline(always)]
    pub fn cim2(&self) -> Cim2R {
        Cim2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel interrupt mask (ch-3)"]
    #[inline(always)]
    pub fn cim3(&self) -> Cim3R {
        Cim3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel interrupt mask (ch-4)"]
    #[inline(always)]
    pub fn cim4(&self) -> Cim4R {
        Cim4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel interrupt mask (ch-5)"]
    #[inline(always)]
    pub fn cim5(&self) -> Cim5R {
        Cim5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel interrupt mask (ch-6)"]
    #[inline(always)]
    pub fn cim6(&self) -> Cim6R {
        Cim6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel interrupt mask (ch-7)"]
    #[inline(always)]
    pub fn cim7(&self) -> Cim7R {
        Cim7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel interrupt mask (ch-8)"]
    #[inline(always)]
    pub fn cim8(&self) -> Cim8R {
        Cim8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel interrupt mask (ch-9)"]
    #[inline(always)]
    pub fn cim9(&self) -> Cim9R {
        Cim9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA stall interrupt mask"]
    #[inline(always)]
    pub fn sim(&self) -> SimR {
        SimR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MFIFO empty interrupt mask"]
    #[inline(always)]
    pub fn meim(&self) -> MeimR {
        MeimR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel interrupt status (ch-0)"]
    #[inline(always)]
    pub fn cis0(&mut self) -> Cis0W<DStatSpec> {
        Cis0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel interrupt status (ch-1)"]
    #[inline(always)]
    pub fn cis1(&mut self) -> Cis1W<DStatSpec> {
        Cis1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel interrupt status (ch-2)"]
    #[inline(always)]
    pub fn cis2(&mut self) -> Cis2W<DStatSpec> {
        Cis2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel interrupt status (ch-3)"]
    #[inline(always)]
    pub fn cis3(&mut self) -> Cis3W<DStatSpec> {
        Cis3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel interrupt status (ch-4)"]
    #[inline(always)]
    pub fn cis4(&mut self) -> Cis4W<DStatSpec> {
        Cis4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel interrupt status (ch-5)"]
    #[inline(always)]
    pub fn cis5(&mut self) -> Cis5W<DStatSpec> {
        Cis5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel interrupt status (ch-6)"]
    #[inline(always)]
    pub fn cis6(&mut self) -> Cis6W<DStatSpec> {
        Cis6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel interrupt status (ch-7)"]
    #[inline(always)]
    pub fn cis7(&mut self) -> Cis7W<DStatSpec> {
        Cis7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel interrupt status (ch-8)"]
    #[inline(always)]
    pub fn cis8(&mut self) -> Cis8W<DStatSpec> {
        Cis8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel interrupt status (ch-9)"]
    #[inline(always)]
    pub fn cis9(&mut self) -> Cis9W<DStatSpec> {
        Cis9W::new(self, 9)
    }
    #[doc = "Bit 13 - DMA Stall interrupt status"]
    #[inline(always)]
    pub fn sis(&mut self) -> SisW<DStatSpec> {
        SisW::new(self, 13)
    }
    #[doc = "Bit 14 - MFIFO empty interrupt status"]
    #[inline(always)]
    pub fn meis(&mut self) -> MeisW<DStatSpec> {
        MeisW::new(self, 14)
    }
    #[doc = "Bit 15 - BUSERR interrupt status"]
    #[inline(always)]
    pub fn beis(&mut self) -> BeisW<DStatSpec> {
        BeisW::new(self, 15)
    }
    #[doc = "Bit 16 - Channel interrupt mask (ch-0)"]
    #[inline(always)]
    pub fn cim0(&mut self) -> Cim0W<DStatSpec> {
        Cim0W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel interrupt mask (ch-1)"]
    #[inline(always)]
    pub fn cim1(&mut self) -> Cim1W<DStatSpec> {
        Cim1W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel interrupt mask (ch-2)"]
    #[inline(always)]
    pub fn cim2(&mut self) -> Cim2W<DStatSpec> {
        Cim2W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel interrupt mask (ch-3)"]
    #[inline(always)]
    pub fn cim3(&mut self) -> Cim3W<DStatSpec> {
        Cim3W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel interrupt mask (ch-4)"]
    #[inline(always)]
    pub fn cim4(&mut self) -> Cim4W<DStatSpec> {
        Cim4W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel interrupt mask (ch-5)"]
    #[inline(always)]
    pub fn cim5(&mut self) -> Cim5W<DStatSpec> {
        Cim5W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel interrupt mask (ch-6)"]
    #[inline(always)]
    pub fn cim6(&mut self) -> Cim6W<DStatSpec> {
        Cim6W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel interrupt mask (ch-7)"]
    #[inline(always)]
    pub fn cim7(&mut self) -> Cim7W<DStatSpec> {
        Cim7W::new(self, 23)
    }
    #[doc = "Bit 24 - Channel interrupt mask (ch-8)"]
    #[inline(always)]
    pub fn cim8(&mut self) -> Cim8W<DStatSpec> {
        Cim8W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel interrupt mask (ch-9)"]
    #[inline(always)]
    pub fn cim9(&mut self) -> Cim9W<DStatSpec> {
        Cim9W::new(self, 25)
    }
    #[doc = "Bit 29 - DMA stall interrupt mask"]
    #[inline(always)]
    pub fn sim(&mut self) -> SimW<DStatSpec> {
        SimW::new(self, 29)
    }
    #[doc = "Bit 30 - MFIFO empty interrupt mask"]
    #[inline(always)]
    pub fn meim(&mut self) -> MeimW<DStatSpec> {
        MeimW::new(self, 30)
    }
}
#[doc = "DMAC status\n\nYou can [`read`](crate::Reg::read) this register and get [`d_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DStatSpec;
impl crate::RegisterSpec for DStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_stat::R`](R) reader structure"]
impl crate::Readable for DStatSpec {}
#[doc = "`write(|w| ..)` method takes [`d_stat::W`](W) writer structure"]
impl crate::Writable for DStatSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xe3ff;
}
#[doc = "`reset()` method sets D_STAT to value 0"]
impl crate::Resettable for DStatSpec {}
