#[doc = "Register `SIO_LCR` reader"]
pub type R = crate::R<SioLcrSpec>;
#[doc = "Register `SIO_LCR` writer"]
pub type W = crate::W<SioLcrSpec>;
#[doc = "The length of each data frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Umodelen {
    #[doc = "0: 8-bit data length."]
    Bit8 = 0,
    #[doc = "1: 7-bit data length."]
    Bit7 = 1,
}
impl From<Umodelen> for bool {
    #[inline(always)]
    fn from(variant: Umodelen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UMODELEN` reader - The length of each data frame."]
pub type UmodelenR = crate::BitReader<Umodelen>;
impl UmodelenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Umodelen {
        match self.bits {
            false => Umodelen::Bit8,
            true => Umodelen::Bit7,
        }
    }
    #[doc = "8-bit data length."]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == Umodelen::Bit8
    }
    #[doc = "7-bit data length."]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == Umodelen::Bit7
    }
}
#[doc = "Field `UMODELEN` writer - The length of each data frame."]
pub type UmodelenW<'a, REG> = crate::BitWriter<'a, REG, Umodelen>;
impl<'a, REG> UmodelenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit data length."]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(Umodelen::Bit8)
    }
    #[doc = "7-bit data length."]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(Umodelen::Bit7)
    }
}
#[doc = "Whether to add a TX wake-up bit after data bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Umodemc {
    #[doc = "0: Multi-Controller off."]
    Disable = 0,
    #[doc = "1: Multi-Controller on."]
    Enable = 1,
}
impl From<Umodemc> for bool {
    #[inline(always)]
    fn from(variant: Umodemc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UMODEMC` reader - Whether to add a TX wake-up bit after data bits."]
pub type UmodemcR = crate::BitReader<Umodemc>;
impl UmodemcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Umodemc {
        match self.bits {
            false => Umodemc::Disable,
            true => Umodemc::Enable,
        }
    }
    #[doc = "Multi-Controller off."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Umodemc::Disable
    }
    #[doc = "Multi-Controller on."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Umodemc::Enable
    }
}
#[doc = "Field `UMODEMC` writer - Whether to add a TX wake-up bit after data bits."]
pub type UmodemcW<'a, REG> = crate::BitWriter<'a, REG, Umodemc>;
impl<'a, REG> UmodemcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi-Controller off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Umodemc::Disable)
    }
    #[doc = "Multi-Controller on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Umodemc::Enable)
    }
}
#[doc = "How many stop bits to add.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbl {
    #[doc = "0: One bit."]
    Bit1 = 0,
    #[doc = "1: Two bits."]
    Bit2 = 1,
}
impl From<Usbl> for bool {
    #[inline(always)]
    fn from(variant: Usbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBL` reader - How many stop bits to add."]
pub type UsblR = crate::BitReader<Usbl>;
impl UsblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbl {
        match self.bits {
            false => Usbl::Bit1,
            true => Usbl::Bit2,
        }
    }
    #[doc = "One bit."]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == Usbl::Bit1
    }
    #[doc = "Two bits."]
    #[inline(always)]
    pub fn is_bit2(&self) -> bool {
        *self == Usbl::Bit2
    }
}
#[doc = "Field `USBL` writer - How many stop bits to add."]
pub type UsblW<'a, REG> = crate::BitWriter<'a, REG, Usbl>;
impl<'a, REG> UsblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One bit."]
    #[inline(always)]
    pub fn bit1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbl::Bit1)
    }
    #[doc = "Two bits."]
    #[inline(always)]
    pub fn bit2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbl::Bit2)
    }
}
#[doc = "Enable/disable parity-checking. MUST BE DISABLED when MCENABLE is on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upen {
    #[doc = "0: Parity check is disabled."]
    Disable = 0,
    #[doc = "1: Parity check is enabled."]
    Enable = 1,
}
impl From<Upen> for bool {
    #[inline(always)]
    fn from(variant: Upen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPEN` reader - Enable/disable parity-checking. MUST BE DISABLED when MCENABLE is on."]
pub type UpenR = crate::BitReader<Upen>;
impl UpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upen {
        match self.bits {
            false => Upen::Disable,
            true => Upen::Enable,
        }
    }
    #[doc = "Parity check is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Upen::Disable
    }
    #[doc = "Parity check is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Upen::Enable
    }
}
#[doc = "Field `UPEN` writer - Enable/disable parity-checking. MUST BE DISABLED when MCENABLE is on."]
pub type UpenW<'a, REG> = crate::BitWriter<'a, REG, Upen>;
impl<'a, REG> UpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity check is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Upen::Disable)
    }
    #[doc = "Parity check is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Upen::Enable)
    }
}
#[doc = "Odd or even parity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ueps {
    #[doc = "0: Use odd parity."]
    Odd = 0,
    #[doc = "1: Use even parity."]
    Even = 1,
}
impl From<Ueps> for bool {
    #[inline(always)]
    fn from(variant: Ueps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEPS` reader - Odd or even parity."]
pub type UepsR = crate::BitReader<Ueps>;
impl UepsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ueps {
        match self.bits {
            false => Ueps::Odd,
            true => Ueps::Even,
        }
    }
    #[doc = "Use odd parity."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Ueps::Odd
    }
    #[doc = "Use even parity."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Ueps::Even
    }
}
#[doc = "Field `UEPS` writer - Odd or even parity."]
pub type UepsW<'a, REG> = crate::BitWriter<'a, REG, Ueps>;
impl<'a, REG> UepsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use odd parity."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Ueps::Odd)
    }
    #[doc = "Use even parity."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Ueps::Even)
    }
}
#[doc = "Selects the serial transfer clock. The baud rate is equal to the chosen clock frequency divided by 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scs {
    #[doc = "0: Uses the internal clock (IMBUSCLK)."]
    Internal = 0,
    #[doc = "1: The IMBUSCLK divided by the baud rate generator output."]
    BrgDivInternal = 1,
    #[doc = "2: Uses an external clock (SCLK)."]
    External = 2,
    #[doc = "3: The SCLK divided by the baud rate generator output."]
    BrgDivExternal = 3,
}
impl From<Scs> for u8 {
    #[inline(always)]
    fn from(variant: Scs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scs {
    type Ux = u8;
}
impl crate::IsEnum for Scs {}
#[doc = "Field `SCS` reader - Selects the serial transfer clock. The baud rate is equal to the chosen clock frequency divided by 16."]
pub type ScsR = crate::FieldReader<Scs>;
impl ScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scs {
        match self.bits {
            0 => Scs::Internal,
            1 => Scs::BrgDivInternal,
            2 => Scs::External,
            3 => Scs::BrgDivExternal,
            _ => unreachable!(),
        }
    }
    #[doc = "Uses the internal clock (IMBUSCLK)."]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Scs::Internal
    }
    #[doc = "The IMBUSCLK divided by the baud rate generator output."]
    #[inline(always)]
    pub fn is_brg_div_internal(&self) -> bool {
        *self == Scs::BrgDivInternal
    }
    #[doc = "Uses an external clock (SCLK)."]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Scs::External
    }
    #[doc = "The SCLK divided by the baud rate generator output."]
    #[inline(always)]
    pub fn is_brg_div_external(&self) -> bool {
        *self == Scs::BrgDivExternal
    }
}
#[doc = "Field `SCS` writer - Selects the serial transfer clock. The baud rate is equal to the chosen clock frequency divided by 16."]
pub type ScsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scs, crate::Safe>;
impl<'a, REG> ScsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Uses the internal clock (IMBUSCLK)."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Internal)
    }
    #[doc = "The IMBUSCLK divided by the baud rate generator output."]
    #[inline(always)]
    pub fn brg_div_internal(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::BrgDivInternal)
    }
    #[doc = "Uses an external clock (SCLK)."]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::External)
    }
    #[doc = "The SCLK divided by the baud rate generator output."]
    #[inline(always)]
    pub fn brg_div_external(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::BrgDivExternal)
    }
}
#[doc = "Enables RTC-CTS flow control. When high, allows data to be queued for transmission in TX FIFO if CTS is high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ufce {
    #[doc = "0: Flow control is disabled."]
    Disable = 0,
    #[doc = "1: Flow control is enabled."]
    Enable = 1,
}
impl From<Ufce> for bool {
    #[inline(always)]
    fn from(variant: Ufce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UFCE` reader - Enables RTC-CTS flow control. When high, allows data to be queued for transmission in TX FIFO if CTS is high."]
pub type UfceR = crate::BitReader<Ufce>;
impl UfceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ufce {
        match self.bits {
            false => Ufce::Disable,
            true => Ufce::Enable,
        }
    }
    #[doc = "Flow control is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ufce::Disable
    }
    #[doc = "Flow control is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ufce::Enable
    }
}
#[doc = "Field `UFCE` writer - Enables RTC-CTS flow control. When high, allows data to be queued for transmission in TX FIFO if CTS is high."]
pub type UfceW<'a, REG> = crate::BitWriter<'a, REG, Ufce>;
impl<'a, REG> UfceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flow control is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ufce::Disable)
    }
    #[doc = "Flow control is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ufce::Enable)
    }
}
#[doc = "Controls the output mode of the TXD signal. Expects the slave controller to set the TXD signal to Open Drain when Multi-Controller is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uode {
    #[doc = "0: Totem pole output."]
    TotemPole = 0,
    #[doc = "1: Open drain output."]
    OpenDrain = 1,
}
impl From<Uode> for bool {
    #[inline(always)]
    fn from(variant: Uode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UODE` reader - Controls the output mode of the TXD signal. Expects the slave controller to set the TXD signal to Open Drain when Multi-Controller is enabled."]
pub type UodeR = crate::BitReader<Uode>;
impl UodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uode {
        match self.bits {
            false => Uode::TotemPole,
            true => Uode::OpenDrain,
        }
    }
    #[doc = "Totem pole output."]
    #[inline(always)]
    pub fn is_totem_pole(&self) -> bool {
        *self == Uode::TotemPole
    }
    #[doc = "Open drain output."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Uode::OpenDrain
    }
}
#[doc = "Field `UODE` writer - Controls the output mode of the TXD signal. Expects the slave controller to set the TXD signal to Open Drain when Multi-Controller is enabled."]
pub type UodeW<'a, REG> = crate::BitWriter<'a, REG, Uode>;
impl<'a, REG> UodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Totem pole output."]
    #[inline(always)]
    pub fn totem_pole(self) -> &'a mut crate::W<REG> {
        self.variant(Uode::TotemPole)
    }
    #[doc = "Open drain output."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Uode::OpenDrain)
    }
}
#[doc = "Specifies the Wake Up Bit (WUB) to use when in Multi-Controller System mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twub {
    #[doc = "0: Data frame transfer."]
    DataFrame = 0,
    #[doc = "1: Address (ID) frame transfer."]
    Address = 1,
}
impl From<Twub> for bool {
    #[inline(always)]
    fn from(variant: Twub) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWUB` reader - Specifies the Wake Up Bit (WUB) to use when in Multi-Controller System mode."]
pub type TwubR = crate::BitReader<Twub>;
impl TwubR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Twub {
        match self.bits {
            false => Twub::DataFrame,
            true => Twub::Address,
        }
    }
    #[doc = "Data frame transfer."]
    #[inline(always)]
    pub fn is_data_frame(&self) -> bool {
        *self == Twub::DataFrame
    }
    #[doc = "Address (ID) frame transfer."]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == Twub::Address
    }
}
#[doc = "Field `TWUB` writer - Specifies the Wake Up Bit (WUB) to use when in Multi-Controller System mode."]
pub type TwubW<'a, REG> = crate::BitWriter<'a, REG, Twub>;
impl<'a, REG> TwubW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data frame transfer."]
    #[inline(always)]
    pub fn data_frame(self) -> &'a mut crate::W<REG> {
        self.variant(Twub::DataFrame)
    }
    #[doc = "Address (ID) frame transfer."]
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(Twub::Address)
    }
}
#[doc = "Specifies whether to expect address (ID) frames with Wake Up Bits set to 1 or data frames with Wake Up Bits set to 0. Only used when in Multi-Controller System mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwub {
    #[doc = "0: Receive data frames."]
    DataFrame = 0,
    #[doc = "1: Receive Address (ID) frames."]
    Address = 1,
}
impl From<Rwub> for bool {
    #[inline(always)]
    fn from(variant: Rwub) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWUB` reader - Specifies whether to expect address (ID) frames with Wake Up Bits set to 1 or data frames with Wake Up Bits set to 0. Only used when in Multi-Controller System mode."]
pub type RwubR = crate::BitReader<Rwub>;
impl RwubR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwub {
        match self.bits {
            false => Rwub::DataFrame,
            true => Rwub::Address,
        }
    }
    #[doc = "Receive data frames."]
    #[inline(always)]
    pub fn is_data_frame(&self) -> bool {
        *self == Rwub::DataFrame
    }
    #[doc = "Receive Address (ID) frames."]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == Rwub::Address
    }
}
#[doc = "Field `RWUB` writer - Specifies whether to expect address (ID) frames with Wake Up Bits set to 1 or data frames with Wake Up Bits set to 0. Only used when in Multi-Controller System mode."]
pub type RwubW<'a, REG> = crate::BitWriter<'a, REG, Rwub>;
impl<'a, REG> RwubW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data frames."]
    #[inline(always)]
    pub fn data_frame(self) -> &'a mut crate::W<REG> {
        self.variant(Rwub::DataFrame)
    }
    #[doc = "Receive Address (ID) frames."]
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(Rwub::Address)
    }
}
impl R {
    #[doc = "Bit 0 - The length of each data frame."]
    #[inline(always)]
    pub fn umodelen(&self) -> UmodelenR {
        UmodelenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Whether to add a TX wake-up bit after data bits."]
    #[inline(always)]
    pub fn umodemc(&self) -> UmodemcR {
        UmodemcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - How many stop bits to add."]
    #[inline(always)]
    pub fn usbl(&self) -> UsblR {
        UsblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable/disable parity-checking. MUST BE DISABLED when MCENABLE is on."]
    #[inline(always)]
    pub fn upen(&self) -> UpenR {
        UpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Odd or even parity."]
    #[inline(always)]
    pub fn ueps(&self) -> UepsR {
        UepsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Selects the serial transfer clock. The baud rate is equal to the chosen clock frequency divided by 16."]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Enables RTC-CTS flow control. When high, allows data to be queued for transmission in TX FIFO if CTS is high."]
    #[inline(always)]
    pub fn ufce(&self) -> UfceR {
        UfceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Controls the output mode of the TXD signal. Expects the slave controller to set the TXD signal to Open Drain when Multi-Controller is enabled."]
    #[inline(always)]
    pub fn uode(&self) -> UodeR {
        UodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Specifies the Wake Up Bit (WUB) to use when in Multi-Controller System mode."]
    #[inline(always)]
    pub fn twub(&self) -> TwubR {
        TwubR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Specifies whether to expect address (ID) frames with Wake Up Bits set to 1 or data frames with Wake Up Bits set to 0. Only used when in Multi-Controller System mode."]
    #[inline(always)]
    pub fn rwub(&self) -> RwubR {
        RwubR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The length of each data frame."]
    #[inline(always)]
    pub fn umodelen(&mut self) -> UmodelenW<SioLcrSpec> {
        UmodelenW::new(self, 0)
    }
    #[doc = "Bit 1 - Whether to add a TX wake-up bit after data bits."]
    #[inline(always)]
    pub fn umodemc(&mut self) -> UmodemcW<SioLcrSpec> {
        UmodemcW::new(self, 1)
    }
    #[doc = "Bit 2 - How many stop bits to add."]
    #[inline(always)]
    pub fn usbl(&mut self) -> UsblW<SioLcrSpec> {
        UsblW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable/disable parity-checking. MUST BE DISABLED when MCENABLE is on."]
    #[inline(always)]
    pub fn upen(&mut self) -> UpenW<SioLcrSpec> {
        UpenW::new(self, 3)
    }
    #[doc = "Bit 4 - Odd or even parity."]
    #[inline(always)]
    pub fn ueps(&mut self) -> UepsW<SioLcrSpec> {
        UepsW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Selects the serial transfer clock. The baud rate is equal to the chosen clock frequency divided by 16."]
    #[inline(always)]
    pub fn scs(&mut self) -> ScsW<SioLcrSpec> {
        ScsW::new(self, 5)
    }
    #[doc = "Bit 8 - Enables RTC-CTS flow control. When high, allows data to be queued for transmission in TX FIFO if CTS is high."]
    #[inline(always)]
    pub fn ufce(&mut self) -> UfceW<SioLcrSpec> {
        UfceW::new(self, 8)
    }
    #[doc = "Bit 13 - Controls the output mode of the TXD signal. Expects the slave controller to set the TXD signal to Open Drain when Multi-Controller is enabled."]
    #[inline(always)]
    pub fn uode(&mut self) -> UodeW<SioLcrSpec> {
        UodeW::new(self, 13)
    }
    #[doc = "Bit 14 - Specifies the Wake Up Bit (WUB) to use when in Multi-Controller System mode."]
    #[inline(always)]
    pub fn twub(&mut self) -> TwubW<SioLcrSpec> {
        TwubW::new(self, 14)
    }
    #[doc = "Bit 15 - Specifies whether to expect address (ID) frames with Wake Up Bits set to 1 or data frames with Wake Up Bits set to 0. Only used when in Multi-Controller System mode."]
    #[inline(always)]
    pub fn rwub(&mut self) -> RwubW<SioLcrSpec> {
        RwubW::new(self, 15)
    }
}
#[doc = "Line Control Register. Controls the serial data format. Matches the same register found in the TMPR4937 CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioLcrSpec;
impl crate::RegisterSpec for SioLcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sio_lcr::R`](R) reader structure"]
impl crate::Readable for SioLcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sio_lcr::W`](W) writer structure"]
impl crate::Writable for SioLcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIO_LCR to value 0"]
impl crate::Resettable for SioLcrSpec {}
