#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "Field `CGIF(1-8)` writer - Channel %s Global interrupt clear"]
pub type CGIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF(1-8)` writer - Channel %s Transfer Complete clear"]
pub type CTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF(1-8)` writer - Channel %s Half Transfer clear"]
pub type CHTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF(1-8)` writer - Channel %s Transfer Error clear"]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Channel (1-8) Global interrupt clear"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CGIF1` field"]
    #[inline(always)]
    #[must_use]
    pub fn cgif(&mut self, n: u8) -> CGIF_W<IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CGIF_W::new(self, n * 4)
    }
    #[doc = "Bit 0 - Channel 1 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 0)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 4)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 8)
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 12)
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 16)
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif6(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 20)
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif7(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 24)
    }
    #[doc = "Bit 28 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif8(&mut self) -> CGIF_W<IFCRrs> {
        CGIF_W::new(self, 28)
    }
    #[doc = "Channel (1-8) Transfer Complete clear"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CTCIF1` field"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif(&mut self, n: u8) -> CTCIF_W<IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CTCIF_W::new(self, n * 4 + 1)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 1)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 5)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 9)
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 13)
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 17)
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 21)
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 25)
    }
    #[doc = "Bit 29 - Channel 8 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif8(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 29)
    }
    #[doc = "Channel (1-8) Half Transfer clear"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CHTIF1` field"]
    #[inline(always)]
    #[must_use]
    pub fn chtif(&mut self, n: u8) -> CHTIF_W<IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CHTIF_W::new(self, n * 4 + 2)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 2)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 6)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 10)
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 14)
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 18)
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 22)
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 26)
    }
    #[doc = "Bit 30 - Channel 8 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif8(&mut self) -> CHTIF_W<IFCRrs> {
        CHTIF_W::new(self, 30)
    }
    #[doc = "Channel (1-8) Transfer Error clear"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CTEIF1` field"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self, n: u8) -> CTEIF_W<IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CTEIF_W::new(self, n * 4 + 3)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 3)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 7)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 11)
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 15)
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 19)
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 23)
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 27)
    }
    #[doc = "Bit 31 - Channel 8 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif8(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 31)
    }
}
#[doc = "DMA interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
