#[doc = "Register `I_MASK` reader"]
pub type R = crate::R<IMaskSpec>;
#[doc = "Register `I_MASK` writer"]
pub type W = crate::W<IMaskSpec>;
#[doc = "Detection of an interrupt from GS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gs {
    #[doc = "0: GS interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: GS interrupts are enabled."]
    Enabled = 1,
}
impl From<Gs> for bool {
    #[inline(always)]
    fn from(variant: Gs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GS` reader - Detection of an interrupt from GS"]
pub type GsR = crate::BitReader<Gs>;
impl GsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gs {
        match self.bits {
            false => Gs::Disabled,
            true => Gs::Enabled,
        }
    }
    #[doc = "GS interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gs::Disabled
    }
    #[doc = "GS interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gs::Enabled
    }
}
#[doc = "Field `GS` writer - Detection of an interrupt from GS"]
pub type GsW<'a, REG> = crate::BitWriter<'a, REG, Gs>;
impl<'a, REG> GsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GS interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gs::Disabled)
    }
    #[doc = "GS interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gs::Enabled)
    }
}
#[doc = "Detection of an interrupt from a peripheral on SBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbus {
    #[doc = "0: SBUS interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: SBUS interrupts are enabled."]
    Enabled = 1,
}
impl From<Sbus> for bool {
    #[inline(always)]
    fn from(variant: Sbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBUS` reader - Detection of an interrupt from a peripheral on SBUS"]
pub type SbusR = crate::BitReader<Sbus>;
impl SbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbus {
        match self.bits {
            false => Sbus::Disabled,
            true => Sbus::Enabled,
        }
    }
    #[doc = "SBUS interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sbus::Disabled
    }
    #[doc = "SBUS interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sbus::Enabled
    }
}
#[doc = "Field `SBUS` writer - Detection of an interrupt from a peripheral on SBUS"]
pub type SbusW<'a, REG> = crate::BitWriter<'a, REG, Sbus>;
impl<'a, REG> SbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SBUS interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sbus::Disabled)
    }
    #[doc = "SBUS interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sbus::Enabled)
    }
}
#[doc = "Start of V-Blank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbon {
    #[doc = "0: Start of V-Blank interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: Start of V-Blank interrupts are enabled."]
    Enabled = 1,
}
impl From<Vbon> for bool {
    #[inline(always)]
    fn from(variant: Vbon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBON` reader - Start of V-Blank"]
pub type VbonR = crate::BitReader<Vbon>;
impl VbonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbon {
        match self.bits {
            false => Vbon::Disabled,
            true => Vbon::Enabled,
        }
    }
    #[doc = "Start of V-Blank interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vbon::Disabled
    }
    #[doc = "Start of V-Blank interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vbon::Enabled
    }
}
#[doc = "Field `VBON` writer - Start of V-Blank"]
pub type VbonW<'a, REG> = crate::BitWriter<'a, REG, Vbon>;
impl<'a, REG> VbonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start of V-Blank interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vbon::Disabled)
    }
    #[doc = "Start of V-Blank interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vbon::Enabled)
    }
}
#[doc = "End of V-Blank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbof {
    #[doc = "0: End of V-Blank interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: End of V-Blank interrupts are enabled."]
    Enabled = 1,
}
impl From<Vbof> for bool {
    #[inline(always)]
    fn from(variant: Vbof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBOF` reader - End of V-Blank"]
pub type VbofR = crate::BitReader<Vbof>;
impl VbofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbof {
        match self.bits {
            false => Vbof::Disabled,
            true => Vbof::Enabled,
        }
    }
    #[doc = "End of V-Blank interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vbof::Disabled
    }
    #[doc = "End of V-Blank interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vbof::Enabled
    }
}
#[doc = "Field `VBOF` writer - End of V-Blank"]
pub type VbofW<'a, REG> = crate::BitWriter<'a, REG, Vbof>;
impl<'a, REG> VbofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of V-Blank interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vbof::Disabled)
    }
    #[doc = "End of V-Blank interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vbof::Enabled)
    }
}
#[doc = "VIF0's detection of VIFcode with an interrupt bit or an exception, or VIF0 stalls with occurrence of an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vif0 {
    #[doc = "0: VIF0 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: VIF0 interrupts are enabled."]
    Enabled = 1,
}
impl From<Vif0> for bool {
    #[inline(always)]
    fn from(variant: Vif0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIF0` reader - VIF0's detection of VIFcode with an interrupt bit or an exception, or VIF0 stalls with occurrence of an interrupt."]
pub type Vif0R = crate::BitReader<Vif0>;
impl Vif0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vif0 {
        match self.bits {
            false => Vif0::Disabled,
            true => Vif0::Enabled,
        }
    }
    #[doc = "VIF0 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vif0::Disabled
    }
    #[doc = "VIF0 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vif0::Enabled
    }
}
#[doc = "Field `VIF0` writer - VIF0's detection of VIFcode with an interrupt bit or an exception, or VIF0 stalls with occurrence of an interrupt."]
pub type Vif0W<'a, REG> = crate::BitWriter<'a, REG, Vif0>;
impl<'a, REG> Vif0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VIF0 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vif0::Disabled)
    }
    #[doc = "VIF0 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vif0::Enabled)
    }
}
#[doc = "VIF1's detection of VIFcode with an interrupt bit or an exception, or VIF1 stalls with occurrence of an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vif1 {
    #[doc = "0: VIF1 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: VIF1 interrupts are enabled."]
    Enabled = 1,
}
impl From<Vif1> for bool {
    #[inline(always)]
    fn from(variant: Vif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIF1` reader - VIF1's detection of VIFcode with an interrupt bit or an exception, or VIF1 stalls with occurrence of an interrupt."]
pub type Vif1R = crate::BitReader<Vif1>;
impl Vif1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vif1 {
        match self.bits {
            false => Vif1::Disabled,
            true => Vif1::Enabled,
        }
    }
    #[doc = "VIF1 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vif1::Disabled
    }
    #[doc = "VIF1 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vif1::Enabled
    }
}
#[doc = "Field `VIF1` writer - VIF1's detection of VIFcode with an interrupt bit or an exception, or VIF1 stalls with occurrence of an interrupt."]
pub type Vif1W<'a, REG> = crate::BitWriter<'a, REG, Vif1>;
impl<'a, REG> Vif1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VIF1 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vif1::Disabled)
    }
    #[doc = "VIF1 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vif1::Enabled)
    }
}
#[doc = "VU0's execution of a microinstruction with an interrupt bit, or VU0 stalls with occurrence of an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vu0 {
    #[doc = "0: VU0 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: VU0 interrupts are enabled."]
    Enabled = 1,
}
impl From<Vu0> for bool {
    #[inline(always)]
    fn from(variant: Vu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VU0` reader - VU0's execution of a microinstruction with an interrupt bit, or VU0 stalls with occurrence of an interrupt."]
pub type Vu0R = crate::BitReader<Vu0>;
impl Vu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vu0 {
        match self.bits {
            false => Vu0::Disabled,
            true => Vu0::Enabled,
        }
    }
    #[doc = "VU0 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vu0::Disabled
    }
    #[doc = "VU0 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vu0::Enabled
    }
}
#[doc = "Field `VU0` writer - VU0's execution of a microinstruction with an interrupt bit, or VU0 stalls with occurrence of an interrupt."]
pub type Vu0W<'a, REG> = crate::BitWriter<'a, REG, Vu0>;
impl<'a, REG> Vu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VU0 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vu0::Disabled)
    }
    #[doc = "VU0 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vu0::Enabled)
    }
}
#[doc = "VU1's execution of a microinstruction with an interrupt bit, or VU1 stalls with occurrence of an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vu1 {
    #[doc = "0: VU1 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: VU1 interrupts are enabled."]
    Enabled = 1,
}
impl From<Vu1> for bool {
    #[inline(always)]
    fn from(variant: Vu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VU1` reader - VU1's execution of a microinstruction with an interrupt bit, or VU1 stalls with occurrence of an interrupt."]
pub type Vu1R = crate::BitReader<Vu1>;
impl Vu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vu1 {
        match self.bits {
            false => Vu1::Disabled,
            true => Vu1::Enabled,
        }
    }
    #[doc = "VU1 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vu1::Disabled
    }
    #[doc = "VU1 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vu1::Enabled
    }
}
#[doc = "Field `VU1` writer - VU1's execution of a microinstruction with an interrupt bit, or VU1 stalls with occurrence of an interrupt."]
pub type Vu1W<'a, REG> = crate::BitWriter<'a, REG, Vu1>;
impl<'a, REG> Vu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VU1 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vu1::Disabled)
    }
    #[doc = "VU1 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vu1::Enabled)
    }
}
#[doc = "IPU's detection of the end of data or an exception IPU stalls with occurrence of an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipu {
    #[doc = "0: IPU interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: IPU interrupts are enabled."]
    Enabled = 1,
}
impl From<Ipu> for bool {
    #[inline(always)]
    fn from(variant: Ipu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPU` reader - IPU's detection of the end of data or an exception IPU stalls with occurrence of an interrupt."]
pub type IpuR = crate::BitReader<Ipu>;
impl IpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipu {
        match self.bits {
            false => Ipu::Disabled,
            true => Ipu::Enabled,
        }
    }
    #[doc = "IPU interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ipu::Disabled
    }
    #[doc = "IPU interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ipu::Enabled
    }
}
#[doc = "Field `IPU` writer - IPU's detection of the end of data or an exception IPU stalls with occurrence of an interrupt."]
pub type IpuW<'a, REG> = crate::BitWriter<'a, REG, Ipu>;
impl<'a, REG> IpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IPU interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ipu::Disabled)
    }
    #[doc = "IPU interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ipu::Enabled)
    }
}
#[doc = "Conditions met in timer 0 settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim0 {
    #[doc = "0: TIMER0 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: TIMER0 interrupts are enabled."]
    Enabled = 1,
}
impl From<Tim0> for bool {
    #[inline(always)]
    fn from(variant: Tim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM0` reader - Conditions met in timer 0 settings"]
pub type Tim0R = crate::BitReader<Tim0>;
impl Tim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim0 {
        match self.bits {
            false => Tim0::Disabled,
            true => Tim0::Enabled,
        }
    }
    #[doc = "TIMER0 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tim0::Disabled
    }
    #[doc = "TIMER0 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tim0::Enabled
    }
}
#[doc = "Field `TIM0` writer - Conditions met in timer 0 settings"]
pub type Tim0W<'a, REG> = crate::BitWriter<'a, REG, Tim0>;
impl<'a, REG> Tim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMER0 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim0::Disabled)
    }
    #[doc = "TIMER0 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim0::Enabled)
    }
}
#[doc = "Conditions met in timer 1 settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1 {
    #[doc = "0: TIMER1 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: TIMER1 interrupts are enabled."]
    Enabled = 1,
}
impl From<Tim1> for bool {
    #[inline(always)]
    fn from(variant: Tim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1` reader - Conditions met in timer 1 settings"]
pub type Tim1R = crate::BitReader<Tim1>;
impl Tim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1 {
        match self.bits {
            false => Tim1::Disabled,
            true => Tim1::Enabled,
        }
    }
    #[doc = "TIMER1 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tim1::Disabled
    }
    #[doc = "TIMER1 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tim1::Enabled
    }
}
#[doc = "Field `TIM1` writer - Conditions met in timer 1 settings"]
pub type Tim1W<'a, REG> = crate::BitWriter<'a, REG, Tim1>;
impl<'a, REG> Tim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMER1 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1::Disabled)
    }
    #[doc = "TIMER1 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1::Enabled)
    }
}
#[doc = "Conditions met in timer 2 settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim2 {
    #[doc = "0: TIMER2 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: TIMER2 interrupts are enabled."]
    Enabled = 1,
}
impl From<Tim2> for bool {
    #[inline(always)]
    fn from(variant: Tim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2` reader - Conditions met in timer 2 settings"]
pub type Tim2R = crate::BitReader<Tim2>;
impl Tim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim2 {
        match self.bits {
            false => Tim2::Disabled,
            true => Tim2::Enabled,
        }
    }
    #[doc = "TIMER2 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tim2::Disabled
    }
    #[doc = "TIMER2 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tim2::Enabled
    }
}
#[doc = "Field `TIM2` writer - Conditions met in timer 2 settings"]
pub type Tim2W<'a, REG> = crate::BitWriter<'a, REG, Tim2>;
impl<'a, REG> Tim2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMER2 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2::Disabled)
    }
    #[doc = "TIMER2 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2::Enabled)
    }
}
#[doc = "Conditions met in timer 3 settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim3 {
    #[doc = "0: TIMER3 interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: TIMER3 interrupts are enabled."]
    Enabled = 1,
}
impl From<Tim3> for bool {
    #[inline(always)]
    fn from(variant: Tim3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3` reader - Conditions met in timer 3 settings"]
pub type Tim3R = crate::BitReader<Tim3>;
impl Tim3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim3 {
        match self.bits {
            false => Tim3::Disabled,
            true => Tim3::Enabled,
        }
    }
    #[doc = "TIMER3 interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tim3::Disabled
    }
    #[doc = "TIMER3 interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tim3::Enabled
    }
}
#[doc = "Field `TIM3` writer - Conditions met in timer 3 settings"]
pub type Tim3W<'a, REG> = crate::BitWriter<'a, REG, Tim3>;
impl<'a, REG> Tim3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMER3 interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3::Disabled)
    }
    #[doc = "TIMER3 interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3::Enabled)
    }
}
#[doc = "Error detection during SFIFO transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfifo {
    #[doc = "0: SFIFO interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: SFIFO interrupts are enabled."]
    Enabled = 1,
}
impl From<Sfifo> for bool {
    #[inline(always)]
    fn from(variant: Sfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFIFO` reader - Error detection during SFIFO transfer"]
pub type SfifoR = crate::BitReader<Sfifo>;
impl SfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sfifo {
        match self.bits {
            false => Sfifo::Disabled,
            true => Sfifo::Enabled,
        }
    }
    #[doc = "SFIFO interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sfifo::Disabled
    }
    #[doc = "SFIFO interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sfifo::Enabled
    }
}
#[doc = "Field `SFIFO` writer - Error detection during SFIFO transfer"]
pub type SfifoW<'a, REG> = crate::BitWriter<'a, REG, Sfifo>;
impl<'a, REG> SfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SFIFO interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sfifo::Disabled)
    }
    #[doc = "SFIFO interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sfifo::Enabled)
    }
}
#[doc = "VU0 in RUN status for a long time continuously; ForceBreak is sent to VU0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vu0wd {
    #[doc = "0: VU0 Watchdog interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: VU0 Watchdog interrupts are enabled."]
    Enabled = 1,
}
impl From<Vu0wd> for bool {
    #[inline(always)]
    fn from(variant: Vu0wd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VU0WD` reader - VU0 in RUN status for a long time continuously; ForceBreak is sent to VU0."]
pub type Vu0wdR = crate::BitReader<Vu0wd>;
impl Vu0wdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vu0wd {
        match self.bits {
            false => Vu0wd::Disabled,
            true => Vu0wd::Enabled,
        }
    }
    #[doc = "VU0 Watchdog interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vu0wd::Disabled
    }
    #[doc = "VU0 Watchdog interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vu0wd::Enabled
    }
}
#[doc = "Field `VU0WD` writer - VU0 in RUN status for a long time continuously; ForceBreak is sent to VU0."]
pub type Vu0wdW<'a, REG> = crate::BitWriter<'a, REG, Vu0wd>;
impl<'a, REG> Vu0wdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VU0 Watchdog interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vu0wd::Disabled)
    }
    #[doc = "VU0 Watchdog interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vu0wd::Enabled)
    }
}
#[doc = "DMA channel from IOP for emulating the PS1 GPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgif {
    #[doc = "0: PGIF interrupts are disabled."]
    Disabled = 0,
    #[doc = "1: PGIF interrupts are enabled."]
    Enabled = 1,
}
impl From<Pgif> for bool {
    #[inline(always)]
    fn from(variant: Pgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGIF` reader - DMA channel from IOP for emulating the PS1 GPU"]
pub type PgifR = crate::BitReader<Pgif>;
impl PgifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgif {
        match self.bits {
            false => Pgif::Disabled,
            true => Pgif::Enabled,
        }
    }
    #[doc = "PGIF interrupts are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pgif::Disabled
    }
    #[doc = "PGIF interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pgif::Enabled
    }
}
#[doc = "Field `PGIF` writer - DMA channel from IOP for emulating the PS1 GPU"]
pub type PgifW<'a, REG> = crate::BitWriter<'a, REG, Pgif>;
impl<'a, REG> PgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PGIF interrupts are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pgif::Disabled)
    }
    #[doc = "PGIF interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pgif::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Detection of an interrupt from GS"]
    #[inline(always)]
    pub fn gs(&self) -> GsR {
        GsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Detection of an interrupt from a peripheral on SBUS"]
    #[inline(always)]
    pub fn sbus(&self) -> SbusR {
        SbusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start of V-Blank"]
    #[inline(always)]
    pub fn vbon(&self) -> VbonR {
        VbonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of V-Blank"]
    #[inline(always)]
    pub fn vbof(&self) -> VbofR {
        VbofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VIF0's detection of VIFcode with an interrupt bit or an exception, or VIF0 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vif0(&self) -> Vif0R {
        Vif0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VIF1's detection of VIFcode with an interrupt bit or an exception, or VIF1 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vif1(&self) -> Vif1R {
        Vif1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VU0's execution of a microinstruction with an interrupt bit, or VU0 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vu0(&self) -> Vu0R {
        Vu0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VU1's execution of a microinstruction with an interrupt bit, or VU1 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vu1(&self) -> Vu1R {
        Vu1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IPU's detection of the end of data or an exception IPU stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn ipu(&self) -> IpuR {
        IpuR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Conditions met in timer 0 settings"]
    #[inline(always)]
    pub fn tim0(&self) -> Tim0R {
        Tim0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Conditions met in timer 1 settings"]
    #[inline(always)]
    pub fn tim1(&self) -> Tim1R {
        Tim1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Conditions met in timer 2 settings"]
    #[inline(always)]
    pub fn tim2(&self) -> Tim2R {
        Tim2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Conditions met in timer 3 settings"]
    #[inline(always)]
    pub fn tim3(&self) -> Tim3R {
        Tim3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error detection during SFIFO transfer"]
    #[inline(always)]
    pub fn sfifo(&self) -> SfifoR {
        SfifoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VU0 in RUN status for a long time continuously; ForceBreak is sent to VU0."]
    #[inline(always)]
    pub fn vu0wd(&self) -> Vu0wdR {
        Vu0wdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA channel from IOP for emulating the PS1 GPU"]
    #[inline(always)]
    pub fn pgif(&self) -> PgifR {
        PgifR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Detection of an interrupt from GS"]
    #[inline(always)]
    pub fn gs(&mut self) -> GsW<IMaskSpec> {
        GsW::new(self, 0)
    }
    #[doc = "Bit 1 - Detection of an interrupt from a peripheral on SBUS"]
    #[inline(always)]
    pub fn sbus(&mut self) -> SbusW<IMaskSpec> {
        SbusW::new(self, 1)
    }
    #[doc = "Bit 2 - Start of V-Blank"]
    #[inline(always)]
    pub fn vbon(&mut self) -> VbonW<IMaskSpec> {
        VbonW::new(self, 2)
    }
    #[doc = "Bit 3 - End of V-Blank"]
    #[inline(always)]
    pub fn vbof(&mut self) -> VbofW<IMaskSpec> {
        VbofW::new(self, 3)
    }
    #[doc = "Bit 4 - VIF0's detection of VIFcode with an interrupt bit or an exception, or VIF0 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vif0(&mut self) -> Vif0W<IMaskSpec> {
        Vif0W::new(self, 4)
    }
    #[doc = "Bit 5 - VIF1's detection of VIFcode with an interrupt bit or an exception, or VIF1 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vif1(&mut self) -> Vif1W<IMaskSpec> {
        Vif1W::new(self, 5)
    }
    #[doc = "Bit 6 - VU0's execution of a microinstruction with an interrupt bit, or VU0 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vu0(&mut self) -> Vu0W<IMaskSpec> {
        Vu0W::new(self, 6)
    }
    #[doc = "Bit 7 - VU1's execution of a microinstruction with an interrupt bit, or VU1 stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn vu1(&mut self) -> Vu1W<IMaskSpec> {
        Vu1W::new(self, 7)
    }
    #[doc = "Bit 8 - IPU's detection of the end of data or an exception IPU stalls with occurrence of an interrupt."]
    #[inline(always)]
    pub fn ipu(&mut self) -> IpuW<IMaskSpec> {
        IpuW::new(self, 8)
    }
    #[doc = "Bit 9 - Conditions met in timer 0 settings"]
    #[inline(always)]
    pub fn tim0(&mut self) -> Tim0W<IMaskSpec> {
        Tim0W::new(self, 9)
    }
    #[doc = "Bit 10 - Conditions met in timer 1 settings"]
    #[inline(always)]
    pub fn tim1(&mut self) -> Tim1W<IMaskSpec> {
        Tim1W::new(self, 10)
    }
    #[doc = "Bit 11 - Conditions met in timer 2 settings"]
    #[inline(always)]
    pub fn tim2(&mut self) -> Tim2W<IMaskSpec> {
        Tim2W::new(self, 11)
    }
    #[doc = "Bit 12 - Conditions met in timer 3 settings"]
    #[inline(always)]
    pub fn tim3(&mut self) -> Tim3W<IMaskSpec> {
        Tim3W::new(self, 12)
    }
    #[doc = "Bit 13 - Error detection during SFIFO transfer"]
    #[inline(always)]
    pub fn sfifo(&mut self) -> SfifoW<IMaskSpec> {
        SfifoW::new(self, 13)
    }
    #[doc = "Bit 14 - VU0 in RUN status for a long time continuously; ForceBreak is sent to VU0."]
    #[inline(always)]
    pub fn vu0wd(&mut self) -> Vu0wdW<IMaskSpec> {
        Vu0wdW::new(self, 14)
    }
    #[doc = "Bit 15 - DMA channel from IOP for emulating the PS1 GPU"]
    #[inline(always)]
    pub fn pgif(&mut self) -> PgifW<IMaskSpec> {
        PgifW::new(self, 15)
    }
}
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`i_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMaskSpec;
impl crate::RegisterSpec for IMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i_mask::R`](R) reader structure"]
impl crate::Readable for IMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`i_mask::W`](W) writer structure"]
impl crate::Writable for IMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I_MASK to value 0"]
impl crate::Resettable for IMaskSpec {}
