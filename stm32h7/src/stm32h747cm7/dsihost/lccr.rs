#[doc = "Register `LCCR` reader"]
pub type R = crate::R<LCCRrs>;
#[doc = "Register `LCCR` writer"]
pub type W = crate::W<LCCRrs>;
#[doc = "Field `CMDSIZE` reader - Command size"]
pub type CMDSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `CMDSIZE` writer - Command size"]
pub type CMDSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Command size"]
    #[inline(always)]
    pub fn cmdsize(&self) -> CMDSIZE_R {
        CMDSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Command size"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsize(&mut self) -> CMDSIZE_W<LCCRrs> {
        CMDSIZE_W::new(self, 0)
    }
}
#[doc = "DSI Host LTDC command configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCCRrs;
impl crate::RegisterSpec for LCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lccr::R`](R) reader structure"]
impl crate::Readable for LCCRrs {}
#[doc = "`write(|w| ..)` method takes [`lccr::W`](W) writer structure"]
impl crate::Writable for LCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCCR to value 0"]
impl crate::Resettable for LCCRrs {
    const RESET_VALUE: u32 = 0;
}
