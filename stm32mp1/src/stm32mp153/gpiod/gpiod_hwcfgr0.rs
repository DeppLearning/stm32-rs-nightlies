#[doc = "Register `GPIOD_HWCFGR0` reader"]
pub type R = crate::R<GPIOD_HWCFGR0rs>;
#[doc = "Field `OR_RES` reader - OR_RES"]
pub type OR_RES_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - OR_RES"]
    #[inline(always)]
    pub fn or_res(&self) -> OR_RES_R {
        OR_RES_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPIO hardware configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOD_HWCFGR0rs;
impl crate::RegisterSpec for GPIOD_HWCFGR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiod_hwcfgr0::R`](R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR0rs {}
#[doc = "`reset()` method sets GPIOD_HWCFGR0 to value 0"]
impl crate::Resettable for GPIOD_HWCFGR0rs {
    const RESET_VALUE: u32 = 0;
}
