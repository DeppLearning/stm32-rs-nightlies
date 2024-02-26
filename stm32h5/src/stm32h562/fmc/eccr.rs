#[doc = "Register `ECCR` reader"]
pub type R = crate::R<ECCRrs>;
#[doc = "Field `ECC` reader - ECC result This field contains the value computed by the ECC computation logic. Table 99 describes the contents of these bitfields."]
pub type ECC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result This field contains the value computed by the ECC computation logic. Table 99 describes the contents of these bitfields."]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
#[doc = "ECC result registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for ECCRrs {}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for ECCRrs {
    const RESET_VALUE: u32 = 0;
}
