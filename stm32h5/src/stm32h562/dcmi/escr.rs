#[doc = "Register `ESCR` reader"]
pub type R = crate::R<ESCRrs>;
#[doc = "Register `ESCR` writer"]
pub type W = crate::W<ESCRrs>;
#[doc = "Field `FSC` reader - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
pub type FSC_R = crate::FieldReader;
#[doc = "Field `FSC` writer - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
pub type FSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LSC` reader - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
pub type LSC_R = crate::FieldReader;
#[doc = "Field `LSC` writer - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
pub type LSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LEC` reader - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FEC` reader - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
pub type FEC_R = crate::FieldReader;
#[doc = "Field `FEC` writer - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
pub type FEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
    #[inline(always)]
    #[must_use]
    pub fn fsc(&mut self) -> FSC_W<ESCRrs> {
        FSC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
    #[inline(always)]
    #[must_use]
    pub fn lsc(&mut self) -> LSC_W<ESCRrs> {
        LSC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<ESCRrs> {
        LEC_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<ESCRrs> {
        FEC_W::new(self, 24)
    }
}
#[doc = "DCMI embedded synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESCRrs;
impl crate::RegisterSpec for ESCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`escr::R`](R) reader structure"]
impl crate::Readable for ESCRrs {}
#[doc = "`write(|w| ..)` method takes [`escr::W`](W) writer structure"]
impl crate::Writable for ESCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESCR to value 0"]
impl crate::Resettable for ESCRrs {
    const RESET_VALUE: u32 = 0;
}
