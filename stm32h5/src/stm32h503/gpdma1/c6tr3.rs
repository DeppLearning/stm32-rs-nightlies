#[doc = "Register `C6TR3` reader"]
pub type R = crate::R<C6TR3rs>;
#[doc = "Register `C6TR3` writer"]
pub type W = crate::W<C6TR3rs>;
#[doc = "Field `SAO` reader - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\\[12:0\\]
for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
pub type SAO_R = crate::FieldReader<u16>;
#[doc = "Field `SAO` writer - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\\[12:0\\]
for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
pub type SAO_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DAO` reader - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\\[12:0\\]
for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type DAO_R = crate::FieldReader<u16>;
#[doc = "Field `DAO` writer - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\\[12:0\\]
for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type DAO_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\\[12:0\\]
for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\\[12:0\\]
for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn dao(&self) -> DAO_R {
        DAO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\\[12:0\\]
for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
    #[inline(always)]
    #[must_use]
    pub fn sao(&mut self) -> SAO_W<C6TR3rs> {
        SAO_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\\[12:0\\]
for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn dao(&mut self) -> DAO_W<C6TR3rs> {
        DAO_W::new(self, 16)
    }
}
#[doc = "GPDMA channel 6 transfer register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6tr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6tr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6TR3rs;
impl crate::RegisterSpec for C6TR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6tr3::R`](R) reader structure"]
impl crate::Readable for C6TR3rs {}
#[doc = "`write(|w| ..)` method takes [`c6tr3::W`](W) writer structure"]
impl crate::Writable for C6TR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C6TR3 to value 0"]
impl crate::Resettable for C6TR3rs {
    const RESET_VALUE: u32 = 0;
}
