#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `CHSIDE` reader - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode."]
pub type CHSIDE_R = crate::BitReader;
#[doc = "Field `UDR` reader - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to page 1385 for the software sequence. Note: This bit is not used in SPI mode."]
pub type UDR_R = crate::BitReader;
#[doc = "Field `CRCERR` reader - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
pub type CRCERR_R = crate::BitReader;
#[doc = "Field `CRCERR` writer - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` reader - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on page 1359 for the software sequence. Note: This bit is not used in I2S mode."]
pub type MODF_R = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to page 1385 for the software sequence."]
pub type OVR_R = crate::BitReader;
#[doc = "Field `BSY` reader - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and ."]
pub type BSY_R = crate::BitReader;
#[doc = "Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software."]
pub type FRE_R = crate::BitReader;
#[doc = "Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in I�S mode and in SPI receive-only mode while CRC calculation is enabled."]
pub type FRLVL_R = crate::FieldReader;
#[doc = "Field `FTLVL` reader - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode."]
pub type FTLVL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode."]
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to page 1385 for the software sequence. Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on page 1359 for the software sequence. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to page 1385 for the software sequence."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and ."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software."]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in I�S mode and in SPI receive-only mode while CRC calculation is enabled."]
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<SRrs> {
        CRCERR_W::new(self, 4)
    }
}
#[doc = "SPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u16 = 0x02;
}
