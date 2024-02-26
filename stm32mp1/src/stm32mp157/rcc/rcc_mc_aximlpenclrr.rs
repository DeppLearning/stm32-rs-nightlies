#[doc = "Register `RCC_MC_AXIMLPENCLRR` reader"]
pub type R = crate::R<RCC_MC_AXIMLPENCLRRrs>;
#[doc = "Register `RCC_MC_AXIMLPENCLRR` writer"]
pub type W = crate::W<RCC_MC_AXIMLPENCLRRrs>;
#[doc = "Field `SYSRAMLPEN` reader - SYSRAMLPEN"]
pub type SYSRAMLPEN_R = crate::BitReader;
#[doc = "Field `SYSRAMLPEN` writer - SYSRAMLPEN"]
pub type SYSRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    pub fn sysramlpen(&self) -> SYSRAMLPEN_R {
        SYSRAMLPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sysramlpen(&mut self) -> SYSRAMLPEN_W<RCC_MC_AXIMLPENCLRRrs> {
        SYSRAMLPEN_W::new(self, 0)
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_aximlpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_aximlpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_AXIMLPENCLRRrs;
impl crate::RegisterSpec for RCC_MC_AXIMLPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_aximlpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_AXIMLPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_aximlpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_AXIMLPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_AXIMLPENCLRR to value 0x01"]
impl crate::Resettable for RCC_MC_AXIMLPENCLRRrs {
    const RESET_VALUE: u32 = 0x01;
}
