#[doc = "Register `CNTH` reader"]
pub type R = crate::R<CNTHrs>;
#[doc = "Register `CNTH` writer"]
pub type W = crate::W<CNTHrs>;
#[doc = "Field `CNTH` reader - RTC counter register high"]
pub type CNTH_R = crate::FieldReader<u16>;
#[doc = "Field `CNTH` writer - RTC counter register high"]
pub type CNTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnth(&self) -> CNTH_R {
        CNTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    #[must_use]
    pub fn cnth(&mut self) -> CNTH_W<CNTHrs> {
        CNTH_W::new(self, 0)
    }
}
#[doc = "RTC Counter Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTHrs;
impl crate::RegisterSpec for CNTHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnth::R`](R) reader structure"]
impl crate::Readable for CNTHrs {}
#[doc = "`write(|w| ..)` method takes [`cnth::W`](W) writer structure"]
impl crate::Writable for CNTHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CNTHrs {
    const RESET_VALUE: u32 = 0;
}
