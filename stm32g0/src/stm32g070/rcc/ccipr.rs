#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CCIPRrs>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CCIPRrs>;
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader;
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub type USART2SEL_R = crate::FieldReader;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S2SEL` reader - I2S1 clock source selection"]
pub type I2S2SEL_R = crate::FieldReader;
#[doc = "Field `I2S2SEL` writer - I2S1 clock source selection"]
pub type I2S2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM1SEL` reader - TIM1 clock source selection"]
pub type TIM1SEL_R = crate::BitReader;
#[doc = "Field `TIM1SEL` writer - TIM1 clock source selection"]
pub type TIM1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SEL` reader - TIM15 clock source selection"]
pub type TIM15SEL_R = crate::BitReader;
#[doc = "Field `TIM15SEL` writer - TIM15 clock source selection"]
pub type TIM15SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub type ADCSEL_R = crate::FieldReader;
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    pub fn tim1sel(&self) -> TIM1SEL_R {
        TIM1SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    pub fn tim15sel(&self) -> TIM15SEL_R {
        TIM15SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPRrs> {
        USART2SEL_W::new(self, 2)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<CCIPRrs> {
        I2S2SEL_W::new(self, 14)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sel(&mut self) -> TIM1SEL_W<CCIPRrs> {
        TIM1SEL_W::new(self, 22)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sel(&mut self) -> TIM15SEL_W<CCIPRrs> {
        TIM15SEL_W::new(self, 24)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPRrs> {
        ADCSEL_W::new(self, 30)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CCIPRrs {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPRrs {
    const RESET_VALUE: u32 = 0;
}
