#[doc = "Register `GPIOD_IPIDR` reader"]
pub type R = crate::R<GPIOD_IPIDRrs>;
#[doc = "Field `IPIDR` reader - IPIDR"]
pub type IPIDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPIDR"]
    #[inline(always)]
    pub fn ipidr(&self) -> IPIDR_R {
        IPIDR_R::new(self.bits)
    }
}
#[doc = "GPIO identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOD_IPIDRrs;
impl crate::RegisterSpec for GPIOD_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiod_ipidr::R`](R) reader structure"]
impl crate::Readable for GPIOD_IPIDRrs {}
#[doc = "`reset()` method sets GPIOD_IPIDR to value 0x000f_0002"]
impl crate::Resettable for GPIOD_IPIDRrs {
    const RESET_VALUE: u32 = 0x000f_0002;
}
