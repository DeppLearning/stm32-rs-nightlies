#[doc = "Register `M4FAR` reader"]
pub type R = crate::R<M4FARrs>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4far::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4FARrs;
impl crate::RegisterSpec for M4FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4far::R`](R) reader structure"]
impl crate::Readable for M4FARrs {}
#[doc = "`reset()` method sets M4FAR to value 0"]
impl crate::Resettable for M4FARrs {
    const RESET_VALUE: u32 = 0;
}
