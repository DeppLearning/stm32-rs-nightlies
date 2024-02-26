#[doc = "Register `CPAR3` reader"]
pub type R = crate::R<CPAR3rs>;
#[doc = "Register `CPAR3` writer"]
pub type W = crate::W<CPAR3rs>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<CPAR3rs> {
        PA_W::new(self, 0)
    }
}
#[doc = "channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPAR3rs;
impl crate::RegisterSpec for CPAR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar3::R`](R) reader structure"]
impl crate::Readable for CPAR3rs {}
#[doc = "`write(|w| ..)` method takes [`cpar3::W`](W) writer structure"]
impl crate::Writable for CPAR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPAR3 to value 0"]
impl crate::Resettable for CPAR3rs {
    const RESET_VALUE: u32 = 0;
}
