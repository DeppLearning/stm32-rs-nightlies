#[doc = "Register `TIM3_CCR6` reader"]
pub type R = crate::R<TIM3_CCR6rs>;
#[doc = "Register `TIM3_CCR6` writer"]
pub type W = crate::W<TIM3_CCR6rs>;
#[doc = "Field `CCR6` reader - CCR6"]
pub type CCR6_R = crate::FieldReader<u16>;
#[doc = "Field `CCR6` writer - CCR6"]
pub type CCR6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CCR6"]
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR6"]
    #[inline(always)]
    #[must_use]
    pub fn ccr6(&mut self) -> CCR6_W<TIM3_CCR6rs> {
        CCR6_W::new(self, 0)
    }
}
#[doc = "TIM3 capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM3_CCR6rs;
impl crate::RegisterSpec for TIM3_CCR6rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim3_ccr6::R`](R) reader structure"]
impl crate::Readable for TIM3_CCR6rs {}
#[doc = "`write(|w| ..)` method takes [`tim3_ccr6::W`](W) writer structure"]
impl crate::Writable for TIM3_CCR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM3_CCR6 to value 0"]
impl crate::Resettable for TIM3_CCR6rs {
    const RESET_VALUE: u16 = 0;
}
