#[doc = "Register `SDMMC_IDMABAR` reader"]
pub type R = crate::R<SDMMC_IDMABARrs>;
#[doc = "Register `SDMMC_IDMABAR` writer"]
pub type W = crate::W<SDMMC_IDMABARrs>;
#[doc = "Field `IDMABA` reader - IDMABA"]
pub type IDMABA_R = crate::FieldReader<u32>;
#[doc = "Field `IDMABA` writer - IDMABA"]
pub type IDMABA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - IDMABA"]
    #[inline(always)]
    pub fn idmaba(&self) -> IDMABA_R {
        IDMABA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - IDMABA"]
    #[inline(always)]
    #[must_use]
    pub fn idmaba(&mut self) -> IDMABA_W<SDMMC_IDMABARrs> {
        IDMABA_W::new(self, 2)
    }
}
#[doc = "SDMMC IDMA linked list memory base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmabar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmabar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_IDMABARrs;
impl crate::RegisterSpec for SDMMC_IDMABARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_idmabar::R`](R) reader structure"]
impl crate::Readable for SDMMC_IDMABARrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_idmabar::W`](W) writer structure"]
impl crate::Writable for SDMMC_IDMABARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMABAR to value 0"]
impl crate::Resettable for SDMMC_IDMABARrs {
    const RESET_VALUE: u32 = 0;
}
