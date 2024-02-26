#[doc = "Register `RXBC` reader"]
pub type R = crate::R<RXBCrs>;
#[doc = "Register `RXBC` writer"]
pub type W = crate::W<RXBCrs>;
#[doc = "Field `RBSA` reader - Rx Buffer Start Address"]
pub type RBSA_R = crate::FieldReader<u16>;
#[doc = "Field `RBSA` writer - Rx Buffer Start Address"]
pub type RBSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 2:15 - Rx Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn rbsa(&mut self) -> RBSA_W<RXBCrs> {
        RBSA_W::new(self, 2)
    }
}
#[doc = "FDCAN Rx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXBCrs;
impl crate::RegisterSpec for RXBCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbc::R`](R) reader structure"]
impl crate::Readable for RXBCrs {}
#[doc = "`write(|w| ..)` method takes [`rxbc::W`](W) writer structure"]
impl crate::Writable for RXBCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXBC to value 0"]
impl crate::Resettable for RXBCrs {
    const RESET_VALUE: u32 = 0;
}
