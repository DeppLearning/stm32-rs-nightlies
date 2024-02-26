#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "Field `CGIF1` writer - global interrupt flag clear for channel 1"]
pub type CGIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - transfer complete flag clear for channel 1"]
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - half transfer flag clear for channel 1"]
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - transfer error flag clear for channel 1"]
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` writer - global interrupt flag clear for channel 2"]
pub type CGIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - transfer complete flag clear for channel 2"]
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - half transfer flag clear for channel 2"]
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - transfer error flag clear for channel 2"]
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` writer - global interrupt flag clear for channel 3"]
pub type CGIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - transfer complete flag clear for channel 3"]
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - half transfer flag clear for channel 3"]
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - transfer error flag clear for channel 3"]
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - global interrupt flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<IFCRrs> {
        CGIF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - transfer complete flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<IFCRrs> {
        CTCIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - half transfer flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<IFCRrs> {
        CHTIF1_W::new(self, 2)
    }
    #[doc = "Bit 3 - transfer error flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<IFCRrs> {
        CTEIF1_W::new(self, 3)
    }
    #[doc = "Bit 4 - global interrupt flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<IFCRrs> {
        CGIF2_W::new(self, 4)
    }
    #[doc = "Bit 5 - transfer complete flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<IFCRrs> {
        CTCIF2_W::new(self, 5)
    }
    #[doc = "Bit 6 - half transfer flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<IFCRrs> {
        CHTIF2_W::new(self, 6)
    }
    #[doc = "Bit 7 - transfer error flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<IFCRrs> {
        CTEIF2_W::new(self, 7)
    }
    #[doc = "Bit 8 - global interrupt flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<IFCRrs> {
        CGIF3_W::new(self, 8)
    }
    #[doc = "Bit 9 - transfer complete flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<IFCRrs> {
        CTCIF3_W::new(self, 9)
    }
    #[doc = "Bit 10 - half transfer flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<IFCRrs> {
        CHTIF3_W::new(self, 10)
    }
    #[doc = "Bit 11 - transfer error flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<IFCRrs> {
        CTEIF3_W::new(self, 11)
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
