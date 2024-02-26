#[doc = "Register `RXF1A` reader"]
pub type R = crate::R<RXF1Ars>;
#[doc = "Register `RXF1A` writer"]
pub type W = crate::W<RXF1Ars>;
#[doc = "Field `F1AI` reader - F1AI"]
pub type F1AI_R = crate::FieldReader;
#[doc = "Field `F1AI` writer - F1AI"]
pub type F1AI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - F1AI"]
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - F1AI"]
    #[inline(always)]
    #[must_use]
    pub fn f1ai(&mut self) -> F1AI_W<RXF1Ars> {
        F1AI_W::new(self, 0)
    }
}
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF1Ars;
impl crate::RegisterSpec for RXF1Ars {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1a::R`](R) reader structure"]
impl crate::Readable for RXF1Ars {}
#[doc = "`write(|w| ..)` method takes [`rxf1a::W`](W) writer structure"]
impl crate::Writable for RXF1Ars {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF1A to value 0"]
impl crate::Resettable for RXF1Ars {
    const RESET_VALUE: u32 = 0;
}
