#[doc = "Register `SIO_RXFIFO` reader"]
pub type R = crate::R<SioRxfifoSpec>;
#[doc = "Field `BUFFER` reader - Read individual bytes from the FIFO buffer."]
pub type BufferR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Read individual bytes from the FIFO buffer."]
    #[inline(always)]
    pub fn buffer(&self) -> BufferR {
        BufferR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Rx FIFO buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`sio_rxfifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SioRxfifoSpec;
impl crate::RegisterSpec for SioRxfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sio_rxfifo::R`](R) reader structure"]
impl crate::Readable for SioRxfifoSpec {}
#[doc = "`reset()` method sets SIO_RXFIFO to value 0"]
impl crate::Resettable for SioRxfifoSpec {}
