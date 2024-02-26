#[doc = "Register `ECCR` reader"]
pub type R = crate::R<ECCRrs>;
#[doc = "Register `ECCR` writer"]
pub type W = crate::W<ECCRrs>;
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type ADDR_ECC_R = crate::FieldReader<u32>;
#[doc = "Field `BK_ECC` reader - BK_ECC"]
pub type BK_ECC_R = crate::BitReader;
#[doc = "Field `SYSF_ECC` reader - SYSF_ECC"]
pub type SYSF_ECC_R = crate::BitReader;
#[doc = "Field `ECCIE` reader - ECC correction interrupt enable"]
pub type ECCIE_R = crate::BitReader;
#[doc = "Field `ECCIE` writer - ECC correction interrupt enable"]
pub type ECCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC2` reader - ECCC2"]
pub type ECCC2_R = crate::BitReader;
#[doc = "Field `ECCC2` writer - ECCC2"]
pub type ECCC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD2` reader - ECCD2"]
pub type ECCD2_R = crate::BitReader;
#[doc = "Field `ECCD2` writer - ECCD2"]
pub type ECCD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC` reader - ECC correction"]
pub type ECCC_R = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction"]
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD` reader - ECC detection"]
pub type ECCD_R = crate::BitReader;
#[doc = "Field `ECCD` writer - ECC detection"]
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 21 - BK_ECC"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SYSF_ECC"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ECCC2"]
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ECCD2"]
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccie(&mut self) -> ECCIE_W<ECCRrs> {
        ECCIE_W::new(self, 24)
    }
    #[doc = "Bit 28 - ECCC2"]
    #[inline(always)]
    #[must_use]
    pub fn eccc2(&mut self) -> ECCC2_W<ECCRrs> {
        ECCC2_W::new(self, 28)
    }
    #[doc = "Bit 29 - ECCD2"]
    #[inline(always)]
    #[must_use]
    pub fn eccd2(&mut self) -> ECCD2_W<ECCRrs> {
        ECCD2_W::new(self, 29)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<ECCRrs> {
        ECCC_W::new(self, 30)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<ECCRrs> {
        ECCD_W::new(self, 31)
    }
}
#[doc = "Flash ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for ECCRrs {}
#[doc = "`write(|w| ..)` method takes [`eccr::W`](W) writer structure"]
impl crate::Writable for ECCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for ECCRrs {
    const RESET_VALUE: u32 = 0;
}
