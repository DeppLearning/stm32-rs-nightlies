#[doc = "Register `IER0` reader"]
pub type R = crate::R<IER0rs>;
#[doc = "Register `IER0` writer"]
pub type W = crate::W<IER0rs>;
#[doc = "Field `AEIE(0-15)` reader - Acknowledge Error %s Interrupt Enable"]
pub type AEIE_R = crate::BitReader;
#[doc = "Field `AEIE(0-15)` writer - Acknowledge Error %s Interrupt Enable"]
pub type AEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE(0-4)` reader - PHY Error %s Interrupt Enable"]
pub type PEIE_R = crate::BitReader;
#[doc = "Field `PEIE(0-4)` writer - PHY Error %s Interrupt Enable"]
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Acknowledge Error (0-15) Interrupt Enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AE0IE` field"]
    #[inline(always)]
    pub fn aeie(&self, n: u8) -> AEIE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AEIE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Acknowledge Error (0-15) Interrupt Enable"]
    #[inline(always)]
    pub fn aeie_iter(&self) -> impl Iterator<Item = AEIE_R> + '_ {
        (0..16).map(move |n| AEIE_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Acknowledge Error 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ae0ie(&self) -> AEIE_R {
        AEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ae1ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ae2ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Error 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ae3ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge Error 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ae4ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Acknowledge Error 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ae5ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Error 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ae6ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledge Error 7 Interrupt Enable"]
    #[inline(always)]
    pub fn ae7ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Acknowledge Error 8 Interrupt Enable"]
    #[inline(always)]
    pub fn ae8ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Acknowledge Error 9 Interrupt Enable"]
    #[inline(always)]
    pub fn ae9ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge Error 10 Interrupt Enable"]
    #[inline(always)]
    pub fn ae10ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge Error 11 Interrupt Enable"]
    #[inline(always)]
    pub fn ae11ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Acknowledge Error 12 Interrupt Enable"]
    #[inline(always)]
    pub fn ae12ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error 13 Interrupt Enable"]
    #[inline(always)]
    pub fn ae13ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Acknowledge Error 14 Interrupt Enable"]
    #[inline(always)]
    pub fn ae14ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Acknowledge Error 15 Interrupt Enable"]
    #[inline(always)]
    pub fn ae15ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "PHY Error (0-4) Interrupt Enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `PE0IE` field"]
    #[inline(always)]
    pub fn peie(&self, n: u8) -> PEIE_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PEIE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "PHY Error (0-4) Interrupt Enable"]
    #[inline(always)]
    pub fn peie_iter(&self) -> impl Iterator<Item = PEIE_R> + '_ {
        (0..5).map(move |n| PEIE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - PHY Error 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pe0ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pe1ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PHY Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pe2ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PHY Error 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pe3ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PHY Error 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pe4ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Acknowledge Error (0-15) Interrupt Enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AE0IE` field"]
    #[inline(always)]
    #[must_use]
    pub fn aeie(&mut self, n: u8) -> AEIE_W<IER0rs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AEIE_W::new(self, n)
    }
    #[doc = "Bit 0 - Acknowledge Error 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae0ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Acknowledge Error 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae1ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Acknowledge Error 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae2ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Acknowledge Error 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae3ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledge Error 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae4ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Acknowledge Error 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae5ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Acknowledge Error 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae6ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Acknowledge Error 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae7ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Acknowledge Error 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae8ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Acknowledge Error 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae9ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge Error 10 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae10ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Acknowledge Error 11 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae11ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Acknowledge Error 12 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae12ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Acknowledge Error 13 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae13ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Acknowledge Error 14 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae14ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Acknowledge Error 15 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae15ie(&mut self) -> AEIE_W<IER0rs> {
        AEIE_W::new(self, 15)
    }
    #[doc = "PHY Error (0-4) Interrupt Enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `PE0IE` field"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self, n: u8) -> PEIE_W<IER0rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PEIE_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - PHY Error 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe0ie(&mut self) -> PEIE_W<IER0rs> {
        PEIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - PHY Error 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe1ie(&mut self) -> PEIE_W<IER0rs> {
        PEIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - PHY Error 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe2ie(&mut self) -> PEIE_W<IER0rs> {
        PEIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - PHY Error 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe3ie(&mut self) -> PEIE_W<IER0rs> {
        PEIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - PHY Error 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe4ie(&mut self) -> PEIE_W<IER0rs> {
        PEIE_W::new(self, 20)
    }
}
#[doc = "DSI Host Interrupt Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER0rs;
impl crate::RegisterSpec for IER0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier0::R`](R) reader structure"]
impl crate::Readable for IER0rs {}
#[doc = "`write(|w| ..)` method takes [`ier0::W`](W) writer structure"]
impl crate::Writable for IER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER0 to value 0"]
impl crate::Resettable for IER0rs {
    const RESET_VALUE: u32 = 0;
}
