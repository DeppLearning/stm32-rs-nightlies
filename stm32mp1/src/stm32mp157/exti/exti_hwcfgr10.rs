#[doc = "Register `EXTI_HWCFGR10` reader"]
pub type R = crate::R<EXTI_HWCFGR10rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXTI_HWCFGR10rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "EXTI hardware configuration register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_HWCFGR10rs;
impl crate::RegisterSpec for EXTI_HWCFGR10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_hwcfgr10::R`](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR10rs {}
#[doc = "`reset()` method sets EXTI_HWCFGR10 to value 0"]
impl crate::Resettable for EXTI_HWCFGR10rs {
    const RESET_VALUE: u32 = 0;
}
