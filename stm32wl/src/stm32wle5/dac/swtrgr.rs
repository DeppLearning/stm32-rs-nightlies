#[doc = "Register `SWTRGR` writer"]
pub type W = crate::W<SWTRGRrs>;
#[doc = "DAC channel1 software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG1 {
    #[doc = "0: No trigger"]
    NoTrigger = 0,
    #[doc = "1: Trigger"]
    Trigger = 1,
}
impl From<SWTRIG1> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger"]
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG1>;
impl<'a, REG> SWTRIG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::NoTrigger)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<SWTRGRrs> {
        SWTRIG1_W::new(self, 0)
    }
}
#[doc = "software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRGRrs;
impl crate::RegisterSpec for SWTRGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure"]
impl crate::Writable for SWTRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWTRGR to value 0"]
impl crate::Resettable for SWTRGRrs {
    const RESET_VALUE: u32 = 0;
}
