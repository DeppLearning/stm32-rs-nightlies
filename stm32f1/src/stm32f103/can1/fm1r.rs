#[doc = "Register `FM1R` reader"]
pub type R = crate::R<FM1Rrs>;
#[doc = "Register `FM1R` writer"]
pub type W = crate::W<FM1Rrs>;
#[doc = "Field `FBM(0-13)` reader - Filter mode"]
pub type FBM_R = crate::BitReader;
#[doc = "Field `FBM(0-13)` writer - Filter mode"]
pub type FBM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Filter mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FBM0` field"]
    #[inline(always)]
    pub fn fbm(&self, n: u8) -> FBM_R {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FBM_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Filter mode"]
    #[inline(always)]
    pub fn fbm_iter(&self) -> impl Iterator<Item = FBM_R> + '_ {
        (0..14).map(move |n| FBM_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&self) -> FBM_R {
        FBM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Filter mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FBM0` field"]
    #[inline(always)]
    #[must_use]
    pub fn fbm(&mut self, n: u8) -> FBM_W<FM1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FBM_W::new(self, n)
    }
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm0(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm1(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm2(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm3(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm4(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm5(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm6(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm7(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm8(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm9(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm10(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm11(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm12(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm13(&mut self) -> FBM_W<FM1Rrs> {
        FBM_W::new(self, 13)
    }
}
#[doc = "CAN_FM1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM1Rrs;
impl crate::RegisterSpec for FM1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm1r::R`](R) reader structure"]
impl crate::Readable for FM1Rrs {}
#[doc = "`write(|w| ..)` method takes [`fm1r::W`](W) writer structure"]
impl crate::Writable for FM1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM1R to value 0"]
impl crate::Resettable for FM1Rrs {
    const RESET_VALUE: u32 = 0;
}
