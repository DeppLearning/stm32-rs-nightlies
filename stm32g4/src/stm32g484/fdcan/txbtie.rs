#[doc = "Register `TXBTIE` reader"]
pub type R = crate::R<TXBTIErs>;
#[doc = "Register `TXBTIE` writer"]
pub type W = crate::W<TXBTIErs>;
#[doc = "Field `TIE` reader - TIE"]
pub type TIE_R = crate::FieldReader;
#[doc = "Field `TIE` writer - TIE"]
pub type TIE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - TIE"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<TXBTIErs> {
        TIE_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBTIErs;
impl crate::RegisterSpec for TXBTIErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbtie::R`](R) reader structure"]
impl crate::Readable for TXBTIErs {}
#[doc = "`write(|w| ..)` method takes [`txbtie::W`](W) writer structure"]
impl crate::Writable for TXBTIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TXBTIErs {
    const RESET_VALUE: u32 = 0;
}
