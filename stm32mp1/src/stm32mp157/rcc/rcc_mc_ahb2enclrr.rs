#[doc = "Register `RCC_MC_AHB2ENCLRR` reader"]
pub type R = crate::R<RCC_MC_AHB2ENCLRRrs>;
#[doc = "Register `RCC_MC_AHB2ENCLRR` writer"]
pub type W = crate::W<RCC_MC_AHB2ENCLRRrs>;
#[doc = "Field `DMA1EN` reader - DMA1EN"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1EN"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2EN"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2EN"]
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXEN` reader - DMAMUXEN"]
pub type DMAMUXEN_R = crate::BitReader;
#[doc = "Field `DMAMUXEN` writer - DMAMUXEN"]
pub type DMAMUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12EN` reader - ADC12EN"]
pub type ADC12EN_R = crate::BitReader;
#[doc = "Field `ADC12EN` writer - ADC12EN"]
pub type ADC12EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOEN` reader - USBOEN"]
pub type USBOEN_R = crate::BitReader;
#[doc = "Field `USBOEN` writer - USBOEN"]
pub type USBOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC3EN` reader - SDMMC3EN"]
pub type SDMMC3EN_R = crate::BitReader;
#[doc = "Field `SDMMC3EN` writer - SDMMC3EN"]
pub type SDMMC3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXEN"]
    #[inline(always)]
    pub fn dmamuxen(&self) -> DMAMUXEN_R {
        DMAMUXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12EN"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USBOEN"]
    #[inline(always)]
    pub fn usboen(&self) -> USBOEN_R {
        USBOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC3EN"]
    #[inline(always)]
    pub fn sdmmc3en(&self) -> SDMMC3EN_R {
        SDMMC3EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1EN"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<RCC_MC_AHB2ENCLRRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2EN"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<RCC_MC_AHB2ENCLRRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUXEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxen(&mut self) -> DMAMUXEN_W<RCC_MC_AHB2ENCLRRrs> {
        DMAMUXEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - ADC12EN"]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<RCC_MC_AHB2ENCLRRrs> {
        ADC12EN_W::new(self, 5)
    }
    #[doc = "Bit 8 - USBOEN"]
    #[inline(always)]
    #[must_use]
    pub fn usboen(&mut self) -> USBOEN_W<RCC_MC_AHB2ENCLRRrs> {
        USBOEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - SDMMC3EN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3en(&mut self) -> SDMMC3EN_W<RCC_MC_AHB2ENCLRRrs> {
        SDMMC3EN_W::new(self, 16)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb2enclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb2enclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_AHB2ENCLRRrs;
impl crate::RegisterSpec for RCC_MC_AHB2ENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_ahb2enclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_AHB2ENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_ahb2enclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_AHB2ENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_AHB2ENCLRR to value 0"]
impl crate::Resettable for RCC_MC_AHB2ENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
