#[doc = "Register `WRPCR` reader"]
pub type R = crate::R<WRPCRrs>;
#[doc = "Register `WRPCR` writer"]
pub type W = crate::W<WRPCRrs>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDIV` reader - NDIV"]
pub type NDIV_R = crate::FieldReader;
#[doc = "Field `NDIV` writer - NDIV"]
pub type NDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IDF` reader - IDF"]
pub type IDF_R = crate::FieldReader;
#[doc = "Field `IDF` writer - IDF"]
pub type IDF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODF` reader - ODF"]
pub type ODF_R = crate::FieldReader;
#[doc = "Field `ODF` writer - ODF"]
pub type ODF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGEN` reader - REGEN"]
pub type REGEN_R = crate::BitReader;
#[doc = "Field `REGEN` writer - REGEN"]
pub type REGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGREN` reader - BGREN"]
pub type BGREN_R = crate::BitReader;
#[doc = "Field `BGREN` writer - BGREN"]
pub type BGREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BGREN"]
    #[inline(always)]
    pub fn bgren(&self) -> BGREN_R {
        BGREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<WRPCRrs> {
        PLLEN_W::new(self, 0)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NDIV_W<WRPCRrs> {
        NDIV_W::new(self, 2)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    #[must_use]
    pub fn idf(&mut self) -> IDF_W<WRPCRrs> {
        IDF_W::new(self, 11)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> ODF_W<WRPCRrs> {
        ODF_W::new(self, 16)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    #[must_use]
    pub fn regen(&mut self) -> REGEN_W<WRPCRrs> {
        REGEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - BGREN"]
    #[inline(always)]
    #[must_use]
    pub fn bgren(&mut self) -> BGREN_W<WRPCRrs> {
        BGREN_W::new(self, 28)
    }
}
#[doc = "DSI wrapper regulator and PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPCRrs;
impl crate::RegisterSpec for WRPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpcr::R`](R) reader structure"]
impl crate::Readable for WRPCRrs {}
#[doc = "`write(|w| ..)` method takes [`wrpcr::W`](W) writer structure"]
impl crate::Writable for WRPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRPCR to value 0"]
impl crate::Resettable for WRPCRrs {
    const RESET_VALUE: u32 = 0;
}
