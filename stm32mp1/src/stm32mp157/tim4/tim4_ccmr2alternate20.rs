#[doc = "Register `TIM4_CCMR2ALTERNATE20` reader"]
pub type R = crate::R<TIM4_CCMR2ALTERNATE20rs>;
#[doc = "Register `TIM4_CCMR2ALTERNATE20` writer"]
pub type W = crate::W<TIM4_CCMR2ALTERNATE20rs>;
#[doc = "Field `CC3S` reader - CC3S"]
pub type CC3S_R = crate::FieldReader;
#[doc = "Field `CC3S` writer - CC3S"]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3PSC` reader - IC3PSC"]
pub type IC3PSC_R = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - IC3PSC"]
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - IC3F"]
pub type IC3F_R = crate::FieldReader;
#[doc = "Field `IC3F` writer - IC3F"]
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC4S` reader - CC4S"]
pub type CC4S_R = crate::FieldReader;
#[doc = "Field `CC4S` writer - CC4S"]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4PSC` reader - IC4PSC"]
pub type IC4PSC_R = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - IC4PSC"]
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - IC4F"]
pub type IC4F_R = crate::FieldReader;
#[doc = "Field `IC4F` writer - IC4F"]
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IC3PSC"]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - IC3F"]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - IC4PSC"]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - IC4F"]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<TIM4_CCMR2ALTERNATE20rs> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - IC3PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> IC3PSC_W<TIM4_CCMR2ALTERNATE20rs> {
        IC3PSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - IC3F"]
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> IC3F_W<TIM4_CCMR2ALTERNATE20rs> {
        IC3F_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<TIM4_CCMR2ALTERNATE20rs> {
        CC4S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - IC4PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> IC4PSC_W<TIM4_CCMR2ALTERNATE20rs> {
        IC4PSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - IC4F"]
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> IC4F_W<TIM4_CCMR2ALTERNATE20rs> {
        IC4F_W::new(self, 12)
    }
}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccmr2alternate20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccmr2alternate20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM4_CCMR2ALTERNATE20rs;
impl crate::RegisterSpec for TIM4_CCMR2ALTERNATE20rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim4_ccmr2alternate20::R`](R) reader structure"]
impl crate::Readable for TIM4_CCMR2ALTERNATE20rs {}
#[doc = "`write(|w| ..)` method takes [`tim4_ccmr2alternate20::W`](W) writer structure"]
impl crate::Writable for TIM4_CCMR2ALTERNATE20rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM4_CCMR2ALTERNATE20 to value 0"]
impl crate::Resettable for TIM4_CCMR2ALTERNATE20rs {
    const RESET_VALUE: u32 = 0;
}
