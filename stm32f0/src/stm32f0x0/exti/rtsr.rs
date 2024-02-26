#[doc = "Register `RTSR` reader"]
pub type R = crate::R<RTSRrs>;
#[doc = "Register `RTSR` writer"]
pub type W = crate::W<RTSRrs>;
#[doc = "Rising trigger event configuration of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR0 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<TR0> for bool {
    #[inline(always)]
    fn from(variant: TR0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR0` reader - Rising trigger event configuration of line 0"]
pub type TR0_R = crate::BitReader<TR0>;
impl TR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TR0 {
        match self.bits {
            false => TR0::Disabled,
            true => TR0::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR0::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR0::Enabled
    }
}
#[doc = "Field `TR0` writer - Rising trigger event configuration of line 0"]
pub type TR0_W<'a, REG> = crate::BitWriter<'a, REG, TR0>;
impl<'a, REG> TR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR0::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR0::Enabled)
    }
}
#[doc = "Field `TR1` reader - Rising trigger event configuration of line 1"]
pub use TR0_R as TR1_R;
#[doc = "Field `TR2` reader - Rising trigger event configuration of line 2"]
pub use TR0_R as TR2_R;
#[doc = "Field `TR3` reader - Rising trigger event configuration of line 3"]
pub use TR0_R as TR3_R;
#[doc = "Field `TR4` reader - Rising trigger event configuration of line 4"]
pub use TR0_R as TR4_R;
#[doc = "Field `TR5` reader - Rising trigger event configuration of line 5"]
pub use TR0_R as TR5_R;
#[doc = "Field `TR6` reader - Rising trigger event configuration of line 6"]
pub use TR0_R as TR6_R;
#[doc = "Field `TR7` reader - Rising trigger event configuration of line 7"]
pub use TR0_R as TR7_R;
#[doc = "Field `TR8` reader - Rising trigger event configuration of line 8"]
pub use TR0_R as TR8_R;
#[doc = "Field `TR9` reader - Rising trigger event configuration of line 9"]
pub use TR0_R as TR9_R;
#[doc = "Field `TR10` reader - Rising trigger event configuration of line 10"]
pub use TR0_R as TR10_R;
#[doc = "Field `TR11` reader - Rising trigger event configuration of line 11"]
pub use TR0_R as TR11_R;
#[doc = "Field `TR12` reader - Rising trigger event configuration of line 12"]
pub use TR0_R as TR12_R;
#[doc = "Field `TR13` reader - Rising trigger event configuration of line 13"]
pub use TR0_R as TR13_R;
#[doc = "Field `TR14` reader - Rising trigger event configuration of line 14"]
pub use TR0_R as TR14_R;
#[doc = "Field `TR15` reader - Rising trigger event configuration of line 15"]
pub use TR0_R as TR15_R;
#[doc = "Field `TR16` reader - Rising trigger event configuration of line 16"]
pub use TR0_R as TR16_R;
#[doc = "Field `TR17` reader - Rising trigger event configuration of line 17"]
pub use TR0_R as TR17_R;
#[doc = "Field `TR19` reader - Rising trigger event configuration of line 19"]
pub use TR0_R as TR19_R;
#[doc = "Field `TR1` writer - Rising trigger event configuration of line 1"]
pub use TR0_W as TR1_W;
#[doc = "Field `TR2` writer - Rising trigger event configuration of line 2"]
pub use TR0_W as TR2_W;
#[doc = "Field `TR3` writer - Rising trigger event configuration of line 3"]
pub use TR0_W as TR3_W;
#[doc = "Field `TR4` writer - Rising trigger event configuration of line 4"]
pub use TR0_W as TR4_W;
#[doc = "Field `TR5` writer - Rising trigger event configuration of line 5"]
pub use TR0_W as TR5_W;
#[doc = "Field `TR6` writer - Rising trigger event configuration of line 6"]
pub use TR0_W as TR6_W;
#[doc = "Field `TR7` writer - Rising trigger event configuration of line 7"]
pub use TR0_W as TR7_W;
#[doc = "Field `TR8` writer - Rising trigger event configuration of line 8"]
pub use TR0_W as TR8_W;
#[doc = "Field `TR9` writer - Rising trigger event configuration of line 9"]
pub use TR0_W as TR9_W;
#[doc = "Field `TR10` writer - Rising trigger event configuration of line 10"]
pub use TR0_W as TR10_W;
#[doc = "Field `TR11` writer - Rising trigger event configuration of line 11"]
pub use TR0_W as TR11_W;
#[doc = "Field `TR12` writer - Rising trigger event configuration of line 12"]
pub use TR0_W as TR12_W;
#[doc = "Field `TR13` writer - Rising trigger event configuration of line 13"]
pub use TR0_W as TR13_W;
#[doc = "Field `TR14` writer - Rising trigger event configuration of line 14"]
pub use TR0_W as TR14_W;
#[doc = "Field `TR15` writer - Rising trigger event configuration of line 15"]
pub use TR0_W as TR15_W;
#[doc = "Field `TR16` writer - Rising trigger event configuration of line 16"]
pub use TR0_W as TR16_W;
#[doc = "Field `TR17` writer - Rising trigger event configuration of line 17"]
pub use TR0_W as TR17_W;
#[doc = "Field `TR19` writer - Rising trigger event configuration of line 19"]
pub use TR0_W as TR19_W;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn tr0(&self) -> TR0_R {
        TR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn tr1(&self) -> TR1_R {
        TR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn tr2(&self) -> TR2_R {
        TR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn tr3(&self) -> TR3_R {
        TR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn tr4(&self) -> TR4_R {
        TR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn tr5(&self) -> TR5_R {
        TR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn tr6(&self) -> TR6_R {
        TR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn tr7(&self) -> TR7_R {
        TR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn tr8(&self) -> TR8_R {
        TR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn tr9(&self) -> TR9_R {
        TR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn tr10(&self) -> TR10_R {
        TR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn tr11(&self) -> TR11_R {
        TR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn tr12(&self) -> TR12_R {
        TR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn tr13(&self) -> TR13_R {
        TR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn tr14(&self) -> TR14_R {
        TR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn tr15(&self) -> TR15_R {
        TR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn tr16(&self) -> TR16_R {
        TR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn tr17(&self) -> TR17_R {
        TR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    pub fn tr19(&self) -> TR19_R {
        TR19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn tr0(&mut self) -> TR0_W<RTSRrs> {
        TR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn tr1(&mut self) -> TR1_W<RTSRrs> {
        TR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn tr2(&mut self) -> TR2_W<RTSRrs> {
        TR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn tr3(&mut self) -> TR3_W<RTSRrs> {
        TR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn tr4(&mut self) -> TR4_W<RTSRrs> {
        TR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn tr5(&mut self) -> TR5_W<RTSRrs> {
        TR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn tr6(&mut self) -> TR6_W<RTSRrs> {
        TR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn tr7(&mut self) -> TR7_W<RTSRrs> {
        TR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn tr8(&mut self) -> TR8_W<RTSRrs> {
        TR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn tr9(&mut self) -> TR9_W<RTSRrs> {
        TR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn tr10(&mut self) -> TR10_W<RTSRrs> {
        TR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn tr11(&mut self) -> TR11_W<RTSRrs> {
        TR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn tr12(&mut self) -> TR12_W<RTSRrs> {
        TR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn tr13(&mut self) -> TR13_W<RTSRrs> {
        TR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn tr14(&mut self) -> TR14_W<RTSRrs> {
        TR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn tr15(&mut self) -> TR15_W<RTSRrs> {
        TR15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn tr16(&mut self) -> TR16_W<RTSRrs> {
        TR16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn tr17(&mut self) -> TR17_W<RTSRrs> {
        TR17_W::new(self, 17)
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn tr19(&mut self) -> TR19_W<RTSRrs> {
        TR19_W::new(self, 19)
    }
}
#[doc = "Rising Trigger selection register (EXTI_RTSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSRrs;
impl crate::RegisterSpec for RTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr::R`](R) reader structure"]
impl crate::Readable for RTSRrs {}
#[doc = "`write(|w| ..)` method takes [`rtsr::W`](W) writer structure"]
impl crate::Writable for RTSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR to value 0"]
impl crate::Resettable for RTSRrs {
    const RESET_VALUE: u32 = 0;
}
