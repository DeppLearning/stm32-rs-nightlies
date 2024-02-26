#[doc = "Register `GPIOK_HWCFGR5` reader"]
pub type R = crate::R<GPIOK_HWCFGR5rs>;
#[doc = "Field `PUPDR_RES` reader - PUPDR_RES"]
pub type PUPDR_RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PUPDR_RES"]
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOK_HWCFGR5rs;
impl crate::RegisterSpec for GPIOK_HWCFGR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiok_hwcfgr5::R`](R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR5rs {}
#[doc = "`reset()` method sets GPIOK_HWCFGR5 to value 0"]
impl crate::Resettable for GPIOK_HWCFGR5rs {
    const RESET_VALUE: u32 = 0;
}
