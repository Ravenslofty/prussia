#[doc = "Register `EXTWRITE` writer"]
pub type W = crate::W<ExtwriteSpec>;
#[doc = "Enable feedback write.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Complete current feedback write."]
    Stop = 0,
    #[doc = "1: Start next feedback write."]
    Start = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` writer - Enable feedback write."]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, Write>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Complete current feedback write."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Stop)
    }
    #[doc = "Start next feedback write."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Start)
    }
}
impl W {
    #[doc = "Bit 0 - Enable feedback write."]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<ExtwriteSpec> {
        WriteW::new(self, 0)
    }
}
#[doc = "Feedback write function control.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extwrite::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtwriteSpec;
impl crate::RegisterSpec for ExtwriteSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`extwrite::W`](W) writer structure"]
impl crate::Writable for ExtwriteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTWRITE to value 0"]
impl crate::Resettable for ExtwriteSpec {}
