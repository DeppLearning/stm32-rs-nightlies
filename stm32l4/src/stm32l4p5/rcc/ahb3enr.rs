#[doc = "Register `AHB3ENR` reader"]
pub type R = crate::R<AHB3ENRrs>;
#[doc = "Register `AHB3ENR` writer"]
pub type W = crate::W<AHB3ENRrs>;
#[doc = "Flexible memory controller clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCEN {
    #[doc = "0: FMC clock disabled"]
    Disabled = 0,
    #[doc = "1: FMC clock enabled"]
    Enabled = 1,
}
impl From<FMCEN> for bool {
    #[inline(always)]
    fn from(variant: FMCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCEN` reader - Flexible memory controller clock enable"]
pub type FMCEN_R = crate::BitReader<FMCEN>;
impl FMCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMCEN {
        match self.bits {
            false => FMCEN::Disabled,
            true => FMCEN::Enabled,
        }
    }
    #[doc = "FMC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCEN::Disabled
    }
    #[doc = "FMC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCEN::Enabled
    }
}
#[doc = "Field `FMCEN` writer - Flexible memory controller clock enable"]
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG, FMCEN>;
impl<'a, REG> FMCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FMC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCEN::Disabled)
    }
    #[doc = "FMC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCEN::Enabled)
    }
}
#[doc = "OctoSPI1 memory interface clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1EN {
    #[doc = "0: OctoSPI x clock disabled"]
    Disabled = 0,
    #[doc = "1: OctoSPI x clock enabled"]
    Enabled = 1,
}
impl From<OSPI1EN> for bool {
    #[inline(always)]
    fn from(variant: OSPI1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSPI1EN` reader - OctoSPI1 memory interface clock enable"]
pub type OSPI1EN_R = crate::BitReader<OSPI1EN>;
impl OSPI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPI1EN {
        match self.bits {
            false => OSPI1EN::Disabled,
            true => OSPI1EN::Enabled,
        }
    }
    #[doc = "OctoSPI x clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPI1EN::Disabled
    }
    #[doc = "OctoSPI x clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPI1EN::Enabled
    }
}
#[doc = "Field `OSPI1EN` writer - OctoSPI1 memory interface clock enable"]
pub type OSPI1EN_W<'a, REG> = crate::BitWriter<'a, REG, OSPI1EN>;
impl<'a, REG> OSPI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OctoSPI x clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1EN::Disabled)
    }
    #[doc = "OctoSPI x clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1EN::Enabled)
    }
}
#[doc = "Field `OSPI2EN` reader - OSPI2EN memory interface clock enable"]
pub use OSPI1EN_R as OSPI2EN_R;
#[doc = "Field `OSPI2EN` writer - OSPI2EN memory interface clock enable"]
pub use OSPI1EN_W as OSPI2EN_W;
impl R {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clock enable"]
    #[inline(always)]
    pub fn ospi1en(&self) -> OSPI1EN_R {
        OSPI1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OSPI2EN memory interface clock enable"]
    #[inline(always)]
    pub fn ospi2en(&self) -> OSPI2EN_R {
        OSPI2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<AHB3ENRrs> {
        FMCEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ospi1en(&mut self) -> OSPI1EN_W<AHB3ENRrs> {
        OSPI1EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - OSPI2EN memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ospi2en(&mut self) -> OSPI2EN_W<AHB3ENRrs> {
        OSPI2EN_W::new(self, 9)
    }
}
#[doc = "AHB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3enr::R`](R) reader structure"]
impl crate::Readable for AHB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure"]
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
