#[doc = "Register `CPT1DR` reader"]
pub type R = crate::R<CPT1DRrs>;
#[doc = "Field `CPT1x` reader - Timerx Capture 1 value"]
pub type CPT1X_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPT1DRrs;
impl crate::RegisterSpec for CPT1DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt1dr::R`](R) reader structure"]
impl crate::Readable for CPT1DRrs {}
#[doc = "`reset()` method sets CPT1DR to value 0"]
impl crate::Resettable for CPT1DRrs {
    const RESET_VALUE: u32 = 0;
}
