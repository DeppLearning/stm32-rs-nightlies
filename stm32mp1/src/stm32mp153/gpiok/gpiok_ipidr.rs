#[doc = "Register `GPIOK_IPIDR` reader"]
pub type R = crate::R<GPIOK_IPIDRrs>;
#[doc = "Field `IPIDR` reader - IPIDR"]
pub type IPIDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPIDR"]
    #[inline(always)]
    pub fn ipidr(&self) -> IPIDR_R {
        IPIDR_R::new(self.bits)
    }
}
#[doc = "GPIO identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOK_IPIDRrs;
impl crate::RegisterSpec for GPIOK_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiok_ipidr::R`](R) reader structure"]
impl crate::Readable for GPIOK_IPIDRrs {}
#[doc = "`reset()` method sets GPIOK_IPIDR to value 0x000f_0002"]
impl crate::Resettable for GPIOK_IPIDRrs {
    const RESET_VALUE: u32 = 0x000f_0002;
}
