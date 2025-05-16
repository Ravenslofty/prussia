#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "SIGNAL event control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signal {
    #[doc = "0: No SIGNAL pending."]
    NoSignal = 0,
    #[doc = "1: SIGNAL generated."]
    SignalGenerated = 1,
}
impl From<Signal> for bool {
    #[inline(always)]
    fn from(variant: Signal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGNAL` reader - SIGNAL event control."]
pub type SignalR = crate::BitReader<Signal>;
impl SignalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Signal {
        match self.bits {
            false => Signal::NoSignal,
            true => Signal::SignalGenerated,
        }
    }
    #[doc = "No SIGNAL pending."]
    #[inline(always)]
    pub fn is_no_signal(&self) -> bool {
        *self == Signal::NoSignal
    }
    #[doc = "SIGNAL generated."]
    #[inline(always)]
    pub fn is_signal_generated(&self) -> bool {
        *self == Signal::SignalGenerated
    }
}
#[doc = "SIGNAL event control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalWO {
    #[doc = "0: No action."]
    NoAction = 0,
    #[doc = "1: Clear old event and enable new event."]
    Enable = 1,
}
impl From<SignalWO> for bool {
    #[inline(always)]
    fn from(variant: SignalWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGNAL` writer - SIGNAL event control."]
pub type SignalW<'a, REG> = crate::BitWriter<'a, REG, SignalWO>;
impl<'a, REG> SignalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(SignalWO::NoAction)
    }
    #[doc = "Clear old event and enable new event."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SignalWO::Enable)
    }
}
#[doc = "FINISH event control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Finish {
    #[doc = "0: No FINISH pending."]
    NoFinish = 0,
    #[doc = "1: FINISH generated."]
    FinishGenerated = 1,
}
impl From<Finish> for bool {
    #[inline(always)]
    fn from(variant: Finish) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINISH` reader - FINISH event control."]
pub type FinishR = crate::BitReader<Finish>;
impl FinishR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Finish {
        match self.bits {
            false => Finish::NoFinish,
            true => Finish::FinishGenerated,
        }
    }
    #[doc = "No FINISH pending."]
    #[inline(always)]
    pub fn is_no_finish(&self) -> bool {
        *self == Finish::NoFinish
    }
    #[doc = "FINISH generated."]
    #[inline(always)]
    pub fn is_finish_generated(&self) -> bool {
        *self == Finish::FinishGenerated
    }
}
#[doc = "FINISH event control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FinishWO {
    #[doc = "0: No action."]
    NoAction = 0,
    #[doc = "1: FINISH event is enabled."]
    Enable = 1,
}
impl From<FinishWO> for bool {
    #[inline(always)]
    fn from(variant: FinishWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINISH` writer - FINISH event control."]
pub type FinishW<'a, REG> = crate::BitWriter<'a, REG, FinishWO>;
impl<'a, REG> FinishW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(FinishWO::NoAction)
    }
    #[doc = "FINISH event is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FinishWO::Enable)
    }
}
#[doc = "HSync interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsint {
    #[doc = "0: No Hsync interrupt pending."]
    NoHsync = 0,
    #[doc = "1: Hsync interrupt has been generated."]
    HsyncGenerated = 1,
}
impl From<Hsint> for bool {
    #[inline(always)]
    fn from(variant: Hsint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSINT` reader - HSync interrupt control."]
pub type HsintR = crate::BitReader<Hsint>;
impl HsintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsint {
        match self.bits {
            false => Hsint::NoHsync,
            true => Hsint::HsyncGenerated,
        }
    }
    #[doc = "No Hsync interrupt pending."]
    #[inline(always)]
    pub fn is_no_hsync(&self) -> bool {
        *self == Hsint::NoHsync
    }
    #[doc = "Hsync interrupt has been generated."]
    #[inline(always)]
    pub fn is_hsync_generated(&self) -> bool {
        *self == Hsint::HsyncGenerated
    }
}
#[doc = "HSync interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsintWO {
    #[doc = "0: No action."]
    NoAction = 0,
    #[doc = "1: HSync interrupt is enabled."]
    Enable = 1,
}
impl From<HsintWO> for bool {
    #[inline(always)]
    fn from(variant: HsintWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSINT` writer - HSync interrupt control."]
pub type HsintW<'a, REG> = crate::BitWriter<'a, REG, HsintWO>;
impl<'a, REG> HsintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(HsintWO::NoAction)
    }
    #[doc = "HSync interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsintWO::Enable)
    }
}
#[doc = "VSync interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsint {
    #[doc = "0: No Vsync interrupt pending."]
    NoVsync = 0,
    #[doc = "1: Vsync interrupt has been generated."]
    VsyncGenerated = 1,
}
impl From<Vsint> for bool {
    #[inline(always)]
    fn from(variant: Vsint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSINT` reader - VSync interrupt control."]
pub type VsintR = crate::BitReader<Vsint>;
impl VsintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsint {
        match self.bits {
            false => Vsint::NoVsync,
            true => Vsint::VsyncGenerated,
        }
    }
    #[doc = "No Vsync interrupt pending."]
    #[inline(always)]
    pub fn is_no_vsync(&self) -> bool {
        *self == Vsint::NoVsync
    }
    #[doc = "Vsync interrupt has been generated."]
    #[inline(always)]
    pub fn is_vsync_generated(&self) -> bool {
        *self == Vsint::VsyncGenerated
    }
}
#[doc = "VSync interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VsintWO {
    #[doc = "0: No action."]
    NoAction = 0,
    #[doc = "1: VSync interrupt is enabled."]
    Enable = 1,
}
impl From<VsintWO> for bool {
    #[inline(always)]
    fn from(variant: VsintWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSINT` writer - VSync interrupt control."]
pub type VsintW<'a, REG> = crate::BitWriter<'a, REG, VsintWO>;
impl<'a, REG> VsintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(VsintWO::NoAction)
    }
    #[doc = "VSync interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VsintWO::Enable)
    }
}
#[doc = "Rectangular area write termination interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edwint {
    #[doc = "0: No rectangular area write interrupt pending."]
    NoEdwint = 0,
    #[doc = "1: Rectangular area write interrupt has been generated."]
    RawriteGenerated = 1,
}
impl From<Edwint> for bool {
    #[inline(always)]
    fn from(variant: Edwint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDWINT` reader - Rectangular area write termination interrupt control."]
pub type EdwintR = crate::BitReader<Edwint>;
impl EdwintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edwint {
        match self.bits {
            false => Edwint::NoEdwint,
            true => Edwint::RawriteGenerated,
        }
    }
    #[doc = "No rectangular area write interrupt pending."]
    #[inline(always)]
    pub fn is_no_edwint(&self) -> bool {
        *self == Edwint::NoEdwint
    }
    #[doc = "Rectangular area write interrupt has been generated."]
    #[inline(always)]
    pub fn is_rawrite_generated(&self) -> bool {
        *self == Edwint::RawriteGenerated
    }
}
#[doc = "Rectangular area write termination interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdwintWO {
    #[doc = "0: No action."]
    NoAction = 0,
    #[doc = "1: Rectangular area write interrupt is enabled."]
    Enable = 1,
}
impl From<EdwintWO> for bool {
    #[inline(always)]
    fn from(variant: EdwintWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDWINT` writer - Rectangular area write termination interrupt control."]
pub type EdwintW<'a, REG> = crate::BitWriter<'a, REG, EdwintWO>;
impl<'a, REG> EdwintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(EdwintWO::NoAction)
    }
    #[doc = "Rectangular area write interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EdwintWO::Enable)
    }
}
#[doc = "Field `ZERO` reader - Must always be zero."]
pub type ZeroR = crate::FieldReader;
#[doc = "Field `ZERO` writer - Must always be zero."]
pub type ZeroW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Drawing suspend and FIFO clear (enabled during data write).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: Resume drawing if suspended (?)"]
    Resume = 0,
    #[doc = "1: Flush the GS FIFO and suspend drawing."]
    Flush = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` writer - Drawing suspend and FIFO clear (enabled during data write)."]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG, Flush>;
impl<'a, REG> FlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume drawing if suspended (?)"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Resume)
    }
    #[doc = "Flush the GS FIFO and suspend drawing."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Flush)
    }
}
#[doc = "GS reset..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Do nothing."]
    DoNothing = 0,
    #[doc = "1: GS soft system reset. Clears FIFOs and resets IMR to all 1's."]
    Reset = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - GS reset.."]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::DoNothing,
            true => Reset::Reset,
        }
    }
    #[doc = "Do nothing."]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == Reset::DoNothing
    }
    #[doc = "GS soft system reset. Clears FIFOs and resets IMR to all 1's."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Reset::Reset
    }
}
#[doc = "Field `RESET` writer - GS reset.."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::DoNothing)
    }
    #[doc = "GS soft system reset. Clears FIFOs and resets IMR to all 1's."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Reset)
    }
}
#[doc = "Field `NFIELD` reader - VSync sampled FIELD."]
pub type NfieldR = crate::BitReader;
#[doc = "Field `NFIELD` writer - VSync sampled FIELD."]
pub type NfieldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Current Field of display \\[page-flipping\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    #[doc = "0: Even display buffer."]
    Even = 0,
    #[doc = "1: Odd display buffer."]
    Odd = 1,
}
impl From<Field> for bool {
    #[inline(always)]
    fn from(variant: Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELD` reader - Current Field of display \\[page-flipping\\]."]
pub type FieldR = crate::BitReader<Field>;
impl FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Field {
        match self.bits {
            false => Field::Even,
            true => Field::Odd,
        }
    }
    #[doc = "Even display buffer."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Field::Even
    }
    #[doc = "Odd display buffer."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Field::Odd
    }
}
#[doc = "GS FIFO status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifo {
    #[doc = "0: Not empty but not near-full."]
    Between = 0,
    #[doc = "1: FIFO is empty."]
    Empty = 1,
    #[doc = "2: Almost full."]
    AlmostFull = 2,
    #[doc = "3: Reserved."]
    Unused = 3,
}
impl From<Fifo> for u8 {
    #[inline(always)]
    fn from(variant: Fifo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifo {
    type Ux = u8;
}
impl crate::IsEnum for Fifo {}
#[doc = "Field `FIFO` reader - GS FIFO status."]
pub type FifoR = crate::FieldReader<Fifo>;
impl FifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifo {
        match self.bits {
            0 => Fifo::Between,
            1 => Fifo::Empty,
            2 => Fifo::AlmostFull,
            3 => Fifo::Unused,
            _ => unreachable!(),
        }
    }
    #[doc = "Not empty but not near-full."]
    #[inline(always)]
    pub fn is_between(&self) -> bool {
        *self == Fifo::Between
    }
    #[doc = "FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Fifo::Empty
    }
    #[doc = "Almost full."]
    #[inline(always)]
    pub fn is_almost_full(&self) -> bool {
        *self == Fifo::AlmostFull
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_unused(&self) -> bool {
        *self == Fifo::Unused
    }
}
#[doc = "Field `REV` reader - GS revision number."]
pub type RevR = crate::FieldReader;
#[doc = "Field `ID` reader - GS Id."]
pub type IdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SIGNAL event control."]
    #[inline(always)]
    pub fn signal(&self) -> SignalR {
        SignalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FINISH event control."]
    #[inline(always)]
    pub fn finish(&self) -> FinishR {
        FinishR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSync interrupt control."]
    #[inline(always)]
    pub fn hsint(&self) -> HsintR {
        HsintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSync interrupt control."]
    #[inline(always)]
    pub fn vsint(&self) -> VsintR {
        VsintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rectangular area write termination interrupt control."]
    #[inline(always)]
    pub fn edwint(&self) -> EdwintR {
        EdwintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Must always be zero."]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 9 - GS reset.."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - VSync sampled FIELD."]
    #[inline(always)]
    pub fn nfield(&self) -> NfieldR {
        NfieldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Current Field of display \\[page-flipping\\]."]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - GS FIFO status."]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - GS revision number."]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GS Id."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SIGNAL event control."]
    #[inline(always)]
    pub fn signal(&mut self) -> SignalW<CsrSpec> {
        SignalW::new(self, 0)
    }
    #[doc = "Bit 1 - FINISH event control."]
    #[inline(always)]
    pub fn finish(&mut self) -> FinishW<CsrSpec> {
        FinishW::new(self, 1)
    }
    #[doc = "Bit 2 - HSync interrupt control."]
    #[inline(always)]
    pub fn hsint(&mut self) -> HsintW<CsrSpec> {
        HsintW::new(self, 2)
    }
    #[doc = "Bit 3 - VSync interrupt control."]
    #[inline(always)]
    pub fn vsint(&mut self) -> VsintW<CsrSpec> {
        VsintW::new(self, 3)
    }
    #[doc = "Bit 4 - Rectangular area write termination interrupt control."]
    #[inline(always)]
    pub fn edwint(&mut self) -> EdwintW<CsrSpec> {
        EdwintW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Must always be zero."]
    #[inline(always)]
    pub fn zero(&mut self) -> ZeroW<CsrSpec> {
        ZeroW::new(self, 5)
    }
    #[doc = "Bit 8 - Drawing suspend and FIFO clear (enabled during data write)."]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<CsrSpec> {
        FlushW::new(self, 8)
    }
    #[doc = "Bit 9 - GS reset.."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<CsrSpec> {
        ResetW::new(self, 9)
    }
    #[doc = "Bit 12 - VSync sampled FIELD."]
    #[inline(always)]
    pub fn nfield(&mut self) -> NfieldW<CsrSpec> {
        NfieldW::new(self, 12)
    }
}
#[doc = "GS system status and control registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
