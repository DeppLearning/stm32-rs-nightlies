#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<FTSR2rs>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<FTSR2rs>;
#[doc = "Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT46 {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<FT46> for bool {
    #[inline(always)]
    fn from(variant: FT46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT46` reader - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT46_R = crate::BitReader<FT46>;
impl FT46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT46 {
        match self.bits {
            false => FT46::Disabled,
            true => FT46::Enabled,
        }
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT46::Disabled
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT46::Enabled
    }
}
#[doc = "Field `FT46` writer - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT46_W<'a, REG> = crate::BitWriter<'a, REG, FT46>;
impl<'a, REG> FT46_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT46::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT46::Enabled)
    }
}
#[doc = "Field `FT50` reader - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub use FT46_R as FT50_R;
#[doc = "Field `FT53` reader - Falling trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub use FT46_R as FT53_R;
#[doc = "Field `FT50` writer - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub use FT46_W as FT50_W;
#[doc = "Field `FT53` writer - Falling trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub use FT46_W as FT53_W;
impl R {
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn ft46(&self) -> FT46_R {
        FT46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn ft50(&self) -> FT50_R {
        FT50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn ft53(&self) -> FT53_R {
        FT53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ft46(&mut self) -> FT46_W<FTSR2rs> {
        FT46_W::new(self, 14)
    }
    #[doc = "Bit 18 - Falling trigger event configuration bit of configurable event input x &lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ft50(&mut self) -> FT50_W<FTSR2rs> {
        FT50_W::new(self, 18)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, FTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FTx can only be accessed with secure access. Non-secure write to this FTx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ft53(&mut self) -> FT53_W<FTSR2rs> {
        FT53_W::new(self, 21)
    }
}
#[doc = "EXTI falling trigger selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for FTSR2rs {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2rs {
    const RESET_VALUE: u32 = 0;
}
