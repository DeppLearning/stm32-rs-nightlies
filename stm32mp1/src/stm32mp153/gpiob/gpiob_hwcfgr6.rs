#[doc = "Register `GPIOB_HWCFGR6` reader"]
pub type R = crate::R<GPIOB_HWCFGR6rs>;
#[doc = "Field `MODER_RES` reader - MODER_RES"]
pub type MODER_RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MODER_RES"]
    #[inline(always)]
    pub fn moder_res(&self) -> MODER_RES_R {
        MODER_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_hwcfgr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOB_HWCFGR6rs;
impl crate::RegisterSpec for GPIOB_HWCFGR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob_hwcfgr6::R`](R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR6rs {}
#[doc = "`reset()` method sets GPIOB_HWCFGR6 to value 0xffff_ffff"]
impl crate::Resettable for GPIOB_HWCFGR6rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
