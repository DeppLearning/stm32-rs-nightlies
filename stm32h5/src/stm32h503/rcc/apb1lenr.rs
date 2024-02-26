#[doc = "Register `APB1LENR` reader"]
pub type R = crate::R<APB1LENRrs>;
#[doc = "Register `APB1LENR` writer"]
pub type W = crate::W<APB1LENRrs>;
#[doc = "TIM2 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - TIM2 clock enable Set and reset by software."]
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::Disabled,
            true => TIM2EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
#[doc = "Field `TIM2EN` writer - TIM2 clock enable Set and reset by software."]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
#[doc = "Field `TIM3EN` reader - TIM3 clock enable Set and reset by software."]
pub use TIM2EN_R as TIM3EN_R;
#[doc = "Field `TIM6EN` reader - TIM6 clock enable Set and reset by software."]
pub use TIM2EN_R as TIM6EN_R;
#[doc = "Field `TIM7EN` reader - TIM7 clock enable Set and reset by software."]
pub use TIM2EN_R as TIM7EN_R;
#[doc = "Field `WWDGEN` reader - WWDG clock enable Set and reset by software."]
pub use TIM2EN_R as WWDGEN_R;
#[doc = "Field `OPAMPEN` reader - OPAMP clock enable Set and reset by software."]
pub use TIM2EN_R as OPAMPEN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable Set and reset by software."]
pub use TIM2EN_R as SPI2EN_R;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable Set and reset by software."]
pub use TIM2EN_R as SPI3EN_R;
#[doc = "Field `COMPEN` reader - COMP clock enable Set and reset by software."]
pub use TIM2EN_R as COMPEN_R;
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and reset by software."]
pub use TIM2EN_R as USART2EN_R;
#[doc = "Field `USART3EN` reader - USART3 clock enable Set and reset by software."]
pub use TIM2EN_R as USART3EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and reset by software."]
pub use TIM2EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable Set and reset by software."]
pub use TIM2EN_R as I2C2EN_R;
#[doc = "Field `I3C1EN` reader - I3C1 clock enable Set and reset by software."]
pub use TIM2EN_R as I3C1EN_R;
#[doc = "Field `CRSEN` reader - CRS clock enable Set and reset by software."]
pub use TIM2EN_R as CRSEN_R;
#[doc = "Field `TIM3EN` writer - TIM3 clock enable Set and reset by software."]
pub use TIM2EN_W as TIM3EN_W;
#[doc = "Field `TIM6EN` writer - TIM6 clock enable Set and reset by software."]
pub use TIM2EN_W as TIM6EN_W;
#[doc = "Field `TIM7EN` writer - TIM7 clock enable Set and reset by software."]
pub use TIM2EN_W as TIM7EN_W;
#[doc = "Field `WWDGEN` writer - WWDG clock enable Set and reset by software."]
pub use TIM2EN_W as WWDGEN_W;
#[doc = "Field `OPAMPEN` writer - OPAMP clock enable Set and reset by software."]
pub use TIM2EN_W as OPAMPEN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable Set and reset by software."]
pub use TIM2EN_W as SPI2EN_W;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable Set and reset by software."]
pub use TIM2EN_W as SPI3EN_W;
#[doc = "Field `COMPEN` writer - COMP clock enable Set and reset by software."]
pub use TIM2EN_W as COMPEN_W;
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and reset by software."]
pub use TIM2EN_W as USART2EN_W;
#[doc = "Field `USART3EN` writer - USART3 clock enable Set and reset by software."]
pub use TIM2EN_W as USART3EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and reset by software."]
pub use TIM2EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable Set and reset by software."]
pub use TIM2EN_W as I2C2EN_W;
#[doc = "Field `I3C1EN` writer - I3C1 clock enable Set and reset by software."]
pub use TIM2EN_W as I3C1EN_W;
#[doc = "Field `CRSEN` writer - CRS clock enable Set and reset by software."]
pub use TIM2EN_W as CRSEN_W;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP clock enable Set and reset by software."]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP clock enable Set and reset by software."]
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i3c1en(&self) -> I3C1EN_R {
        I3C1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1LENRrs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APB1LENRrs> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1LENRrs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1LENRrs> {
        TIM7EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - WWDG clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1LENRrs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - OPAMP clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<APB1LENRrs> {
        OPAMPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1LENRrs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<APB1LENRrs> {
        SPI3EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - COMP clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> COMPEN_W<APB1LENRrs> {
        COMPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1LENRrs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APB1LENRrs> {
        USART3EN_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1LENRrs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1LENRrs> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c1en(&mut self) -> I3C1EN_W<APB1LENRrs> {
        I3C1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<APB1LENRrs> {
        CRSEN_W::new(self, 24)
    }
}
#[doc = "RCC APB1 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LENRrs;
impl crate::RegisterSpec for APB1LENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lenr::R`](R) reader structure"]
impl crate::Readable for APB1LENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1lenr::W`](W) writer structure"]
impl crate::Writable for APB1LENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LENR to value 0"]
impl crate::Resettable for APB1LENRrs {
    const RESET_VALUE: u32 = 0;
}
