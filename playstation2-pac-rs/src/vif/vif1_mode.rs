#[doc = "Register `VIF1_MODE` reader"]
pub type R = crate::R<Vif1ModeSpec>;
#[doc = "Addition mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mod {
    #[doc = "0: No addition processing"]
    Noaddition = 0,
    #[doc = "1: Offset mode (Row+dV -> VU Mem)"]
    Offset = 1,
    #[doc = "2: Difference mode (Row+dV -> Row -> VU Mem)"]
    Difference = 2,
}
impl From<Mod> for u8 {
    #[inline(always)]
    fn from(variant: Mod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mod {
    type Ux = u8;
}
impl crate::IsEnum for Mod {}
#[doc = "Field `MOD` reader - Addition mode"]
pub type ModR = crate::FieldReader<Mod>;
impl ModR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mod> {
        match self.bits {
            0 => Some(Mod::Noaddition),
            1 => Some(Mod::Offset),
            2 => Some(Mod::Difference),
            _ => None,
        }
    }
    #[doc = "No addition processing"]
    #[inline(always)]
    pub fn is_noaddition(&self) -> bool {
        *self == Mod::Noaddition
    }
    #[doc = "Offset mode (Row+dV -> VU Mem)"]
    #[inline(always)]
    pub fn is_offset(&self) -> bool {
        *self == Mod::Offset
    }
    #[doc = "Difference mode (Row+dV -> Row -> VU Mem)"]
    #[inline(always)]
    pub fn is_difference(&self) -> bool {
        *self == Mod::Difference
    }
}
impl R {
    #[doc = "Bits 0:1 - Addition mode"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new((self.bits & 3) as u8)
    }
}
#[doc = "ADD mode\n\nYou can [`read`](crate::Reg::read) this register and get [`vif1_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vif1ModeSpec;
impl crate::RegisterSpec for Vif1ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vif1_mode::R`](R) reader structure"]
impl crate::Readable for Vif1ModeSpec {}
#[doc = "`reset()` method sets VIF1_MODE to value 0"]
impl crate::Resettable for Vif1ModeSpec {}
