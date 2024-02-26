#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMRrs>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMRrs>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRNEIE` reader - Control Buffer Ready Interrupt Enable"]
pub type CSRNEIE_R = crate::BitReader;
#[doc = "Field `CSRNEIE` writer - Control Buffer Ready Interrupt Enable"]
pub type CSRNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PERRIE_R = crate::BitReader;
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - Overrun error Interrupt Enable"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - Overrun error Interrupt Enable"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBLKIE` reader - Synchronization Block Detected Interrupt Enable"]
pub type SBLKIE_R = crate::BitReader;
#[doc = "Field `SBLKIE` writer - Synchronization Block Detected Interrupt Enable"]
pub type SBLKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIE` reader - Synchronization Done"]
pub type SYNCDIE_R = crate::BitReader;
#[doc = "Field `SYNCDIE` writer - Synchronization Done"]
pub type SYNCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFEIE` reader - Serial Interface Error Interrupt Enable"]
pub type IFEIE_R = crate::BitReader;
#[doc = "Field `IFEIE` writer - Serial Interface Error Interrupt Enable"]
pub type IFEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn csrneie(&self) -> CSRNEIE_R {
        CSRNEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error Interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Block Detected Interrupt Enable"]
    #[inline(always)]
    pub fn sblkie(&self) -> SBLKIE_R {
        SBLKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncdie(&self) -> SYNCDIE_R {
        SYNCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Serial Interface Error Interrupt Enable"]
    #[inline(always)]
    pub fn ifeie(&self) -> IFEIE_R {
        IFEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXNE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<IMRrs> {
        RXNEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Control Buffer Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csrneie(&mut self) -> CSRNEIE_W<IMRrs> {
        CSRNEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Parity error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<IMRrs> {
        PERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IMRrs> {
        OVRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Synchronization Block Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sblkie(&mut self) -> SBLKIE_W<IMRrs> {
        SBLKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    #[must_use]
    pub fn syncdie(&mut self) -> SYNCDIE_W<IMRrs> {
        SYNCDIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Serial Interface Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ifeie(&mut self) -> IFEIE_W<IMRrs> {
        IFEIE_W::new(self, 6)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMRrs {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMRrs {
    const RESET_VALUE: u32 = 0;
}
