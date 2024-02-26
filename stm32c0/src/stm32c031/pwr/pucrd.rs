#[doc = "Register `PUCRD` reader"]
pub type R = crate::R<PUCRDrs>;
#[doc = "Register `PUCRD` writer"]
pub type W = crate::W<PUCRDrs>;
#[doc = "Field `PU0` reader - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU0_R = crate::BitReader;
#[doc = "Field `PU0` writer - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU1_R = crate::BitReader;
#[doc = "Field `PU1` writer - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU2_R = crate::BitReader;
#[doc = "Field `PU2` writer - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU3` reader - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU3_R = crate::BitReader;
#[doc = "Field `PU3` writer - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PUCRDrs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PUCRDrs> {
        PU1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PUCRDrs> {
        PU2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port D pull-up bit i (i = 3 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PUCRDrs> {
        PU3_W::new(self, 3)
    }
}
#[doc = "PWR Port D pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRDrs;
impl crate::RegisterSpec for PUCRDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrd::R`](R) reader structure"]
impl crate::Readable for PUCRDrs {}
#[doc = "`write(|w| ..)` method takes [`pucrd::W`](W) writer structure"]
impl crate::Writable for PUCRDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRD to value 0"]
impl crate::Resettable for PUCRDrs {
    const RESET_VALUE: u32 = 0;
}
