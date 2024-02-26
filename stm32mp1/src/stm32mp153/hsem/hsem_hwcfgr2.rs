#[doc = "Register `HSEM_HWCFGR2` reader"]
pub type R = crate::R<HSEM_HWCFGR2rs>;
#[doc = "Field `MASTERID1` reader - MASTERID1"]
pub type MASTERID1_R = crate::FieldReader;
#[doc = "Field `MASTERID2` reader - MASTERID2"]
pub type MASTERID2_R = crate::FieldReader;
#[doc = "Field `MASTERID3` reader - MASTERID3"]
pub type MASTERID3_R = crate::FieldReader;
#[doc = "Field `MASTERID4` reader - MASTERID4"]
pub type MASTERID4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - MASTERID1"]
    #[inline(always)]
    pub fn masterid1(&self) -> MASTERID1_R {
        MASTERID1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MASTERID2"]
    #[inline(always)]
    pub fn masterid2(&self) -> MASTERID2_R {
        MASTERID2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - MASTERID3"]
    #[inline(always)]
    pub fn masterid3(&self) -> MASTERID3_R {
        MASTERID3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MASTERID4"]
    #[inline(always)]
    pub fn masterid4(&self) -> MASTERID4_R {
        MASTERID4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "HSEM hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_HWCFGR2rs;
impl crate::RegisterSpec for HSEM_HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_hwcfgr2::R`](R) reader structure"]
impl crate::Readable for HSEM_HWCFGR2rs {}
#[doc = "`reset()` method sets HSEM_HWCFGR2 to value 0x21"]
impl crate::Resettable for HSEM_HWCFGR2rs {
    const RESET_VALUE: u32 = 0x21;
}
