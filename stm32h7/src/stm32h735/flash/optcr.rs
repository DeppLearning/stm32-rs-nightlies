#[doc = "Register `OPTCR` reader"]
pub type R = crate::R<OPTCRrs>;
#[doc = "Register `OPTCR` writer"]
pub type W = crate::W<OPTCRrs>;
#[doc = "Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit"]
pub type OPTLOCK_R = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit"]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTART` reader - Option byte start change option configuration bit"]
pub type OPTSTART_R = crate::BitReader;
#[doc = "Field `OPTSTART` writer - Option byte start change option configuration bit"]
pub type OPTSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Flash mass erase enable bit"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Flash mass erase enable bit"]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit"]
pub type OPTCHANGEERRIE_R = crate::BitReader;
#[doc = "Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit"]
pub type OPTCHANGEERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_BANK` reader - Bank swapping configuration bit"]
pub type SWAP_BANK_R = crate::BitReader;
#[doc = "Field `SWAP_BANK` writer - Bank swapping configuration bit"]
pub type SWAP_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&self) -> OPTSTART_R {
        OPTSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn optstart(&mut self) -> OPTSTART_W<OPTCRrs> {
        OPTSTART_W::new(self, 1)
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<OPTCRrs> {
        MER_W::new(self, 4)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W<OPTCRrs> {
        OPTCHANGEERRIE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<OPTCRrs> {
        SWAP_BANK_W::new(self, 31)
    }
}
#[doc = "FLASH option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCRrs;
impl crate::RegisterSpec for OPTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optcr::R`](R) reader structure"]
impl crate::Readable for OPTCRrs {}
#[doc = "`write(|w| ..)` method takes [`optcr::W`](W) writer structure"]
impl crate::Writable for OPTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTCR to value 0"]
impl crate::Resettable for OPTCRrs {
    const RESET_VALUE: u32 = 0;
}
