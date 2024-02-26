#[doc = "Register `DFSDM2_EXMAX` reader"]
pub type R = crate::R<DFSDM2_EXMAXrs>;
#[doc = "Field `EXMAXCH` reader - Extremes detector maximum data channel"]
pub type EXMAXCH_R = crate::FieldReader;
#[doc = "Field `EXMAX` reader - Extremes detector maximum value"]
pub type EXMAX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector maximum data channel"]
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Extremes detector maximum value"]
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "DFSDM Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_exmax::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM2_EXMAXrs;
impl crate::RegisterSpec for DFSDM2_EXMAXrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm2_exmax::R`](R) reader structure"]
impl crate::Readable for DFSDM2_EXMAXrs {}
#[doc = "`reset()` method sets DFSDM2_EXMAX to value 0"]
impl crate::Resettable for DFSDM2_EXMAXrs {
    const RESET_VALUE: u32 = 0;
}
