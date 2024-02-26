#[doc = "Register `DFSDM_FLT5AWHTR` reader"]
pub type R = crate::R<DFSDM_FLT5AWHTRrs>;
#[doc = "Register `DFSDM_FLT5AWHTR` writer"]
pub type W = crate::W<DFSDM_FLT5AWHTRrs>;
#[doc = "Field `BKAWH` reader - BKAWH"]
pub type BKAWH_R = crate::FieldReader;
#[doc = "Field `BKAWH` writer - BKAWH"]
pub type BKAWH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWHT` reader - AWHT"]
pub type AWHT_R = crate::FieldReader<u32>;
#[doc = "Field `AWHT` writer - AWHT"]
pub type AWHT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - BKAWH"]
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - AWHT"]
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - BKAWH"]
    #[inline(always)]
    #[must_use]
    pub fn bkawh(&mut self) -> BKAWH_W<DFSDM_FLT5AWHTRrs> {
        BKAWH_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - AWHT"]
    #[inline(always)]
    #[must_use]
    pub fn awht(&mut self) -> AWHT_W<DFSDM_FLT5AWHTRrs> {
        AWHT_W::new(self, 8)
    }
}
#[doc = "DFSDM filter 5 analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt5awhtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt5awhtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT5AWHTRrs;
impl crate::RegisterSpec for DFSDM_FLT5AWHTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt5awhtr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT5AWHTRrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt5awhtr::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT5AWHTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT5AWHTR to value 0"]
impl crate::Resettable for DFSDM_FLT5AWHTRrs {
    const RESET_VALUE: u32 = 0;
}
