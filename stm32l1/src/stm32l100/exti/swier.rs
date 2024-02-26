#[doc = "Register `SWIER` reader"]
pub type R = crate::R<SWIERrs>;
#[doc = "Register `SWIER` writer"]
pub type W = crate::W<SWIERrs>;
#[doc = "Software interrupt on line x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIER0W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER0W> for bool {
    #[inline(always)]
    fn from(variant: SWIER0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER0` reader - Software interrupt on line x"]
pub type SWIER0_R = crate::BitReader<SWIER0W>;
impl SWIER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWIER0W> {
        match self.bits {
            true => Some(SWIER0W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER0W::Pend
    }
}
#[doc = "Field `SWIER0` writer - Software interrupt on line x"]
pub type SWIER0_W<'a, REG> = crate::BitWriter<'a, REG, SWIER0W>;
impl<'a, REG> SWIER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWIER0W::Pend)
    }
}
#[doc = "Field `SWIER1` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER1_R;
#[doc = "Field `SWIER2` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER2_R;
#[doc = "Field `SWIER3` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER3_R;
#[doc = "Field `SWIER4` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER4_R;
#[doc = "Field `SWIER5` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER5_R;
#[doc = "Field `SWIER6` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER6_R;
#[doc = "Field `SWIER7` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER7_R;
#[doc = "Field `SWIER8` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER8_R;
#[doc = "Field `SWIER9` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER9_R;
#[doc = "Field `SWIER10` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER10_R;
#[doc = "Field `SWIER11` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER11_R;
#[doc = "Field `SWIER12` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER12_R;
#[doc = "Field `SWIER13` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER13_R;
#[doc = "Field `SWIER14` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER14_R;
#[doc = "Field `SWIER15` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER15_R;
#[doc = "Field `SWIER16` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER16_R;
#[doc = "Field `SWIER17` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER17_R;
#[doc = "Field `SWIER18` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER18_R;
#[doc = "Field `SWIER19` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER19_R;
#[doc = "Field `SWIER20` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER20_R;
#[doc = "Field `SWIER21` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER21_R;
#[doc = "Field `SWIER22` reader - Software interrupt on line x"]
pub use SWIER0_R as SWIER22_R;
#[doc = "Field `SWIER1` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER1_W;
#[doc = "Field `SWIER2` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER2_W;
#[doc = "Field `SWIER3` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER3_W;
#[doc = "Field `SWIER4` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER4_W;
#[doc = "Field `SWIER5` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER5_W;
#[doc = "Field `SWIER6` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER6_W;
#[doc = "Field `SWIER7` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER7_W;
#[doc = "Field `SWIER8` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER8_W;
#[doc = "Field `SWIER9` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER9_W;
#[doc = "Field `SWIER10` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER10_W;
#[doc = "Field `SWIER11` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER11_W;
#[doc = "Field `SWIER12` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER12_W;
#[doc = "Field `SWIER13` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER13_W;
#[doc = "Field `SWIER14` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER14_W;
#[doc = "Field `SWIER15` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER15_W;
#[doc = "Field `SWIER16` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER16_W;
#[doc = "Field `SWIER17` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER17_W;
#[doc = "Field `SWIER18` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER18_W;
#[doc = "Field `SWIER19` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER19_W;
#[doc = "Field `SWIER20` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER20_W;
#[doc = "Field `SWIER21` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER21_W;
#[doc = "Field `SWIER22` writer - Software interrupt on line x"]
pub use SWIER0_W as SWIER22_W;
impl R {
    #[doc = "Bit 0 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier17(&self) -> SWIER17_R {
        SWIER17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier18(&self) -> SWIER18_R {
        SWIER18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier19(&self) -> SWIER19_R {
        SWIER19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier20(&self) -> SWIER20_R {
        SWIER20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier21(&self) -> SWIER21_R {
        SWIER21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swier22(&self) -> SWIER22_R {
        SWIER22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier0(&mut self) -> SWIER0_W<SWIERrs> {
        SWIER0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier1(&mut self) -> SWIER1_W<SWIERrs> {
        SWIER1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier2(&mut self) -> SWIER2_W<SWIERrs> {
        SWIER2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier3(&mut self) -> SWIER3_W<SWIERrs> {
        SWIER3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier4(&mut self) -> SWIER4_W<SWIERrs> {
        SWIER4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier5(&mut self) -> SWIER5_W<SWIERrs> {
        SWIER5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier6(&mut self) -> SWIER6_W<SWIERrs> {
        SWIER6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier7(&mut self) -> SWIER7_W<SWIERrs> {
        SWIER7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier8(&mut self) -> SWIER8_W<SWIERrs> {
        SWIER8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier9(&mut self) -> SWIER9_W<SWIERrs> {
        SWIER9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier10(&mut self) -> SWIER10_W<SWIERrs> {
        SWIER10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier11(&mut self) -> SWIER11_W<SWIERrs> {
        SWIER11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier12(&mut self) -> SWIER12_W<SWIERrs> {
        SWIER12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier13(&mut self) -> SWIER13_W<SWIERrs> {
        SWIER13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier14(&mut self) -> SWIER14_W<SWIERrs> {
        SWIER14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier15(&mut self) -> SWIER15_W<SWIERrs> {
        SWIER15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier16(&mut self) -> SWIER16_W<SWIERrs> {
        SWIER16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier17(&mut self) -> SWIER17_W<SWIERrs> {
        SWIER17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier18(&mut self) -> SWIER18_W<SWIERrs> {
        SWIER18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier19(&mut self) -> SWIER19_W<SWIERrs> {
        SWIER19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier20(&mut self) -> SWIER20_W<SWIERrs> {
        SWIER20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier21(&mut self) -> SWIER21_W<SWIERrs> {
        SWIER21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swier22(&mut self) -> SWIER22_W<SWIERrs> {
        SWIER22_W::new(self, 22)
    }
}
#[doc = "SWIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIERrs;
impl crate::RegisterSpec for SWIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier::R`](R) reader structure"]
impl crate::Readable for SWIERrs {}
#[doc = "`write(|w| ..)` method takes [`swier::W`](W) writer structure"]
impl crate::Writable for SWIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SWIERrs {
    const RESET_VALUE: u32 = 0;
}
