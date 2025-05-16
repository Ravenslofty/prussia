#[doc = "Register `SIO_TXFIFO` writer"]
pub type W = crate::W<SioTxfifoSpec>;
#[doc = "Field `BUFFER` writer - Write individual bytes to the FIFO buffer."]
pub type BufferW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write individual bytes to the FIFO buffer."]
    #[inline(always)]
    pub fn buffer(&mut self) -> BufferW<SioTxfifoSpec> {
        BufferW::new(self, 0)
    }
}
#[doc = "Tx FIFO buffer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sio_txfifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioTxfifoSpec;
impl crate::RegisterSpec for SioTxfifoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sio_txfifo::W`](W) writer structure"]
impl crate::Writable for SioTxfifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIO_TXFIFO to value 0"]
impl crate::Resettable for SioTxfifoSpec {}
