#[doc = "Register `PDCRH` reader"]
pub type R = crate::R<PDCRHrs>;
#[doc = "Register `PDCRH` writer"]
pub type W = crate::W<PDCRHrs>;
#[doc = "Port H pull-down bit y (y=0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0 {
    #[doc = "0: Pull-Down on Pxx is disabled"]
    Disabled = 0,
    #[doc = "1: Pull-Down on Pxx is enabled"]
    Enabled = 1,
}
impl From<PD0> for bool {
    #[inline(always)]
    fn from(variant: PD0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD0` reader - Port H pull-down bit y (y=0..15)"]
pub type PD0_R = crate::BitReader<PD0>;
impl PD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD0 {
        match self.bits {
            false => PD0::Disabled,
            true => PD0::Enabled,
        }
    }
    #[doc = "Pull-Down on Pxx is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD0::Disabled
    }
    #[doc = "Pull-Down on Pxx is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD0::Enabled
    }
}
#[doc = "Field `PD0` writer - Port H pull-down bit y (y=0..15)"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG, PD0>;
impl<'a, REG> PD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-Down on Pxx is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD0::Disabled)
    }
    #[doc = "Pull-Down on Pxx is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD0::Enabled)
    }
}
#[doc = "Field `PD1` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD1_R;
#[doc = "Field `PD2` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD2_R;
#[doc = "Field `PD3` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD3_R;
#[doc = "Field `PD4` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD4_R;
#[doc = "Field `PD5` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD5_R;
#[doc = "Field `PD6` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD6_R;
#[doc = "Field `PD7` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD7_R;
#[doc = "Field `PD8` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD8_R;
#[doc = "Field `PD9` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD9_R;
#[doc = "Field `PD10` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD10_R;
#[doc = "Field `PD11` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD11_R;
#[doc = "Field `PD12` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD12_R;
#[doc = "Field `PD13` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD13_R;
#[doc = "Field `PD14` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD14_R;
#[doc = "Field `PD15` reader - Port H pull-down bit y (y=0..15)"]
pub use PD0_R as PD15_R;
#[doc = "Field `PD1` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD1_W;
#[doc = "Field `PD2` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD2_W;
#[doc = "Field `PD3` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD3_W;
#[doc = "Field `PD4` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD4_W;
#[doc = "Field `PD5` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD5_W;
#[doc = "Field `PD6` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD6_W;
#[doc = "Field `PD7` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD7_W;
#[doc = "Field `PD8` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD8_W;
#[doc = "Field `PD9` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD9_W;
#[doc = "Field `PD10` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD10_W;
#[doc = "Field `PD11` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD11_W;
#[doc = "Field `PD12` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD12_W;
#[doc = "Field `PD13` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD13_W;
#[doc = "Field `PD14` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD14_W;
#[doc = "Field `PD15` writer - Port H pull-down bit y (y=0..15)"]
pub use PD0_W as PD15_W;
impl R {
    #[doc = "Bit 0 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDCRHrs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDCRHrs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PDCRHrs> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PDCRHrs> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PDCRHrs> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PDCRHrs> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PDCRHrs> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PDCRHrs> {
        PD7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<PDCRHrs> {
        PD8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<PDCRHrs> {
        PD9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<PDCRHrs> {
        PD10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<PDCRHrs> {
        PD11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<PDCRHrs> {
        PD12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<PDCRHrs> {
        PD13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<PDCRHrs> {
        PD14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port H pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<PDCRHrs> {
        PD15_W::new(self, 15)
    }
}
#[doc = "Power Port H pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRHrs;
impl crate::RegisterSpec for PDCRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrh::R`](R) reader structure"]
impl crate::Readable for PDCRHrs {}
#[doc = "`write(|w| ..)` method takes [`pdcrh::W`](W) writer structure"]
impl crate::Writable for PDCRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRH to value 0"]
impl crate::Resettable for PDCRHrs {
    const RESET_VALUE: u32 = 0;
}
