#[doc = "Register `VIF1_STAT` reader"]
pub type R = crate::R<Vif1StatSpec>;
#[doc = "Register `VIF1_STAT` writer"]
pub type W = crate::W<Vif1StatSpec>;
#[doc = "VIF Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vps {
    #[doc = "0: Idle"]
    Idle = 0,
    #[doc = "1: Waits for data following the VIFcode"]
    Waiting = 1,
    #[doc = "2: Decoding the VIFcode"]
    Decoding = 2,
    #[doc = "3: Decompressing/Transferring the data following the VIFcode"]
    Processing = 3,
}
impl From<Vps> for u8 {
    #[inline(always)]
    fn from(variant: Vps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vps {
    type Ux = u8;
}
impl crate::IsEnum for Vps {}
#[doc = "Field `VPS` reader - VIF Status"]
pub type VpsR = crate::FieldReader<Vps>;
impl VpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vps {
        match self.bits {
            0 => Vps::Idle,
            1 => Vps::Waiting,
            2 => Vps::Decoding,
            3 => Vps::Processing,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Vps::Idle
    }
    #[doc = "Waits for data following the VIFcode"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == Vps::Waiting
    }
    #[doc = "Decoding the VIFcode"]
    #[inline(always)]
    pub fn is_decoding(&self) -> bool {
        *self == Vps::Decoding
    }
    #[doc = "Decompressing/Transferring the data following the VIFcode"]
    #[inline(always)]
    pub fn is_processing(&self) -> bool {
        *self == Vps::Processing
    }
}
#[doc = "VIF E-bit Wait\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vew {
    #[doc = "0: Not-wait"]
    Notwait = 0,
    #[doc = "1: Wait (VU is executing a microprogram)"]
    Wait = 1,
}
impl From<Vew> for bool {
    #[inline(always)]
    fn from(variant: Vew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEW` reader - VIF E-bit Wait"]
pub type VewR = crate::BitReader<Vew>;
impl VewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vew {
        match self.bits {
            false => Vew::Notwait,
            true => Vew::Wait,
        }
    }
    #[doc = "Not-wait"]
    #[inline(always)]
    pub fn is_notwait(&self) -> bool {
        *self == Vew::Notwait
    }
    #[doc = "Wait (VU is executing a microprogram)"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Vew::Wait
    }
}
#[doc = "Status waiting for end of GIF transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vgw {
    #[doc = "0: Not-wait"]
    Notwait = 0,
    #[doc = "1: Wait (Stopped status with one of FLUSH/FLUSHA, DIRECT/DIRECTHL and MSCALF commands when GIF != idle.)"]
    Wait = 1,
}
impl From<Vgw> for bool {
    #[inline(always)]
    fn from(variant: Vgw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VGW` reader - Status waiting for end of GIF transfer"]
pub type VgwR = crate::BitReader<Vgw>;
impl VgwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vgw {
        match self.bits {
            false => Vgw::Notwait,
            true => Vgw::Wait,
        }
    }
    #[doc = "Not-wait"]
    #[inline(always)]
    pub fn is_notwait(&self) -> bool {
        *self == Vgw::Notwait
    }
    #[doc = "Wait (Stopped status with one of FLUSH/FLUSHA, DIRECT/DIRECTHL and MSCALF commands when GIF != idle.)"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Vgw::Wait
    }
}
#[doc = "VIF MARK detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrk {
    #[doc = "0: Not-detect"]
    Notdetect = 0,
    #[doc = "1: Detect"]
    Detect = 1,
}
impl From<Mrk> for bool {
    #[inline(always)]
    fn from(variant: Mrk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRK` reader - VIF MARK detect"]
pub type MrkR = crate::BitReader<Mrk>;
impl MrkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrk {
        match self.bits {
            false => Mrk::Notdetect,
            true => Mrk::Detect,
        }
    }
    #[doc = "Not-detect"]
    #[inline(always)]
    pub fn is_notdetect(&self) -> bool {
        *self == Mrk::Notdetect
    }
    #[doc = "Detect"]
    #[inline(always)]
    pub fn is_detect(&self) -> bool {
        *self == Mrk::Detect
    }
}
#[doc = "Double buffer flag. Cleared to 0 by OFFSET and reversed by MSCAL/MSCALF/MSCNT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbf {
    #[doc = "0: TOPS=BASE"]
    Base = 0,
    #[doc = "1: TOPS=BASE+OFFSET"]
    Baseplusoffset = 1,
}
impl From<Dbf> for bool {
    #[inline(always)]
    fn from(variant: Dbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBF` reader - Double buffer flag. Cleared to 0 by OFFSET and reversed by MSCAL/MSCALF/MSCNT."]
pub type DbfR = crate::BitReader<Dbf>;
impl DbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbf {
        match self.bits {
            false => Dbf::Base,
            true => Dbf::Baseplusoffset,
        }
    }
    #[doc = "TOPS=BASE"]
    #[inline(always)]
    pub fn is_base(&self) -> bool {
        *self == Dbf::Base
    }
    #[doc = "TOPS=BASE+OFFSET"]
    #[inline(always)]
    pub fn is_baseplusoffset(&self) -> bool {
        *self == Dbf::Baseplusoffset
    }
}
#[doc = "Stop by STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vss {
    #[doc = "0: Not-stall"]
    Notstall = 0,
    #[doc = "1: Stall"]
    Stall = 1,
}
impl From<Vss> for bool {
    #[inline(always)]
    fn from(variant: Vss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSS` reader - Stop by STOP"]
pub type VssR = crate::BitReader<Vss>;
impl VssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vss {
        match self.bits {
            false => Vss::Notstall,
            true => Vss::Stall,
        }
    }
    #[doc = "Not-stall"]
    #[inline(always)]
    pub fn is_notstall(&self) -> bool {
        *self == Vss::Notstall
    }
    #[doc = "Stall"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == Vss::Stall
    }
}
#[doc = "Stop by ForceBreak\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vfs {
    #[doc = "0: Not-stall"]
    Notstall = 0,
    #[doc = "1: Stall"]
    Stall = 1,
}
impl From<Vfs> for bool {
    #[inline(always)]
    fn from(variant: Vfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VFS` reader - Stop by ForceBreak"]
pub type VfsR = crate::BitReader<Vfs>;
impl VfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vfs {
        match self.bits {
            false => Vfs::Notstall,
            true => Vfs::Stall,
        }
    }
    #[doc = "Not-stall"]
    #[inline(always)]
    pub fn is_notstall(&self) -> bool {
        *self == Vfs::Notstall
    }
    #[doc = "Stall"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == Vfs::Stall
    }
}
#[doc = "VIF interrupt stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vis {
    #[doc = "0: Not-stall"]
    Notstall = 0,
    #[doc = "1: Stall"]
    Stall = 1,
}
impl From<Vis> for bool {
    #[inline(always)]
    fn from(variant: Vis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIS` reader - VIF interrupt stall"]
pub type VisR = crate::BitReader<Vis>;
impl VisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vis {
        match self.bits {
            false => Vis::Notstall,
            true => Vis::Stall,
        }
    }
    #[doc = "Not-stall"]
    #[inline(always)]
    pub fn is_notstall(&self) -> bool {
        *self == Vis::Notstall
    }
    #[doc = "Stall"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == Vis::Stall
    }
}
#[doc = "Interrupt by the i bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: Not-detect"]
    Notdetect = 0,
    #[doc = "1: Detect"]
    Detect = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - Interrupt by the i bit"]
pub type IntR = crate::BitReader<Int>;
impl IntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int {
        match self.bits {
            false => Int::Notdetect,
            true => Int::Detect,
        }
    }
    #[doc = "Not-detect"]
    #[inline(always)]
    pub fn is_notdetect(&self) -> bool {
        *self == Int::Notdetect
    }
    #[doc = "Detect"]
    #[inline(always)]
    pub fn is_detect(&self) -> bool {
        *self == Int::Detect
    }
}
#[doc = "DMAtag Mismatch error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Er0 {
    #[doc = "0: No error"]
    Noerror = 0,
    #[doc = "1: Error (DMAtag has been detected in the VIF packet.)"]
    Error = 1,
}
impl From<Er0> for bool {
    #[inline(always)]
    fn from(variant: Er0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ER0` reader - DMAtag Mismatch error"]
pub type Er0R = crate::BitReader<Er0>;
impl Er0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Er0 {
        match self.bits {
            false => Er0::Noerror,
            true => Er0::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Er0::Noerror
    }
    #[doc = "Error (DMAtag has been detected in the VIF packet.)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Er0::Error
    }
}
#[doc = "VIFcode error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Er1 {
    #[doc = "0: Not-detect"]
    Notdetect = 0,
    #[doc = "1: Detect (Undefined VIFcode has been detected.)"]
    Detect = 1,
}
impl From<Er1> for bool {
    #[inline(always)]
    fn from(variant: Er1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ER1` reader - VIFcode error"]
pub type Er1R = crate::BitReader<Er1>;
impl Er1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Er1 {
        match self.bits {
            false => Er1::Notdetect,
            true => Er1::Detect,
        }
    }
    #[doc = "Not-detect"]
    #[inline(always)]
    pub fn is_notdetect(&self) -> bool {
        *self == Er1::Notdetect
    }
    #[doc = "Detect (Undefined VIFcode has been detected.)"]
    #[inline(always)]
    pub fn is_detect(&self) -> bool {
        *self == Er1::Detect
    }
}
#[doc = "VIF-FIFO transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdr {
    #[doc = "0: Main memory/SPRAM -> VIF"]
    Intovif = 0,
    #[doc = "1: VIF -> Main memory/SPRAM"]
    Fromvif = 1,
}
impl From<Fdr> for bool {
    #[inline(always)]
    fn from(variant: Fdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDR` reader - VIF-FIFO transfer direction"]
pub type FdrR = crate::BitReader<Fdr>;
impl FdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdr {
        match self.bits {
            false => Fdr::Intovif,
            true => Fdr::Fromvif,
        }
    }
    #[doc = "Main memory/SPRAM -> VIF"]
    #[inline(always)]
    pub fn is_intovif(&self) -> bool {
        *self == Fdr::Intovif
    }
    #[doc = "VIF -> Main memory/SPRAM"]
    #[inline(always)]
    pub fn is_fromvif(&self) -> bool {
        *self == Fdr::Fromvif
    }
}
#[doc = "Field `FDR` writer - VIF-FIFO transfer direction"]
pub type FdrW<'a, REG> = crate::BitWriter<'a, REG, Fdr>;
impl<'a, REG> FdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main memory/SPRAM -> VIF"]
    #[inline(always)]
    pub fn intovif(self) -> &'a mut crate::W<REG> {
        self.variant(Fdr::Intovif)
    }
    #[doc = "VIF -> Main memory/SPRAM"]
    #[inline(always)]
    pub fn fromvif(self) -> &'a mut crate::W<REG> {
        self.variant(Fdr::Fromvif)
    }
}
#[doc = "Field `FQC` reader - Amount of effective data in the VIF-FIFO"]
pub type FqcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - VIF Status"]
    #[inline(always)]
    pub fn vps(&self) -> VpsR {
        VpsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - VIF E-bit Wait"]
    #[inline(always)]
    pub fn vew(&self) -> VewR {
        VewR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status waiting for end of GIF transfer"]
    #[inline(always)]
    pub fn vgw(&self) -> VgwR {
        VgwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - VIF MARK detect"]
    #[inline(always)]
    pub fn mrk(&self) -> MrkR {
        MrkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Double buffer flag. Cleared to 0 by OFFSET and reversed by MSCAL/MSCALF/MSCNT."]
    #[inline(always)]
    pub fn dbf(&self) -> DbfR {
        DbfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop by STOP"]
    #[inline(always)]
    pub fn vss(&self) -> VssR {
        VssR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop by ForceBreak"]
    #[inline(always)]
    pub fn vfs(&self) -> VfsR {
        VfsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VIF interrupt stall"]
    #[inline(always)]
    pub fn vis(&self) -> VisR {
        VisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt by the i bit"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAtag Mismatch error"]
    #[inline(always)]
    pub fn er0(&self) -> Er0R {
        Er0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VIFcode error"]
    #[inline(always)]
    pub fn er1(&self) -> Er1R {
        Er1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 23 - VIF-FIFO transfer direction"]
    #[inline(always)]
    pub fn fdr(&self) -> FdrR {
        FdrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Amount of effective data in the VIF-FIFO"]
    #[inline(always)]
    pub fn fqc(&self) -> FqcR {
        FqcR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - VIF-FIFO transfer direction"]
    #[inline(always)]
    pub fn fdr(&mut self) -> FdrW<Vif1StatSpec> {
        FdrW::new(self, 23)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vif1_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1StatSpec;
impl crate::RegisterSpec for Vif1StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_stat::R`](R) reader structure"]
impl crate::Readable for Vif1StatSpec {}
#[doc = "`write(|w| ..)` method takes [`vif1_stat::W`](W) writer structure"]
impl crate::Writable for Vif1StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIF1_STAT to value 0"]
impl crate::Resettable for Vif1StatSpec {}
