#[doc = "Register `IWDG_SR` reader"]
pub type R = crate::R<IWDG_SRrs>;
#[doc = "Field `PVU` reader - PVU"]
pub type PVU_R = crate::BitReader;
#[doc = "Field `RVU` reader - RVU"]
pub type RVU_R = crate::BitReader;
#[doc = "Field `WVU` reader - WVU"]
pub type WVU_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PVU"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RVU"]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WVU"]
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_SRrs;
impl crate::RegisterSpec for IWDG_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_sr::R`](R) reader structure"]
impl crate::Readable for IWDG_SRrs {}
#[doc = "`reset()` method sets IWDG_SR to value 0"]
impl crate::Resettable for IWDG_SRrs {
    const RESET_VALUE: u32 = 0;
}
