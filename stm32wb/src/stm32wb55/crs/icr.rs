#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `SYNCOKC` reader - SYNC event OK clear flag"]
pub type SYNCOKC_R = crate::BitReader;
#[doc = "Field `SYNCOKC` writer - SYNC event OK clear flag"]
pub type SYNCOKC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCWARNC` reader - warning clear flag"]
pub type SYNCWARNC_R = crate::BitReader;
#[doc = "Field `SYNCWARNC` writer - warning clear flag"]
pub type SYNCWARNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` reader - Error clear flag"]
pub type ERRC_R = crate::BitReader;
#[doc = "Field `ERRC` writer - Error clear flag"]
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESYNCC` reader - Expected SYNC clear flag"]
pub type ESYNCC_R = crate::BitReader;
#[doc = "Field `ESYNCC` writer - Expected SYNC clear flag"]
pub type ESYNCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYNC event OK clear flag"]
    #[inline(always)]
    pub fn syncokc(&self) -> SYNCOKC_R {
        SYNCOKC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - warning clear flag"]
    #[inline(always)]
    pub fn syncwarnc(&self) -> SYNCWARNC_R {
        SYNCWARNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error clear flag"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag"]
    #[inline(always)]
    pub fn esyncc(&self) -> ESYNCC_R {
        ESYNCC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncokc(&mut self) -> SYNCOKC_W<ICRrs> {
        SYNCOKC_W::new(self, 0)
    }
    #[doc = "Bit 1 - warning clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncwarnc(&mut self) -> SYNCWARNC_W<ICRrs> {
        SYNCWARNC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<ICRrs> {
        ERRC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn esyncc(&mut self) -> ESYNCC_W<ICRrs> {
        ESYNCC_W::new(self, 3)
    }
}
#[doc = "CRS interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
