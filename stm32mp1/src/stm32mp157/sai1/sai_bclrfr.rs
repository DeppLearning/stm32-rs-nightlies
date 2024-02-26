#[doc = "Register `SAI_BCLRFR` writer"]
pub type W = crate::W<SAI_BCLRFRrs>;
#[doc = "Field `COVRUDR` writer - COVRUDR"]
pub type COVRUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUTEDET` writer - CMUTEDET"]
pub type CMUTEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWCKCFG` writer - CWCKCFG"]
pub type CWCKCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCNRDY` writer - CCNRDY"]
pub type CCNRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAFSDET` writer - CAFSDET"]
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLFSDET` writer - CLFSDET"]
pub type CLFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - COVRUDR"]
    #[inline(always)]
    #[must_use]
    pub fn covrudr(&mut self) -> COVRUDR_W<SAI_BCLRFRrs> {
        COVRUDR_W::new(self, 0)
    }
    #[doc = "Bit 1 - CMUTEDET"]
    #[inline(always)]
    #[must_use]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<SAI_BCLRFRrs> {
        CMUTEDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - CWCKCFG"]
    #[inline(always)]
    #[must_use]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<SAI_BCLRFRrs> {
        CWCKCFG_W::new(self, 2)
    }
    #[doc = "Bit 4 - CCNRDY"]
    #[inline(always)]
    #[must_use]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<SAI_BCLRFRrs> {
        CCNRDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - CAFSDET"]
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<SAI_BCLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    #[doc = "Bit 6 - CLFSDET"]
    #[inline(always)]
    #[must_use]
    pub fn clfsdet(&mut self) -> CLFSDET_W<SAI_BCLRFRrs> {
        CLFSDET_W::new(self, 6)
    }
}
#[doc = "Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_BCLRFRrs;
impl crate::RegisterSpec for SAI_BCLRFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sai_bclrfr::W`](W) writer structure"]
impl crate::Writable for SAI_BCLRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAI_BCLRFR to value 0"]
impl crate::Resettable for SAI_BCLRFRrs {
    const RESET_VALUE: u32 = 0;
}
