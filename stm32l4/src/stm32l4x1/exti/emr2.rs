#[doc = "Register `EMR2` reader"]
pub type R = crate::R<EMR2rs>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<EMR2rs>;
#[doc = "Event mask on external/internal line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR32 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR32> for bool {
    #[inline(always)]
    fn from(variant: MR32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR32` reader - Event mask on external/internal line 32"]
pub type MR32_R = crate::BitReader<MR32>;
impl MR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR32 {
        match self.bits {
            false => MR32::Masked,
            true => MR32::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR32::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR32::Unmasked
    }
}
#[doc = "Field `MR32` writer - Event mask on external/internal line 32"]
pub type MR32_W<'a, REG> = crate::BitWriter<'a, REG, MR32>;
impl<'a, REG> MR32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MR32::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(MR32::Unmasked)
    }
}
#[doc = "Field `MR33` reader - Event mask on external/internal line 33"]
pub use MR32_R as MR33_R;
#[doc = "Field `MR34` reader - Event mask on external/internal line 34"]
pub use MR32_R as MR34_R;
#[doc = "Field `MR35` reader - Event mask on external/internal line 35"]
pub use MR32_R as MR35_R;
#[doc = "Field `MR36` reader - Event mask on external/internal line 36"]
pub use MR32_R as MR36_R;
#[doc = "Field `MR37` reader - Event mask on external/internal line 37"]
pub use MR32_R as MR37_R;
#[doc = "Field `MR38` reader - Event mask on external/internal line 38"]
pub use MR32_R as MR38_R;
#[doc = "Field `MR39` reader - Event mask on external/internal line 39"]
pub use MR32_R as MR39_R;
#[doc = "Field `MR33` writer - Event mask on external/internal line 33"]
pub use MR32_W as MR33_W;
#[doc = "Field `MR34` writer - Event mask on external/internal line 34"]
pub use MR32_W as MR34_W;
#[doc = "Field `MR35` writer - Event mask on external/internal line 35"]
pub use MR32_W as MR35_W;
#[doc = "Field `MR36` writer - Event mask on external/internal line 36"]
pub use MR32_W as MR36_W;
#[doc = "Field `MR37` writer - Event mask on external/internal line 37"]
pub use MR32_W as MR37_W;
#[doc = "Field `MR38` writer - Event mask on external/internal line 38"]
pub use MR32_W as MR38_W;
#[doc = "Field `MR39` writer - Event mask on external/internal line 39"]
pub use MR32_W as MR39_W;
impl R {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event mask on external/internal line 38"]
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event mask on external/internal line 39"]
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<EMR2rs> {
        MR32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<EMR2rs> {
        MR33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<EMR2rs> {
        MR34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<EMR2rs> {
        MR35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    #[must_use]
    pub fn mr36(&mut self) -> MR36_W<EMR2rs> {
        MR36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    #[must_use]
    pub fn mr37(&mut self) -> MR37_W<EMR2rs> {
        MR37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Event mask on external/internal line 38"]
    #[inline(always)]
    #[must_use]
    pub fn mr38(&mut self) -> MR38_W<EMR2rs> {
        MR38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Event mask on external/internal line 39"]
    #[inline(always)]
    #[must_use]
    pub fn mr39(&mut self) -> MR39_W<EMR2rs> {
        MR39_W::new(self, 7)
    }
}
#[doc = "Event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for EMR2rs {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2rs {
    const RESET_VALUE: u32 = 0;
}
