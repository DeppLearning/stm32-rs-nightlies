#[doc = "Register `CSELR` reader"]
pub type R = crate::R<CSELRrs>;
#[doc = "Register `CSELR` writer"]
pub type W = crate::W<CSELRrs>;
#[doc = "Field `MA` reader - peripheral address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - peripheral address"]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CSELRrs> {
        MA_W::new(self, 0)
    }
}
#[doc = "channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSELRrs;
impl crate::RegisterSpec for CSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cselr::R`](R) reader structure"]
impl crate::Readable for CSELRrs {}
#[doc = "`write(|w| ..)` method takes [`cselr::W`](W) writer structure"]
impl crate::Writable for CSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSELR to value 0"]
impl crate::Resettable for CSELRrs {
    const RESET_VALUE: u32 = 0;
}
