#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTLrs>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTLrs>;
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub type STPPCLK_R = crate::BitReader;
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub type STPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GATEHCLK_R = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSUSP` reader - PHY suspended"]
pub type PHYSUSP_R = crate::BitReader;
#[doc = "Field `PHYSUSP` writer - PHY suspended"]
pub type PHYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> STPPCLK_W<PCGCCTLrs> {
        STPPCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<PCGCCTLrs> {
        GATEHCLK_W::new(self, 1)
    }
    #[doc = "Bit 4 - PHY suspended"]
    #[inline(always)]
    #[must_use]
    pub fn physusp(&mut self) -> PHYSUSP_W<PCGCCTLrs> {
        PHYSUSP_W::new(self, 4)
    }
}
#[doc = "Power and clock gating control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTLrs;
impl crate::RegisterSpec for PCGCCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTLrs {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PCGCCTLrs {
    const RESET_VALUE: u32 = 0;
}
