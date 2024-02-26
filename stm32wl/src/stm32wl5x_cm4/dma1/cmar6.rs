#[doc = "Register `CMAR6` reader"]
pub type R = crate::R<CMAR6rs>;
#[doc = "Register `CMAR6` writer"]
pub type W = crate::W<CMAR6rs>;
#[doc = "Field `MA` reader - peripheral address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - peripheral address"]
pub type MA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
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
    pub fn ma(&mut self) -> MA_W<CMAR6rs> {
        MA_W::new(self, 0)
    }
}
#[doc = "channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMAR6rs;
impl crate::RegisterSpec for CMAR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar6::R`](R) reader structure"]
impl crate::Readable for CMAR6rs {}
#[doc = "`write(|w| ..)` method takes [`cmar6::W`](W) writer structure"]
impl crate::Writable for CMAR6rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMAR6 to value 0"]
impl crate::Resettable for CMAR6rs {
    const RESET_VALUE: u32 = 0;
}
