#[doc = "Register `GIF_STAT` reader"]
pub type R = crate::R<GifStatSpec>;
#[doc = "PATH3 mask status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M3r {
    #[doc = "0: (Initial value) Enabled."]
    Enable = 0,
    #[doc = "1: Disabled (Masked by the MR3 flag of the GIF_MODE register.)"]
    Disable = 1,
}
impl From<M3r> for bool {
    #[inline(always)]
    fn from(variant: M3r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M3R` reader - PATH3 mask status."]
pub type M3rR = crate::BitReader<M3r>;
impl M3rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M3r {
        match self.bits {
            false => M3r::Enable,
            true => M3r::Disable,
        }
    }
    #[doc = "(Initial value) Enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == M3r::Enable
    }
    #[doc = "Disabled (Masked by the MR3 flag of the GIF_MODE register.)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == M3r::Disable
    }
}
#[doc = "PATH3 VIF mask status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M3p {
    #[doc = "0: (Initial value) Enabled."]
    Enable = 0,
    #[doc = "1: Disabled (Masked by the VIF MASKP3 register.)"]
    Disable = 1,
}
impl From<M3p> for bool {
    #[inline(always)]
    fn from(variant: M3p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M3P` reader - PATH3 VIF mask status."]
pub type M3pR = crate::BitReader<M3p>;
impl M3pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M3p {
        match self.bits {
            false => M3p::Enable,
            true => M3p::Disable,
        }
    }
    #[doc = "(Initial value) Enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == M3p::Enable
    }
    #[doc = "Disabled (Masked by the VIF MASKP3 register.)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == M3p::Disable
    }
}
#[doc = "PATH3 IMT status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imt {
    #[doc = "0: (Initial value) Continuous transfer mode."]
    ContinuousTransferMode = 0,
    #[doc = "1: Intermittent transfer mode."]
    IntermittentTransferMode = 1,
}
impl From<Imt> for bool {
    #[inline(always)]
    fn from(variant: Imt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMT` reader - PATH3 IMT status."]
pub type ImtR = crate::BitReader<Imt>;
impl ImtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imt {
        match self.bits {
            false => Imt::ContinuousTransferMode,
            true => Imt::IntermittentTransferMode,
        }
    }
    #[doc = "(Initial value) Continuous transfer mode."]
    #[inline(always)]
    pub fn is_continuous_transfer_mode(&self) -> bool {
        *self == Imt::ContinuousTransferMode
    }
    #[doc = "Intermittent transfer mode."]
    #[inline(always)]
    pub fn is_intermittent_transfer_mode(&self) -> bool {
        *self == Imt::IntermittentTransferMode
    }
}
#[doc = "Temporary transfer stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pse {
    #[doc = "0: (Initial value) Normal."]
    Normal = 0,
    #[doc = "1: Temporary stop state triggered by PSE flag of GIF_CTRL register."]
    TempStop = 1,
}
impl From<Pse> for bool {
    #[inline(always)]
    fn from(variant: Pse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSE` reader - Temporary transfer stop."]
pub type PseR = crate::BitReader<Pse>;
impl PseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pse {
        match self.bits {
            false => Pse::Normal,
            true => Pse::TempStop,
        }
    }
    #[doc = "(Initial value) Normal."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Pse::Normal
    }
    #[doc = "Temporary stop state triggered by PSE flag of GIF_CTRL register."]
    #[inline(always)]
    pub fn is_temp_stop(&self) -> bool {
        *self == Pse::TempStop
    }
}
#[doc = "Interrupted PATH3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ip3 {
    #[doc = "0: (Initial value) No interrupted transfer via PATH3."]
    NoInterrupt = 0,
    #[doc = "1: Interrupted transfer via PATH3."]
    Interrupt = 1,
}
impl From<Ip3> for bool {
    #[inline(always)]
    fn from(variant: Ip3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IP3` reader - Interrupted PATH3."]
pub type Ip3R = crate::BitReader<Ip3>;
impl Ip3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ip3 {
        match self.bits {
            false => Ip3::NoInterrupt,
            true => Ip3::Interrupt,
        }
    }
    #[doc = "(Initial value) No interrupted transfer via PATH3."]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Ip3::NoInterrupt
    }
    #[doc = "Interrupted transfer via PATH3."]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Ip3::Interrupt
    }
}
#[doc = "PATH3 in queue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3q {
    #[doc = "0: (Initial value) No request to wait for processing in PATH3."]
    NoRequest = 0,
    #[doc = "1: Request to wait for processing in PATH3."]
    Request = 1,
}
impl From<P3q> for bool {
    #[inline(always)]
    fn from(variant: P3q) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3Q` reader - PATH3 in queue."]
pub type P3qR = crate::BitReader<P3q>;
impl P3qR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3q {
        match self.bits {
            false => P3q::NoRequest,
            true => P3q::Request,
        }
    }
    #[doc = "(Initial value) No request to wait for processing in PATH3."]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == P3q::NoRequest
    }
    #[doc = "Request to wait for processing in PATH3."]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == P3q::Request
    }
}
#[doc = "PATH2 in queue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2q {
    #[doc = "0: (Initial value) No request to wait for processing in PATH2."]
    NoRequest = 0,
    #[doc = "1: Request to wait for processing in PATH2."]
    Request = 1,
}
impl From<P2q> for bool {
    #[inline(always)]
    fn from(variant: P2q) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2Q` reader - PATH2 in queue."]
pub type P2qR = crate::BitReader<P2q>;
impl P2qR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2q {
        match self.bits {
            false => P2q::NoRequest,
            true => P2q::Request,
        }
    }
    #[doc = "(Initial value) No request to wait for processing in PATH2."]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == P2q::NoRequest
    }
    #[doc = "Request to wait for processing in PATH2."]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == P2q::Request
    }
}
#[doc = "PATH1 in queue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1q {
    #[doc = "0: (Initial value) No request to wait for processing in PATH1."]
    NoRequest = 0,
    #[doc = "1: Request to wait for processing in PATH1."]
    Request = 1,
}
impl From<P1q> for bool {
    #[inline(always)]
    fn from(variant: P1q) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1Q` reader - PATH1 in queue."]
pub type P1qR = crate::BitReader<P1q>;
impl P1qR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1q {
        match self.bits {
            false => P1q::NoRequest,
            true => P1q::Request,
        }
    }
    #[doc = "(Initial value) No request to wait for processing in PATH1."]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == P1q::NoRequest
    }
    #[doc = "Request to wait for processing in PATH1."]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == P1q::Request
    }
}
#[doc = "Output path.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oph {
    #[doc = "0: (Initial value) Idle state."]
    Idle = 0,
    #[doc = "1: Outputting data."]
    Outputting = 1,
}
impl From<Oph> for bool {
    #[inline(always)]
    fn from(variant: Oph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPH` reader - Output path."]
pub type OphR = crate::BitReader<Oph>;
impl OphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oph {
        match self.bits {
            false => Oph::Idle,
            true => Oph::Outputting,
        }
    }
    #[doc = "(Initial value) Idle state."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Oph::Idle
    }
    #[doc = "Outputting data."]
    #[inline(always)]
    pub fn is_outputting(&self) -> bool {
        *self == Oph::Outputting
    }
}
#[doc = "Data path transferring data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apath {
    #[doc = "0: (Initial value) Idle state."]
    Idle = 0,
    #[doc = "1: Transferring data via PATH1."]
    Path1 = 1,
    #[doc = "2: Transferring data via PATH2."]
    Path2 = 2,
    #[doc = "3: Transferring data via PATH3."]
    Path3 = 3,
}
impl From<Apath> for u8 {
    #[inline(always)]
    fn from(variant: Apath) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apath {
    type Ux = u8;
}
impl crate::IsEnum for Apath {}
#[doc = "Field `APATH` reader - Data path transferring data."]
pub type ApathR = crate::FieldReader<Apath>;
impl ApathR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apath {
        match self.bits {
            0 => Apath::Idle,
            1 => Apath::Path1,
            2 => Apath::Path2,
            3 => Apath::Path3,
            _ => unreachable!(),
        }
    }
    #[doc = "(Initial value) Idle state."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Apath::Idle
    }
    #[doc = "Transferring data via PATH1."]
    #[inline(always)]
    pub fn is_path1(&self) -> bool {
        *self == Apath::Path1
    }
    #[doc = "Transferring data via PATH2."]
    #[inline(always)]
    pub fn is_path2(&self) -> bool {
        *self == Apath::Path2
    }
    #[doc = "Transferring data via PATH3."]
    #[inline(always)]
    pub fn is_path3(&self) -> bool {
        *self == Apath::Path3
    }
}
#[doc = "Transfer direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: (Initial value) EE to GS."]
    EeToGs = 0,
    #[doc = "1: GS to EE."]
    GsToEe = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Transfer direction."]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::EeToGs,
            true => Dir::GsToEe,
        }
    }
    #[doc = "(Initial value) EE to GS."]
    #[inline(always)]
    pub fn is_ee_to_gs(&self) -> bool {
        *self == Dir::EeToGs
    }
    #[doc = "GS to EE."]
    #[inline(always)]
    pub fn is_gs_to_ee(&self) -> bool {
        *self == Dir::GsToEe
    }
}
#[doc = "Field `FQC` reader - Effective data count in GIF-FIFO (0-16: in qword units)."]
pub type FqcR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - PATH3 mask status."]
    #[inline(always)]
    pub fn m3r(&self) -> M3rR {
        M3rR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PATH3 VIF mask status."]
    #[inline(always)]
    pub fn m3p(&self) -> M3pR {
        M3pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PATH3 IMT status."]
    #[inline(always)]
    pub fn imt(&self) -> ImtR {
        ImtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Temporary transfer stop."]
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupted PATH3."]
    #[inline(always)]
    pub fn ip3(&self) -> Ip3R {
        Ip3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PATH3 in queue."]
    #[inline(always)]
    pub fn p3q(&self) -> P3qR {
        P3qR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PATH2 in queue."]
    #[inline(always)]
    pub fn p2q(&self) -> P2qR {
        P2qR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PATH1 in queue."]
    #[inline(always)]
    pub fn p1q(&self) -> P1qR {
        P1qR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output path."]
    #[inline(always)]
    pub fn oph(&self) -> OphR {
        OphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Data path transferring data."]
    #[inline(always)]
    pub fn apath(&self) -> ApathR {
        ApathR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Transfer direction."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Effective data count in GIF-FIFO (0-16: in qword units)."]
    #[inline(always)]
    pub fn fqc(&self) -> FqcR {
        FqcR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "GIF status\n\nYou can [`read`](crate::Reg::read) this register and get [`gif_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GifStatSpec;
impl crate::RegisterSpec for GifStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gif_stat::R`](R) reader structure"]
impl crate::Readable for GifStatSpec {}
#[doc = "`reset()` method sets GIF_STAT to value 0"]
impl crate::Resettable for GifStatSpec {}
