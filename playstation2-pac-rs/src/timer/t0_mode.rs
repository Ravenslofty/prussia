#[doc = "Register `T0_MODE` reader"]
pub type R = crate::R<T0ModeSpec>;
#[doc = "Register `T0_MODE` writer"]
pub type W = crate::W<T0ModeSpec>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clks {
    #[doc = "0: BUSCLK (147.456MHz)"]
    Busclk = 0,
    #[doc = "1: 1/16 of the BUSCLK"]
    Busclk16 = 1,
    #[doc = "2: 1/256 of the BUSCLK"]
    Busclk256 = 2,
    #[doc = "3: External Clock (H-BLNK)"]
    Hblnk = 3,
}
impl From<Clks> for u8 {
    #[inline(always)]
    fn from(variant: Clks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clks {
    type Ux = u8;
}
impl crate::IsEnum for Clks {}
#[doc = "Field `CLKS` reader - Clock Selection"]
pub type ClksR = crate::FieldReader<Clks>;
impl ClksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clks {
        match self.bits {
            0 => Clks::Busclk,
            1 => Clks::Busclk16,
            2 => Clks::Busclk256,
            3 => Clks::Hblnk,
            _ => unreachable!(),
        }
    }
    #[doc = "BUSCLK (147.456MHz)"]
    #[inline(always)]
    pub fn is_busclk(&self) -> bool {
        *self == Clks::Busclk
    }
    #[doc = "1/16 of the BUSCLK"]
    #[inline(always)]
    pub fn is_busclk16(&self) -> bool {
        *self == Clks::Busclk16
    }
    #[doc = "1/256 of the BUSCLK"]
    #[inline(always)]
    pub fn is_busclk256(&self) -> bool {
        *self == Clks::Busclk256
    }
    #[doc = "External Clock (H-BLNK)"]
    #[inline(always)]
    pub fn is_hblnk(&self) -> bool {
        *self == Clks::Hblnk
    }
}
#[doc = "Field `CLKS` writer - Clock Selection"]
pub type ClksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clks, crate::Safe>;
impl<'a, REG> ClksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BUSCLK (147.456MHz)"]
    #[inline(always)]
    pub fn busclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::Busclk)
    }
    #[doc = "1/16 of the BUSCLK"]
    #[inline(always)]
    pub fn busclk16(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::Busclk16)
    }
    #[doc = "1/256 of the BUSCLK"]
    #[inline(always)]
    pub fn busclk256(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::Busclk256)
    }
    #[doc = "External Clock (H-BLNK)"]
    #[inline(always)]
    pub fn hblnk(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::Hblnk)
    }
}
#[doc = "Gate Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    #[doc = "0: Gate function is not used."]
    Disabled = 0,
    #[doc = "1: Gate function is used."]
    Enabled = 1,
}
impl From<Gate> for bool {
    #[inline(always)]
    fn from(variant: Gate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GATE` reader - Gate Function Enable"]
pub type GateR = crate::BitReader<Gate>;
impl GateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gate {
        match self.bits {
            false => Gate::Disabled,
            true => Gate::Enabled,
        }
    }
    #[doc = "Gate function is not used."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gate::Disabled
    }
    #[doc = "Gate function is used."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gate::Enabled
    }
}
#[doc = "Field `GATE` writer - Gate Function Enable"]
pub type GateW<'a, REG> = crate::BitWriter<'a, REG, Gate>;
impl<'a, REG> GateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gate function is not used."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gate::Disabled)
    }
    #[doc = "Gate function is used."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gate::Enabled)
    }
}
#[doc = "Gate Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gats {
    #[doc = "0: H-BLNK (Disabled when CLKS equals to 11.)"]
    Hblnk = 0,
    #[doc = "1: V-BLNK"]
    Vblnk = 1,
}
impl From<Gats> for bool {
    #[inline(always)]
    fn from(variant: Gats) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GATS` reader - Gate Selection"]
pub type GatsR = crate::BitReader<Gats>;
impl GatsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gats {
        match self.bits {
            false => Gats::Hblnk,
            true => Gats::Vblnk,
        }
    }
    #[doc = "H-BLNK (Disabled when CLKS equals to 11.)"]
    #[inline(always)]
    pub fn is_hblnk(&self) -> bool {
        *self == Gats::Hblnk
    }
    #[doc = "V-BLNK"]
    #[inline(always)]
    pub fn is_vblnk(&self) -> bool {
        *self == Gats::Vblnk
    }
}
#[doc = "Field `GATS` writer - Gate Selection"]
pub type GatsW<'a, REG> = crate::BitWriter<'a, REG, Gats>;
impl<'a, REG> GatsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "H-BLNK (Disabled when CLKS equals to 11.)"]
    #[inline(always)]
    pub fn hblnk(self) -> &'a mut crate::W<REG> {
        self.variant(Gats::Hblnk)
    }
    #[doc = "V-BLNK"]
    #[inline(always)]
    pub fn vblnk(self) -> &'a mut crate::W<REG> {
        self.variant(Gats::Vblnk)
    }
}
#[doc = "Gate Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gatm {
    #[doc = "0: Counts while the gate signal is low."]
    Low = 0,
    #[doc = "1: Resets and starts counting at the gate signal's rising edge."]
    Rising = 1,
    #[doc = "2: Resets and starts counting at the gate signal's falling edge."]
    Falling = 2,
    #[doc = "3: Resets and starts counting at both edges of the gate signal."]
    Both = 3,
}
impl From<Gatm> for u8 {
    #[inline(always)]
    fn from(variant: Gatm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gatm {
    type Ux = u8;
}
impl crate::IsEnum for Gatm {}
#[doc = "Field `GATM` reader - Gate Mode"]
pub type GatmR = crate::FieldReader<Gatm>;
impl GatmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gatm {
        match self.bits {
            0 => Gatm::Low,
            1 => Gatm::Rising,
            2 => Gatm::Falling,
            3 => Gatm::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Counts while the gate signal is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Gatm::Low
    }
    #[doc = "Resets and starts counting at the gate signal's rising edge."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Gatm::Rising
    }
    #[doc = "Resets and starts counting at the gate signal's falling edge."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Gatm::Falling
    }
    #[doc = "Resets and starts counting at both edges of the gate signal."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Gatm::Both
    }
}
#[doc = "Field `GATM` writer - Gate Mode"]
pub type GatmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gatm, crate::Safe>;
impl<'a, REG> GatmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts while the gate signal is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Gatm::Low)
    }
    #[doc = "Resets and starts counting at the gate signal's rising edge."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Gatm::Rising)
    }
    #[doc = "Resets and starts counting at the gate signal's falling edge."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Gatm::Falling)
    }
    #[doc = "Resets and starts counting at both edges of the gate signal."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Gatm::Both)
    }
}
#[doc = "Zero Return\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zret {
    #[doc = "0: The counter keeps counting, ignoring the reference value."]
    Disabled = 0,
    #[doc = "1: The counter is cleared to 0 when the counter value is equal to the reference value."]
    Enabled = 1,
}
impl From<Zret> for bool {
    #[inline(always)]
    fn from(variant: Zret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZRET` reader - Zero Return"]
pub type ZretR = crate::BitReader<Zret>;
impl ZretR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zret {
        match self.bits {
            false => Zret::Disabled,
            true => Zret::Enabled,
        }
    }
    #[doc = "The counter keeps counting, ignoring the reference value."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Zret::Disabled
    }
    #[doc = "The counter is cleared to 0 when the counter value is equal to the reference value."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Zret::Enabled
    }
}
#[doc = "Field `ZRET` writer - Zero Return"]
pub type ZretW<'a, REG> = crate::BitWriter<'a, REG, Zret>;
impl<'a, REG> ZretW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter keeps counting, ignoring the reference value."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Zret::Disabled)
    }
    #[doc = "The counter is cleared to 0 when the counter value is equal to the reference value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Zret::Enabled)
    }
}
#[doc = "Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cue {
    #[doc = "0: Stops counting."]
    Disabled = 0,
    #[doc = "1: Starts/restarts counting."]
    Enabled = 1,
}
impl From<Cue> for bool {
    #[inline(always)]
    fn from(variant: Cue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CUE` reader - Count Up Enable"]
pub type CueR = crate::BitReader<Cue>;
impl CueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cue {
        match self.bits {
            false => Cue::Disabled,
            true => Cue::Enabled,
        }
    }
    #[doc = "Stops counting."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cue::Disabled
    }
    #[doc = "Starts/restarts counting."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cue::Enabled
    }
}
#[doc = "Field `CUE` writer - Count Up Enable"]
pub type CueW<'a, REG> = crate::BitWriter<'a, REG, Cue>;
impl<'a, REG> CueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops counting."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cue::Disabled)
    }
    #[doc = "Starts/restarts counting."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cue::Enabled)
    }
}
#[doc = "Compare-Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpe {
    #[doc = "0: A compare-interrupt is not generated."]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when the counter value is equal to the reference value."]
    Enabled = 1,
}
impl From<Cmpe> for bool {
    #[inline(always)]
    fn from(variant: Cmpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPE` reader - Compare-Interrupt Enable"]
pub type CmpeR = crate::BitReader<Cmpe>;
impl CmpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpe {
        match self.bits {
            false => Cmpe::Disabled,
            true => Cmpe::Enabled,
        }
    }
    #[doc = "A compare-interrupt is not generated."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cmpe::Disabled
    }
    #[doc = "An interrupt is generated when the counter value is equal to the reference value."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmpe::Enabled
    }
}
#[doc = "Field `CMPE` writer - Compare-Interrupt Enable"]
pub type CmpeW<'a, REG> = crate::BitWriter<'a, REG, Cmpe>;
impl<'a, REG> CmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A compare-interrupt is not generated."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::Disabled)
    }
    #[doc = "An interrupt is generated when the counter value is equal to the reference value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::Enabled)
    }
}
#[doc = "Overflow-Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfe {
    #[doc = "0: An overflow interrupt is not generated."]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when an overflow occurs."]
    Enabled = 1,
}
impl From<Ovfe> for bool {
    #[inline(always)]
    fn from(variant: Ovfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFE` reader - Overflow-Interrupt Enable"]
pub type OvfeR = crate::BitReader<Ovfe>;
impl OvfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfe {
        match self.bits {
            false => Ovfe::Disabled,
            true => Ovfe::Enabled,
        }
    }
    #[doc = "An overflow interrupt is not generated."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovfe::Disabled
    }
    #[doc = "An interrupt is generated when an overflow occurs."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovfe::Enabled
    }
}
#[doc = "Field `OVFE` writer - Overflow-Interrupt Enable"]
pub type OvfeW<'a, REG> = crate::BitWriter<'a, REG, Ovfe>;
impl<'a, REG> OvfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An overflow interrupt is not generated."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfe::Disabled)
    }
    #[doc = "An interrupt is generated when an overflow occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfe::Enabled)
    }
}
#[doc = "Equal Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Equf {
    #[doc = "0: No compare-interrupt has occurred."]
    Clear = 0,
    #[doc = "1: A compare-interrupt has occurred."]
    Set = 1,
}
impl From<Equf> for bool {
    #[inline(always)]
    fn from(variant: Equf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EQUF` reader - Equal Flag"]
pub type EqufR = crate::BitReader<Equf>;
impl EqufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Equf {
        match self.bits {
            false => Equf::Clear,
            true => Equf::Set,
        }
    }
    #[doc = "No compare-interrupt has occurred."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Equf::Clear
    }
    #[doc = "A compare-interrupt has occurred."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Equf::Set
    }
}
#[doc = "Field `EQUF` writer - Equal Flag"]
pub type EqufW<'a, REG> = crate::BitWriter1C<'a, REG, Equf>;
impl<'a, REG> EqufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare-interrupt has occurred."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Equf::Clear)
    }
    #[doc = "A compare-interrupt has occurred."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Equf::Set)
    }
}
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovff {
    #[doc = "0: No overflow-interrupt has occurred."]
    Clear = 0,
    #[doc = "1: An overflow-interrupt has occurred."]
    Set = 1,
}
impl From<Ovff> for bool {
    #[inline(always)]
    fn from(variant: Ovff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFF` reader - Overflow Flag"]
pub type OvffR = crate::BitReader<Ovff>;
impl OvffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovff {
        match self.bits {
            false => Ovff::Clear,
            true => Ovff::Set,
        }
    }
    #[doc = "No overflow-interrupt has occurred."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ovff::Clear
    }
    #[doc = "An overflow-interrupt has occurred."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ovff::Set
    }
}
#[doc = "Field `OVFF` writer - Overflow Flag"]
pub type OvffW<'a, REG> = crate::BitWriter1C<'a, REG, Ovff>;
impl<'a, REG> OvffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overflow-interrupt has occurred."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ovff::Clear)
    }
    #[doc = "An overflow-interrupt has occurred."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ovff::Set)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Gate Function Enable"]
    #[inline(always)]
    pub fn gate(&self) -> GateR {
        GateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gate Selection"]
    #[inline(always)]
    pub fn gats(&self) -> GatsR {
        GatsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Gate Mode"]
    #[inline(always)]
    pub fn gatm(&self) -> GatmR {
        GatmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Zero Return"]
    #[inline(always)]
    pub fn zret(&self) -> ZretR {
        ZretR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Count Up Enable"]
    #[inline(always)]
    pub fn cue(&self) -> CueR {
        CueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare-Interrupt Enable"]
    #[inline(always)]
    pub fn cmpe(&self) -> CmpeR {
        CmpeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Overflow-Interrupt Enable"]
    #[inline(always)]
    pub fn ovfe(&self) -> OvfeR {
        OvfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Equal Flag"]
    #[inline(always)]
    pub fn equf(&self) -> EqufR {
        EqufR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&self) -> OvffR {
        OvffR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    pub fn clks(&mut self) -> ClksW<T0ModeSpec> {
        ClksW::new(self, 0)
    }
    #[doc = "Bit 2 - Gate Function Enable"]
    #[inline(always)]
    pub fn gate(&mut self) -> GateW<T0ModeSpec> {
        GateW::new(self, 2)
    }
    #[doc = "Bit 3 - Gate Selection"]
    #[inline(always)]
    pub fn gats(&mut self) -> GatsW<T0ModeSpec> {
        GatsW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Gate Mode"]
    #[inline(always)]
    pub fn gatm(&mut self) -> GatmW<T0ModeSpec> {
        GatmW::new(self, 4)
    }
    #[doc = "Bit 6 - Zero Return"]
    #[inline(always)]
    pub fn zret(&mut self) -> ZretW<T0ModeSpec> {
        ZretW::new(self, 6)
    }
    #[doc = "Bit 7 - Count Up Enable"]
    #[inline(always)]
    pub fn cue(&mut self) -> CueW<T0ModeSpec> {
        CueW::new(self, 7)
    }
    #[doc = "Bit 8 - Compare-Interrupt Enable"]
    #[inline(always)]
    pub fn cmpe(&mut self) -> CmpeW<T0ModeSpec> {
        CmpeW::new(self, 8)
    }
    #[doc = "Bit 9 - Overflow-Interrupt Enable"]
    #[inline(always)]
    pub fn ovfe(&mut self) -> OvfeW<T0ModeSpec> {
        OvfeW::new(self, 9)
    }
    #[doc = "Bit 10 - Equal Flag"]
    #[inline(always)]
    pub fn equf(&mut self) -> EqufW<T0ModeSpec> {
        EqufW::new(self, 10)
    }
    #[doc = "Bit 11 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&mut self) -> OvffW<T0ModeSpec> {
        OvffW::new(self, 11)
    }
}
#[doc = "Register for setting modes and reading status\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0ModeSpec;
impl crate::RegisterSpec for T0ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_mode::R`](R) reader structure"]
impl crate::Readable for T0ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`t0_mode::W`](W) writer structure"]
impl crate::Writable for T0ModeSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c00;
}
#[doc = "`reset()` method sets T0_MODE to value 0"]
impl crate::Resettable for T0ModeSpec {}
