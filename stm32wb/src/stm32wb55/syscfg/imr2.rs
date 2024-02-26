#[doc = "Register `IMR2` reader"]
pub type R = crate::R<IMR2rs>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<IMR2rs>;
#[doc = "Field `PVM1IM` reader - Peripheral PVM1 interrupt mask to CPU1"]
pub type PVM1IM_R = crate::BitReader;
#[doc = "Field `PVM1IM` writer - Peripheral PVM1 interrupt mask to CPU1"]
pub type PVM1IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVM3IM` reader - Peripheral PVM3 interrupt mask to CPU1"]
pub type PVM3IM_R = crate::BitReader;
#[doc = "Field `PVM3IM` writer - Peripheral PVM3 interrupt mask to CPU1"]
pub type PVM3IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDIM` reader - Peripheral PVD interrupt mask to CPU1"]
pub type PVDIM_R = crate::BitReader;
#[doc = "Field `PVDIM` writer - Peripheral PVD interrupt mask to CPU1"]
pub type PVDIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Peripheral PVM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&self) -> PVM1IM_R {
        PVM1IM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral PVM3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral PVD interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Peripheral PVM1 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pvm1im(&mut self) -> PVM1IM_W<IMR2rs> {
        PVM1IM_W::new(self, 16)
    }
    #[doc = "Bit 18 - Peripheral PVM3 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pvm3im(&mut self) -> PVM3IM_W<IMR2rs> {
        PVM3IM_W::new(self, 18)
    }
    #[doc = "Bit 20 - Peripheral PVD interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pvdim(&mut self) -> PVDIM_W<IMR2rs> {
        PVDIM_W::new(self, 20)
    }
}
#[doc = "CPU1 interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for IMR2rs {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0"]
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0;
}
