#[doc = "Register `M1SR` reader"]
pub type R = crate::R<M1SRrs>;
#[doc = "Register `M1SR` writer"]
pub type W = crate::W<M1SRrs>;
#[doc = "Field `ECCSEIE` reader - ECC single error interrupt enable"]
pub type ECCSEIE_R = crate::BitReader;
#[doc = "Field `ECCSEIE` writer - ECC single error interrupt enable"]
pub type ECCSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCDEIE` reader - ECC double error interrupt enable"]
pub type ECCDEIE_R = crate::BitReader;
#[doc = "Field `ECCDEIE` writer - ECC double error interrupt enable"]
pub type ECCDEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCDEBWIE` reader - ECC double error on byte write (BW) interrupt enable"]
pub type ECCDEBWIE_R = crate::BitReader;
#[doc = "Field `ECCDEBWIE` writer - ECC double error on byte write (BW) interrupt enable"]
pub type ECCDEBWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCELEN` reader - ECC error latching enable"]
pub type ECCELEN_R = crate::BitReader;
#[doc = "Field `ECCELEN` writer - ECC error latching enable"]
pub type ECCELEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&self) -> ECCDEIE_R {
        ECCDEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&self) -> ECCDEBWIE_R {
        ECCDEBWIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&self) -> ECCELEN_R {
        ECCELEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccseie(&mut self) -> ECCSEIE_W<M1SRrs> {
        ECCSEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccdeie(&mut self) -> ECCDEIE_W<M1SRrs> {
        ECCDEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccdebwie(&mut self) -> ECCDEBWIE_W<M1SRrs> {
        ECCDEBWIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccelen(&mut self) -> ECCELEN_W<M1SRrs> {
        ECCELEN_W::new(self, 5)
    }
}
#[doc = "RAMECC monitor x status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1SRrs;
impl crate::RegisterSpec for M1SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1sr::R`](R) reader structure"]
impl crate::Readable for M1SRrs {}
#[doc = "`write(|w| ..)` method takes [`m1sr::W`](W) writer structure"]
impl crate::Writable for M1SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1SR to value 0"]
impl crate::Resettable for M1SRrs {
    const RESET_VALUE: u32 = 0;
}
