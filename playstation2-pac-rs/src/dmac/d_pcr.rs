#[doc = "Register `D_PCR` reader"]
pub type R = crate::R<DPcrSpec>;
#[doc = "Register `D_PCR` writer"]
pub type W = crate::W<DPcrSpec>;
#[doc = "COP control (ch-0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc0 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc0> for bool {
    #[inline(always)]
    fn from(variant: Cpc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC0` reader - COP control (ch-0)"]
pub type Cpc0R = crate::BitReader<Cpc0>;
impl Cpc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc0 {
        match self.bits {
            false => Cpc0::Off,
            true => Cpc0::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc0::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc0::On
    }
}
#[doc = "Field `CPC0` writer - COP control (ch-0)"]
pub type Cpc0W<'a, REG> = crate::BitWriter<'a, REG, Cpc0>;
impl<'a, REG> Cpc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc0::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc0::On)
    }
}
#[doc = "COP control (ch-1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc1 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc1> for bool {
    #[inline(always)]
    fn from(variant: Cpc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC1` reader - COP control (ch-1)"]
pub type Cpc1R = crate::BitReader<Cpc1>;
impl Cpc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc1 {
        match self.bits {
            false => Cpc1::Off,
            true => Cpc1::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc1::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc1::On
    }
}
#[doc = "Field `CPC1` writer - COP control (ch-1)"]
pub type Cpc1W<'a, REG> = crate::BitWriter<'a, REG, Cpc1>;
impl<'a, REG> Cpc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc1::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc1::On)
    }
}
#[doc = "COP control (ch-2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc2 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc2> for bool {
    #[inline(always)]
    fn from(variant: Cpc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC2` reader - COP control (ch-2)"]
pub type Cpc2R = crate::BitReader<Cpc2>;
impl Cpc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc2 {
        match self.bits {
            false => Cpc2::Off,
            true => Cpc2::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc2::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc2::On
    }
}
#[doc = "Field `CPC2` writer - COP control (ch-2)"]
pub type Cpc2W<'a, REG> = crate::BitWriter<'a, REG, Cpc2>;
impl<'a, REG> Cpc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc2::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc2::On)
    }
}
#[doc = "COP control (ch-3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc3 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc3> for bool {
    #[inline(always)]
    fn from(variant: Cpc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC3` reader - COP control (ch-3)"]
pub type Cpc3R = crate::BitReader<Cpc3>;
impl Cpc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc3 {
        match self.bits {
            false => Cpc3::Off,
            true => Cpc3::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc3::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc3::On
    }
}
#[doc = "Field `CPC3` writer - COP control (ch-3)"]
pub type Cpc3W<'a, REG> = crate::BitWriter<'a, REG, Cpc3>;
impl<'a, REG> Cpc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc3::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc3::On)
    }
}
#[doc = "COP control (ch-4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc4 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc4> for bool {
    #[inline(always)]
    fn from(variant: Cpc4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC4` reader - COP control (ch-4)"]
pub type Cpc4R = crate::BitReader<Cpc4>;
impl Cpc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc4 {
        match self.bits {
            false => Cpc4::Off,
            true => Cpc4::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc4::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc4::On
    }
}
#[doc = "Field `CPC4` writer - COP control (ch-4)"]
pub type Cpc4W<'a, REG> = crate::BitWriter<'a, REG, Cpc4>;
impl<'a, REG> Cpc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc4::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc4::On)
    }
}
#[doc = "COP control (ch-5)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc5 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc5> for bool {
    #[inline(always)]
    fn from(variant: Cpc5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC5` reader - COP control (ch-5)"]
pub type Cpc5R = crate::BitReader<Cpc5>;
impl Cpc5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc5 {
        match self.bits {
            false => Cpc5::Off,
            true => Cpc5::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc5::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc5::On
    }
}
#[doc = "Field `CPC5` writer - COP control (ch-5)"]
pub type Cpc5W<'a, REG> = crate::BitWriter<'a, REG, Cpc5>;
impl<'a, REG> Cpc5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc5::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc5::On)
    }
}
#[doc = "COP control (ch-6)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc6 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc6> for bool {
    #[inline(always)]
    fn from(variant: Cpc6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC6` reader - COP control (ch-6)"]
pub type Cpc6R = crate::BitReader<Cpc6>;
impl Cpc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc6 {
        match self.bits {
            false => Cpc6::Off,
            true => Cpc6::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc6::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc6::On
    }
}
#[doc = "Field `CPC6` writer - COP control (ch-6)"]
pub type Cpc6W<'a, REG> = crate::BitWriter<'a, REG, Cpc6>;
impl<'a, REG> Cpc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc6::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc6::On)
    }
}
#[doc = "COP control (ch-7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc7 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc7> for bool {
    #[inline(always)]
    fn from(variant: Cpc7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC7` reader - COP control (ch-7)"]
pub type Cpc7R = crate::BitReader<Cpc7>;
impl Cpc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc7 {
        match self.bits {
            false => Cpc7::Off,
            true => Cpc7::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc7::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc7::On
    }
}
#[doc = "Field `CPC7` writer - COP control (ch-7)"]
pub type Cpc7W<'a, REG> = crate::BitWriter<'a, REG, Cpc7>;
impl<'a, REG> Cpc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc7::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc7::On)
    }
}
#[doc = "COP control (ch-8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc8 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc8> for bool {
    #[inline(always)]
    fn from(variant: Cpc8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC8` reader - COP control (ch-8)"]
pub type Cpc8R = crate::BitReader<Cpc8>;
impl Cpc8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc8 {
        match self.bits {
            false => Cpc8::Off,
            true => Cpc8::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc8::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc8::On
    }
}
#[doc = "Field `CPC8` writer - COP control (ch-8)"]
pub type Cpc8W<'a, REG> = crate::BitWriter<'a, REG, Cpc8>;
impl<'a, REG> Cpc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc8::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc8::On)
    }
}
#[doc = "COP control (ch-9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpc9 {
    #[doc = "0: Does not output applicable channel status to CPCOND\\[0\\]"]
    Off = 0,
    #[doc = "1: Outputs applicable channel status to CPCOND\\[0\\]"]
    On = 1,
}
impl From<Cpc9> for bool {
    #[inline(always)]
    fn from(variant: Cpc9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPC9` reader - COP control (ch-9)"]
pub type Cpc9R = crate::BitReader<Cpc9>;
impl Cpc9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpc9 {
        match self.bits {
            false => Cpc9::Off,
            true => Cpc9::On,
        }
    }
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cpc9::Off
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cpc9::On
    }
}
#[doc = "Field `CPC9` writer - COP control (ch-9)"]
pub type Cpc9W<'a, REG> = crate::BitWriter<'a, REG, Cpc9>;
impl<'a, REG> Cpc9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc9::Off)
    }
    #[doc = "Outputs applicable channel status to CPCOND\\[0\\]"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cpc9::On)
    }
}
#[doc = "Channel DMA enable (ch-0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde0 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde0> for bool {
    #[inline(always)]
    fn from(variant: Cde0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE0` reader - Channel DMA enable (ch-0)"]
pub type Cde0R = crate::BitReader<Cde0>;
impl Cde0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde0 {
        match self.bits {
            false => Cde0::Disable,
            true => Cde0::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde0::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde0::Enable
    }
}
#[doc = "Field `CDE0` writer - Channel DMA enable (ch-0)"]
pub type Cde0W<'a, REG> = crate::BitWriter<'a, REG, Cde0>;
impl<'a, REG> Cde0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde0::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde0::Enable)
    }
}
#[doc = "Channel DMA enable (ch-1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde1 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde1> for bool {
    #[inline(always)]
    fn from(variant: Cde1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE1` reader - Channel DMA enable (ch-1)"]
pub type Cde1R = crate::BitReader<Cde1>;
impl Cde1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde1 {
        match self.bits {
            false => Cde1::Disable,
            true => Cde1::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde1::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde1::Enable
    }
}
#[doc = "Field `CDE1` writer - Channel DMA enable (ch-1)"]
pub type Cde1W<'a, REG> = crate::BitWriter<'a, REG, Cde1>;
impl<'a, REG> Cde1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde1::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde1::Enable)
    }
}
#[doc = "Channel DMA enable (ch-2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde2 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde2> for bool {
    #[inline(always)]
    fn from(variant: Cde2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE2` reader - Channel DMA enable (ch-2)"]
pub type Cde2R = crate::BitReader<Cde2>;
impl Cde2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde2 {
        match self.bits {
            false => Cde2::Disable,
            true => Cde2::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde2::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde2::Enable
    }
}
#[doc = "Field `CDE2` writer - Channel DMA enable (ch-2)"]
pub type Cde2W<'a, REG> = crate::BitWriter<'a, REG, Cde2>;
impl<'a, REG> Cde2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde2::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde2::Enable)
    }
}
#[doc = "Channel DMA enable (ch-3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde3 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde3> for bool {
    #[inline(always)]
    fn from(variant: Cde3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE3` reader - Channel DMA enable (ch-3)"]
pub type Cde3R = crate::BitReader<Cde3>;
impl Cde3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde3 {
        match self.bits {
            false => Cde3::Disable,
            true => Cde3::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde3::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde3::Enable
    }
}
#[doc = "Field `CDE3` writer - Channel DMA enable (ch-3)"]
pub type Cde3W<'a, REG> = crate::BitWriter<'a, REG, Cde3>;
impl<'a, REG> Cde3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde3::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde3::Enable)
    }
}
#[doc = "Channel DMA enable (ch-4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde4 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde4> for bool {
    #[inline(always)]
    fn from(variant: Cde4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE4` reader - Channel DMA enable (ch-4)"]
pub type Cde4R = crate::BitReader<Cde4>;
impl Cde4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde4 {
        match self.bits {
            false => Cde4::Disable,
            true => Cde4::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde4::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde4::Enable
    }
}
#[doc = "Field `CDE4` writer - Channel DMA enable (ch-4)"]
pub type Cde4W<'a, REG> = crate::BitWriter<'a, REG, Cde4>;
impl<'a, REG> Cde4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde4::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde4::Enable)
    }
}
#[doc = "Channel DMA enable (ch-5)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde5 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde5> for bool {
    #[inline(always)]
    fn from(variant: Cde5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE5` reader - Channel DMA enable (ch-5)"]
pub type Cde5R = crate::BitReader<Cde5>;
impl Cde5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde5 {
        match self.bits {
            false => Cde5::Disable,
            true => Cde5::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde5::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde5::Enable
    }
}
#[doc = "Field `CDE5` writer - Channel DMA enable (ch-5)"]
pub type Cde5W<'a, REG> = crate::BitWriter<'a, REG, Cde5>;
impl<'a, REG> Cde5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde5::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde5::Enable)
    }
}
#[doc = "Channel DMA enable (ch-6)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde6 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde6> for bool {
    #[inline(always)]
    fn from(variant: Cde6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE6` reader - Channel DMA enable (ch-6)"]
pub type Cde6R = crate::BitReader<Cde6>;
impl Cde6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde6 {
        match self.bits {
            false => Cde6::Disable,
            true => Cde6::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde6::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde6::Enable
    }
}
#[doc = "Field `CDE6` writer - Channel DMA enable (ch-6)"]
pub type Cde6W<'a, REG> = crate::BitWriter<'a, REG, Cde6>;
impl<'a, REG> Cde6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde6::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde6::Enable)
    }
}
#[doc = "Channel DMA enable (ch-7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde7 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde7> for bool {
    #[inline(always)]
    fn from(variant: Cde7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE7` reader - Channel DMA enable (ch-7)"]
pub type Cde7R = crate::BitReader<Cde7>;
impl Cde7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde7 {
        match self.bits {
            false => Cde7::Disable,
            true => Cde7::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde7::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde7::Enable
    }
}
#[doc = "Field `CDE7` writer - Channel DMA enable (ch-7)"]
pub type Cde7W<'a, REG> = crate::BitWriter<'a, REG, Cde7>;
impl<'a, REG> Cde7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde7::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde7::Enable)
    }
}
#[doc = "Channel DMA enable (ch-0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde8 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde8> for bool {
    #[inline(always)]
    fn from(variant: Cde8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE8` reader - Channel DMA enable (ch-0)"]
pub type Cde8R = crate::BitReader<Cde8>;
impl Cde8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde8 {
        match self.bits {
            false => Cde8::Disable,
            true => Cde8::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde8::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde8::Enable
    }
}
#[doc = "Field `CDE8` writer - Channel DMA enable (ch-0)"]
pub type Cde8W<'a, REG> = crate::BitWriter<'a, REG, Cde8>;
impl<'a, REG> Cde8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde8::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde8::Enable)
    }
}
#[doc = "Channel DMA enable (ch-0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cde9 {
    #[doc = "0: Disable (low priority)"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cde9> for bool {
    #[inline(always)]
    fn from(variant: Cde9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDE9` reader - Channel DMA enable (ch-0)"]
pub type Cde9R = crate::BitReader<Cde9>;
impl Cde9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cde9 {
        match self.bits {
            false => Cde9::Disable,
            true => Cde9::Enable,
        }
    }
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cde9::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cde9::Enable
    }
}
#[doc = "Field `CDE9` writer - Channel DMA enable (ch-0)"]
pub type Cde9W<'a, REG> = crate::BitWriter<'a, REG, Cde9>;
impl<'a, REG> Cde9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (low priority)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde9::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cde9::Enable)
    }
}
#[doc = "Priority control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pce {
    #[doc = "0: CDEn bit disable (regarded as 1)"]
    Disable = 0,
    #[doc = "1: CDEn bit enable"]
    Enable = 1,
}
impl From<Pce> for bool {
    #[inline(always)]
    fn from(variant: Pce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Priority control enable"]
pub type PceR = crate::BitReader<Pce>;
impl PceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pce {
        match self.bits {
            false => Pce::Disable,
            true => Pce::Enable,
        }
    }
    #[doc = "CDEn bit disable (regarded as 1)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pce::Disable
    }
    #[doc = "CDEn bit enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pce::Enable
    }
}
#[doc = "Field `PCE` writer - Priority control enable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG, Pce>;
impl<'a, REG> PceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CDEn bit disable (regarded as 1)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Disable)
    }
    #[doc = "CDEn bit enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - COP control (ch-0)"]
    #[inline(always)]
    pub fn cpc0(&self) -> Cpc0R {
        Cpc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COP control (ch-1)"]
    #[inline(always)]
    pub fn cpc1(&self) -> Cpc1R {
        Cpc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - COP control (ch-2)"]
    #[inline(always)]
    pub fn cpc2(&self) -> Cpc2R {
        Cpc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COP control (ch-3)"]
    #[inline(always)]
    pub fn cpc3(&self) -> Cpc3R {
        Cpc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COP control (ch-4)"]
    #[inline(always)]
    pub fn cpc4(&self) -> Cpc4R {
        Cpc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COP control (ch-5)"]
    #[inline(always)]
    pub fn cpc5(&self) -> Cpc5R {
        Cpc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COP control (ch-6)"]
    #[inline(always)]
    pub fn cpc6(&self) -> Cpc6R {
        Cpc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COP control (ch-7)"]
    #[inline(always)]
    pub fn cpc7(&self) -> Cpc7R {
        Cpc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COP control (ch-8)"]
    #[inline(always)]
    pub fn cpc8(&self) -> Cpc8R {
        Cpc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - COP control (ch-9)"]
    #[inline(always)]
    pub fn cpc9(&self) -> Cpc9R {
        Cpc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel DMA enable (ch-0)"]
    #[inline(always)]
    pub fn cde0(&self) -> Cde0R {
        Cde0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel DMA enable (ch-1)"]
    #[inline(always)]
    pub fn cde1(&self) -> Cde1R {
        Cde1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel DMA enable (ch-2)"]
    #[inline(always)]
    pub fn cde2(&self) -> Cde2R {
        Cde2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel DMA enable (ch-3)"]
    #[inline(always)]
    pub fn cde3(&self) -> Cde3R {
        Cde3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel DMA enable (ch-4)"]
    #[inline(always)]
    pub fn cde4(&self) -> Cde4R {
        Cde4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel DMA enable (ch-5)"]
    #[inline(always)]
    pub fn cde5(&self) -> Cde5R {
        Cde5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel DMA enable (ch-6)"]
    #[inline(always)]
    pub fn cde6(&self) -> Cde6R {
        Cde6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel DMA enable (ch-7)"]
    #[inline(always)]
    pub fn cde7(&self) -> Cde7R {
        Cde7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel DMA enable (ch-0)"]
    #[inline(always)]
    pub fn cde8(&self) -> Cde8R {
        Cde8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel DMA enable (ch-0)"]
    #[inline(always)]
    pub fn cde9(&self) -> Cde9R {
        Cde9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Priority control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COP control (ch-0)"]
    #[inline(always)]
    pub fn cpc0(&mut self) -> Cpc0W<DPcrSpec> {
        Cpc0W::new(self, 0)
    }
    #[doc = "Bit 1 - COP control (ch-1)"]
    #[inline(always)]
    pub fn cpc1(&mut self) -> Cpc1W<DPcrSpec> {
        Cpc1W::new(self, 1)
    }
    #[doc = "Bit 2 - COP control (ch-2)"]
    #[inline(always)]
    pub fn cpc2(&mut self) -> Cpc2W<DPcrSpec> {
        Cpc2W::new(self, 2)
    }
    #[doc = "Bit 3 - COP control (ch-3)"]
    #[inline(always)]
    pub fn cpc3(&mut self) -> Cpc3W<DPcrSpec> {
        Cpc3W::new(self, 3)
    }
    #[doc = "Bit 4 - COP control (ch-4)"]
    #[inline(always)]
    pub fn cpc4(&mut self) -> Cpc4W<DPcrSpec> {
        Cpc4W::new(self, 4)
    }
    #[doc = "Bit 5 - COP control (ch-5)"]
    #[inline(always)]
    pub fn cpc5(&mut self) -> Cpc5W<DPcrSpec> {
        Cpc5W::new(self, 5)
    }
    #[doc = "Bit 6 - COP control (ch-6)"]
    #[inline(always)]
    pub fn cpc6(&mut self) -> Cpc6W<DPcrSpec> {
        Cpc6W::new(self, 6)
    }
    #[doc = "Bit 7 - COP control (ch-7)"]
    #[inline(always)]
    pub fn cpc7(&mut self) -> Cpc7W<DPcrSpec> {
        Cpc7W::new(self, 7)
    }
    #[doc = "Bit 8 - COP control (ch-8)"]
    #[inline(always)]
    pub fn cpc8(&mut self) -> Cpc8W<DPcrSpec> {
        Cpc8W::new(self, 8)
    }
    #[doc = "Bit 9 - COP control (ch-9)"]
    #[inline(always)]
    pub fn cpc9(&mut self) -> Cpc9W<DPcrSpec> {
        Cpc9W::new(self, 9)
    }
    #[doc = "Bit 16 - Channel DMA enable (ch-0)"]
    #[inline(always)]
    pub fn cde0(&mut self) -> Cde0W<DPcrSpec> {
        Cde0W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel DMA enable (ch-1)"]
    #[inline(always)]
    pub fn cde1(&mut self) -> Cde1W<DPcrSpec> {
        Cde1W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel DMA enable (ch-2)"]
    #[inline(always)]
    pub fn cde2(&mut self) -> Cde2W<DPcrSpec> {
        Cde2W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel DMA enable (ch-3)"]
    #[inline(always)]
    pub fn cde3(&mut self) -> Cde3W<DPcrSpec> {
        Cde3W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel DMA enable (ch-4)"]
    #[inline(always)]
    pub fn cde4(&mut self) -> Cde4W<DPcrSpec> {
        Cde4W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel DMA enable (ch-5)"]
    #[inline(always)]
    pub fn cde5(&mut self) -> Cde5W<DPcrSpec> {
        Cde5W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel DMA enable (ch-6)"]
    #[inline(always)]
    pub fn cde6(&mut self) -> Cde6W<DPcrSpec> {
        Cde6W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel DMA enable (ch-7)"]
    #[inline(always)]
    pub fn cde7(&mut self) -> Cde7W<DPcrSpec> {
        Cde7W::new(self, 23)
    }
    #[doc = "Bit 24 - Channel DMA enable (ch-0)"]
    #[inline(always)]
    pub fn cde8(&mut self) -> Cde8W<DPcrSpec> {
        Cde8W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel DMA enable (ch-0)"]
    #[inline(always)]
    pub fn cde9(&mut self) -> Cde9W<DPcrSpec> {
        Cde9W::new(self, 25)
    }
    #[doc = "Bit 31 - Priority control enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<DPcrSpec> {
        PceW::new(self, 31)
    }
}
#[doc = "DMAC priority control\n\nYou can [`read`](crate::Reg::read) this register and get [`d_pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPcrSpec;
impl crate::RegisterSpec for DPcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_pcr::R`](R) reader structure"]
impl crate::Readable for DPcrSpec {}
#[doc = "`write(|w| ..)` method takes [`d_pcr::W`](W) writer structure"]
impl crate::Writable for DPcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_PCR to value 0"]
impl crate::Resettable for DPcrSpec {}
