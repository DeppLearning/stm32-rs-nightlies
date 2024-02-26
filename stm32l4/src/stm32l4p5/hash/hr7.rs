#[doc = "Register `HR7` reader"]
pub type R = crate::R<HR7rs>;
#[doc = "Field `H7` reader - H7"]
pub type H7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H7"]
    #[inline(always)]
    pub fn h7(&self) -> H7_R {
        H7_R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR7rs;
impl crate::RegisterSpec for HR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr7::R`](R) reader structure"]
impl crate::Readable for HR7rs {}
#[doc = "`reset()` method sets HR7 to value 0"]
impl crate::Resettable for HR7rs {
    const RESET_VALUE: u32 = 0;
}
