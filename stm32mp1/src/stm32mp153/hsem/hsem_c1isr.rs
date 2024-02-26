#[doc = "Register `HSEM_C1ISR` reader"]
pub type R = crate::R<HSEM_C1ISRrs>;
#[doc = "Field `ISF` reader - ISF"]
pub type ISF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ISF"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(self.bits)
    }
}
#[doc = "HSEM i1terrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C1ISRrs;
impl crate::RegisterSpec for HSEM_C1ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1isr::R`](R) reader structure"]
impl crate::Readable for HSEM_C1ISRrs {}
#[doc = "`reset()` method sets HSEM_C1ISR to value 0"]
impl crate::Resettable for HSEM_C1ISRrs {
    const RESET_VALUE: u32 = 0;
}
