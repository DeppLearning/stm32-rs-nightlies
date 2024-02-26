#[doc = "Register `DMA_S0PAR` reader"]
pub type R = crate::R<DMA_S0PARrs>;
#[doc = "Register `DMA_S0PAR` writer"]
pub type W = crate::W<DMA_S0PARrs>;
#[doc = "Field `PAR` reader - PAR"]
pub type PAR_R = crate::FieldReader<u32>;
#[doc = "Field `PAR` writer - PAR"]
pub type PAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAR"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAR"]
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> PAR_W<DMA_S0PARrs> {
        PAR_W::new(self, 0)
    }
}
#[doc = "DMA stream 0 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s0par::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s0par::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_S0PARrs;
impl crate::RegisterSpec for DMA_S0PARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_s0par::R`](R) reader structure"]
impl crate::Readable for DMA_S0PARrs {}
#[doc = "`write(|w| ..)` method takes [`dma_s0par::W`](W) writer structure"]
impl crate::Writable for DMA_S0PARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_S0PAR to value 0"]
impl crate::Resettable for DMA_S0PARrs {
    const RESET_VALUE: u32 = 0;
}
