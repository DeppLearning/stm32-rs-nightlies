#[doc = "Register `CPT2BR` reader"]
pub type R = crate::R<CPT2BRrs>;
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type CPT2X_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2br::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPT2BRrs;
impl crate::RegisterSpec for CPT2BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt2br::R`](R) reader structure"]
impl crate::Readable for CPT2BRrs {}
#[doc = "`reset()` method sets CPT2BR to value 0"]
impl crate::Resettable for CPT2BRrs {
    const RESET_VALUE: u32 = 0;
}
