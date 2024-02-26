#[doc = "Register `FDCAN_HPMS` reader"]
pub type R = crate::R<FDCAN_HPMSrs>;
#[doc = "Field `BIDX` reader - Buffer Index"]
pub type BIDX_R = crate::FieldReader;
#[doc = "Field `MSI` reader - Message Storage Indicator"]
pub type MSI_R = crate::FieldReader;
#[doc = "Field `FIDX` reader - Filter Index"]
pub type FIDX_R = crate::FieldReader;
#[doc = "Field `FLST` reader - Filter List"]
pub type FLST_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FDCAN High Priority Message Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_hpms::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_HPMSrs;
impl crate::RegisterSpec for FDCAN_HPMSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_hpms::R`](R) reader structure"]
impl crate::Readable for FDCAN_HPMSrs {}
#[doc = "`reset()` method sets FDCAN_HPMS to value 0"]
impl crate::Resettable for FDCAN_HPMSrs {
    const RESET_VALUE: u32 = 0;
}
