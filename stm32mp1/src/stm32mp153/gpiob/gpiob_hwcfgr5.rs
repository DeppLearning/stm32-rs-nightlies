#[doc = "Register `GPIOB_HWCFGR5` reader"]
pub type R = crate::R<GPIOB_HWCFGR5rs>;
#[doc = "Field `PUPDR_RES` reader - PUPDR_RES"]
pub type PUPDR_RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PUPDR_RES"]
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_hwcfgr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOB_HWCFGR5rs;
impl crate::RegisterSpec for GPIOB_HWCFGR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob_hwcfgr5::R`](R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR5rs {}
#[doc = "`reset()` method sets GPIOB_HWCFGR5 to value 0"]
impl crate::Resettable for GPIOB_HWCFGR5rs {
    const RESET_VALUE: u32 = 0;
}
