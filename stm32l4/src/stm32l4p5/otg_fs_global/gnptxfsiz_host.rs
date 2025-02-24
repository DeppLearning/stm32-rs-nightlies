#[doc = "Register `GNPTXFSIZ_Host` reader"]
pub type R = crate::R<GNPTXFSIZ_HOSTrs>;
#[doc = "Register `GNPTXFSIZ_Host` writer"]
pub type W = crate::W<GNPTXFSIZ_HOSTrs>;
#[doc = "Field `NPTXFSA` reader - Non-periodic transmit RAM start address"]
pub type NPTXFSA_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSA` writer - Non-periodic transmit RAM start address"]
pub type NPTXFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFD` reader - Non-periodic TxFIFO depth"]
pub type NPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFD` writer - Non-periodic TxFIFO depth"]
pub type NPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<GNPTXFSIZ_HOSTrs> {
        NPTXFSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfd(&mut self) -> NPTXFD_W<GNPTXFSIZ_HOSTrs> {
        NPTXFD_W::new(self, 16)
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_host::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_host::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_HOSTrs;
impl crate::RegisterSpec for GNPTXFSIZ_HOSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz_host::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_HOSTrs {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz_host::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_HOSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ_Host to value 0x0200"]
impl crate::Resettable for GNPTXFSIZ_HOSTrs {
    const RESET_VALUE: u32 = 0x0200;
}
