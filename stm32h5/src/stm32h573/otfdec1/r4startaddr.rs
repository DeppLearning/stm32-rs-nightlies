#[doc = "Register `R4STARTADDR` reader"]
pub type R = crate::R<R4STARTADDRrs>;
#[doc = "Register `R4STARTADDR` writer"]
pub type W = crate::W<R4STARTADDRrs>;
#[doc = "Field `REGx_START_ADDR` reader - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
pub type REGX_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `REGx_START_ADDR` writer - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
pub type REGX_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
    #[inline(always)]
    pub fn regx_start_addr(&self) -> REGX_START_ADDR_R {
        REGX_START_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
    #[inline(always)]
    #[must_use]
    pub fn regx_start_addr(&mut self) -> REGX_START_ADDR_W<R4STARTADDRrs> {
        REGX_START_ADDR_W::new(self, 0)
    }
}
#[doc = "OTFDEC region 4 start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4startaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4startaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R4STARTADDRrs;
impl crate::RegisterSpec for R4STARTADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r4startaddr::R`](R) reader structure"]
impl crate::Readable for R4STARTADDRrs {}
#[doc = "`write(|w| ..)` method takes [`r4startaddr::W`](W) writer structure"]
impl crate::Writable for R4STARTADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R4STARTADDR to value 0"]
impl crate::Resettable for R4STARTADDRrs {
    const RESET_VALUE: u32 = 0;
}
