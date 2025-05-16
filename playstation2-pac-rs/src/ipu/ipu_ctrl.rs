#[doc = "Register `IPU_CTRL` reader"]
pub type R = crate::R<IpuCtrlSpec>;
#[doc = "Register `IPU_CTRL` writer"]
pub type W = crate::W<IpuCtrlSpec>;
#[doc = "Field `IFC` reader - Input FIFO counter. Same as IPU_BP IFC field."]
pub type IfcR = crate::FieldReader;
#[doc = "Field `OFC` reader - Output FIFO counter."]
pub type OfcR = crate::FieldReader;
#[doc = "Field `CBP` reader - Coded block pattern."]
pub type CbpR = crate::FieldReader;
#[doc = "Error code detection. Set to 0 when a new command is issued, then set to 1 if an error is encountered.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecd {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Ecd> for bool {
    #[inline(always)]
    fn from(variant: Ecd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECD` reader - Error code detection. Set to 0 when a new command is issued, then set to 1 if an error is encountered."]
pub type EcdR = crate::BitReader<Ecd>;
impl EcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecd {
        match self.bits {
            false => Ecd::NotDetected,
            true => Ecd::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Ecd::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Ecd::Detected
    }
}
#[doc = "Start code detection. Set to 0 when a new command is issued, then set to 1 if a start code is encountered.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scd {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Scd> for bool {
    #[inline(always)]
    fn from(variant: Scd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCD` reader - Start code detection. Set to 0 when a new command is issued, then set to 1 if a start code is encountered."]
pub type ScdR = crate::BitReader<Scd>;
impl ScdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scd {
        match self.bits {
            false => Scd::NotDetected,
            true => Scd::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Scd::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Scd::Detected
    }
}
#[doc = "Intra DC presition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idp {
    #[doc = "0: 8 bits."]
    Bits8 = 0,
    #[doc = "1: 9 bits."]
    Bits9 = 1,
    #[doc = "2: 10 bits."]
    Bits10 = 2,
}
impl From<Idp> for u8 {
    #[inline(always)]
    fn from(variant: Idp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idp {
    type Ux = u8;
}
impl crate::IsEnum for Idp {}
#[doc = "Field `IDP` reader - Intra DC presition."]
pub type IdpR = crate::FieldReader<Idp>;
impl IdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idp {
        match self.bits {
            0 => Idp::Bits8,
            1 => Idp::Bits9,
            2 => Idp::Bits10,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bits."]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == Idp::Bits8
    }
    #[doc = "9 bits."]
    #[inline(always)]
    pub fn is_bits9(&self) -> bool {
        *self == Idp::Bits9
    }
    #[doc = "10 bits."]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == Idp::Bits10
    }
}
#[doc = "Field `IDP` writer - Intra DC presition."]
pub type IdpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Idp>;
impl<'a, REG> IdpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits."]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(Idp::Bits8)
    }
    #[doc = "9 bits."]
    #[inline(always)]
    pub fn bits9(self) -> &'a mut crate::W<REG> {
        self.variant(Idp::Bits9)
    }
    #[doc = "10 bits."]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut crate::W<REG> {
        self.variant(Idp::Bits10)
    }
}
#[doc = "Alternate scan.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum As {
    #[doc = "0: Zigzag scanning."]
    Zigzag = 0,
    #[doc = "1: Alternate scanning."]
    Alternate = 1,
}
impl From<As> for bool {
    #[inline(always)]
    fn from(variant: As) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AS` reader - Alternate scan."]
pub type AsR = crate::BitReader<As>;
impl AsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> As {
        match self.bits {
            false => As::Zigzag,
            true => As::Alternate,
        }
    }
    #[doc = "Zigzag scanning."]
    #[inline(always)]
    pub fn is_zigzag(&self) -> bool {
        *self == As::Zigzag
    }
    #[doc = "Alternate scanning."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == As::Alternate
    }
}
#[doc = "Field `AS` writer - Alternate scan."]
pub type AsW<'a, REG> = crate::BitWriter<'a, REG, As>;
impl<'a, REG> AsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Zigzag scanning."]
    #[inline(always)]
    pub fn zigzag(self) -> &'a mut crate::W<REG> {
        self.variant(As::Zigzag)
    }
    #[doc = "Alternate scanning."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(As::Alternate)
    }
}
#[doc = "Intra VLC format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ivf {
    #[doc = "0: MPEG1-compatible 2-dimensional VLC table."]
    Mpeg1Compatible = 0,
    #[doc = "1: 2-dimensional VLC table specially for intra macro block."]
    IntraMacroBlock = 1,
}
impl From<Ivf> for bool {
    #[inline(always)]
    fn from(variant: Ivf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IVF` reader - Intra VLC format."]
pub type IvfR = crate::BitReader<Ivf>;
impl IvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ivf {
        match self.bits {
            false => Ivf::Mpeg1Compatible,
            true => Ivf::IntraMacroBlock,
        }
    }
    #[doc = "MPEG1-compatible 2-dimensional VLC table."]
    #[inline(always)]
    pub fn is_mpeg1_compatible(&self) -> bool {
        *self == Ivf::Mpeg1Compatible
    }
    #[doc = "2-dimensional VLC table specially for intra macro block."]
    #[inline(always)]
    pub fn is_intra_macro_block(&self) -> bool {
        *self == Ivf::IntraMacroBlock
    }
}
#[doc = "Field `IVF` writer - Intra VLC format."]
pub type IvfW<'a, REG> = crate::BitWriter<'a, REG, Ivf>;
impl<'a, REG> IvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPEG1-compatible 2-dimensional VLC table."]
    #[inline(always)]
    pub fn mpeg1_compatible(self) -> &'a mut crate::W<REG> {
        self.variant(Ivf::Mpeg1Compatible)
    }
    #[doc = "2-dimensional VLC table specially for intra macro block."]
    #[inline(always)]
    pub fn intra_macro_block(self) -> &'a mut crate::W<REG> {
        self.variant(Ivf::IntraMacroBlock)
    }
}
#[doc = "Q scale step.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qst {
    #[doc = "0: Linear step."]
    Linear = 0,
    #[doc = "1: Non-linear step."]
    NonLinear = 1,
}
impl From<Qst> for bool {
    #[inline(always)]
    fn from(variant: Qst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QST` reader - Q scale step."]
pub type QstR = crate::BitReader<Qst>;
impl QstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qst {
        match self.bits {
            false => Qst::Linear,
            true => Qst::NonLinear,
        }
    }
    #[doc = "Linear step."]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == Qst::Linear
    }
    #[doc = "Non-linear step."]
    #[inline(always)]
    pub fn is_non_linear(&self) -> bool {
        *self == Qst::NonLinear
    }
}
#[doc = "Field `QST` writer - Q scale step."]
pub type QstW<'a, REG> = crate::BitWriter<'a, REG, Qst>;
impl<'a, REG> QstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Linear step."]
    #[inline(always)]
    pub fn linear(self) -> &'a mut crate::W<REG> {
        self.variant(Qst::Linear)
    }
    #[doc = "Non-linear step."]
    #[inline(always)]
    pub fn non_linear(self) -> &'a mut crate::W<REG> {
        self.variant(Qst::NonLinear)
    }
}
#[doc = "Bit stream MPEG version.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mp1 {
    #[doc = "0: MPEG2 bit stream."]
    Mpeg2 = 0,
    #[doc = "1: MPEG1 bit stream."]
    Mpeg1 = 1,
}
impl From<Mp1> for bool {
    #[inline(always)]
    fn from(variant: Mp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MP1` reader - Bit stream MPEG version."]
pub type Mp1R = crate::BitReader<Mp1>;
impl Mp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mp1 {
        match self.bits {
            false => Mp1::Mpeg2,
            true => Mp1::Mpeg1,
        }
    }
    #[doc = "MPEG2 bit stream."]
    #[inline(always)]
    pub fn is_mpeg2(&self) -> bool {
        *self == Mp1::Mpeg2
    }
    #[doc = "MPEG1 bit stream."]
    #[inline(always)]
    pub fn is_mpeg1(&self) -> bool {
        *self == Mp1::Mpeg1
    }
}
#[doc = "Field `MP1` writer - Bit stream MPEG version."]
pub type Mp1W<'a, REG> = crate::BitWriter<'a, REG, Mp1>;
impl<'a, REG> Mp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPEG2 bit stream."]
    #[inline(always)]
    pub fn mpeg2(self) -> &'a mut crate::W<REG> {
        self.variant(Mp1::Mpeg2)
    }
    #[doc = "MPEG1 bit stream."]
    #[inline(always)]
    pub fn mpeg1(self) -> &'a mut crate::W<REG> {
        self.variant(Mp1::Mpeg1)
    }
}
#[doc = "Picture type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pct {
    #[doc = "1: I-PICTURE."]
    IPicture = 1,
    #[doc = "2: P-PICTURE."]
    PPicture = 2,
    #[doc = "3: B-PICTURE."]
    BPicture = 3,
    #[doc = "4: D-PICTURE."]
    DPicture = 4,
}
impl From<Pct> for u8 {
    #[inline(always)]
    fn from(variant: Pct) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pct {
    type Ux = u8;
}
impl crate::IsEnum for Pct {}
#[doc = "Field `PCT` reader - Picture type."]
pub type PctR = crate::FieldReader<Pct>;
impl PctR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pct> {
        match self.bits {
            1 => Some(Pct::IPicture),
            2 => Some(Pct::PPicture),
            3 => Some(Pct::BPicture),
            4 => Some(Pct::DPicture),
            _ => None,
        }
    }
    #[doc = "I-PICTURE."]
    #[inline(always)]
    pub fn is_i_picture(&self) -> bool {
        *self == Pct::IPicture
    }
    #[doc = "P-PICTURE."]
    #[inline(always)]
    pub fn is_p_picture(&self) -> bool {
        *self == Pct::PPicture
    }
    #[doc = "B-PICTURE."]
    #[inline(always)]
    pub fn is_b_picture(&self) -> bool {
        *self == Pct::BPicture
    }
    #[doc = "D-PICTURE."]
    #[inline(always)]
    pub fn is_d_picture(&self) -> bool {
        *self == Pct::DPicture
    }
}
#[doc = "Field `PCT` writer - Picture type."]
pub type PctW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pct>;
impl<'a, REG> PctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I-PICTURE."]
    #[inline(always)]
    pub fn i_picture(self) -> &'a mut crate::W<REG> {
        self.variant(Pct::IPicture)
    }
    #[doc = "P-PICTURE."]
    #[inline(always)]
    pub fn p_picture(self) -> &'a mut crate::W<REG> {
        self.variant(Pct::PPicture)
    }
    #[doc = "B-PICTURE."]
    #[inline(always)]
    pub fn b_picture(self) -> &'a mut crate::W<REG> {
        self.variant(Pct::BPicture)
    }
    #[doc = "D-PICTURE."]
    #[inline(always)]
    pub fn d_picture(self) -> &'a mut crate::W<REG> {
        self.variant(Pct::DPicture)
    }
}
#[doc = "Reset. Writing 1 will force-abandon the current command and data, and reset the IPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rst {
    #[doc = "1: Trigger Reset."]
    Reset = 1,
}
impl From<Rst> for bool {
    #[inline(always)]
    fn from(variant: Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` writer - Reset. Writing 1 will force-abandon the current command and data, and reset the IPU."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rst>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger Reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Reset)
    }
}
#[doc = "IPU BUSY status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: IPU ready (idle)."]
    Ready = 0,
    #[doc = "1: IPU busy (executing a command.)"]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - IPU BUSY status."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Ready,
            true => Busy::Busy,
        }
    }
    #[doc = "IPU ready (idle)."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Busy::Ready
    }
    #[doc = "IPU busy (executing a command.)"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
impl R {
    #[doc = "Bits 0:3 - Input FIFO counter. Same as IPU_BP IFC field."]
    #[inline(always)]
    pub fn ifc(&self) -> IfcR {
        IfcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Output FIFO counter."]
    #[inline(always)]
    pub fn ofc(&self) -> OfcR {
        OfcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Coded block pattern."]
    #[inline(always)]
    pub fn cbp(&self) -> CbpR {
        CbpR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Error code detection. Set to 0 when a new command is issued, then set to 1 if an error is encountered."]
    #[inline(always)]
    pub fn ecd(&self) -> EcdR {
        EcdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Start code detection. Set to 0 when a new command is issued, then set to 1 if a start code is encountered."]
    #[inline(always)]
    pub fn scd(&self) -> ScdR {
        ScdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Intra DC presition."]
    #[inline(always)]
    pub fn idp(&self) -> IdpR {
        IdpR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Alternate scan."]
    #[inline(always)]
    pub fn as_(&self) -> AsR {
        AsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Intra VLC format."]
    #[inline(always)]
    pub fn ivf(&self) -> IvfR {
        IvfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Q scale step."]
    #[inline(always)]
    pub fn qst(&self) -> QstR {
        QstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bit stream MPEG version."]
    #[inline(always)]
    pub fn mp1(&self) -> Mp1R {
        Mp1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Picture type."]
    #[inline(always)]
    pub fn pct(&self) -> PctR {
        PctR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - IPU BUSY status."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Intra DC presition."]
    #[inline(always)]
    pub fn idp(&mut self) -> IdpW<IpuCtrlSpec> {
        IdpW::new(self, 16)
    }
    #[doc = "Bit 20 - Alternate scan."]
    #[inline(always)]
    pub fn as_(&mut self) -> AsW<IpuCtrlSpec> {
        AsW::new(self, 20)
    }
    #[doc = "Bit 21 - Intra VLC format."]
    #[inline(always)]
    pub fn ivf(&mut self) -> IvfW<IpuCtrlSpec> {
        IvfW::new(self, 21)
    }
    #[doc = "Bit 22 - Q scale step."]
    #[inline(always)]
    pub fn qst(&mut self) -> QstW<IpuCtrlSpec> {
        QstW::new(self, 22)
    }
    #[doc = "Bit 23 - Bit stream MPEG version."]
    #[inline(always)]
    pub fn mp1(&mut self) -> Mp1W<IpuCtrlSpec> {
        Mp1W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Picture type."]
    #[inline(always)]
    pub fn pct(&mut self) -> PctW<IpuCtrlSpec> {
        PctW::new(self, 24)
    }
    #[doc = "Bit 30 - Reset. Writing 1 will force-abandon the current command and data, and reset the IPU."]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<IpuCtrlSpec> {
        RstW::new(self, 30)
    }
}
#[doc = "IPU control\n\nYou can [`read`](crate::Reg::read) this register and get [`ipu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpuCtrlSpec;
impl crate::RegisterSpec for IpuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipu_ctrl::R`](R) reader structure"]
impl crate::Readable for IpuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipu_ctrl::W`](W) writer structure"]
impl crate::Writable for IpuCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPU_CTRL to value 0"]
impl crate::Resettable for IpuCtrlSpec {}
