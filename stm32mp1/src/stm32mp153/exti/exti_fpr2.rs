#[doc = "Register `EXTI_FPR2` reader"]
pub type R = crate::R<EXTI_FPR2rs>;
#[doc = "Register `EXTI_FPR2` writer"]
pub type W = crate::W<EXTI_FPR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXTI_FPR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_FPR2rs;
impl crate::RegisterSpec for EXTI_FPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_fpr2::R`](R) reader structure"]
impl crate::Readable for EXTI_FPR2rs {}
#[doc = "`write(|w| ..)` method takes [`exti_fpr2::W`](W) writer structure"]
impl crate::Writable for EXTI_FPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_FPR2 to value 0"]
impl crate::Resettable for EXTI_FPR2rs {
    const RESET_VALUE: u32 = 0;
}
