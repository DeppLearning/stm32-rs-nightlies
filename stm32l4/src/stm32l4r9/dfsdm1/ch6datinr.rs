#[doc = "Register `CH6DATINR` reader"]
pub type R = crate::R<CH6DATINRrs>;
#[doc = "Register `CH6DATINR` writer"]
pub type W = crate::W<CH6DATINRrs>;
#[doc = "Field `INDAT0` reader - INDAT0"]
pub type INDAT0_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT0` writer - INDAT0"]
pub type INDAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INDAT1` reader - INDAT1"]
pub type INDAT1_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT1` writer - INDAT1"]
pub type INDAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    #[must_use]
    pub fn indat0(&mut self) -> INDAT0_W<CH6DATINRrs> {
        INDAT0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    #[must_use]
    pub fn indat1(&mut self) -> INDAT1_W<CH6DATINRrs> {
        INDAT1_W::new(self, 16)
    }
}
#[doc = "CH6DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6datinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6datinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6DATINRrs;
impl crate::RegisterSpec for CH6DATINRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6datinr::R`](R) reader structure"]
impl crate::Readable for CH6DATINRrs {}
#[doc = "`write(|w| ..)` method takes [`ch6datinr::W`](W) writer structure"]
impl crate::Writable for CH6DATINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6DATINR to value 0"]
impl crate::Resettable for CH6DATINRrs {
    const RESET_VALUE: u32 = 0;
}
