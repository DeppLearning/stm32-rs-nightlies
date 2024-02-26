#[doc = "Register `FDCAN_TXFQS` reader"]
pub type R = crate::R<FDCAN_TXFQSrs>;
#[doc = "Field `TFFL` reader - Tx FIFO Free Level"]
pub type TFFL_R = crate::FieldReader;
#[doc = "Field `TFGI` reader - TFGI"]
pub type TFGI_R = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index"]
pub type TFQPI_R = crate::FieldReader;
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full"]
pub type TFQF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - TFGI"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "FDCAN Tx FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXFQSrs;
impl crate::RegisterSpec for FDCAN_TXFQSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txfqs::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXFQSrs {}
#[doc = "`reset()` method sets FDCAN_TXFQS to value 0x03"]
impl crate::Resettable for FDCAN_TXFQSrs {
    const RESET_VALUE: u32 = 0x03;
}
