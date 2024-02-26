#[doc = "Register `DFSDM_FLT0JCHGR` reader"]
pub type R = crate::R<DFSDM_FLT0JCHGRrs>;
#[doc = "Register `DFSDM_FLT0JCHGR` writer"]
pub type W = crate::W<DFSDM_FLT0JCHGRrs>;
#[doc = "Field `JCHG` reader - Injected channel group selection"]
pub type JCHG_R = crate::FieldReader;
#[doc = "Field `JCHG` writer - Injected channel group selection"]
pub type JCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    #[must_use]
    pub fn jchg(&mut self) -> JCHG_W<DFSDM_FLT0JCHGRrs> {
        JCHG_W::new(self, 0)
    }
}
#[doc = "injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0jchgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0jchgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0JCHGRrs;
impl crate::RegisterSpec for DFSDM_FLT0JCHGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0jchgr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0JCHGRrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt0jchgr::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT0JCHGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT0JCHGR to value 0x01"]
impl crate::Resettable for DFSDM_FLT0JCHGRrs {
    const RESET_VALUE: u32 = 0x01;
}
