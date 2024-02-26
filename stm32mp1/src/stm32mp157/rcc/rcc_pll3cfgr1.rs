#[doc = "Register `RCC_PLL3CFGR1` reader"]
pub type R = crate::R<RCC_PLL3CFGR1rs>;
#[doc = "Register `RCC_PLL3CFGR1` writer"]
pub type W = crate::W<RCC_PLL3CFGR1rs>;
#[doc = "Field `DIVN` reader - DIVN"]
pub type DIVN_R = crate::FieldReader<u16>;
#[doc = "Field `DIVN` writer - DIVN"]
pub type DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIVM3` reader - DIVM3"]
pub type DIVM3_R = crate::FieldReader;
#[doc = "Field `DIVM3` writer - DIVM3"]
pub type DIVM3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IFRGE` reader - IFRGE"]
pub type IFRGE_R = crate::FieldReader;
#[doc = "Field `IFRGE` writer - IFRGE"]
pub type IFRGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - DIVM3"]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    pub fn ifrge(&self) -> IFRGE_R {
        IFRGE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    #[must_use]
    pub fn divn(&mut self) -> DIVN_W<RCC_PLL3CFGR1rs> {
        DIVN_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - DIVM3"]
    #[inline(always)]
    #[must_use]
    pub fn divm3(&mut self) -> DIVM3_W<RCC_PLL3CFGR1rs> {
        DIVM3_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    #[must_use]
    pub fn ifrge(&mut self) -> IFRGE_W<RCC_PLL3CFGR1rs> {
        IFRGE_W::new(self, 24)
    }
}
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PLL3CFGR1rs;
impl crate::RegisterSpec for RCC_PLL3CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pll3cfgr1::R`](R) reader structure"]
impl crate::Readable for RCC_PLL3CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_pll3cfgr1::W`](W) writer structure"]
impl crate::Writable for RCC_PLL3CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PLL3CFGR1 to value 0x0001_0031"]
impl crate::Resettable for RCC_PLL3CFGR1rs {
    const RESET_VALUE: u32 = 0x0001_0031;
}
