#[doc = "Register `I2SCFGR` reader"]
pub type R = crate::R<I2SCFGRrs>;
#[doc = "Register `I2SCFGR` writer"]
pub type W = crate::W<I2SCFGRrs>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type CHLEN_R = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATLEN` reader - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKPOL` reader - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CKPOL_R = crate::BitReader;
#[doc = "Field `CKPOL` writer - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SSTD_R = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PCMSYNC_R = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SCFG` reader - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SCFG_R = crate::FieldReader;
#[doc = "Field `I2SCFG` writer - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SE` reader - I2S enable Note: This bit is not used in SPI mode."]
pub type I2SE_R = crate::BitReader;
#[doc = "Field `I2SE` writer - I2S enable Note: This bit is not used in SPI mode."]
pub type I2SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SMOD` reader - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2SMOD_R = crate::BitReader;
#[doc = "Field `I2SMOD` writer - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASTRTEN` reader - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type ASTRTEN_R = crate::BitReader;
#[doc = "Field `ASTRTEN` writer - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type ASTRTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<I2SCFGRrs> {
        CHLEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<I2SCFGRrs> {
        DATLEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<I2SCFGRrs> {
        CKPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<I2SCFGRrs> {
        I2SCFG_W::new(self, 8)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<I2SCFGRrs> {
        I2SE_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<I2SCFGRrs> {
        I2SMOD_W::new(self, 11)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn astrten(&mut self) -> ASTRTEN_W<I2SCFGRrs> {
        ASTRTEN_W::new(self, 12)
    }
}
#[doc = "SPI_I2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCFGRrs;
impl crate::RegisterSpec for I2SCFGRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`i2scfgr::R`](R) reader structure"]
impl crate::Readable for I2SCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure"]
impl crate::Writable for I2SCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGRrs {
    const RESET_VALUE: u16 = 0;
}
