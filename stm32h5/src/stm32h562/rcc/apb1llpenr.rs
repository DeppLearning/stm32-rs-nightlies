#[doc = "Register `APB1LLPENR` reader"]
pub type R = crate::R<APB1LLPENRrs>;
#[doc = "Register `APB1LLPENR` writer"]
pub type W = crate::W<APB1LLPENRrs>;
#[doc = "TIM2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2LPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<TIM2LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN>;
impl TIM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2LPEN {
        match self.bits {
            false => TIM2LPEN::Disabled,
            true => TIM2LPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2LPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2LPEN::Enabled
    }
}
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2LPEN>;
impl<'a, REG> TIM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Enabled)
    }
}
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM3LPEN_R;
#[doc = "Field `TIM4LPEN` reader - TIM4 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM4LPEN_R;
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM5LPEN_R;
#[doc = "Field `TIM6LPEN` reader - TIM6 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM6LPEN_R;
#[doc = "Field `TIM7LPEN` reader - TIM7 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM7LPEN_R;
#[doc = "Field `TIM12LPEN` reader - TIM12 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM12LPEN_R;
#[doc = "Field `TIM13LPEN` reader - TIM13 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM13LPEN_R;
#[doc = "Field `TIM14LPEN` reader - TIM14 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM14LPEN_R;
#[doc = "Field `WWDGLPEN` reader - WWDG clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as WWDGLPEN_R;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as SPI2LPEN_R;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as SPI3LPEN_R;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as USART2LPEN_R;
#[doc = "Field `USART3LPEN` reader - USART3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as USART3LPEN_R;
#[doc = "Field `UART4LPEN` reader - UART4 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as UART4LPEN_R;
#[doc = "Field `UART5LPEN` reader - UART5 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as UART5LPEN_R;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as I2C1LPEN_R;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as I2C2LPEN_R;
#[doc = "Field `I3C1LPEN` reader - I3C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as I3C1LPEN_R;
#[doc = "Field `CRSLPEN` reader - CRS clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as CRSLPEN_R;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as USART6LPEN_R;
#[doc = "Field `USART10LPEN` reader - USART10 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as USART10LPEN_R;
#[doc = "Field `USART11LPEN` reader - USART11 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as USART11LPEN_R;
#[doc = "Field `CECLPEN` reader - HDMI-CEC clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as CECLPEN_R;
#[doc = "Field `UART7LPEN` reader - UART7 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as UART7LPEN_R;
#[doc = "Field `UART8LPEN` reader - UART8 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as UART8LPEN_R;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM3LPEN_W;
#[doc = "Field `TIM4LPEN` writer - TIM4 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM4LPEN_W;
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM5LPEN_W;
#[doc = "Field `TIM6LPEN` writer - TIM6 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM6LPEN_W;
#[doc = "Field `TIM7LPEN` writer - TIM7 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM7LPEN_W;
#[doc = "Field `TIM12LPEN` writer - TIM12 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM12LPEN_W;
#[doc = "Field `TIM13LPEN` writer - TIM13 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM13LPEN_W;
#[doc = "Field `TIM14LPEN` writer - TIM14 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM14LPEN_W;
#[doc = "Field `WWDGLPEN` writer - WWDG clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as WWDGLPEN_W;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as SPI2LPEN_W;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as SPI3LPEN_W;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as USART2LPEN_W;
#[doc = "Field `USART3LPEN` writer - USART3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as USART3LPEN_W;
#[doc = "Field `UART4LPEN` writer - UART4 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as UART4LPEN_W;
#[doc = "Field `UART5LPEN` writer - UART5 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as UART5LPEN_W;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as I2C1LPEN_W;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as I2C2LPEN_W;
#[doc = "Field `I3C1LPEN` writer - I3C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as I3C1LPEN_W;
#[doc = "Field `CRSLPEN` writer - CRS clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as CRSLPEN_W;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as USART6LPEN_W;
#[doc = "Field `USART10LPEN` writer - USART10 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as USART10LPEN_W;
#[doc = "Field `USART11LPEN` writer - USART11 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as USART11LPEN_W;
#[doc = "Field `CECLPEN` writer - HDMI-CEC clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as CECLPEN_W;
#[doc = "Field `UART7LPEN` writer - UART7 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as UART7LPEN_W;
#[doc = "Field `UART8LPEN` writer - UART8 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as UART8LPEN_W;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i3c1lpen(&self) -> I3C1LPEN_R {
        I3C1LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USART6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USART10 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart10lpen(&self) -> USART10LPEN_R {
        USART10LPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USART11 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart11lpen(&self) -> USART11LPEN_R {
        USART11LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HDMI-CEC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ceclpen(&self) -> CECLPEN_R {
        CECLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<APB1LLPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<APB1LLPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<APB1LLPENRrs> {
        TIM4LPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<APB1LLPENRrs> {
        TIM5LPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<APB1LLPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<APB1LLPENRrs> {
        TIM7LPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - TIM12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W<APB1LLPENRrs> {
        TIM12LPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - TIM13 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W<APB1LLPENRrs> {
        TIM13LPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIM14 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W<APB1LLPENRrs> {
        TIM14LPEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<APB1LLPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<APB1LLPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<APB1LLPENRrs> {
        SPI3LPEN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<APB1LLPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<APB1LLPENRrs> {
        USART3LPEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<APB1LLPENRrs> {
        UART4LPEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<APB1LLPENRrs> {
        UART5LPEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<APB1LLPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<APB1LLPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c1lpen(&mut self) -> I3C1LPEN_W<APB1LLPENRrs> {
        I3C1LPEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crslpen(&mut self) -> CRSLPEN_W<APB1LLPENRrs> {
        CRSLPEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - USART6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<APB1LLPENRrs> {
        USART6LPEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - USART10 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart10lpen(&mut self) -> USART10LPEN_W<APB1LLPENRrs> {
        USART10LPEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - USART11 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart11lpen(&mut self) -> USART11LPEN_W<APB1LLPENRrs> {
        USART11LPEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - HDMI-CEC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ceclpen(&mut self) -> CECLPEN_W<APB1LLPENRrs> {
        CECLPEN_W::new(self, 28)
    }
    #[doc = "Bit 30 - UART7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W<APB1LLPENRrs> {
        UART7LPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - UART8 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W<APB1LLPENRrs> {
        UART8LPEN_W::new(self, 31)
    }
}
#[doc = "RCC APB1 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1llpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1llpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LLPENRrs;
impl crate::RegisterSpec for APB1LLPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1llpenr::R`](R) reader structure"]
impl crate::Readable for APB1LLPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1llpenr::W`](W) writer structure"]
impl crate::Writable for APB1LLPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LLPENR to value 0xdffe_c9ff"]
impl crate::Resettable for APB1LLPENRrs {
    const RESET_VALUE: u32 = 0xdffe_c9ff;
}
