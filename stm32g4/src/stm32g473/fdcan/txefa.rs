#[doc = "Register `TXEFA` reader"]
pub type R = crate::R<TXEFArs>;
#[doc = "Register `TXEFA` writer"]
pub type W = crate::W<TXEFArs>;
#[doc = "Field `EFAI` reader - EFAI"]
pub type EFAI_R = crate::FieldReader;
#[doc = "Field `EFAI` writer - EFAI"]
pub type EFAI_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - EFAI"]
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - EFAI"]
    #[inline(always)]
    #[must_use]
    pub fn efai(&mut self) -> EFAI_W<TXEFArs> {
        EFAI_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFArs;
impl crate::RegisterSpec for TXEFArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefa::R`](R) reader structure"]
impl crate::Readable for TXEFArs {}
#[doc = "`write(|w| ..)` method takes [`txefa::W`](W) writer structure"]
impl crate::Writable for TXEFArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXEFA to value 0"]
impl crate::Resettable for TXEFArs {
    const RESET_VALUE: u32 = 0;
}
