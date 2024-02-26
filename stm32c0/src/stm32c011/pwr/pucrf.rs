#[doc = "Register `PUCRF` reader"]
pub type R = crate::R<PUCRFrs>;
#[doc = "Register `PUCRF` writer"]
pub type W = crate::W<PUCRFrs>;
#[doc = "Field `PU2` reader - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
pub type PU2_R = crate::BitReader;
#[doc = "Field `PU2` writer - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PUCRFrs> {
        PU2_W::new(self, 2)
    }
}
#[doc = "PWR Port F pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRFrs;
impl crate::RegisterSpec for PUCRFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrf::R`](R) reader structure"]
impl crate::Readable for PUCRFrs {}
#[doc = "`write(|w| ..)` method takes [`pucrf::W`](W) writer structure"]
impl crate::Writable for PUCRFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRF to value 0"]
impl crate::Resettable for PUCRFrs {
    const RESET_VALUE: u32 = 0;
}
