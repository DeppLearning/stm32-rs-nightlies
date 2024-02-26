#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2rs>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2rs>;
#[doc = "Software interrupt on line x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIER49W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER49W> for bool {
    #[inline(always)]
    fn from(variant: SWIER49W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER49` reader - Software interrupt on line x+32"]
pub type SWIER49_R = crate::BitReader<SWIER49W>;
impl SWIER49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWIER49W> {
        match self.bits {
            true => Some(SWIER49W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER49W::Pend
    }
}
#[doc = "Field `SWIER49` writer - Software interrupt on line x+32"]
pub type SWIER49_W<'a, REG> = crate::BitWriter<'a, REG, SWIER49W>;
impl<'a, REG> SWIER49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWIER49W::Pend)
    }
}
#[doc = "Field `SWIER51` reader - Software interrupt on line x+32"]
pub use SWIER49_R as SWIER51_R;
#[doc = "Field `SWIER51` writer - Software interrupt on line x+32"]
pub use SWIER49_W as SWIER51_W;
impl R {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier49(&self) -> SWIER49_R {
        SWIER49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier51(&self) -> SWIER51_R {
        SWIER51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    #[must_use]
    pub fn swier49(&mut self) -> SWIER49_W<SWIER2rs> {
        SWIER49_W::new(self, 17)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    #[must_use]
    pub fn swier51(&mut self) -> SWIER51_W<SWIER2rs> {
        SWIER51_W::new(self, 19)
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for SWIER2rs {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2rs {
    const RESET_VALUE: u32 = 0;
}
