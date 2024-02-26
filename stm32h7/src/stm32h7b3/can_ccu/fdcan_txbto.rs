#[doc = "Register `FDCAN_TXBTO` reader"]
pub type R = crate::R<FDCAN_TXBTOrs>;
#[doc = "Register `FDCAN_TXBTO` writer"]
pub type W = crate::W<FDCAN_TXBTOrs>;
#[doc = "Field `TO` reader - Transmission Occurred."]
pub type TO_R = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - Transmission Occurred."]
pub type TO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<FDCAN_TXBTOrs> {
        TO_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbto::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBTOrs;
impl crate::RegisterSpec for FDCAN_TXBTOrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbto::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBTOrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbto::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBTOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBTO to value 0"]
impl crate::Resettable for FDCAN_TXBTOrs {
    const RESET_VALUE: u32 = 0;
}
