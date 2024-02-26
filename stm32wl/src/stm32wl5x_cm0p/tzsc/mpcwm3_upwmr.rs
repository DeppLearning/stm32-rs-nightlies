#[doc = "Register `MPCWM3_UPWMR` reader"]
pub type R = crate::R<MPCWM3_UPWMRrs>;
#[doc = "Register `MPCWM3_UPWMR` writer"]
pub type W = crate::W<MPCWM3_UPWMRrs>;
#[doc = "Field `LGTH` reader - LGTH"]
pub type LGTH_R = crate::FieldReader<u16>;
#[doc = "Field `LGTH` writer - LGTH"]
pub type LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - LGTH"]
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - LGTH"]
    #[inline(always)]
    #[must_use]
    pub fn lgth(&mut self) -> LGTH_W<MPCWM3_UPWMRrs> {
        LGTH_W::new(self, 16)
    }
}
#[doc = "Unprivileged Water Mark 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm3_upwmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm3_upwmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCWM3_UPWMRrs;
impl crate::RegisterSpec for MPCWM3_UPWMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcwm3_upwmr::R`](R) reader structure"]
impl crate::Readable for MPCWM3_UPWMRrs {}
#[doc = "`write(|w| ..)` method takes [`mpcwm3_upwmr::W`](W) writer structure"]
impl crate::Writable for MPCWM3_UPWMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCWM3_UPWMR to value 0x0fff_0000"]
impl crate::Resettable for MPCWM3_UPWMRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
