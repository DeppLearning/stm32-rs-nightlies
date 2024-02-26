#[doc = "Register `DFSDM_CH6CFGR2` reader"]
pub type R = crate::R<DFSDM_CH6CFGR2rs>;
#[doc = "Register `DFSDM_CH6CFGR2` writer"]
pub type W = crate::W<DFSDM_CH6CFGR2rs>;
#[doc = "Field `DTRBS` reader - DTRBS"]
pub type DTRBS_R = crate::FieldReader;
#[doc = "Field `DTRBS` writer - DTRBS"]
pub type DTRBS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET` reader - OFFSET"]
pub type OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET` writer - OFFSET"]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    #[must_use]
    pub fn dtrbs(&mut self) -> DTRBS_W<DFSDM_CH6CFGR2rs> {
        DTRBS_W::new(self, 3)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<DFSDM_CH6CFGR2rs> {
        OFFSET_W::new(self, 8)
    }
}
#[doc = "This register specifies the parameters used by channel y.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_ch6cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_ch6cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_CH6CFGR2rs;
impl crate::RegisterSpec for DFSDM_CH6CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_ch6cfgr2::R`](R) reader structure"]
impl crate::Readable for DFSDM_CH6CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_ch6cfgr2::W`](W) writer structure"]
impl crate::Writable for DFSDM_CH6CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_CH6CFGR2 to value 0"]
impl crate::Resettable for DFSDM_CH6CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
