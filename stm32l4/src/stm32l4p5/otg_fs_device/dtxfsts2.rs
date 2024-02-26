#[doc = "Register `DTXFSTS2` reader"]
pub type R = crate::R<DTXFSTS2rs>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS2rs;
impl crate::RegisterSpec for DTXFSTS2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts2::R`](R) reader structure"]
impl crate::Readable for DTXFSTS2rs {}
#[doc = "`reset()` method sets DTXFSTS2 to value 0"]
impl crate::Resettable for DTXFSTS2rs {
    const RESET_VALUE: u32 = 0;
}
