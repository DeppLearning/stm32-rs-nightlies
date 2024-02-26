#[doc = "Register `C2APB1ENR1` reader"]
pub type R = crate::R<C2APB1ENR1rs>;
#[doc = "Register `C2APB1ENR1` writer"]
pub type W = crate::W<C2APB1ENR1rs>;
#[doc = "Field `TIM2EN` reader - CPU2 TIM2 timer clock enable"]
pub type TIM2EN_R = crate::BitReader;
#[doc = "Field `TIM2EN` writer - CPU2 TIM2 timer clock enable"]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDEN` reader - CPU2 LCD clock enable"]
pub type LCDEN_R = crate::BitReader;
#[doc = "Field `LCDEN` writer - CPU2 LCD clock enable"]
pub type LCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - CPU2 RTC APB clock enable"]
pub type RTCAPBEN_R = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - CPU2 RTC APB clock enable"]
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - CPU2 SPI2 clock enable"]
pub type SPI2EN_R = crate::BitReader;
#[doc = "Field `SPI2EN` writer - CPU2 SPI2 clock enable"]
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - CPU2 I2C1 clock enable"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - CPU2 I2C1 clock enable"]
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3EN` reader - CPU2 I2C3 clock enable"]
pub type I2C3EN_R = crate::BitReader;
#[doc = "Field `I2C3EN` writer - CPU2 I2C3 clock enable"]
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSEN` reader - CPU2 CRS clock enable"]
pub type CRSEN_R = crate::BitReader;
#[doc = "Field `CRSEN` writer - CPU2 CRS clock enable"]
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEN` reader - CPU2 USB clock enable"]
pub type USBEN_R = crate::BitReader;
#[doc = "Field `USBEN` writer - CPU2 USB clock enable"]
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1EN` reader - CPU2 Low power timer 1 clock enable"]
pub type LPTIM1EN_R = crate::BitReader;
#[doc = "Field `LPTIM1EN` writer - CPU2 Low power timer 1 clock enable"]
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU2 TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - CPU2 LCD clock enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU2 RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU2 SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU2 I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU2 I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU2 CRS clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU2 USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU2 Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 TIM2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<C2APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 9 - CPU2 LCD clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LCDEN_W<C2APB1ENR1rs> {
        LCDEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU2 RTC APB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<C2APB1ENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    #[doc = "Bit 14 - CPU2 SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<C2APB1ENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 21 - CPU2 I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<C2APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 23 - CPU2 I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<C2APB1ENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CPU2 CRS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<C2APB1ENR1rs> {
        CRSEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - CPU2 USB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<C2APB1ENR1rs> {
        USBEN_W::new(self, 26)
    }
    #[doc = "Bit 31 - CPU2 Low power timer 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<C2APB1ENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
#[doc = "CPU2 APB1ENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb1enr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb1enr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB1ENR1rs;
impl crate::RegisterSpec for C2APB1ENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb1enr1::R`](R) reader structure"]
impl crate::Readable for C2APB1ENR1rs {}
#[doc = "`write(|w| ..)` method takes [`c2apb1enr1::W`](W) writer structure"]
impl crate::Writable for C2APB1ENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB1ENR1 to value 0x0400"]
impl crate::Resettable for C2APB1ENR1rs {
    const RESET_VALUE: u32 = 0x0400;
}
