#[doc = "Register `AHBSMENR` reader"]
pub type R = crate::R<AHBSMENRrs>;
#[doc = "Register `AHBSMENR` writer"]
pub type W = crate::W<AHBSMENRrs>;
#[doc = "Field `DMA1SMEN` reader - DMA1 clock enable during Sleep mode"]
pub type DMA1SMEN_R = crate::BitReader;
#[doc = "Field `DMA1SMEN` writer - DMA1 clock enable during Sleep mode"]
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2SMEN` reader - DMA2 clock enable during Sleep mode"]
pub type DMA2SMEN_R = crate::BitReader;
#[doc = "Field `DMA2SMEN` writer - DMA2 clock enable during Sleep mode"]
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode"]
pub type FLASHSMEN_R = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode"]
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode"]
pub type SRAMSMEN_R = crate::BitReader;
#[doc = "Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode"]
pub type SRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRC clock enable during Sleep mode"]
pub type CRCSMEN_R = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRC clock enable during Sleep mode"]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESSMEN` reader - AES hardware accelerator clock enable during Sleep mode"]
pub type AESSMEN_R = crate::BitReader;
#[doc = "Field `AESSMEN` writer - AES hardware accelerator clock enable during Sleep mode"]
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSMEN` reader - Random number generator clock enable during Sleep mode"]
pub type RNGSMEN_R = crate::BitReader;
#[doc = "Field `RNGSMEN` writer - Random number generator clock enable during Sleep mode"]
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<AHBSMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<AHBSMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<AHBSMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - AES hardware accelerator clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<AHBSMENRrs> {
        AESSMEN_W::new(self, 16)
    }
    #[doc = "Bit 18 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<AHBSMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
}
#[doc = "AHB peripheral clock enable in Sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbsmenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBSMENRrs;
impl crate::RegisterSpec for AHBSMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbsmenr::R`](R) reader structure"]
impl crate::Readable for AHBSMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure"]
impl crate::Writable for AHBSMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBSMENR to value 0x0005_1303"]
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0x0005_1303;
}
