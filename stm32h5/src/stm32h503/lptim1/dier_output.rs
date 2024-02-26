#[doc = "Register `DIER_output` reader"]
pub type R = crate::R<DIER_OUTPUTrs>;
#[doc = "Register `DIER_output` writer"]
pub type W = crate::W<DIER_OUTPUTrs>;
#[doc = "Field `CC1IE` reader - Capture/compare 1 interrupt enable"]
pub type CC1IE_R = crate::BitReader;
#[doc = "Field `CC1IE` writer - Capture/compare 1 interrupt enable"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ARRMIE_R = crate::BitReader;
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_R = crate::BitReader;
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1OKIE` reader - Compare register 1 update OK interrupt enable"]
pub type CMP1OKIE_R = crate::BitReader;
#[doc = "Field `CMP1OKIE` writer - Compare register 1 update OK interrupt enable"]
pub type CMP1OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_R = crate::BitReader;
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_R = crate::BitReader;
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEIE` reader - Update event interrupt enable"]
pub type UEIE_R = crate::BitReader;
#[doc = "Field `UEIE` writer - Update event interrupt enable"]
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOKIE` reader - Repetition register update OK interrupt Enable"]
pub type REPOKIE_R = crate::BitReader;
#[doc = "Field `REPOKIE` writer - Repetition register update OK interrupt Enable"]
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_R = crate::BitReader;
#[doc = "Field `CC2IE` writer - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2OKIE` reader - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CMP2OKIE_R = crate::BitReader;
#[doc = "Field `CMP2OKIE` writer - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CMP2OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEDE` reader - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_R = crate::BitReader;
#[doc = "Field `UEDE` writer - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register 1 update OK interrupt enable"]
    #[inline(always)]
    pub fn cmp1okie(&self) -> CMP1OKIE_R {
        CMP1OKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cmp2okie(&self) -> CMP2OKIE_R {
        CMP2OKIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn uede(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<DIER_OUTPUTrs> {
        CC1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<DIER_OUTPUTrs> {
        ARRMIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<DIER_OUTPUTrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register 1 update OK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1okie(&mut self) -> CMP1OKIE_W<DIER_OUTPUTrs> {
        CMP1OKIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<DIER_OUTPUTrs> {
        ARROKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<DIER_OUTPUTrs> {
        UPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<DIER_OUTPUTrs> {
        DOWNIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ueie(&mut self) -> UEIE_W<DIER_OUTPUTrs> {
        UEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn repokie(&mut self) -> REPOKIE_W<DIER_OUTPUTrs> {
        REPOKIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIER_OUTPUTrs> {
        CC2IE_W::new(self, 9)
    }
    #[doc = "Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2okie(&mut self) -> CMP2OKIE_W<DIER_OUTPUTrs> {
        CMP2OKIE_W::new(self, 19)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn uede(&mut self) -> UEDE_W<DIER_OUTPUTrs> {
        UEDE_W::new(self, 23)
    }
}
#[doc = "LPTIM1 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIER_OUTPUTrs;
impl crate::RegisterSpec for DIER_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier_output::R`](R) reader structure"]
impl crate::Readable for DIER_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`dier_output::W`](W) writer structure"]
impl crate::Writable for DIER_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER_output to value 0"]
impl crate::Resettable for DIER_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
