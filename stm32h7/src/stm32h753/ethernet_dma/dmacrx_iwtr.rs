#[doc = "Register `DMACRxIWTR` reader"]
pub type R = crate::R<DMACRX_IWTRrs>;
#[doc = "Register `DMACRxIWTR` writer"]
pub type W = crate::W<DMACRX_IWTRrs>;
#[doc = "Field `RWT` reader - Receive Interrupt Watchdog Timer Count"]
pub type RWT_R = crate::FieldReader;
#[doc = "Field `RWT` writer - Receive Interrupt Watchdog Timer Count"]
pub type RWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMACRX_IWTRrs> {
        RWT_W::new(self, 0)
    }
}
#[doc = "Channel Rx interrupt watchdog timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_iwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRX_IWTRrs;
impl crate::RegisterSpec for DMACRX_IWTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_iwtr::R`](R) reader structure"]
impl crate::Readable for DMACRX_IWTRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_iwtr::W`](W) writer structure"]
impl crate::Writable for DMACRX_IWTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRxIWTR to value 0"]
impl crate::Resettable for DMACRX_IWTRrs {
    const RESET_VALUE: u32 = 0;
}
