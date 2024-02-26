#[doc = "Register `PRIVCR` reader"]
pub type R = crate::R<PRIVCRrs>;
#[doc = "Register `PRIVCR` writer"]
pub type W = crate::W<PRIVCRrs>;
#[doc = "Field `ALRAPRIV` reader - ALRAPRIV"]
pub type ALRAPRIV_R = crate::BitReader;
#[doc = "Field `ALRAPRIV` writer - ALRAPRIV"]
pub type ALRAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBPRIV` reader - ALRBPRIV"]
pub type ALRBPRIV_R = crate::BitReader;
#[doc = "Field `ALRBPRIV` writer - ALRBPRIV"]
pub type ALRBPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTPRIV` reader - WUTPRIV"]
pub type WUTPRIV_R = crate::BitReader;
#[doc = "Field `WUTPRIV` writer - WUTPRIV"]
pub type WUTPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSPRIV` reader - TSPRIV"]
pub type TSPRIV_R = crate::BitReader;
#[doc = "Field `TSPRIV` writer - TSPRIV"]
pub type TSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALPRIV` reader - CALPRIV"]
pub type CALPRIV_R = crate::BitReader;
#[doc = "Field `CALPRIV` writer - CALPRIV"]
pub type CALPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITPRIV` reader - INITPRIV"]
pub type INITPRIV_R = crate::BitReader;
#[doc = "Field `INITPRIV` writer - INITPRIV"]
pub type INITPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV` reader - PRIV"]
pub type PRIV_R = crate::BitReader;
#[doc = "Field `PRIV` writer - PRIV"]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W<PRIVCRrs> {
        ALRAPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W<PRIVCRrs> {
        ALRBPRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn wutpriv(&mut self) -> WUTPRIV_W<PRIVCRrs> {
        WUTPRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tspriv(&mut self) -> TSPRIV_W<PRIVCRrs> {
        TSPRIV_W::new(self, 3)
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn calpriv(&mut self) -> CALPRIV_W<PRIVCRrs> {
        CALPRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn initpriv(&mut self) -> INITPRIV_W<PRIVCRrs> {
        INITPRIV_W::new(self, 14)
    }
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<PRIVCRrs> {
        PRIV_W::new(self, 15)
    }
}
#[doc = "RTC privilege mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCRrs;
impl crate::RegisterSpec for PRIVCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcr::R`](R) reader structure"]
impl crate::Readable for PRIVCRrs {}
#[doc = "`write(|w| ..)` method takes [`privcr::W`](W) writer structure"]
impl crate::Writable for PRIVCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCR to value 0"]
impl crate::Resettable for PRIVCRrs {
    const RESET_VALUE: u32 = 0;
}
