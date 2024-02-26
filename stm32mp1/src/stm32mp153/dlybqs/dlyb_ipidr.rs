#[doc = "Register `DLYB_IPIDR` reader"]
pub type R = crate::R<DLYB_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "DLYB IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyb_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLYB_IPIDRrs;
impl crate::RegisterSpec for DLYB_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlyb_ipidr::R`](R) reader structure"]
impl crate::Readable for DLYB_IPIDRrs {}
#[doc = "`reset()` method sets DLYB_IPIDR to value 0x0014_0051"]
impl crate::Resettable for DLYB_IPIDRrs {
    const RESET_VALUE: u32 = 0x0014_0051;
}
