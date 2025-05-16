#[doc = "Register `BUSDIR` reader"]
pub type R = crate::R<BusdirSpec>;
#[doc = "Host to local direction, or vice versa.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Host to local bus transfer."]
    HostToLocal = 0,
    #[doc = "1: Local to host bus transfer."]
    LocalToHost = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Host to local direction, or vice versa."]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::HostToLocal,
            true => Dir::LocalToHost,
        }
    }
    #[doc = "Host to local bus transfer."]
    #[inline(always)]
    pub fn is_host_to_local(&self) -> bool {
        *self == Dir::HostToLocal
    }
    #[doc = "Local to host bus transfer."]
    #[inline(always)]
    pub fn is_local_to_host(&self) -> bool {
        *self == Dir::LocalToHost
    }
}
impl R {
    #[doc = "Bit 0 - Host to local direction, or vice versa."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
}
#[doc = "GS bus direction.\n\nYou can [`read`](crate::Reg::read) this register and get [`busdir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusdirSpec;
impl crate::RegisterSpec for BusdirSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`busdir::R`](R) reader structure"]
impl crate::Readable for BusdirSpec {}
#[doc = "`reset()` method sets BUSDIR to value 0"]
impl crate::Resettable for BusdirSpec {}
