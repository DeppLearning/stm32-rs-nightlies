#[doc = "Register `PUCRH` reader"]
pub type R = crate::R<PUCRHrs>;
#[doc = "Register `PUCRH` writer"]
pub type W = crate::W<PUCRHrs>;
#[doc = "Field `PU0` reader - Port G pull-up bit y (y=0..15)"]
pub type PU0_R = crate::BitReader;
#[doc = "Field `PU0` writer - Port G pull-up bit y (y=0..15)"]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port G pull-up bit y (y=0..15)"]
pub type PU1_R = crate::BitReader;
#[doc = "Field `PU1` writer - Port G pull-up bit y (y=0..15)"]
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - Port G pull-up bit y (y=0..15)"]
pub type PU2_R = crate::BitReader;
#[doc = "Field `PU2` writer - Port G pull-up bit y (y=0..15)"]
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU3` reader - Port G pull-up bit y (y=0..15)"]
pub type PU3_R = crate::BitReader;
#[doc = "Field `PU3` writer - Port G pull-up bit y (y=0..15)"]
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU4` reader - Port G pull-up bit y (y=0..15)"]
pub type PU4_R = crate::BitReader;
#[doc = "Field `PU4` writer - Port G pull-up bit y (y=0..15)"]
pub type PU4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU5` reader - Port G pull-up bit y (y=0..15)"]
pub type PU5_R = crate::BitReader;
#[doc = "Field `PU5` writer - Port G pull-up bit y (y=0..15)"]
pub type PU5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU6` reader - Port G pull-up bit y (y=0..15)"]
pub type PU6_R = crate::BitReader;
#[doc = "Field `PU6` writer - Port G pull-up bit y (y=0..15)"]
pub type PU6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU7` reader - Port G pull-up bit y (y=0..15)"]
pub type PU7_R = crate::BitReader;
#[doc = "Field `PU7` writer - Port G pull-up bit y (y=0..15)"]
pub type PU7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU8` reader - Port G pull-up bit y (y=0..15)"]
pub type PU8_R = crate::BitReader;
#[doc = "Field `PU8` writer - Port G pull-up bit y (y=0..15)"]
pub type PU8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU9` reader - Port G pull-up bit y (y=0..15)"]
pub type PU9_R = crate::BitReader;
#[doc = "Field `PU9` writer - Port G pull-up bit y (y=0..15)"]
pub type PU9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU10` reader - Port G pull-up bit y (y=0..15)"]
pub type PU10_R = crate::BitReader;
#[doc = "Field `PU10` writer - Port G pull-up bit y (y=0..15)"]
pub type PU10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU11` reader - Port G pull-up bit y (y=0..15)"]
pub type PU11_R = crate::BitReader;
#[doc = "Field `PU11` writer - Port G pull-up bit y (y=0..15)"]
pub type PU11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU12` reader - Port G pull-up bit y (y=0..15)"]
pub type PU12_R = crate::BitReader;
#[doc = "Field `PU12` writer - Port G pull-up bit y (y=0..15)"]
pub type PU12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU13` reader - Port G pull-up bit y (y=0..15)"]
pub type PU13_R = crate::BitReader;
#[doc = "Field `PU13` writer - Port G pull-up bit y (y=0..15)"]
pub type PU13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU14` reader - Port G pull-up bit y (y=0..15)"]
pub type PU14_R = crate::BitReader;
#[doc = "Field `PU14` writer - Port G pull-up bit y (y=0..15)"]
pub type PU14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU15` reader - Port G pull-up bit y (y=0..15)"]
pub type PU15_R = crate::BitReader;
#[doc = "Field `PU15` writer - Port G pull-up bit y (y=0..15)"]
pub type PU15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PUCRHrs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PUCRHrs> {
        PU1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PUCRHrs> {
        PU2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PUCRHrs> {
        PU3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<PUCRHrs> {
        PU4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<PUCRHrs> {
        PU5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<PUCRHrs> {
        PU6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<PUCRHrs> {
        PU7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> PU8_W<PUCRHrs> {
        PU8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> PU9_W<PUCRHrs> {
        PU9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu10(&mut self) -> PU10_W<PUCRHrs> {
        PU10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu11(&mut self) -> PU11_W<PUCRHrs> {
        PU11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu12(&mut self) -> PU12_W<PUCRHrs> {
        PU12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu13(&mut self) -> PU13_W<PUCRHrs> {
        PU13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu14(&mut self) -> PU14_W<PUCRHrs> {
        PU14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu15(&mut self) -> PU15_W<PUCRHrs> {
        PU15_W::new(self, 15)
    }
}
#[doc = "Power Port H pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRHrs;
impl crate::RegisterSpec for PUCRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrh::R`](R) reader structure"]
impl crate::Readable for PUCRHrs {}
#[doc = "`write(|w| ..)` method takes [`pucrh::W`](W) writer structure"]
impl crate::Writable for PUCRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRH to value 0"]
impl crate::Resettable for PUCRHrs {
    const RESET_VALUE: u32 = 0;
}
