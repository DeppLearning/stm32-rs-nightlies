#[doc = "Register `IPIDR` reader"]
pub type R = crate::R<IPIDRrs>;
#[doc = "Field `ID` reader - IP identification"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IP identification"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "DMAMUX IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPIDRrs;
impl crate::RegisterSpec for IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipidr::R`](R) reader structure"]
impl crate::Readable for IPIDRrs {}
#[doc = "`reset()` method sets IPIDR to value 0x0010_0011"]
impl crate::Resettable for IPIDRrs {
    const RESET_VALUE: u32 = 0x0010_0011;
}
