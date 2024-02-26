#[doc = "Register `PERBR` reader"]
pub type R = crate::R<PERBRrs>;
#[doc = "Register `PERBR` writer"]
pub type W = crate::W<PERBRrs>;
#[doc = "Field `PERx` reader - Timerx Period value"]
pub type PERX_R = crate::FieldReader<u16>;
#[doc = "Field `PERx` writer - Timerx Period value"]
pub type PERX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PERX_W<PERBRrs> {
        PERX_W::new(self, 0)
    }
}
#[doc = "Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERBRrs;
impl crate::RegisterSpec for PERBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perbr::R`](R) reader structure"]
impl crate::Readable for PERBRrs {}
#[doc = "`write(|w| ..)` method takes [`perbr::W`](W) writer structure"]
impl crate::Writable for PERBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERBR to value 0xffff"]
impl crate::Resettable for PERBRrs {
    const RESET_VALUE: u32 = 0xffff;
}
