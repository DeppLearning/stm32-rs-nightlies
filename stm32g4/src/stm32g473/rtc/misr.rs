#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISRrs>;
#[doc = "Field `ALRAMF` reader - ALRAMF"]
pub type ALRAMF_R = crate::BitReader;
#[doc = "Field `ALRBMF` reader - ALRBMF"]
pub type ALRBMF_R = crate::BitReader;
#[doc = "Field `WUTMF` reader - WUTMF"]
pub type WUTMF_R = crate::BitReader;
#[doc = "Field `TSMF` reader - TSMF"]
pub type TSMF_R = crate::BitReader;
#[doc = "Field `TSOVMF` reader - TSOVMF"]
pub type TSOVMF_R = crate::BitReader;
#[doc = "Field `ITSMF` reader - ITSMF"]
pub type ITSMF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ALRAMF"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBMF"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTMF"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSMF"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TSOVMF"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITSMF"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISRrs {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
