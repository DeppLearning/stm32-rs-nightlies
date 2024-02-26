#[doc = "Register `FDCAN_TTCTC` reader"]
pub type R = crate::R<FDCAN_TTCTCrs>;
#[doc = "Field `CT` reader - Cycle Time"]
pub type CT_R = crate::FieldReader<u16>;
#[doc = "Field `CC` reader - Cycle Count"]
pub type CC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Cycle Time"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Cycle Count"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "FDCAN TT Cycle Time and Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttctc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTCTCrs;
impl crate::RegisterSpec for FDCAN_TTCTCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttctc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTCTCrs {}
#[doc = "`reset()` method sets FDCAN_TTCTC to value 0"]
impl crate::Resettable for FDCAN_TTCTCrs {
    const RESET_VALUE: u32 = 0;
}
