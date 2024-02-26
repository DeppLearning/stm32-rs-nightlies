#[doc = "Register `C2APB2SMENR` reader"]
pub type R = crate::R<C2APB2SMENRrs>;
#[doc = "Register `C2APB2SMENR` writer"]
pub type W = crate::W<C2APB2SMENRrs>;
#[doc = "Field `ADCSMEN` reader - ADC clocks enable during CPU2 Csleep and CStop modes"]
pub type ADCSMEN_R = crate::BitReader;
#[doc = "Field `ADCSMEN` writer - ADC clocks enable during CPU2 Csleep and CStop modes"]
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during CPU2 CSleep mode"]
pub type TIM1SMEN_R = crate::BitReader;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during CPU2 CSleep mode"]
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during CPU2 CSleep mode"]
pub type SPI1SMEN_R = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during CPU2 CSleep mode"]
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SMEN` reader - USART1clock enable during CPU2 CSleep and CStop mode"]
pub type USART1SMEN_R = crate::BitReader;
#[doc = "Field `USART1SMEN` writer - USART1clock enable during CPU2 CSleep and CStop mode"]
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during CPU2 CSleep mode"]
pub type TIM16SMEN_R = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during CPU2 CSleep mode"]
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clock enable during CPU2 CSleep mode"]
pub type TIM17SMEN_R = crate::BitReader;
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clock enable during CPU2 CSleep mode"]
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - ADC clocks enable during CPU2 Csleep and CStop modes"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable during CPU2 CSleep and CStop mode"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - ADC clocks enable during CPU2 Csleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<C2APB2SMENRrs> {
        ADCSMEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<C2APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<C2APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1clock enable during CPU2 CSleep and CStop mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<C2APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<C2APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<C2APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
}
#[doc = "CPU2 APB2 peripheral clocks enable in Sleep mode register \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB2SMENRrs;
impl crate::RegisterSpec for C2APB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb2smenr::R`](R) reader structure"]
impl crate::Readable for C2APB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2apb2smenr::W`](W) writer structure"]
impl crate::Writable for C2APB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB2SMENR to value 0x0006_5a00"]
impl crate::Resettable for C2APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0006_5a00;
}
